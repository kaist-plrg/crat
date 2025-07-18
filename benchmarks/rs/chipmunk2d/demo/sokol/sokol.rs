use ::libc;
extern "C" {
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    pub type __GLXFBConfig;
    pub type __GLXcontext;
    fn glBlendFuncSeparate(
        sfactorRGB: GLenum,
        dfactorRGB: GLenum,
        sfactorAlpha: GLenum,
        dfactorAlpha: GLenum,
    );
    fn glBindBuffer(target: GLenum, buffer: GLuint);
    fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint);
    fn glGenBuffers(n: GLsizei, buffers: *mut GLuint);
    fn glBufferData(
        target: GLenum,
        size: GLsizeiptr,
        data: *const libc::c_void,
        usage: GLenum,
    );
    fn glBufferSubData(
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const libc::c_void,
    );
    fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum);
    fn glDrawBuffers(n: GLsizei, bufs: *const GLenum);
    fn glStencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
    fn glStencilFuncSeparate(face: GLenum, func: GLenum, ref_0: GLint, mask: GLuint);
    fn glAttachShader(program: GLuint, shader: GLuint);
    fn glCompileShader(shader: GLuint);
    fn glCreateProgram() -> GLuint;
    fn glCreateShader(type_0: GLenum) -> GLuint;
    fn glDeleteProgram(program: GLuint);
    fn glDeleteShader(shader: GLuint);
    fn glDisableVertexAttribArray(index: GLuint);
    fn glEnableVertexAttribArray(index: GLuint);
    fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint;
    fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
    fn glGetProgramInfoLog(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );
    fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
    fn glGetShaderInfoLog(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );
    fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
    fn glLinkProgram(program: GLuint);
    fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    );
    fn glUseProgram(program: GLuint);
    fn glUniform1i(location: GLint, v0: GLint);
    fn glUniform1fv(location: GLint, count: GLsizei, value: *const GLfloat);
    fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat);
    fn glUniform3fv(location: GLint, count: GLsizei, value: *const GLfloat);
    fn glUniform4fv(location: GLint, count: GLsizei, value: *const GLfloat);
    fn glUniformMatrix4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    fn glVertexAttribPointer(
        index: GLuint,
        size: GLint,
        type_0: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const libc::c_void,
    );
    fn glClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
    fn glClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
    fn glClearBufferfi(
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    );
    fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte;
    fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint);
    fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint);
    fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint);
    fn glRenderbufferStorage(
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
    fn glBindFramebuffer(target: GLenum, framebuffer: GLuint);
    fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint);
    fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint);
    fn glCheckFramebufferStatus(target: GLenum) -> GLenum;
    fn glFramebufferTexture2D(
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    );
    fn glFramebufferRenderbuffer(
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    );
    fn glBlitFramebuffer(
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    );
    fn glRenderbufferStorageMultisample(
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
    fn glFramebufferTextureLayer(
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    );
    fn glBindVertexArray(array: GLuint);
    fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint);
    fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint);
    fn glDrawArraysInstanced(
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
    );
    fn glDrawElementsInstanced(
        mode: GLenum,
        count: GLsizei,
        type_0: GLenum,
        indices: *const libc::c_void,
        instancecount: GLsizei,
    );
    fn glVertexAttribDivisor(index: GLuint, divisor: GLuint);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sokol_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> sapp_desc;
    fn XkbSetDetectableAutoRepeat(
        _: *mut Display,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn XrmDestroyDatabase(_: XrmDatabase);
    fn XrmGetResource(
        _: XrmDatabase,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: *mut XrmValue,
    ) -> libc::c_int;
    fn XrmGetStringDatabase(_: *const libc::c_char) -> XrmDatabase;
    fn XAllocSizeHints() -> *mut XSizeHints;
    fn XLookupString(
        _: *mut XKeyEvent,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut KeySym,
        _: *mut XComposeStatus,
    ) -> libc::c_int;
    fn XSetWMNormalHints(_: *mut Display, _: Window, _: *mut XSizeHints);
    fn Xutf8SetWMProperties(
        _: *mut Display,
        _: Window,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut XSizeHints,
        _: *mut XWMHints,
        _: *mut XClassHint,
    );
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn glClear(mask: GLbitfield);
    fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
    fn glCullFace(mode: GLenum);
    fn glFrontFace(mode: GLenum);
    fn glPolygonOffset(factor: GLfloat, units: GLfloat);
    fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn glReadBuffer(mode: GLenum);
    fn glEnable(cap: GLenum);
    fn glDisable(cap: GLenum);
    fn glGetIntegerv(pname: GLenum, params: *mut GLint);
    fn glClearDepth(depth: GLclampd);
    fn glDepthFunc(func: GLenum);
    fn glDepthMask(flag: GLboolean);
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);
    fn glCompressedTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const libc::c_void,
    );
    fn glCompressedTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const libc::c_void,
    );
    fn glActiveTexture(texture: GLenum);
    fn glBlendColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn glTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_0: GLenum,
        pixels: *const libc::c_void,
    );
    fn glTexImage3D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_0: GLenum,
        pixels: *const libc::c_void,
    );
    fn glTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_0: GLenum,
        pixels: *const libc::c_void,
    );
    fn glBindTexture(target: GLenum, texture: GLuint);
    fn glDeleteTextures(n: GLsizei, textures: *const GLuint);
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
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
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    fn glClearStencil(s: GLint);
    fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum);
    fn glStencilMask(mask: GLuint);
    fn glStencilFunc(func: GLenum, ref_0: GLint, mask: GLuint);
    fn glDrawElements(
        mode: GLenum,
        count: GLsizei,
        type_0: GLenum,
        indices: *const libc::c_void,
    );
    fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    fn XrmInitialize();
    fn XInternAtom(_: *mut Display, _: *const libc::c_char, _: libc::c_int) -> Atom;
    fn XCreateColormap(
        _: *mut Display,
        _: Window,
        _: *mut Visual,
        _: libc::c_int,
    ) -> Colormap;
    fn XCreateWindow(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_uint,
        _: *mut Visual,
        _: libc::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> Window;
    fn XGetKeyboardMapping(
        _: *mut Display,
        _: KeyCode,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> *mut KeySym;
    fn XResourceManagerString(_: *mut Display) -> *mut libc::c_char;
    fn XInitThreads() -> libc::c_int;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn XFlush(_: *mut Display) -> libc::c_int;
    fn XFree(_: *mut libc::c_void) -> libc::c_int;
    fn XFreeColormap(_: *mut Display, _: Colormap) -> libc::c_int;
    fn XGetWindowProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: libc::c_long,
        _: libc::c_long,
        _: libc::c_int,
        _: Atom,
        _: *mut Atom,
        _: *mut libc::c_int,
        _: *mut libc::c_ulong,
        _: *mut libc::c_ulong,
        _: *mut *mut libc::c_uchar,
    ) -> libc::c_int;
    fn XGetWindowAttributes(
        _: *mut Display,
        _: Window,
        _: *mut XWindowAttributes,
    ) -> libc::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XNextEvent(_: *mut Display, _: *mut XEvent) -> libc::c_int;
    fn XPending(_: *mut Display) -> libc::c_int;
    fn XRaiseWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XSync(_: *mut Display, _: libc::c_int) -> libc::c_int;
    fn XDestroyWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XCloseDisplay(_: *mut Display) -> libc::c_int;
    fn XChangeProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: Atom,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_uchar,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XSetWMProtocols(
        _: *mut Display,
        _: Window,
        _: *mut Atom,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XSetErrorHandler(_: XErrorHandler) -> XErrorHandler;
    fn XUnmapWindow(_: *mut Display, _: Window) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type sapp_event_type = libc::c_uint;
pub const _SAPP_EVENTTYPE_FORCE_U32: sapp_event_type = 134217727;
pub const _SAPP_EVENTTYPE_NUM: sapp_event_type = 20;
pub const SAPP_EVENTTYPE_UPDATE_CURSOR: sapp_event_type = 19;
pub const SAPP_EVENTTYPE_RESUMED: sapp_event_type = 18;
pub const SAPP_EVENTTYPE_SUSPENDED: sapp_event_type = 17;
pub const SAPP_EVENTTYPE_RESTORED: sapp_event_type = 16;
pub const SAPP_EVENTTYPE_ICONIFIED: sapp_event_type = 15;
pub const SAPP_EVENTTYPE_RESIZED: sapp_event_type = 14;
pub const SAPP_EVENTTYPE_TOUCHES_CANCELLED: sapp_event_type = 13;
pub const SAPP_EVENTTYPE_TOUCHES_ENDED: sapp_event_type = 12;
pub const SAPP_EVENTTYPE_TOUCHES_MOVED: sapp_event_type = 11;
pub const SAPP_EVENTTYPE_TOUCHES_BEGAN: sapp_event_type = 10;
pub const SAPP_EVENTTYPE_MOUSE_LEAVE: sapp_event_type = 9;
pub const SAPP_EVENTTYPE_MOUSE_ENTER: sapp_event_type = 8;
pub const SAPP_EVENTTYPE_MOUSE_MOVE: sapp_event_type = 7;
pub const SAPP_EVENTTYPE_MOUSE_SCROLL: sapp_event_type = 6;
pub const SAPP_EVENTTYPE_MOUSE_UP: sapp_event_type = 5;
pub const SAPP_EVENTTYPE_MOUSE_DOWN: sapp_event_type = 4;
pub const SAPP_EVENTTYPE_CHAR: sapp_event_type = 3;
pub const SAPP_EVENTTYPE_KEY_UP: sapp_event_type = 2;
pub const SAPP_EVENTTYPE_KEY_DOWN: sapp_event_type = 1;
pub const SAPP_EVENTTYPE_INVALID: sapp_event_type = 0;
pub type sapp_keycode = libc::c_uint;
pub const SAPP_KEYCODE_MENU: sapp_keycode = 348;
pub const SAPP_KEYCODE_RIGHT_SUPER: sapp_keycode = 347;
pub const SAPP_KEYCODE_RIGHT_ALT: sapp_keycode = 346;
pub const SAPP_KEYCODE_RIGHT_CONTROL: sapp_keycode = 345;
pub const SAPP_KEYCODE_RIGHT_SHIFT: sapp_keycode = 344;
pub const SAPP_KEYCODE_LEFT_SUPER: sapp_keycode = 343;
pub const SAPP_KEYCODE_LEFT_ALT: sapp_keycode = 342;
pub const SAPP_KEYCODE_LEFT_CONTROL: sapp_keycode = 341;
pub const SAPP_KEYCODE_LEFT_SHIFT: sapp_keycode = 340;
pub const SAPP_KEYCODE_KP_EQUAL: sapp_keycode = 336;
pub const SAPP_KEYCODE_KP_ENTER: sapp_keycode = 335;
pub const SAPP_KEYCODE_KP_ADD: sapp_keycode = 334;
pub const SAPP_KEYCODE_KP_SUBTRACT: sapp_keycode = 333;
pub const SAPP_KEYCODE_KP_MULTIPLY: sapp_keycode = 332;
pub const SAPP_KEYCODE_KP_DIVIDE: sapp_keycode = 331;
pub const SAPP_KEYCODE_KP_DECIMAL: sapp_keycode = 330;
pub const SAPP_KEYCODE_KP_9: sapp_keycode = 329;
pub const SAPP_KEYCODE_KP_8: sapp_keycode = 328;
pub const SAPP_KEYCODE_KP_7: sapp_keycode = 327;
pub const SAPP_KEYCODE_KP_6: sapp_keycode = 326;
pub const SAPP_KEYCODE_KP_5: sapp_keycode = 325;
pub const SAPP_KEYCODE_KP_4: sapp_keycode = 324;
pub const SAPP_KEYCODE_KP_3: sapp_keycode = 323;
pub const SAPP_KEYCODE_KP_2: sapp_keycode = 322;
pub const SAPP_KEYCODE_KP_1: sapp_keycode = 321;
pub const SAPP_KEYCODE_KP_0: sapp_keycode = 320;
pub const SAPP_KEYCODE_F25: sapp_keycode = 314;
pub const SAPP_KEYCODE_F24: sapp_keycode = 313;
pub const SAPP_KEYCODE_F23: sapp_keycode = 312;
pub const SAPP_KEYCODE_F22: sapp_keycode = 311;
pub const SAPP_KEYCODE_F21: sapp_keycode = 310;
pub const SAPP_KEYCODE_F20: sapp_keycode = 309;
pub const SAPP_KEYCODE_F19: sapp_keycode = 308;
pub const SAPP_KEYCODE_F18: sapp_keycode = 307;
pub const SAPP_KEYCODE_F17: sapp_keycode = 306;
pub const SAPP_KEYCODE_F16: sapp_keycode = 305;
pub const SAPP_KEYCODE_F15: sapp_keycode = 304;
pub const SAPP_KEYCODE_F14: sapp_keycode = 303;
pub const SAPP_KEYCODE_F13: sapp_keycode = 302;
pub const SAPP_KEYCODE_F12: sapp_keycode = 301;
pub const SAPP_KEYCODE_F11: sapp_keycode = 300;
pub const SAPP_KEYCODE_F10: sapp_keycode = 299;
pub const SAPP_KEYCODE_F9: sapp_keycode = 298;
pub const SAPP_KEYCODE_F8: sapp_keycode = 297;
pub const SAPP_KEYCODE_F7: sapp_keycode = 296;
pub const SAPP_KEYCODE_F6: sapp_keycode = 295;
pub const SAPP_KEYCODE_F5: sapp_keycode = 294;
pub const SAPP_KEYCODE_F4: sapp_keycode = 293;
pub const SAPP_KEYCODE_F3: sapp_keycode = 292;
pub const SAPP_KEYCODE_F2: sapp_keycode = 291;
pub const SAPP_KEYCODE_F1: sapp_keycode = 290;
pub const SAPP_KEYCODE_PAUSE: sapp_keycode = 284;
pub const SAPP_KEYCODE_PRINT_SCREEN: sapp_keycode = 283;
pub const SAPP_KEYCODE_NUM_LOCK: sapp_keycode = 282;
pub const SAPP_KEYCODE_SCROLL_LOCK: sapp_keycode = 281;
pub const SAPP_KEYCODE_CAPS_LOCK: sapp_keycode = 280;
pub const SAPP_KEYCODE_END: sapp_keycode = 269;
pub const SAPP_KEYCODE_HOME: sapp_keycode = 268;
pub const SAPP_KEYCODE_PAGE_DOWN: sapp_keycode = 267;
pub const SAPP_KEYCODE_PAGE_UP: sapp_keycode = 266;
pub const SAPP_KEYCODE_UP: sapp_keycode = 265;
pub const SAPP_KEYCODE_DOWN: sapp_keycode = 264;
pub const SAPP_KEYCODE_LEFT: sapp_keycode = 263;
pub const SAPP_KEYCODE_RIGHT: sapp_keycode = 262;
pub const SAPP_KEYCODE_DELETE: sapp_keycode = 261;
pub const SAPP_KEYCODE_INSERT: sapp_keycode = 260;
pub const SAPP_KEYCODE_BACKSPACE: sapp_keycode = 259;
pub const SAPP_KEYCODE_TAB: sapp_keycode = 258;
pub const SAPP_KEYCODE_ENTER: sapp_keycode = 257;
pub const SAPP_KEYCODE_ESCAPE: sapp_keycode = 256;
pub const SAPP_KEYCODE_WORLD_2: sapp_keycode = 162;
pub const SAPP_KEYCODE_WORLD_1: sapp_keycode = 161;
pub const SAPP_KEYCODE_GRAVE_ACCENT: sapp_keycode = 96;
pub const SAPP_KEYCODE_RIGHT_BRACKET: sapp_keycode = 93;
pub const SAPP_KEYCODE_BACKSLASH: sapp_keycode = 92;
pub const SAPP_KEYCODE_LEFT_BRACKET: sapp_keycode = 91;
pub const SAPP_KEYCODE_Z: sapp_keycode = 90;
pub const SAPP_KEYCODE_Y: sapp_keycode = 89;
pub const SAPP_KEYCODE_X: sapp_keycode = 88;
pub const SAPP_KEYCODE_W: sapp_keycode = 87;
pub const SAPP_KEYCODE_V: sapp_keycode = 86;
pub const SAPP_KEYCODE_U: sapp_keycode = 85;
pub const SAPP_KEYCODE_T: sapp_keycode = 84;
pub const SAPP_KEYCODE_S: sapp_keycode = 83;
pub const SAPP_KEYCODE_R: sapp_keycode = 82;
pub const SAPP_KEYCODE_Q: sapp_keycode = 81;
pub const SAPP_KEYCODE_P: sapp_keycode = 80;
pub const SAPP_KEYCODE_O: sapp_keycode = 79;
pub const SAPP_KEYCODE_N: sapp_keycode = 78;
pub const SAPP_KEYCODE_M: sapp_keycode = 77;
pub const SAPP_KEYCODE_L: sapp_keycode = 76;
pub const SAPP_KEYCODE_K: sapp_keycode = 75;
pub const SAPP_KEYCODE_J: sapp_keycode = 74;
pub const SAPP_KEYCODE_I: sapp_keycode = 73;
pub const SAPP_KEYCODE_H: sapp_keycode = 72;
pub const SAPP_KEYCODE_G: sapp_keycode = 71;
pub const SAPP_KEYCODE_F: sapp_keycode = 70;
pub const SAPP_KEYCODE_E: sapp_keycode = 69;
pub const SAPP_KEYCODE_D: sapp_keycode = 68;
pub const SAPP_KEYCODE_C: sapp_keycode = 67;
pub const SAPP_KEYCODE_B: sapp_keycode = 66;
pub const SAPP_KEYCODE_A: sapp_keycode = 65;
pub const SAPP_KEYCODE_EQUAL: sapp_keycode = 61;
pub const SAPP_KEYCODE_SEMICOLON: sapp_keycode = 59;
pub const SAPP_KEYCODE_9: sapp_keycode = 57;
pub const SAPP_KEYCODE_8: sapp_keycode = 56;
pub const SAPP_KEYCODE_7: sapp_keycode = 55;
pub const SAPP_KEYCODE_6: sapp_keycode = 54;
pub const SAPP_KEYCODE_5: sapp_keycode = 53;
pub const SAPP_KEYCODE_4: sapp_keycode = 52;
pub const SAPP_KEYCODE_3: sapp_keycode = 51;
pub const SAPP_KEYCODE_2: sapp_keycode = 50;
pub const SAPP_KEYCODE_1: sapp_keycode = 49;
pub const SAPP_KEYCODE_0: sapp_keycode = 48;
pub const SAPP_KEYCODE_SLASH: sapp_keycode = 47;
pub const SAPP_KEYCODE_PERIOD: sapp_keycode = 46;
pub const SAPP_KEYCODE_MINUS: sapp_keycode = 45;
pub const SAPP_KEYCODE_COMMA: sapp_keycode = 44;
pub const SAPP_KEYCODE_APOSTROPHE: sapp_keycode = 39;
pub const SAPP_KEYCODE_SPACE: sapp_keycode = 32;
pub const SAPP_KEYCODE_INVALID: sapp_keycode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sapp_touchpoint {
    pub identifier: uintptr_t,
    pub pos_x: libc::c_float,
    pub pos_y: libc::c_float,
    pub changed: bool,
}
pub type sapp_mousebutton = libc::c_int;
pub const SAPP_MOUSEBUTTON_MIDDLE: sapp_mousebutton = 2;
pub const SAPP_MOUSEBUTTON_RIGHT: sapp_mousebutton = 1;
pub const SAPP_MOUSEBUTTON_LEFT: sapp_mousebutton = 0;
pub const SAPP_MOUSEBUTTON_INVALID: sapp_mousebutton = -1;
pub type C2RustUnnamed = libc::c_uint;
pub const SAPP_MODIFIER_SUPER: C2RustUnnamed = 8;
pub const SAPP_MODIFIER_ALT: C2RustUnnamed = 4;
pub const SAPP_MODIFIER_CTRL: C2RustUnnamed = 2;
pub const SAPP_MODIFIER_SHIFT: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sapp_event {
    pub type_0: sapp_event_type,
    pub frame_count: uint32_t,
    pub key_code: sapp_keycode,
    pub char_code: uint32_t,
    pub key_repeat: bool,
    pub modifiers: uint32_t,
    pub mouse_button: sapp_mousebutton,
    pub mouse_x: libc::c_float,
    pub mouse_y: libc::c_float,
    pub scroll_x: libc::c_float,
    pub scroll_y: libc::c_float,
    pub num_touches: libc::c_int,
    pub touches: [sapp_touchpoint; 8],
    pub window_width: libc::c_int,
    pub window_height: libc::c_int,
    pub framebuffer_width: libc::c_int,
    pub framebuffer_height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sapp_desc {
    pub init_cb: Option::<unsafe extern "C" fn() -> ()>,
    pub frame_cb: Option::<unsafe extern "C" fn() -> ()>,
    pub cleanup_cb: Option::<unsafe extern "C" fn() -> ()>,
    pub event_cb: Option::<unsafe extern "C" fn(*const sapp_event) -> ()>,
    pub fail_cb: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    pub user_data: *mut libc::c_void,
    pub init_userdata_cb: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub frame_userdata_cb: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub cleanup_userdata_cb: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub event_userdata_cb: Option::<
        unsafe extern "C" fn(*const sapp_event, *mut libc::c_void) -> (),
    >,
    pub fail_userdata_cb: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
    >,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub sample_count: libc::c_int,
    pub swap_interval: libc::c_int,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub alpha: bool,
    pub window_title: *const libc::c_char,
    pub user_cursor: bool,
    pub html5_canvas_name: *const libc::c_char,
    pub html5_canvas_resize: bool,
    pub html5_preserve_drawing_buffer: bool,
    pub html5_premultiplied_alpha: bool,
    pub ios_keyboard_resizes_canvas: bool,
    pub gl_force_gles2: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sapp_state {
    pub valid: bool,
    pub window_width: libc::c_int,
    pub window_height: libc::c_int,
    pub framebuffer_width: libc::c_int,
    pub framebuffer_height: libc::c_int,
    pub sample_count: libc::c_int,
    pub swap_interval: libc::c_int,
    pub dpi_scale: libc::c_float,
    pub gles2_fallback: bool,
    pub first_frame: bool,
    pub init_called: bool,
    pub cleanup_called: bool,
    pub html5_canvas_resize: bool,
    pub html5_canvas_name: *const libc::c_char,
    pub window_title: [libc::c_char; 128],
    pub window_title_wide: [wchar_t; 128],
    pub frame_count: uint32_t,
    pub mouse_x: libc::c_float,
    pub mouse_y: libc::c_float,
    pub win32_mouse_tracked: bool,
    pub onscreen_keyboard_shown: bool,
    pub event: sapp_event,
    pub desc: sapp_desc,
    pub keycodes: [sapp_keycode; 512],
}
pub type wchar_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sapp_gl_fbconfig {
    pub red_bits: libc::c_int,
    pub green_bits: libc::c_int,
    pub blue_bits: libc::c_int,
    pub alpha_bits: libc::c_int,
    pub depth_bits: libc::c_int,
    pub stencil_bits: libc::c_int,
    pub samples: libc::c_int,
    pub doublebuffer: bool,
    pub handle: uintptr_t,
}
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type KeySym = XID;
pub type KeyCode = libc::c_uchar;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option::<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: libc::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: libc::c_ulong,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWindowAttributes {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub depth: libc::c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: libc::c_int,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub colormap: Colormap,
    pub map_installed: libc::c_int,
    pub map_state: libc::c_int,
    pub all_event_masks: libc::c_long,
    pub your_event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub screen: *mut Screen,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option::<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option::<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
pub type XEvent = _XEvent;
pub type XErrorHandler = Option::<
    unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XrmValue {
    pub size: libc::c_uint,
    pub addr: XPointer,
}
pub type XrmDatabase = *mut _XrmHashBucketRec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSizeHints {
    pub flags: libc::c_long,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub min_width: libc::c_int,
    pub min_height: libc::c_int,
    pub max_width: libc::c_int,
    pub max_height: libc::c_int,
    pub width_inc: libc::c_int,
    pub height_inc: libc::c_int,
    pub min_aspect: C2RustUnnamed_2,
    pub max_aspect: C2RustUnnamed_2,
    pub base_width: libc::c_int,
    pub base_height: libc::c_int,
    pub win_gravity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWMHints {
    pub flags: libc::c_long,
    pub input: libc::c_int,
    pub initial_state: libc::c_int,
    pub icon_pixmap: Pixmap,
    pub icon_window: Window,
    pub icon_x: libc::c_int,
    pub icon_y: libc::c_int,
    pub icon_mask: Pixmap,
    pub window_group: XID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClassHint {
    pub res_name: *mut libc::c_char,
    pub res_class: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XComposeStatus {
    pub compose_ptr: XPointer,
    pub chars_matched: libc::c_int,
}
pub type XComposeStatus = _XComposeStatus;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisualInfo {
    pub visual: *mut Visual,
    pub visualid: VisualID,
    pub screen: libc::c_int,
    pub depth: libc::c_int,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub colormap_size: libc::c_int,
    pub bits_per_rgb: libc::c_int,
}
pub type CARD32 = libc::c_uint;
pub type GLenum = libc::c_uint;
pub type GLboolean = libc::c_uchar;
pub type GLbitfield = libc::c_uint;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLubyte = libc::c_uchar;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLclampd = libc::c_double;
pub type khronos_intptr_t = libc::c_long;
pub type khronos_ssize_t = libc::c_long;
pub type GLsizeiptr = khronos_ssize_t;
pub type GLintptr = khronos_intptr_t;
pub type GLchar = libc::c_char;
pub type GLXWindow = XID;
pub type GLXDrawable = XID;
pub type GLXFBConfig = *mut __GLXFBConfig;
pub type GLXContext = *mut __GLXcontext;
pub type __GLXextproc = Option::<unsafe extern "C" fn() -> ()>;
pub type PFNGLXGETFBCONFIGATTRIBPROC = Option::<
    unsafe extern "C" fn(
        *mut Display,
        GLXFBConfig,
        libc::c_int,
        *mut libc::c_int,
    ) -> libc::c_int,
>;
pub type PFNGLXGETCLIENTSTRINGPROC = Option::<
    unsafe extern "C" fn(*mut Display, libc::c_int) -> *const libc::c_char,
>;
pub type PFNGLXQUERYEXTENSIONPROC = Option::<
    unsafe extern "C" fn(*mut Display, *mut libc::c_int, *mut libc::c_int) -> libc::c_int,
>;
pub type PFNGLXQUERYVERSIONPROC = Option::<
    unsafe extern "C" fn(*mut Display, *mut libc::c_int, *mut libc::c_int) -> libc::c_int,
>;
pub type PFNGLXDESTROYCONTEXTPROC = Option::<
    unsafe extern "C" fn(*mut Display, GLXContext) -> (),
>;
pub type PFNGLXMAKECURRENTPROC = Option::<
    unsafe extern "C" fn(*mut Display, GLXDrawable, GLXContext) -> libc::c_int,
>;
pub type PFNGLXSWAPBUFFERSPROC = Option::<
    unsafe extern "C" fn(*mut Display, GLXDrawable) -> (),
>;
pub type PFNGLXQUERYEXTENSIONSSTRINGPROC = Option::<
    unsafe extern "C" fn(*mut Display, libc::c_int) -> *const libc::c_char,
>;
pub type PFNGLXGETFBCONFIGSPROC = Option::<
    unsafe extern "C" fn(*mut Display, libc::c_int, *mut libc::c_int) -> *mut GLXFBConfig,
>;
pub type PFNGLXCREATENEWCONTEXTPROC = Option::<
    unsafe extern "C" fn(
        *mut Display,
        GLXFBConfig,
        libc::c_int,
        GLXContext,
        libc::c_int,
    ) -> GLXContext,
>;
pub type PFNGLXGETPROCADDRESSPROC = Option::<
    unsafe extern "C" fn(*const GLubyte) -> __GLXextproc,
>;
pub type PFNGLXSWAPINTERVALEXTPROC = Option::<
    unsafe extern "C" fn(*mut Display, GLXDrawable, libc::c_int) -> (),
>;
pub type PFNGLXGETVISUALFROMFBCONFIGPROC = Option::<
    unsafe extern "C" fn(*mut Display, GLXFBConfig) -> *mut XVisualInfo,
>;
pub type PFNGLXCREATEWINDOWPROC = Option::<
    unsafe extern "C" fn(
        *mut Display,
        GLXFBConfig,
        Window,
        *const libc::c_int,
    ) -> GLXWindow,
>;
pub type PFNGLXDESTROYWINDOWPROC = Option::<
    unsafe extern "C" fn(*mut Display, GLXWindow) -> (),
>;
pub type PFNGLXSWAPINTERVALMESAPROC = Option::<
    unsafe extern "C" fn(libc::c_int) -> libc::c_int,
>;
pub type PFNGLXCREATECONTEXTATTRIBSARBPROC = Option::<
    unsafe extern "C" fn(
        *mut Display,
        GLXFBConfig,
        GLXContext,
        libc::c_int,
        *const libc::c_int,
    ) -> GLXContext,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sapp_x11_codepair {
    pub keysym: uint16_t,
    pub ucs: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub state: CARD32,
    pub icon: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _stm_state_t {
    pub initialized: uint32_t,
    pub start: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_buffer {
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_image {
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader {
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_pipeline {
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_pass {
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_context {
    pub id: uint32_t,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const SG_MAX_TEXTUREARRAY_LAYERS: C2RustUnnamed_4 = 128;
pub const SG_MAX_MIPMAPS: C2RustUnnamed_4 = 16;
pub const SG_MAX_VERTEX_ATTRIBUTES: C2RustUnnamed_4 = 16;
pub const SG_MAX_UB_MEMBERS: C2RustUnnamed_4 = 16;
pub const SG_MAX_SHADERSTAGE_UBS: C2RustUnnamed_4 = 4;
pub const SG_MAX_SHADERSTAGE_IMAGES: C2RustUnnamed_4 = 12;
pub const SG_MAX_SHADERSTAGE_BUFFERS: C2RustUnnamed_4 = 4;
pub const SG_MAX_COLOR_ATTACHMENTS: C2RustUnnamed_4 = 4;
pub const SG_NUM_INFLIGHT_FRAMES: C2RustUnnamed_4 = 2;
pub const SG_NUM_SHADER_STAGES: C2RustUnnamed_4 = 2;
pub const SG_INVALID_ID: C2RustUnnamed_4 = 0;
pub type sg_backend = libc::c_uint;
pub const SG_BACKEND_DUMMY: sg_backend = 7;
pub const SG_BACKEND_METAL_SIMULATOR: sg_backend = 6;
pub const SG_BACKEND_METAL_MACOS: sg_backend = 5;
pub const SG_BACKEND_METAL_IOS: sg_backend = 4;
pub const SG_BACKEND_D3D11: sg_backend = 3;
pub const SG_BACKEND_GLES3: sg_backend = 2;
pub const SG_BACKEND_GLES2: sg_backend = 1;
pub const SG_BACKEND_GLCORE33: sg_backend = 0;
pub type sg_feature = libc::c_uint;
pub const SG_NUM_FEATURES: sg_feature = 14;
pub const SG_FEATURE_IMAGETYPE_ARRAY: sg_feature = 13;
pub const SG_FEATURE_IMAGETYPE_3D: sg_feature = 12;
pub const SG_FEATURE_MULTIPLE_RENDER_TARGET: sg_feature = 11;
pub const SG_FEATURE_PACKED_VERTEX_FORMAT_10_2: sg_feature = 10;
pub const SG_FEATURE_MSAA_RENDER_TARGETS: sg_feature = 9;
pub const SG_FEATURE_ORIGIN_TOP_LEFT: sg_feature = 8;
pub const SG_FEATURE_ORIGIN_BOTTOM_LEFT: sg_feature = 7;
pub const SG_FEATURE_TEXTURE_HALF_FLOAT: sg_feature = 6;
pub const SG_FEATURE_TEXTURE_FLOAT: sg_feature = 5;
pub const SG_FEATURE_TEXTURE_COMPRESSION_ETC2: sg_feature = 4;
pub const SG_FEATURE_TEXTURE_COMPRESSION_ATC: sg_feature = 3;
pub const SG_FEATURE_TEXTURE_COMPRESSION_PVRTC: sg_feature = 2;
pub const SG_FEATURE_TEXTURE_COMPRESSION_DXT: sg_feature = 1;
pub const SG_FEATURE_INSTANCING: sg_feature = 0;
pub type sg_resource_state = libc::c_uint;
pub const _SG_RESOURCESTATE_FORCE_U32: sg_resource_state = 2147483647;
pub const SG_RESOURCESTATE_INVALID: sg_resource_state = 4;
pub const SG_RESOURCESTATE_FAILED: sg_resource_state = 3;
pub const SG_RESOURCESTATE_VALID: sg_resource_state = 2;
pub const SG_RESOURCESTATE_ALLOC: sg_resource_state = 1;
pub const SG_RESOURCESTATE_INITIAL: sg_resource_state = 0;
pub type sg_usage = libc::c_uint;
pub const _SG_USAGE_FORCE_U32: sg_usage = 2147483647;
pub const _SG_USAGE_NUM: sg_usage = 4;
pub const SG_USAGE_STREAM: sg_usage = 3;
pub const SG_USAGE_DYNAMIC: sg_usage = 2;
pub const SG_USAGE_IMMUTABLE: sg_usage = 1;
pub const _SG_USAGE_DEFAULT: sg_usage = 0;
pub type sg_buffer_type = libc::c_uint;
pub const _SG_BUFFERTYPE_FORCE_U32: sg_buffer_type = 2147483647;
pub const _SG_BUFFERTYPE_NUM: sg_buffer_type = 3;
pub const SG_BUFFERTYPE_INDEXBUFFER: sg_buffer_type = 2;
pub const SG_BUFFERTYPE_VERTEXBUFFER: sg_buffer_type = 1;
pub const _SG_BUFFERTYPE_DEFAULT: sg_buffer_type = 0;
pub type sg_index_type = libc::c_uint;
pub const _SG_INDEXTYPE_FORCE_U32: sg_index_type = 2147483647;
pub const _SG_INDEXTYPE_NUM: sg_index_type = 4;
pub const SG_INDEXTYPE_UINT32: sg_index_type = 3;
pub const SG_INDEXTYPE_UINT16: sg_index_type = 2;
pub const SG_INDEXTYPE_NONE: sg_index_type = 1;
pub const _SG_INDEXTYPE_DEFAULT: sg_index_type = 0;
pub type sg_image_type = libc::c_uint;
pub const _SG_IMAGETYPE_FORCE_U32: sg_image_type = 2147483647;
pub const _SG_IMAGETYPE_NUM: sg_image_type = 5;
pub const SG_IMAGETYPE_ARRAY: sg_image_type = 4;
pub const SG_IMAGETYPE_3D: sg_image_type = 3;
pub const SG_IMAGETYPE_CUBE: sg_image_type = 2;
pub const SG_IMAGETYPE_2D: sg_image_type = 1;
pub const _SG_IMAGETYPE_DEFAULT: sg_image_type = 0;
pub type sg_shader_stage = libc::c_uint;
pub const _SG_SHADERSTAGE_FORCE_U32: sg_shader_stage = 2147483647;
pub const SG_SHADERSTAGE_FS: sg_shader_stage = 1;
pub const SG_SHADERSTAGE_VS: sg_shader_stage = 0;
pub type sg_pixel_format = libc::c_uint;
pub const _SG_PIXELFORMAT_FORCE_U32: sg_pixel_format = 2147483647;
pub const _SG_PIXELFORMAT_NUM: sg_pixel_format = 24;
pub const SG_PIXELFORMAT_ETC2_SRGB8: sg_pixel_format = 23;
pub const SG_PIXELFORMAT_ETC2_RGB8: sg_pixel_format = 22;
pub const SG_PIXELFORMAT_PVRTC4_RGBA: sg_pixel_format = 21;
pub const SG_PIXELFORMAT_PVRTC2_RGBA: sg_pixel_format = 20;
pub const SG_PIXELFORMAT_PVRTC4_RGB: sg_pixel_format = 19;
pub const SG_PIXELFORMAT_PVRTC2_RGB: sg_pixel_format = 18;
pub const SG_PIXELFORMAT_DEPTHSTENCIL: sg_pixel_format = 17;
pub const SG_PIXELFORMAT_DEPTH: sg_pixel_format = 16;
pub const SG_PIXELFORMAT_DXT5: sg_pixel_format = 15;
pub const SG_PIXELFORMAT_DXT3: sg_pixel_format = 14;
pub const SG_PIXELFORMAT_DXT1: sg_pixel_format = 13;
pub const SG_PIXELFORMAT_L8: sg_pixel_format = 12;
pub const SG_PIXELFORMAT_R16F: sg_pixel_format = 11;
pub const SG_PIXELFORMAT_R32F: sg_pixel_format = 10;
pub const SG_PIXELFORMAT_RGBA16F: sg_pixel_format = 9;
pub const SG_PIXELFORMAT_RGBA32F: sg_pixel_format = 8;
pub const SG_PIXELFORMAT_R10G10B10A2: sg_pixel_format = 7;
pub const SG_PIXELFORMAT_R5G5B5A1: sg_pixel_format = 6;
pub const SG_PIXELFORMAT_R5G6B5: sg_pixel_format = 5;
pub const SG_PIXELFORMAT_RGBA4: sg_pixel_format = 4;
pub const SG_PIXELFORMAT_RGB8: sg_pixel_format = 3;
pub const SG_PIXELFORMAT_RGBA8: sg_pixel_format = 2;
pub const SG_PIXELFORMAT_NONE: sg_pixel_format = 1;
pub const _SG_PIXELFORMAT_DEFAULT: sg_pixel_format = 0;
pub type sg_primitive_type = libc::c_uint;
pub const _SG_PRIMITIVETYPE_FORCE_U32: sg_primitive_type = 2147483647;
pub const _SG_PRIMITIVETYPE_NUM: sg_primitive_type = 6;
pub const SG_PRIMITIVETYPE_TRIANGLE_STRIP: sg_primitive_type = 5;
pub const SG_PRIMITIVETYPE_TRIANGLES: sg_primitive_type = 4;
pub const SG_PRIMITIVETYPE_LINE_STRIP: sg_primitive_type = 3;
pub const SG_PRIMITIVETYPE_LINES: sg_primitive_type = 2;
pub const SG_PRIMITIVETYPE_POINTS: sg_primitive_type = 1;
pub const _SG_PRIMITIVETYPE_DEFAULT: sg_primitive_type = 0;
pub type sg_filter = libc::c_uint;
pub const _SG_FILTER_FORCE_U32: sg_filter = 2147483647;
pub const _SG_FILTER_NUM: sg_filter = 7;
pub const SG_FILTER_LINEAR_MIPMAP_LINEAR: sg_filter = 6;
pub const SG_FILTER_LINEAR_MIPMAP_NEAREST: sg_filter = 5;
pub const SG_FILTER_NEAREST_MIPMAP_LINEAR: sg_filter = 4;
pub const SG_FILTER_NEAREST_MIPMAP_NEAREST: sg_filter = 3;
pub const SG_FILTER_LINEAR: sg_filter = 2;
pub const SG_FILTER_NEAREST: sg_filter = 1;
pub const _SG_FILTER_DEFAULT: sg_filter = 0;
pub type sg_wrap = libc::c_uint;
pub const _SG_WRAP_FORCE_U32: sg_wrap = 2147483647;
pub const _SG_WRAP_NUM: sg_wrap = 4;
pub const SG_WRAP_MIRRORED_REPEAT: sg_wrap = 3;
pub const SG_WRAP_CLAMP_TO_EDGE: sg_wrap = 2;
pub const SG_WRAP_REPEAT: sg_wrap = 1;
pub const _SG_WRAP_DEFAULT: sg_wrap = 0;
pub type sg_vertex_format = libc::c_uint;
pub const _SG_VERTEXFORMAT_FORCE_U32: sg_vertex_format = 2147483647;
pub const _SG_VERTEXFORMAT_NUM: sg_vertex_format = 14;
pub const SG_VERTEXFORMAT_UINT10_N2: sg_vertex_format = 13;
pub const SG_VERTEXFORMAT_SHORT4N: sg_vertex_format = 12;
pub const SG_VERTEXFORMAT_SHORT4: sg_vertex_format = 11;
pub const SG_VERTEXFORMAT_SHORT2N: sg_vertex_format = 10;
pub const SG_VERTEXFORMAT_SHORT2: sg_vertex_format = 9;
pub const SG_VERTEXFORMAT_UBYTE4N: sg_vertex_format = 8;
pub const SG_VERTEXFORMAT_UBYTE4: sg_vertex_format = 7;
pub const SG_VERTEXFORMAT_BYTE4N: sg_vertex_format = 6;
pub const SG_VERTEXFORMAT_BYTE4: sg_vertex_format = 5;
pub const SG_VERTEXFORMAT_FLOAT4: sg_vertex_format = 4;
pub const SG_VERTEXFORMAT_FLOAT3: sg_vertex_format = 3;
pub const SG_VERTEXFORMAT_FLOAT2: sg_vertex_format = 2;
pub const SG_VERTEXFORMAT_FLOAT: sg_vertex_format = 1;
pub const SG_VERTEXFORMAT_INVALID: sg_vertex_format = 0;
pub type sg_vertex_step = libc::c_uint;
pub const _SG_VERTEXSTEP_FORCE_U32: sg_vertex_step = 2147483647;
pub const _SG_VERTEXSTEP_NUM: sg_vertex_step = 3;
pub const SG_VERTEXSTEP_PER_INSTANCE: sg_vertex_step = 2;
pub const SG_VERTEXSTEP_PER_VERTEX: sg_vertex_step = 1;
pub const _SG_VERTEXSTEP_DEFAULT: sg_vertex_step = 0;
pub type sg_uniform_type = libc::c_uint;
pub const _SG_UNIFORMTYPE_FORCE_U32: sg_uniform_type = 2147483647;
pub const _SG_UNIFORMTYPE_NUM: sg_uniform_type = 6;
pub const SG_UNIFORMTYPE_MAT4: sg_uniform_type = 5;
pub const SG_UNIFORMTYPE_FLOAT4: sg_uniform_type = 4;
pub const SG_UNIFORMTYPE_FLOAT3: sg_uniform_type = 3;
pub const SG_UNIFORMTYPE_FLOAT2: sg_uniform_type = 2;
pub const SG_UNIFORMTYPE_FLOAT: sg_uniform_type = 1;
pub const SG_UNIFORMTYPE_INVALID: sg_uniform_type = 0;
pub type sg_cull_mode = libc::c_uint;
pub const _SG_CULLMODE_FORCE_U32: sg_cull_mode = 2147483647;
pub const _SG_CULLMODE_NUM: sg_cull_mode = 4;
pub const SG_CULLMODE_BACK: sg_cull_mode = 3;
pub const SG_CULLMODE_FRONT: sg_cull_mode = 2;
pub const SG_CULLMODE_NONE: sg_cull_mode = 1;
pub const _SG_CULLMODE_DEFAULT: sg_cull_mode = 0;
pub type sg_face_winding = libc::c_uint;
pub const _SG_FACEWINDING_FORCE_U32: sg_face_winding = 2147483647;
pub const _SG_FACEWINDING_NUM: sg_face_winding = 3;
pub const SG_FACEWINDING_CW: sg_face_winding = 2;
pub const SG_FACEWINDING_CCW: sg_face_winding = 1;
pub const _SG_FACEWINDING_DEFAULT: sg_face_winding = 0;
pub type sg_compare_func = libc::c_uint;
pub const _SG_COMPAREFUNC_FORCE_U32: sg_compare_func = 2147483647;
pub const _SG_COMPAREFUNC_NUM: sg_compare_func = 9;
pub const SG_COMPAREFUNC_ALWAYS: sg_compare_func = 8;
pub const SG_COMPAREFUNC_GREATER_EQUAL: sg_compare_func = 7;
pub const SG_COMPAREFUNC_NOT_EQUAL: sg_compare_func = 6;
pub const SG_COMPAREFUNC_GREATER: sg_compare_func = 5;
pub const SG_COMPAREFUNC_LESS_EQUAL: sg_compare_func = 4;
pub const SG_COMPAREFUNC_EQUAL: sg_compare_func = 3;
pub const SG_COMPAREFUNC_LESS: sg_compare_func = 2;
pub const SG_COMPAREFUNC_NEVER: sg_compare_func = 1;
pub const _SG_COMPAREFUNC_DEFAULT: sg_compare_func = 0;
pub type sg_stencil_op = libc::c_uint;
pub const _SG_STENCILOP_FORCE_U32: sg_stencil_op = 2147483647;
pub const _SG_STENCILOP_NUM: sg_stencil_op = 9;
pub const SG_STENCILOP_DECR_WRAP: sg_stencil_op = 8;
pub const SG_STENCILOP_INCR_WRAP: sg_stencil_op = 7;
pub const SG_STENCILOP_INVERT: sg_stencil_op = 6;
pub const SG_STENCILOP_DECR_CLAMP: sg_stencil_op = 5;
pub const SG_STENCILOP_INCR_CLAMP: sg_stencil_op = 4;
pub const SG_STENCILOP_REPLACE: sg_stencil_op = 3;
pub const SG_STENCILOP_ZERO: sg_stencil_op = 2;
pub const SG_STENCILOP_KEEP: sg_stencil_op = 1;
pub const _SG_STENCILOP_DEFAULT: sg_stencil_op = 0;
pub type sg_blend_factor = libc::c_uint;
pub const _SG_BLENDFACTOR_FORCE_U32: sg_blend_factor = 2147483647;
pub const _SG_BLENDFACTOR_NUM: sg_blend_factor = 16;
pub const SG_BLENDFACTOR_ONE_MINUS_BLEND_ALPHA: sg_blend_factor = 15;
pub const SG_BLENDFACTOR_BLEND_ALPHA: sg_blend_factor = 14;
pub const SG_BLENDFACTOR_ONE_MINUS_BLEND_COLOR: sg_blend_factor = 13;
pub const SG_BLENDFACTOR_BLEND_COLOR: sg_blend_factor = 12;
pub const SG_BLENDFACTOR_SRC_ALPHA_SATURATED: sg_blend_factor = 11;
pub const SG_BLENDFACTOR_ONE_MINUS_DST_ALPHA: sg_blend_factor = 10;
pub const SG_BLENDFACTOR_DST_ALPHA: sg_blend_factor = 9;
pub const SG_BLENDFACTOR_ONE_MINUS_DST_COLOR: sg_blend_factor = 8;
pub const SG_BLENDFACTOR_DST_COLOR: sg_blend_factor = 7;
pub const SG_BLENDFACTOR_ONE_MINUS_SRC_ALPHA: sg_blend_factor = 6;
pub const SG_BLENDFACTOR_SRC_ALPHA: sg_blend_factor = 5;
pub const SG_BLENDFACTOR_ONE_MINUS_SRC_COLOR: sg_blend_factor = 4;
pub const SG_BLENDFACTOR_SRC_COLOR: sg_blend_factor = 3;
pub const SG_BLENDFACTOR_ONE: sg_blend_factor = 2;
pub const SG_BLENDFACTOR_ZERO: sg_blend_factor = 1;
pub const _SG_BLENDFACTOR_DEFAULT: sg_blend_factor = 0;
pub type sg_blend_op = libc::c_uint;
pub const _SG_BLENDOP_FORCE_U32: sg_blend_op = 2147483647;
pub const _SG_BLENDOP_NUM: sg_blend_op = 4;
pub const SG_BLENDOP_REVERSE_SUBTRACT: sg_blend_op = 3;
pub const SG_BLENDOP_SUBTRACT: sg_blend_op = 2;
pub const SG_BLENDOP_ADD: sg_blend_op = 1;
pub const _SG_BLENDOP_DEFAULT: sg_blend_op = 0;
pub type sg_color_mask = libc::c_uint;
pub const _SG_COLORMASK_FORCE_U32: sg_color_mask = 2147483647;
pub const SG_COLORMASK_RGBA: sg_color_mask = 15;
pub const SG_COLORMASK_RGB: sg_color_mask = 7;
pub const SG_COLORMASK_A: sg_color_mask = 8;
pub const SG_COLORMASK_B: sg_color_mask = 4;
pub const SG_COLORMASK_G: sg_color_mask = 2;
pub const SG_COLORMASK_R: sg_color_mask = 1;
pub const SG_COLORMASK_NONE: sg_color_mask = 16;
pub const _SG_COLORMASK_DEFAULT: sg_color_mask = 0;
pub type sg_action = libc::c_uint;
pub const _SG_ACTION_FORCE_U32: sg_action = 2147483647;
pub const _SG_ACTION_NUM: sg_action = 4;
pub const SG_ACTION_DONTCARE: sg_action = 3;
pub const SG_ACTION_LOAD: sg_action = 2;
pub const SG_ACTION_CLEAR: sg_action = 1;
pub const _SG_ACTION_DEFAULT: sg_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_color_attachment_action {
    pub action: sg_action,
    pub val: [libc::c_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_depth_attachment_action {
    pub action: sg_action,
    pub val: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_stencil_attachment_action {
    pub action: sg_action,
    pub val: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_pass_action {
    pub _start_canary: uint32_t,
    pub colors: [sg_color_attachment_action; 4],
    pub depth: sg_depth_attachment_action,
    pub stencil: sg_stencil_attachment_action,
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_bindings {
    pub _start_canary: uint32_t,
    pub vertex_buffers: [sg_buffer; 4],
    pub vertex_buffer_offsets: [libc::c_int; 4],
    pub index_buffer: sg_buffer,
    pub index_buffer_offset: libc::c_int,
    pub vs_images: [sg_image; 12],
    pub fs_images: [sg_image; 12],
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_buffer_desc {
    pub _start_canary: uint32_t,
    pub size: libc::c_int,
    pub type_0: sg_buffer_type,
    pub usage: sg_usage,
    pub content: *const libc::c_void,
    pub label: *const libc::c_char,
    pub gl_buffers: [uint32_t; 2],
    pub mtl_buffers: [*const libc::c_void; 2],
    pub d3d11_buffer: *const libc::c_void,
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_subimage_content {
    pub ptr: *const libc::c_void,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_image_content {
    pub subimage: [[sg_subimage_content; 16]; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_image_desc {
    pub _start_canary: uint32_t,
    pub type_0: sg_image_type,
    pub render_target: bool,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub num_mipmaps: libc::c_int,
    pub usage: sg_usage,
    pub pixel_format: sg_pixel_format,
    pub sample_count: libc::c_int,
    pub min_filter: sg_filter,
    pub mag_filter: sg_filter,
    pub wrap_u: sg_wrap,
    pub wrap_v: sg_wrap,
    pub wrap_w: sg_wrap,
    pub max_anisotropy: uint32_t,
    pub min_lod: libc::c_float,
    pub max_lod: libc::c_float,
    pub content: sg_image_content,
    pub label: *const libc::c_char,
    pub gl_textures: [uint32_t; 2],
    pub mtl_textures: [*const libc::c_void; 2],
    pub d3d11_texture: *const libc::c_void,
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub depth: libc::c_int,
    pub layers: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_attr_desc {
    pub name: *const libc::c_char,
    pub sem_name: *const libc::c_char,
    pub sem_index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_uniform_desc {
    pub name: *const libc::c_char,
    pub type_0: sg_uniform_type,
    pub array_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_uniform_block_desc {
    pub size: libc::c_int,
    pub uniforms: [sg_shader_uniform_desc; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_image_desc {
    pub name: *const libc::c_char,
    pub type_0: sg_image_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_stage_desc {
    pub source: *const libc::c_char,
    pub byte_code: *const uint8_t,
    pub byte_code_size: libc::c_int,
    pub entry: *const libc::c_char,
    pub uniform_blocks: [sg_shader_uniform_block_desc; 4],
    pub images: [sg_shader_image_desc; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_desc {
    pub _start_canary: uint32_t,
    pub attrs: [sg_shader_attr_desc; 16],
    pub vs: sg_shader_stage_desc,
    pub fs: sg_shader_stage_desc,
    pub label: *const libc::c_char,
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_buffer_layout_desc {
    pub stride: libc::c_int,
    pub step_func: sg_vertex_step,
    pub step_rate: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_vertex_attr_desc {
    pub buffer_index: libc::c_int,
    pub offset: libc::c_int,
    pub format: sg_vertex_format,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_layout_desc {
    pub buffers: [sg_buffer_layout_desc; 4],
    pub attrs: [sg_vertex_attr_desc; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_stencil_state {
    pub fail_op: sg_stencil_op,
    pub depth_fail_op: sg_stencil_op,
    pub pass_op: sg_stencil_op,
    pub compare_func: sg_compare_func,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_depth_stencil_state {
    pub stencil_front: sg_stencil_state,
    pub stencil_back: sg_stencil_state,
    pub depth_compare_func: sg_compare_func,
    pub depth_write_enabled: bool,
    pub stencil_enabled: bool,
    pub stencil_read_mask: uint8_t,
    pub stencil_write_mask: uint8_t,
    pub stencil_ref: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_blend_state {
    pub enabled: bool,
    pub src_factor_rgb: sg_blend_factor,
    pub dst_factor_rgb: sg_blend_factor,
    pub op_rgb: sg_blend_op,
    pub src_factor_alpha: sg_blend_factor,
    pub dst_factor_alpha: sg_blend_factor,
    pub op_alpha: sg_blend_op,
    pub color_write_mask: uint8_t,
    pub color_attachment_count: libc::c_int,
    pub color_format: sg_pixel_format,
    pub depth_format: sg_pixel_format,
    pub blend_color: [libc::c_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_rasterizer_state {
    pub alpha_to_coverage_enabled: bool,
    pub cull_mode: sg_cull_mode,
    pub face_winding: sg_face_winding,
    pub sample_count: libc::c_int,
    pub depth_bias: libc::c_float,
    pub depth_bias_slope_scale: libc::c_float,
    pub depth_bias_clamp: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_pipeline_desc {
    pub _start_canary: uint32_t,
    pub layout: sg_layout_desc,
    pub shader: sg_shader,
    pub primitive_type: sg_primitive_type,
    pub index_type: sg_index_type,
    pub depth_stencil: sg_depth_stencil_state,
    pub blend: sg_blend_state,
    pub rasterizer: sg_rasterizer_state,
    pub label: *const libc::c_char,
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_attachment_desc {
    pub image: sg_image,
    pub mip_level: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub face: libc::c_int,
    pub layer: libc::c_int,
    pub slice: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_pass_desc {
    pub _start_canary: uint32_t,
    pub color_attachments: [sg_attachment_desc; 4],
    pub depth_stencil_attachment: sg_attachment_desc,
    pub label: *const libc::c_char,
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_trace_hooks {
    pub user_data: *mut libc::c_void,
    pub query_feature: Option::<
        unsafe extern "C" fn(sg_feature, bool, *mut libc::c_void) -> (),
    >,
    pub reset_state_cache: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub make_buffer: Option::<
        unsafe extern "C" fn(*const sg_buffer_desc, sg_buffer, *mut libc::c_void) -> (),
    >,
    pub make_image: Option::<
        unsafe extern "C" fn(*const sg_image_desc, sg_image, *mut libc::c_void) -> (),
    >,
    pub make_shader: Option::<
        unsafe extern "C" fn(*const sg_shader_desc, sg_shader, *mut libc::c_void) -> (),
    >,
    pub make_pipeline: Option::<
        unsafe extern "C" fn(
            *const sg_pipeline_desc,
            sg_pipeline,
            *mut libc::c_void,
        ) -> (),
    >,
    pub make_pass: Option::<
        unsafe extern "C" fn(*const sg_pass_desc, sg_pass, *mut libc::c_void) -> (),
    >,
    pub destroy_buffer: Option::<
        unsafe extern "C" fn(sg_buffer, *mut libc::c_void) -> (),
    >,
    pub destroy_image: Option::<unsafe extern "C" fn(sg_image, *mut libc::c_void) -> ()>,
    pub destroy_shader: Option::<
        unsafe extern "C" fn(sg_shader, *mut libc::c_void) -> (),
    >,
    pub destroy_pipeline: Option::<
        unsafe extern "C" fn(sg_pipeline, *mut libc::c_void) -> (),
    >,
    pub destroy_pass: Option::<unsafe extern "C" fn(sg_pass, *mut libc::c_void) -> ()>,
    pub update_buffer: Option::<
        unsafe extern "C" fn(
            sg_buffer,
            *const libc::c_void,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
    pub update_image: Option::<
        unsafe extern "C" fn(sg_image, *const sg_image_content, *mut libc::c_void) -> (),
    >,
    pub append_buffer: Option::<
        unsafe extern "C" fn(
            sg_buffer,
            *const libc::c_void,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
    pub begin_default_pass: Option::<
        unsafe extern "C" fn(
            *const sg_pass_action,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
    pub begin_pass: Option::<
        unsafe extern "C" fn(sg_pass, *const sg_pass_action, *mut libc::c_void) -> (),
    >,
    pub apply_viewport: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            bool,
            *mut libc::c_void,
        ) -> (),
    >,
    pub apply_scissor_rect: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            bool,
            *mut libc::c_void,
        ) -> (),
    >,
    pub apply_pipeline: Option::<
        unsafe extern "C" fn(sg_pipeline, *mut libc::c_void) -> (),
    >,
    pub apply_bindings: Option::<
        unsafe extern "C" fn(*const sg_bindings, *mut libc::c_void) -> (),
    >,
    pub apply_uniforms: Option::<
        unsafe extern "C" fn(
            sg_shader_stage,
            libc::c_int,
            *const libc::c_void,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
    pub draw: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
    pub end_pass: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub commit: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub alloc_buffer: Option::<unsafe extern "C" fn(sg_buffer, *mut libc::c_void) -> ()>,
    pub alloc_image: Option::<unsafe extern "C" fn(sg_image, *mut libc::c_void) -> ()>,
    pub alloc_shader: Option::<unsafe extern "C" fn(sg_shader, *mut libc::c_void) -> ()>,
    pub alloc_pipeline: Option::<
        unsafe extern "C" fn(sg_pipeline, *mut libc::c_void) -> (),
    >,
    pub alloc_pass: Option::<unsafe extern "C" fn(sg_pass, *mut libc::c_void) -> ()>,
    pub init_buffer: Option::<
        unsafe extern "C" fn(sg_buffer, *const sg_buffer_desc, *mut libc::c_void) -> (),
    >,
    pub init_image: Option::<
        unsafe extern "C" fn(sg_image, *const sg_image_desc, *mut libc::c_void) -> (),
    >,
    pub init_shader: Option::<
        unsafe extern "C" fn(sg_shader, *const sg_shader_desc, *mut libc::c_void) -> (),
    >,
    pub init_pipeline: Option::<
        unsafe extern "C" fn(
            sg_pipeline,
            *const sg_pipeline_desc,
            *mut libc::c_void,
        ) -> (),
    >,
    pub init_pass: Option::<
        unsafe extern "C" fn(sg_pass, *const sg_pass_desc, *mut libc::c_void) -> (),
    >,
    pub fail_buffer: Option::<unsafe extern "C" fn(sg_buffer, *mut libc::c_void) -> ()>,
    pub fail_image: Option::<unsafe extern "C" fn(sg_image, *mut libc::c_void) -> ()>,
    pub fail_shader: Option::<unsafe extern "C" fn(sg_shader, *mut libc::c_void) -> ()>,
    pub fail_pipeline: Option::<
        unsafe extern "C" fn(sg_pipeline, *mut libc::c_void) -> (),
    >,
    pub fail_pass: Option::<unsafe extern "C" fn(sg_pass, *mut libc::c_void) -> ()>,
    pub push_debug_group: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
    >,
    pub pop_debug_group: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_buffer_pool_exhausted: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> (),
    >,
    pub err_image_pool_exhausted: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> (),
    >,
    pub err_shader_pool_exhausted: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> (),
    >,
    pub err_pipeline_pool_exhausted: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> (),
    >,
    pub err_pass_pool_exhausted: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_context_mismatch: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_pass_invalid: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_draw_invalid: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_bindings_invalid: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_slot_info {
    pub state: sg_resource_state,
    pub res_id: uint32_t,
    pub ctx_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_buffer_info {
    pub slot: sg_slot_info,
    pub update_frame_index: uint32_t,
    pub append_frame_index: uint32_t,
    pub append_pos: libc::c_int,
    pub append_overflow: bool,
    pub num_slots: libc::c_int,
    pub active_slot: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_image_info {
    pub slot: sg_slot_info,
    pub upd_frame_index: uint32_t,
    pub num_slots: libc::c_int,
    pub active_slot: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_info {
    pub slot: sg_slot_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_pipeline_info {
    pub slot: sg_slot_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_pass_info {
    pub slot: sg_slot_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_desc {
    pub _start_canary: uint32_t,
    pub buffer_pool_size: libc::c_int,
    pub image_pool_size: libc::c_int,
    pub shader_pool_size: libc::c_int,
    pub pipeline_pool_size: libc::c_int,
    pub pass_pool_size: libc::c_int,
    pub context_pool_size: libc::c_int,
    pub gl_force_gles2: bool,
    pub mtl_device: *const libc::c_void,
    pub mtl_renderpass_descriptor_cb: Option::<
        unsafe extern "C" fn() -> *const libc::c_void,
    >,
    pub mtl_drawable_cb: Option::<unsafe extern "C" fn() -> *const libc::c_void>,
    pub mtl_global_uniform_buffer_size: libc::c_int,
    pub mtl_sampler_cache_size: libc::c_int,
    pub d3d11_device: *const libc::c_void,
    pub d3d11_device_context: *const libc::c_void,
    pub d3d11_render_target_view_cb: Option::<
        unsafe extern "C" fn() -> *const libc::c_void,
    >,
    pub d3d11_depth_stencil_view_cb: Option::<
        unsafe extern "C" fn() -> *const libc::c_void,
    >,
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_state_t {
    pub valid: bool,
    pub desc: sg_desc,
    pub frame_index: uint32_t,
    pub active_context: sg_context,
    pub cur_pass: sg_pass,
    pub cur_pipeline: sg_pipeline,
    pub pass_valid: bool,
    pub bindings_valid: bool,
    pub next_draw_valid: bool,
    pub pools: _sg_pools_t,
    pub gl: _sg_gl_backend_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_gl_backend_t {
    pub valid: bool,
    pub gles2: bool,
    pub in_pass: bool,
    pub cur_pass_width: libc::c_int,
    pub cur_pass_height: libc::c_int,
    pub cur_context: *mut _sg_context_t,
    pub cur_pass: *mut _sg_pass_t,
    pub cur_pass_id: sg_pass,
    pub cache: _sg_gl_state_cache_t,
    pub features: [bool; 14],
    pub ext_anisotropic: bool,
    pub max_anisotropy: GLint,
    pub max_combined_texture_image_units: GLint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_gl_state_cache_t {
    pub ds: sg_depth_stencil_state,
    pub blend: sg_blend_state,
    pub rast: sg_rasterizer_state,
    pub polygon_offset_enabled: bool,
    pub attrs: [_sg_gl_cache_attr_t; 16],
    pub vertex_buffer: GLuint,
    pub index_buffer: GLuint,
    pub stored_vertex_buffer: GLuint,
    pub stored_index_buffer: GLuint,
    pub textures: [_sg_gl_texture_bind_slot; 12],
    pub stored_texture: _sg_gl_texture_bind_slot,
    pub cur_ib_offset: libc::c_int,
    pub cur_primitive_type: GLenum,
    pub cur_index_type: GLenum,
    pub cur_pipeline: *mut _sg_pipeline_t,
    pub cur_pipeline_id: sg_pipeline,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_pipeline_t {
    pub slot: _sg_slot_t,
    pub shader: *mut _sg_shader_t,
    pub shader_id: sg_shader,
    pub primitive_type: sg_primitive_type,
    pub index_type: sg_index_type,
    pub vertex_layout_valid: [bool; 4],
    pub color_attachment_count: libc::c_int,
    pub color_format: sg_pixel_format,
    pub depth_format: sg_pixel_format,
    pub sample_count: libc::c_int,
    pub gl_attrs: [_sg_gl_attr_t; 16],
    pub depth_stencil: sg_depth_stencil_state,
    pub blend: sg_blend_state,
    pub rast: sg_rasterizer_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_gl_attr_t {
    pub vb_index: int8_t,
    pub divisor: int8_t,
    pub stride: uint8_t,
    pub size: uint8_t,
    pub normalized: uint8_t,
    pub offset: libc::c_int,
    pub type_0: GLenum,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_shader_t {
    pub slot: _sg_slot_t,
    pub gl_prog: GLuint,
    pub attrs: [_sg_shader_attr_t; 16],
    pub stage: [_sg_shader_stage_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_shader_stage_t {
    pub num_uniform_blocks: libc::c_int,
    pub num_images: libc::c_int,
    pub uniform_blocks: [_sg_uniform_block_t; 4],
    pub images: [_sg_shader_image_t; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_shader_image_t {
    pub type_0: sg_image_type,
    pub gl_loc: GLint,
    pub gl_tex_slot: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_uniform_block_t {
    pub size: libc::c_int,
    pub num_uniforms: libc::c_int,
    pub uniforms: [_sg_uniform_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_uniform_t {
    pub gl_loc: GLint,
    pub type_0: sg_uniform_type,
    pub count: uint8_t,
    pub offset: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_shader_attr_t {
    pub name: _sg_str_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_str_t {
    pub buf: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_slot_t {
    pub id: uint32_t,
    pub ctx_id: uint32_t,
    pub state: sg_resource_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_gl_texture_bind_slot {
    pub target: GLenum,
    pub texture: GLuint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_gl_cache_attr_t {
    pub gl_attr: _sg_gl_attr_t,
    pub gl_vbuf: GLuint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_pass_t {
    pub slot: _sg_slot_t,
    pub gl_fb: GLuint,
    pub num_color_atts: libc::c_int,
    pub color_atts: [_sg_attachment_t; 4],
    pub ds_att: _sg_attachment_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_attachment_t {
    pub image: *mut _sg_image_t,
    pub image_id: sg_image,
    pub mip_level: libc::c_int,
    pub slice: libc::c_int,
    pub gl_msaa_resolve_buffer: GLuint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_image_t {
    pub slot: _sg_slot_t,
    pub type_0: sg_image_type,
    pub render_target: bool,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub depth: libc::c_int,
    pub num_mipmaps: libc::c_int,
    pub usage: sg_usage,
    pub pixel_format: sg_pixel_format,
    pub sample_count: libc::c_int,
    pub min_filter: sg_filter,
    pub mag_filter: sg_filter,
    pub wrap_u: sg_wrap,
    pub wrap_v: sg_wrap,
    pub wrap_w: sg_wrap,
    pub max_anisotropy: uint32_t,
    pub gl_target: GLenum,
    pub gl_depth_render_buffer: GLuint,
    pub gl_msaa_render_buffer: GLuint,
    pub upd_frame_index: uint32_t,
    pub num_slots: libc::c_int,
    pub active_slot: libc::c_int,
    pub gl_tex: [GLuint; 2],
    pub ext_textures: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_context_t {
    pub slot: _sg_slot_t,
    pub vao: GLuint,
    pub default_framebuffer: GLuint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_pools_t {
    pub buffer_pool: _sg_pool_t,
    pub image_pool: _sg_pool_t,
    pub shader_pool: _sg_pool_t,
    pub pipeline_pool: _sg_pool_t,
    pub pass_pool: _sg_pool_t,
    pub context_pool: _sg_pool_t,
    pub buffers: *mut _sg_buffer_t,
    pub images: *mut _sg_image_t,
    pub shaders: *mut _sg_shader_t,
    pub pipelines: *mut _sg_pipeline_t,
    pub passes: *mut _sg_pass_t,
    pub contexts: *mut _sg_context_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_buffer_t {
    pub slot: _sg_slot_t,
    pub size: libc::c_int,
    pub append_pos: libc::c_int,
    pub append_overflow: bool,
    pub type_0: sg_buffer_type,
    pub usage: sg_usage,
    pub update_frame_index: uint32_t,
    pub append_frame_index: uint32_t,
    pub num_slots: libc::c_int,
    pub active_slot: libc::c_int,
    pub gl_buf: [GLuint; 2],
    pub ext_buffers: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sg_pool_t {
    pub size: libc::c_int,
    pub queue_top: libc::c_int,
    pub gen_ctrs: *mut uint32_t,
    pub free_queue: *mut libc::c_int,
}
pub const _SG_SLOT_MASK: C2RustUnnamed_7 = 65535;
pub const _SG_SLOT_SHIFT: C2RustUnnamed_7 = 16;
pub const _SG_MTL_DEFAULT_SAMPLER_CACHE_CAPACITY: C2RustUnnamed_7 = 64;
pub const _SG_MTL_DEFAULT_UB_SIZE: C2RustUnnamed_7 = 4194304;
pub const _SG_DEFAULT_CONTEXT_POOL_SIZE: C2RustUnnamed_7 = 16;
pub const _SG_DEFAULT_PASS_POOL_SIZE: C2RustUnnamed_7 = 16;
pub const _SG_DEFAULT_PIPELINE_POOL_SIZE: C2RustUnnamed_7 = 64;
pub const _SG_DEFAULT_SHADER_POOL_SIZE: C2RustUnnamed_7 = 32;
pub const _SG_DEFAULT_IMAGE_POOL_SIZE: C2RustUnnamed_7 = 128;
pub const _SG_DEFAULT_BUFFER_POOL_SIZE: C2RustUnnamed_7 = 128;
pub const _SG_STRING_SIZE: C2RustUnnamed_7 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_draw_state {
    pub _start_canary: uint32_t,
    pub pipeline: sg_pipeline,
    pub vertex_buffers: [sg_buffer; 4],
    pub vertex_buffer_offsets: [libc::c_int; 4],
    pub index_buffer: sg_buffer,
    pub index_buffer_offset: libc::c_int,
    pub vs_images: [sg_image; 12],
    pub fs_images: [sg_image; 12],
    pub _end_canary: uint32_t,
}
pub type C2RustUnnamed_7 = libc::c_uint;
pub const _SG_MAX_POOL_SIZE: C2RustUnnamed_7 = 65536;
pub unsafe extern "C" fn sapp_run(mut desc: *const sapp_desc) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sapp_win32_get_hwnd() -> *const libc::c_void {
    return 0 as *const libc::c_void;
}
pub unsafe extern "C" fn sapp_d3d11_get_depth_stencil_view() -> *const libc::c_void {
    return 0 as *const libc::c_void;
}
pub unsafe extern "C" fn sapp_d3d11_get_render_target_view() -> *const libc::c_void {
    return 0 as *const libc::c_void;
}
pub unsafe extern "C" fn sapp_d3d11_get_device_context() -> *const libc::c_void {
    return 0 as *const libc::c_void;
}
pub unsafe extern "C" fn sapp_d3d11_get_device() -> *const libc::c_void {
    return 0 as *const libc::c_void;
}
pub unsafe extern "C" fn sapp_ios_get_window() -> *const libc::c_void {
    return 0 as *const libc::c_void;
}
pub unsafe extern "C" fn sapp_macos_get_window() -> *const libc::c_void {
    return 0 as *const libc::c_void;
}
pub unsafe extern "C" fn sapp_metal_get_drawable() -> *const libc::c_void {
    return 0 as *const libc::c_void;
}
pub unsafe extern "C" fn sapp_metal_get_renderpass_descriptor() -> *const libc::c_void {
    return 0 as *const libc::c_void;
}
pub unsafe extern "C" fn sapp_metal_get_device() -> *const libc::c_void {
    return 0 as *const libc::c_void;
}
pub unsafe extern "C" fn sapp_gles2() -> bool {
    return _sapp.gles2_fallback;
}
pub unsafe extern "C" fn sapp_keyboard_shown() -> bool {
    return _sapp.onscreen_keyboard_shown;
}
pub unsafe extern "C" fn sapp_show_keyboard(mut shown: bool) {}
pub unsafe extern "C" fn sapp_dpi_scale() -> libc::c_float {
    return _sapp.dpi_scale;
}
pub unsafe extern "C" fn sapp_high_dpi() -> bool {
    return _sapp.desc.high_dpi as libc::c_int != 0 && _sapp.dpi_scale > 1.5f32;
}
pub unsafe extern "C" fn sapp_height() -> libc::c_int {
    return if _sapp.framebuffer_height > 0 as libc::c_int {
        _sapp.framebuffer_height
    } else {
        1 as libc::c_int
    };
}
pub unsafe extern "C" fn sapp_width() -> libc::c_int {
    return if _sapp.framebuffer_width > 0 as libc::c_int {
        _sapp.framebuffer_width
    } else {
        1 as libc::c_int
    };
}
static mut _sapp: _sapp_state = _sapp_state {
    valid: false,
    window_width: 0,
    window_height: 0,
    framebuffer_width: 0,
    framebuffer_height: 0,
    sample_count: 0,
    swap_interval: 0,
    dpi_scale: 0.,
    gles2_fallback: false,
    first_frame: false,
    init_called: false,
    cleanup_called: false,
    html5_canvas_resize: false,
    html5_canvas_name: 0 as *const libc::c_char,
    window_title: [0; 128],
    window_title_wide: [0; 128],
    frame_count: 0,
    mouse_x: 0.,
    mouse_y: 0.,
    win32_mouse_tracked: false,
    onscreen_keyboard_shown: false,
    event: sapp_event {
        type_0: SAPP_EVENTTYPE_INVALID,
        frame_count: 0,
        key_code: SAPP_KEYCODE_INVALID,
        char_code: 0,
        key_repeat: false,
        modifiers: 0,
        mouse_button: SAPP_MOUSEBUTTON_LEFT,
        mouse_x: 0.,
        mouse_y: 0.,
        scroll_x: 0.,
        scroll_y: 0.,
        num_touches: 0,
        touches: [sapp_touchpoint {
            identifier: 0,
            pos_x: 0.,
            pos_y: 0.,
            changed: false,
        }; 8],
        window_width: 0,
        window_height: 0,
        framebuffer_width: 0,
        framebuffer_height: 0,
    },
    desc: sapp_desc {
        init_cb: None,
        frame_cb: None,
        cleanup_cb: None,
        event_cb: None,
        fail_cb: None,
        user_data: 0 as *const libc::c_void as *mut libc::c_void,
        init_userdata_cb: None,
        frame_userdata_cb: None,
        cleanup_userdata_cb: None,
        event_userdata_cb: None,
        fail_userdata_cb: None,
        width: 0,
        height: 0,
        sample_count: 0,
        swap_interval: 0,
        high_dpi: false,
        fullscreen: false,
        alpha: false,
        window_title: 0 as *const libc::c_char,
        user_cursor: false,
        html5_canvas_name: 0 as *const libc::c_char,
        html5_canvas_resize: false,
        html5_preserve_drawing_buffer: false,
        html5_premultiplied_alpha: false,
        ios_keyboard_resizes_canvas: false,
        gl_force_gles2: false,
    },
    keycodes: [SAPP_KEYCODE_INVALID; 512],
};
pub unsafe extern "C" fn sapp_isvalid() -> bool {
    return _sapp.valid;
}
pub unsafe extern "C" fn sg_apply_uniform_block(
    mut stage: sg_shader_stage,
    mut ub_index: libc::c_int,
    mut data: *const libc::c_void,
    mut num_bytes: libc::c_int,
) {
    sg_apply_uniforms(stage, ub_index, data, num_bytes);
}
pub unsafe extern "C" fn sg_apply_draw_state(mut ds: *const sg_draw_state) {
    sg_apply_pipeline((*ds).pipeline);
    let mut bindings: sg_bindings = sg_bindings {
        _start_canary: 0,
        vertex_buffers: [sg_buffer { id: 0 }; 4],
        vertex_buffer_offsets: [0; 4],
        index_buffer: sg_buffer { id: 0 },
        index_buffer_offset: 0,
        vs_images: [sg_image { id: 0 }; 12],
        fs_images: [sg_image { id: 0 }; 12],
        _end_canary: 0,
    };
    memset(
        &mut bindings as *mut sg_bindings as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sg_bindings>() as libc::c_ulong,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < SG_MAX_SHADERSTAGE_BUFFERS as libc::c_int {
        bindings.vertex_buffers[i as usize] = (*ds).vertex_buffers[i as usize];
        bindings
            .vertex_buffer_offsets[i as usize] = (*ds).vertex_buffer_offsets[i as usize];
        i += 1;
        i;
    }
    bindings.index_buffer = (*ds).index_buffer;
    bindings.index_buffer_offset = (*ds).index_buffer_offset;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < SG_MAX_SHADERSTAGE_IMAGES as libc::c_int {
        bindings.vs_images[i_0 as usize] = (*ds).vs_images[i_0 as usize];
        bindings.fs_images[i_0 as usize] = (*ds).fs_images[i_0 as usize];
        i_0 += 1;
        i_0;
    }
    sg_apply_bindings(&mut bindings);
}
unsafe extern "C" fn _sg_reset_context(mut ctx: *mut _sg_context_t) {
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_sg_context_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn sg_discard_context(mut ctx_id: sg_context) {
    _sg_destroy_all_resources(&mut _sg.pools, ctx_id.id);
    let mut ctx: *mut _sg_context_t = _sg_lookup_context(&mut _sg.pools, ctx_id.id);
    if !ctx.is_null() {
        _sg_destroy_context(ctx);
        _sg_reset_context(ctx);
        _sg_pool_free_index(&mut _sg.pools.context_pool, _sg_slot_index(ctx_id.id));
    }
    _sg.active_context.id = SG_INVALID_ID as libc::c_int as uint32_t;
    _sg_activate_context(0 as *mut _sg_context_t);
}
pub unsafe extern "C" fn sg_activate_context(mut ctx_id: sg_context) {
    _sg.active_context = ctx_id;
    let mut ctx: *mut _sg_context_t = _sg_lookup_context(&mut _sg.pools, ctx_id.id);
    _sg_activate_context(ctx);
}
pub unsafe extern "C" fn sg_query_pass_info(mut pass_id: sg_pass) -> sg_pass_info {
    let mut info: sg_pass_info = sg_pass_info {
        slot: sg_slot_info {
            state: SG_RESOURCESTATE_INITIAL,
            res_id: 0,
            ctx_id: 0,
        },
    };
    memset(
        &mut info as *mut sg_pass_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sg_pass_info>() as libc::c_ulong,
    );
    let mut pass: *const _sg_pass_t = _sg_lookup_pass(&mut _sg.pools, pass_id.id);
    if !pass.is_null() {
        info.slot.state = (*pass).slot.state;
        info.slot.res_id = (*pass).slot.id;
        info.slot.ctx_id = (*pass).slot.ctx_id;
    }
    return info;
}
pub unsafe extern "C" fn sg_query_pipeline_info(
    mut pip_id: sg_pipeline,
) -> sg_pipeline_info {
    let mut info: sg_pipeline_info = sg_pipeline_info {
        slot: sg_slot_info {
            state: SG_RESOURCESTATE_INITIAL,
            res_id: 0,
            ctx_id: 0,
        },
    };
    memset(
        &mut info as *mut sg_pipeline_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sg_pipeline_info>() as libc::c_ulong,
    );
    let mut pip: *const _sg_pipeline_t = _sg_lookup_pipeline(&mut _sg.pools, pip_id.id);
    if !pip.is_null() {
        info.slot.state = (*pip).slot.state;
        info.slot.res_id = (*pip).slot.id;
        info.slot.ctx_id = (*pip).slot.ctx_id;
    }
    return info;
}
pub unsafe extern "C" fn sg_query_shader_info(mut shd_id: sg_shader) -> sg_shader_info {
    let mut info: sg_shader_info = sg_shader_info {
        slot: sg_slot_info {
            state: SG_RESOURCESTATE_INITIAL,
            res_id: 0,
            ctx_id: 0,
        },
    };
    memset(
        &mut info as *mut sg_shader_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sg_shader_info>() as libc::c_ulong,
    );
    let mut shd: *const _sg_shader_t = _sg_lookup_shader(&mut _sg.pools, shd_id.id);
    if !shd.is_null() {
        info.slot.state = (*shd).slot.state;
        info.slot.res_id = (*shd).slot.id;
        info.slot.ctx_id = (*shd).slot.ctx_id;
    }
    return info;
}
pub unsafe extern "C" fn sg_query_image_info(mut img_id: sg_image) -> sg_image_info {
    let mut info: sg_image_info = sg_image_info {
        slot: sg_slot_info {
            state: SG_RESOURCESTATE_INITIAL,
            res_id: 0,
            ctx_id: 0,
        },
        upd_frame_index: 0,
        num_slots: 0,
        active_slot: 0,
    };
    memset(
        &mut info as *mut sg_image_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sg_image_info>() as libc::c_ulong,
    );
    let mut img: *const _sg_image_t = _sg_lookup_image(&mut _sg.pools, img_id.id);
    if !img.is_null() {
        info.slot.state = (*img).slot.state;
        info.slot.res_id = (*img).slot.id;
        info.slot.ctx_id = (*img).slot.ctx_id;
        info.num_slots = (*img).num_slots;
        info.active_slot = (*img).active_slot;
    }
    return info;
}
pub unsafe extern "C" fn sg_query_buffer_info(mut buf_id: sg_buffer) -> sg_buffer_info {
    let mut info: sg_buffer_info = sg_buffer_info {
        slot: sg_slot_info {
            state: SG_RESOURCESTATE_INITIAL,
            res_id: 0,
            ctx_id: 0,
        },
        update_frame_index: 0,
        append_frame_index: 0,
        append_pos: 0,
        append_overflow: false,
        num_slots: 0,
        active_slot: 0,
    };
    memset(
        &mut info as *mut sg_buffer_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sg_buffer_info>() as libc::c_ulong,
    );
    let mut buf: *const _sg_buffer_t = _sg_lookup_buffer(&mut _sg.pools, buf_id.id);
    if !buf.is_null() {
        info.slot.state = (*buf).slot.state;
        info.slot.res_id = (*buf).slot.id;
        info.slot.ctx_id = (*buf).slot.ctx_id;
        info.update_frame_index = (*buf).update_frame_index;
        info.append_frame_index = (*buf).append_frame_index;
        info.append_pos = (*buf).append_pos;
        info.append_overflow = (*buf).append_overflow;
        info.num_slots = (*buf).num_slots;
        info.active_slot = (*buf).active_slot;
    }
    return info;
}
pub unsafe extern "C" fn sg_fail_pass(mut pass_id: sg_pass) {
    let mut pass: *mut _sg_pass_t = _sg_lookup_pass(&mut _sg.pools, pass_id.id);
    (*pass).slot.ctx_id = _sg.active_context.id;
    (*pass).slot.state = SG_RESOURCESTATE_FAILED;
}
pub unsafe extern "C" fn sg_fail_pipeline(mut pip_id: sg_pipeline) {
    let mut pip: *mut _sg_pipeline_t = _sg_lookup_pipeline(&mut _sg.pools, pip_id.id);
    (*pip).slot.ctx_id = _sg.active_context.id;
    (*pip).slot.state = SG_RESOURCESTATE_FAILED;
}
pub unsafe extern "C" fn sg_fail_shader(mut shd_id: sg_shader) {
    let mut shd: *mut _sg_shader_t = _sg_lookup_shader(&mut _sg.pools, shd_id.id);
    (*shd).slot.ctx_id = _sg.active_context.id;
    (*shd).slot.state = SG_RESOURCESTATE_FAILED;
}
pub unsafe extern "C" fn sg_fail_image(mut img_id: sg_image) {
    let mut img: *mut _sg_image_t = _sg_lookup_image(&mut _sg.pools, img_id.id);
    (*img).slot.ctx_id = _sg.active_context.id;
    (*img).slot.state = SG_RESOURCESTATE_FAILED;
}
pub unsafe extern "C" fn sg_fail_buffer(mut buf_id: sg_buffer) {
    let mut buf: *mut _sg_buffer_t = _sg_lookup_buffer(&mut _sg.pools, buf_id.id);
    (*buf).slot.ctx_id = _sg.active_context.id;
    (*buf).slot.state = SG_RESOURCESTATE_FAILED;
}
pub unsafe extern "C" fn sg_init_pass(
    mut pass_id: sg_pass,
    mut desc: *const sg_pass_desc,
) {
    let mut desc_def: sg_pass_desc = _sg_pass_desc_defaults(desc);
    _sg_init_pass(pass_id, &mut desc_def);
}
pub unsafe extern "C" fn sg_init_pipeline(
    mut pip_id: sg_pipeline,
    mut desc: *const sg_pipeline_desc,
) {
    let mut desc_def: sg_pipeline_desc = _sg_pipeline_desc_defaults(desc);
    _sg_init_pipeline(pip_id, &mut desc_def);
}
pub unsafe extern "C" fn sg_init_shader(
    mut shd_id: sg_shader,
    mut desc: *const sg_shader_desc,
) {
    let mut desc_def: sg_shader_desc = _sg_shader_desc_defaults(desc);
    _sg_init_shader(shd_id, &mut desc_def);
}
pub unsafe extern "C" fn sg_init_image(
    mut img_id: sg_image,
    mut desc: *const sg_image_desc,
) {
    let mut desc_def: sg_image_desc = _sg_image_desc_defaults(desc);
    _sg_init_image(img_id, &mut desc_def);
}
pub unsafe extern "C" fn sg_init_buffer(
    mut buf_id: sg_buffer,
    mut desc: *const sg_buffer_desc,
) {
    let mut desc_def: sg_buffer_desc = _sg_buffer_desc_defaults(desc);
    _sg_init_buffer(buf_id, &mut desc_def);
}
pub unsafe extern "C" fn sg_alloc_pass() -> sg_pass {
    let mut res: sg_pass = _sg_alloc_pass();
    return res;
}
pub unsafe extern "C" fn sg_alloc_pipeline() -> sg_pipeline {
    let mut res: sg_pipeline = _sg_alloc_pipeline();
    return res;
}
pub unsafe extern "C" fn sg_alloc_shader() -> sg_shader {
    let mut res: sg_shader = _sg_alloc_shader();
    return res;
}
pub unsafe extern "C" fn sg_alloc_image() -> sg_image {
    let mut res: sg_image = _sg_alloc_image();
    return res;
}
pub unsafe extern "C" fn sg_alloc_buffer() -> sg_buffer {
    let mut res: sg_buffer = _sg_alloc_buffer();
    return res;
}
unsafe extern "C" fn _sg_commit() {
    _sg_gl_clear_buffer_bindings(0 as libc::c_int != 0);
    _sg_gl_clear_texture_bindings(0 as libc::c_int != 0);
}
pub unsafe extern "C" fn sg_commit() {
    _sg_commit();
    _sg.frame_index = (_sg.frame_index).wrapping_add(1);
    _sg.frame_index;
}
unsafe extern "C" fn _sg_end_pass() {
    if !_sg.gl.gles2 && !(_sg.gl.cur_pass).is_null() {
        let mut pass: *const _sg_pass_t = _sg.gl.cur_pass;
        let mut is_msaa: bool = 0 as libc::c_int as libc::c_uint
            != (*_sg.gl.cur_pass)
                .color_atts[0 as libc::c_int as usize]
                .gl_msaa_resolve_buffer;
        if is_msaa {
            glBindFramebuffer(0x8ca8 as libc::c_int as GLenum, (*pass).gl_fb);
            let w: libc::c_int = (*(*pass).color_atts[0 as libc::c_int as usize].image)
                .width;
            let h: libc::c_int = (*(*pass).color_atts[0 as libc::c_int as usize].image)
                .height;
            let mut att_index: libc::c_int = 0 as libc::c_int;
            while att_index < SG_MAX_COLOR_ATTACHMENTS as libc::c_int {
                let mut att: *const _sg_attachment_t = &*((*pass).color_atts)
                    .as_ptr()
                    .offset(att_index as isize) as *const _sg_attachment_t;
                if ((*att).image).is_null() {
                    break;
                }
                glBindFramebuffer(
                    0x8ca9 as libc::c_int as GLenum,
                    (*att).gl_msaa_resolve_buffer,
                );
                glReadBuffer((0x8ce0 as libc::c_int + att_index) as GLenum);
                let gl_att: GLenum = 0x8ce0 as libc::c_int as GLenum;
                glDrawBuffers(1 as libc::c_int, &gl_att);
                glBlitFramebuffer(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    w,
                    h,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    w,
                    h,
                    0x4000 as libc::c_int as GLbitfield,
                    0x2600 as libc::c_int as GLenum,
                );
                att_index += 1;
                att_index;
            }
        }
    }
    _sg.gl.cur_pass = 0 as *mut _sg_pass_t;
    _sg.gl.cur_pass_id.id = SG_INVALID_ID as libc::c_int as uint32_t;
    _sg.gl.cur_pass_width = 0 as libc::c_int;
    _sg.gl.cur_pass_height = 0 as libc::c_int;
    glBindFramebuffer(
        0x8d40 as libc::c_int as GLenum,
        (*_sg.gl.cur_context).default_framebuffer,
    );
    _sg.gl.in_pass = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn sg_end_pass() {
    if !_sg.pass_valid {
        return;
    }
    _sg_end_pass();
    _sg.cur_pass.id = SG_INVALID_ID as libc::c_int as uint32_t;
    _sg.cur_pipeline.id = SG_INVALID_ID as libc::c_int as uint32_t;
    _sg.pass_valid = 0 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_draw(
    mut base_element: libc::c_int,
    mut num_elements: libc::c_int,
    mut num_instances: libc::c_int,
) {
    let i_type: GLenum = _sg.gl.cache.cur_index_type;
    let p_type: GLenum = _sg.gl.cache.cur_primitive_type;
    if 0 as libc::c_int as libc::c_uint != i_type {
        let i_size: libc::c_int = if i_type == 0x1403 as libc::c_int as libc::c_uint {
            2 as libc::c_int
        } else {
            4 as libc::c_int
        };
        let ib_offset: libc::c_int = _sg.gl.cache.cur_ib_offset;
        let mut indices: *const libc::c_void = (base_element * i_size + ib_offset)
            as GLintptr as *const libc::c_void;
        if num_instances == 1 as libc::c_int {
            glDrawElements(p_type, num_elements, i_type, indices);
        } else if _sg.gl.features[SG_FEATURE_INSTANCING as libc::c_int as usize] {
            glDrawElementsInstanced(
                p_type,
                num_elements,
                i_type,
                indices,
                num_instances,
            );
        }
    } else if num_instances == 1 as libc::c_int {
        glDrawArrays(p_type, base_element, num_elements);
    } else if _sg.gl.features[SG_FEATURE_INSTANCING as libc::c_int as usize] {
        glDrawArraysInstanced(p_type, base_element, num_elements, num_instances);
    }
}
pub unsafe extern "C" fn sg_draw(
    mut base_element: libc::c_int,
    mut num_elements: libc::c_int,
    mut num_instances: libc::c_int,
) {
    if !_sg.pass_valid {
        return;
    }
    if !_sg.next_draw_valid {
        return;
    }
    if !_sg.bindings_valid {
        return;
    }
    _sg_draw(base_element, num_elements, num_instances);
}
unsafe extern "C" fn _sg_validate_apply_uniforms(
    mut stage_index: sg_shader_stage,
    mut ub_index: libc::c_int,
    mut data: *const libc::c_void,
    mut num_bytes: libc::c_int,
) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_apply_uniforms(
    mut stage_index: sg_shader_stage,
    mut ub_index: libc::c_int,
    mut data: *const libc::c_void,
    mut num_bytes: libc::c_int,
) {
    let mut stage: *mut _sg_shader_stage_t = &mut *((*(*_sg.gl.cache.cur_pipeline)
        .shader)
        .stage)
        .as_mut_ptr()
        .offset(stage_index as isize) as *mut _sg_shader_stage_t;
    let mut ub: *mut _sg_uniform_block_t = &mut *((*stage).uniform_blocks)
        .as_mut_ptr()
        .offset(ub_index as isize) as *mut _sg_uniform_block_t;
    let mut u_index: libc::c_int = 0 as libc::c_int;
    while u_index < (*ub).num_uniforms {
        let mut u: *mut _sg_uniform_t = &mut *((*ub).uniforms)
            .as_mut_ptr()
            .offset(u_index as isize) as *mut _sg_uniform_t;
        if !((*u).gl_loc == -(1 as libc::c_int)) {
            let mut ptr: *mut GLfloat = (data as *mut uint8_t)
                .offset((*u).offset as libc::c_int as isize) as *mut GLfloat;
            match (*u).type_0 as libc::c_uint {
                1 => {
                    glUniform1fv((*u).gl_loc, (*u).count as GLsizei, ptr);
                }
                2 => {
                    glUniform2fv((*u).gl_loc, (*u).count as GLsizei, ptr);
                }
                3 => {
                    glUniform3fv((*u).gl_loc, (*u).count as GLsizei, ptr);
                }
                4 => {
                    glUniform4fv((*u).gl_loc, (*u).count as GLsizei, ptr);
                }
                5 => {
                    glUniformMatrix4fv(
                        (*u).gl_loc,
                        (*u).count as GLsizei,
                        0 as libc::c_int as GLboolean,
                        ptr,
                    );
                }
                0 | _ => {}
            }
        }
        u_index += 1;
        u_index;
    }
}
pub unsafe extern "C" fn sg_apply_uniforms(
    mut stage: sg_shader_stage,
    mut ub_index: libc::c_int,
    mut data: *const libc::c_void,
    mut num_bytes: libc::c_int,
) {
    if !_sg_validate_apply_uniforms(stage, ub_index, data, num_bytes) {
        _sg.next_draw_valid = 0 as libc::c_int != 0;
        return;
    }
    if !_sg.pass_valid {
        return;
    }
    !_sg.next_draw_valid;
    _sg_apply_uniforms(stage, ub_index, data, num_bytes);
}
unsafe extern "C" fn _sg_validate_apply_bindings(
    mut bindings: *const sg_bindings,
) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_apply_bindings(
    mut pip: *mut _sg_pipeline_t,
    mut vbs: *mut *mut _sg_buffer_t,
    mut vb_offsets: *const libc::c_int,
    mut num_vbs: libc::c_int,
    mut ib: *mut _sg_buffer_t,
    mut ib_offset: libc::c_int,
    mut vs_imgs: *mut *mut _sg_image_t,
    mut num_vs_imgs: libc::c_int,
    mut fs_imgs: *mut *mut _sg_image_t,
    mut num_fs_imgs: libc::c_int,
) {
    let mut stage_index: libc::c_int = 0 as libc::c_int;
    while stage_index < SG_NUM_SHADER_STAGES as libc::c_int {
        let mut stage: *const _sg_shader_stage_t = &mut *((*(*pip).shader).stage)
            .as_mut_ptr()
            .offset(stage_index as isize) as *mut _sg_shader_stage_t;
        let mut imgs: *mut *mut _sg_image_t = if stage_index
            == SG_SHADERSTAGE_VS as libc::c_int
        {
            vs_imgs
        } else {
            fs_imgs
        };
        let mut img_index: libc::c_int = 0 as libc::c_int;
        while img_index < (*stage).num_images {
            let mut shd_img: *const _sg_shader_image_t = &*((*stage).images)
                .as_ptr()
                .offset(img_index as isize) as *const _sg_shader_image_t;
            if (*shd_img).gl_loc != -(1 as libc::c_int) {
                let mut img: *mut _sg_image_t = *imgs.offset(img_index as isize);
                let gl_tex: GLuint = (*img).gl_tex[(*img).active_slot as usize];
                glUniform1i((*shd_img).gl_loc, (*shd_img).gl_tex_slot);
                _sg_gl_bind_texture((*shd_img).gl_tex_slot, (*img).gl_target, gl_tex);
            }
            img_index += 1;
            img_index;
        }
        stage_index += 1;
        stage_index;
    }
    let gl_ib: GLuint = if !ib.is_null() {
        (*ib).gl_buf[(*ib).active_slot as usize]
    } else {
        0 as libc::c_int as libc::c_uint
    };
    _sg_gl_bind_buffer(0x8893 as libc::c_int as GLenum, gl_ib);
    _sg.gl.cache.cur_ib_offset = ib_offset;
    let mut attr_index: libc::c_int = 0 as libc::c_int;
    while attr_index < SG_MAX_VERTEX_ATTRIBUTES as libc::c_int {
        let mut attr: *mut _sg_gl_attr_t = &mut *((*pip).gl_attrs)
            .as_mut_ptr()
            .offset(attr_index as isize) as *mut _sg_gl_attr_t;
        let mut cache_attr: *mut _sg_gl_cache_attr_t = &mut *(_sg.gl.cache.attrs)
            .as_mut_ptr()
            .offset(attr_index as isize) as *mut _sg_gl_cache_attr_t;
        let mut cache_attr_dirty: bool = 0 as libc::c_int != 0;
        let mut vb_offset: libc::c_int = 0 as libc::c_int;
        let mut gl_vb: GLuint = 0 as libc::c_int as GLuint;
        if (*attr).vb_index as libc::c_int >= 0 as libc::c_int {
            let mut vb: *mut _sg_buffer_t = *vbs.offset((*attr).vb_index as isize);
            gl_vb = (*vb).gl_buf[(*vb).active_slot as usize];
            vb_offset = *vb_offsets.offset((*attr).vb_index as isize) + (*attr).offset;
            if gl_vb != (*cache_attr).gl_vbuf
                || (*attr).size as libc::c_int
                    != (*cache_attr).gl_attr.size as libc::c_int
                || (*attr).type_0 != (*cache_attr).gl_attr.type_0
                || (*attr).normalized as libc::c_int
                    != (*cache_attr).gl_attr.normalized as libc::c_int
                || (*attr).stride as libc::c_int
                    != (*cache_attr).gl_attr.stride as libc::c_int
                || vb_offset != (*cache_attr).gl_attr.offset
                || (*cache_attr).gl_attr.divisor as libc::c_int
                    != (*attr).divisor as libc::c_int
            {
                _sg_gl_bind_buffer(0x8892 as libc::c_int as GLenum, gl_vb);
                glVertexAttribPointer(
                    attr_index as GLuint,
                    (*attr).size as GLint,
                    (*attr).type_0,
                    (*attr).normalized,
                    (*attr).stride as GLsizei,
                    vb_offset as GLintptr as *const libc::c_void,
                );
                if _sg.gl.features[SG_FEATURE_INSTANCING as libc::c_int as usize] {
                    glVertexAttribDivisor(
                        attr_index as GLuint,
                        (*attr).divisor as GLuint,
                    );
                }
                cache_attr_dirty = 1 as libc::c_int != 0;
            }
            if (*cache_attr).gl_attr.vb_index as libc::c_int == -(1 as libc::c_int) {
                glEnableVertexAttribArray(attr_index as GLuint);
                cache_attr_dirty = 1 as libc::c_int != 0;
            }
        } else if (*cache_attr).gl_attr.vb_index as libc::c_int != -(1 as libc::c_int) {
            glDisableVertexAttribArray(attr_index as GLuint);
            cache_attr_dirty = 1 as libc::c_int != 0;
        }
        if cache_attr_dirty {
            (*cache_attr).gl_attr = *attr;
            (*cache_attr).gl_attr.offset = vb_offset;
            (*cache_attr).gl_vbuf = gl_vb;
        }
        attr_index += 1;
        attr_index;
    }
}
pub unsafe extern "C" fn sg_apply_bindings(mut bindings: *const sg_bindings) {
    if !_sg_validate_apply_bindings(bindings) {
        _sg.next_draw_valid = 0 as libc::c_int != 0;
        return;
    }
    _sg.bindings_valid = 1 as libc::c_int != 0;
    let mut pip: *mut _sg_pipeline_t = _sg_lookup_pipeline(
        &mut _sg.pools,
        _sg.cur_pipeline.id,
    );
    let mut vbs: [*mut _sg_buffer_t; 4] = [
        0 as *mut _sg_buffer_t,
        0 as *mut _sg_buffer_t,
        0 as *mut _sg_buffer_t,
        0 as *mut _sg_buffer_t,
    ];
    let mut num_vbs: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < SG_MAX_SHADERSTAGE_BUFFERS as libc::c_int {
        if !((*bindings).vertex_buffers[i as usize].id != 0) {
            break;
        }
        vbs[i
            as usize] = _sg_lookup_buffer(
            &mut _sg.pools,
            (*bindings).vertex_buffers[i as usize].id,
        );
        _sg
            .next_draw_valid = (_sg.next_draw_valid as libc::c_int
            & (SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
                == (*vbs[i as usize]).slot.state as libc::c_uint) as libc::c_int)
            != 0;
        _sg
            .next_draw_valid = (_sg.next_draw_valid as libc::c_int
            & !(*vbs[i as usize]).append_overflow as libc::c_int) != 0;
        i += 1;
        i;
        num_vbs += 1;
        num_vbs;
    }
    let mut ib: *mut _sg_buffer_t = 0 as *mut _sg_buffer_t;
    if (*bindings).index_buffer.id != 0 {
        ib = _sg_lookup_buffer(&mut _sg.pools, (*bindings).index_buffer.id);
        _sg
            .next_draw_valid = (_sg.next_draw_valid as libc::c_int
            & (SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
                == (*ib).slot.state as libc::c_uint) as libc::c_int) != 0;
        _sg
            .next_draw_valid = (_sg.next_draw_valid as libc::c_int
            & !(*ib).append_overflow as libc::c_int) != 0;
    }
    let mut vs_imgs: [*mut _sg_image_t; 12] = [
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
    ];
    let mut num_vs_imgs: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < SG_MAX_SHADERSTAGE_IMAGES as libc::c_int {
        if !((*bindings).vs_images[i_0 as usize].id != 0) {
            break;
        }
        vs_imgs[i_0
            as usize] = _sg_lookup_image(
            &mut _sg.pools,
            (*bindings).vs_images[i_0 as usize].id,
        );
        _sg
            .next_draw_valid = (_sg.next_draw_valid as libc::c_int
            & (SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
                == (*vs_imgs[i_0 as usize]).slot.state as libc::c_uint) as libc::c_int)
            != 0;
        i_0 += 1;
        i_0;
        num_vs_imgs += 1;
        num_vs_imgs;
    }
    let mut fs_imgs: [*mut _sg_image_t; 12] = [
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
        0 as *mut _sg_image_t,
    ];
    let mut num_fs_imgs: libc::c_int = 0 as libc::c_int;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < SG_MAX_SHADERSTAGE_IMAGES as libc::c_int {
        if !((*bindings).fs_images[i_1 as usize].id != 0) {
            break;
        }
        fs_imgs[i_1
            as usize] = _sg_lookup_image(
            &mut _sg.pools,
            (*bindings).fs_images[i_1 as usize].id,
        );
        _sg
            .next_draw_valid = (_sg.next_draw_valid as libc::c_int
            & (SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
                == (*fs_imgs[i_1 as usize]).slot.state as libc::c_uint) as libc::c_int)
            != 0;
        i_1 += 1;
        i_1;
        num_fs_imgs += 1;
        num_fs_imgs;
    }
    if _sg.next_draw_valid {
        let mut vb_offsets: *const libc::c_int = ((*bindings).vertex_buffer_offsets)
            .as_ptr();
        let mut ib_offset: libc::c_int = (*bindings).index_buffer_offset;
        _sg_apply_bindings(
            pip,
            vbs.as_mut_ptr(),
            vb_offsets,
            num_vbs,
            ib,
            ib_offset,
            vs_imgs.as_mut_ptr(),
            num_vs_imgs,
            fs_imgs.as_mut_ptr(),
            num_fs_imgs,
        );
    }
}
unsafe extern "C" fn _sg_validate_apply_pipeline(mut pip_id: sg_pipeline) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_gl_primitive_type(mut t: sg_primitive_type) -> GLenum {
    match t as libc::c_uint {
        1 => return 0 as libc::c_int as GLenum,
        2 => return 0x1 as libc::c_int as GLenum,
        3 => return 0x3 as libc::c_int as GLenum,
        4 => return 0x4 as libc::c_int as GLenum,
        5 => return 0x5 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_index_type(mut t: sg_index_type) -> GLenum {
    match t as libc::c_uint {
        1 => return 0 as libc::c_int as GLenum,
        2 => return 0x1403 as libc::c_int as GLenum,
        3 => return 0x1405 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_compare_func(mut cmp: sg_compare_func) -> GLenum {
    match cmp as libc::c_uint {
        1 => return 0x200 as libc::c_int as GLenum,
        2 => return 0x201 as libc::c_int as GLenum,
        3 => return 0x202 as libc::c_int as GLenum,
        4 => return 0x203 as libc::c_int as GLenum,
        5 => return 0x204 as libc::c_int as GLenum,
        6 => return 0x205 as libc::c_int as GLenum,
        7 => return 0x206 as libc::c_int as GLenum,
        8 => return 0x207 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_stencil_op(mut op: sg_stencil_op) -> GLenum {
    match op as libc::c_uint {
        1 => return 0x1e00 as libc::c_int as GLenum,
        2 => return 0 as libc::c_int as GLenum,
        3 => return 0x1e01 as libc::c_int as GLenum,
        4 => return 0x1e02 as libc::c_int as GLenum,
        5 => return 0x1e03 as libc::c_int as GLenum,
        6 => return 0x150a as libc::c_int as GLenum,
        7 => return 0x8507 as libc::c_int as GLenum,
        8 => return 0x8508 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_blend_factor(mut f: sg_blend_factor) -> GLenum {
    match f as libc::c_uint {
        1 => return 0 as libc::c_int as GLenum,
        2 => return 1 as libc::c_int as GLenum,
        3 => return 0x300 as libc::c_int as GLenum,
        4 => return 0x301 as libc::c_int as GLenum,
        5 => return 0x302 as libc::c_int as GLenum,
        6 => return 0x303 as libc::c_int as GLenum,
        7 => return 0x306 as libc::c_int as GLenum,
        8 => return 0x307 as libc::c_int as GLenum,
        9 => return 0x304 as libc::c_int as GLenum,
        10 => return 0x305 as libc::c_int as GLenum,
        11 => return 0x308 as libc::c_int as GLenum,
        12 => return 0x8001 as libc::c_int as GLenum,
        13 => return 0x8002 as libc::c_int as GLenum,
        14 => return 0x8003 as libc::c_int as GLenum,
        15 => return 0x8004 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_blend_op(mut op: sg_blend_op) -> GLenum {
    match op as libc::c_uint {
        1 => return 0x8006 as libc::c_int as GLenum,
        2 => return 0x800a as libc::c_int as GLenum,
        3 => return 0x800b as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_apply_pipeline(mut pip: *mut _sg_pipeline_t) {
    if _sg.gl.cache.cur_pipeline != pip
        || _sg.gl.cache.cur_pipeline_id.id != (*pip).slot.id
    {
        _sg.gl.cache.cur_pipeline = pip;
        _sg.gl.cache.cur_pipeline_id.id = (*pip).slot.id;
        _sg.gl.cache.cur_primitive_type = _sg_gl_primitive_type((*pip).primitive_type);
        _sg.gl.cache.cur_index_type = _sg_gl_index_type((*pip).index_type);
        let mut new_ds: *const sg_depth_stencil_state = &mut (*pip).depth_stencil;
        let mut cache_ds: *mut sg_depth_stencil_state = &mut _sg.gl.cache.ds;
        if (*new_ds).depth_compare_func as libc::c_uint
            != (*cache_ds).depth_compare_func as libc::c_uint
        {
            (*cache_ds).depth_compare_func = (*new_ds).depth_compare_func;
            glDepthFunc(_sg_gl_compare_func((*new_ds).depth_compare_func));
        }
        if (*new_ds).depth_write_enabled as libc::c_int
            != (*cache_ds).depth_write_enabled as libc::c_int
        {
            (*cache_ds).depth_write_enabled = (*new_ds).depth_write_enabled;
            glDepthMask((*new_ds).depth_write_enabled as GLboolean);
        }
        if (*new_ds).stencil_enabled as libc::c_int
            != (*cache_ds).stencil_enabled as libc::c_int
        {
            (*cache_ds).stencil_enabled = (*new_ds).stencil_enabled;
            if (*new_ds).stencil_enabled {
                glEnable(0xb90 as libc::c_int as GLenum);
            } else {
                glDisable(0xb90 as libc::c_int as GLenum);
            }
        }
        if (*new_ds).stencil_write_mask as libc::c_int
            != (*cache_ds).stencil_write_mask as libc::c_int
        {
            (*cache_ds).stencil_write_mask = (*new_ds).stencil_write_mask;
            glStencilMask((*new_ds).stencil_write_mask as GLuint);
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            let mut new_ss: *const sg_stencil_state = if i == 0 as libc::c_int {
                &(*new_ds).stencil_front
            } else {
                &(*new_ds).stencil_back
            };
            let mut cache_ss: *mut sg_stencil_state = if i == 0 as libc::c_int {
                &mut (*cache_ds).stencil_front
            } else {
                &mut (*cache_ds).stencil_back
            };
            let mut gl_face: GLenum = (if i == 0 as libc::c_int {
                0x404 as libc::c_int
            } else {
                0x405 as libc::c_int
            }) as GLenum;
            if (*new_ss).compare_func as libc::c_uint
                != (*cache_ss).compare_func as libc::c_uint
                || (*new_ds).stencil_read_mask as libc::c_int
                    != (*cache_ds).stencil_read_mask as libc::c_int
                || (*new_ds).stencil_ref as libc::c_int
                    != (*cache_ds).stencil_ref as libc::c_int
            {
                (*cache_ss).compare_func = (*new_ss).compare_func;
                glStencilFuncSeparate(
                    gl_face,
                    _sg_gl_compare_func((*new_ss).compare_func),
                    (*new_ds).stencil_ref as GLint,
                    (*new_ds).stencil_read_mask as GLuint,
                );
            }
            if (*new_ss).fail_op as libc::c_uint != (*cache_ss).fail_op as libc::c_uint
                || (*new_ss).depth_fail_op as libc::c_uint
                    != (*cache_ss).depth_fail_op as libc::c_uint
                || (*new_ss).pass_op as libc::c_uint
                    != (*cache_ss).pass_op as libc::c_uint
            {
                (*cache_ss).fail_op = (*new_ss).fail_op;
                (*cache_ss).depth_fail_op = (*new_ss).depth_fail_op;
                (*cache_ss).pass_op = (*new_ss).pass_op;
                glStencilOpSeparate(
                    gl_face,
                    _sg_gl_stencil_op((*new_ss).fail_op),
                    _sg_gl_stencil_op((*new_ss).depth_fail_op),
                    _sg_gl_stencil_op((*new_ss).pass_op),
                );
            }
            i += 1;
            i;
        }
        (*cache_ds).stencil_read_mask = (*new_ds).stencil_read_mask;
        (*cache_ds).stencil_ref = (*new_ds).stencil_ref;
        let mut new_b: *const sg_blend_state = &mut (*pip).blend;
        let mut cache_b: *mut sg_blend_state = &mut _sg.gl.cache.blend;
        if (*new_b).enabled as libc::c_int != (*cache_b).enabled as libc::c_int {
            (*cache_b).enabled = (*new_b).enabled;
            if (*new_b).enabled {
                glEnable(0xbe2 as libc::c_int as GLenum);
            } else {
                glDisable(0xbe2 as libc::c_int as GLenum);
            }
        }
        if (*new_b).src_factor_rgb as libc::c_uint
            != (*cache_b).src_factor_rgb as libc::c_uint
            || (*new_b).dst_factor_rgb as libc::c_uint
                != (*cache_b).dst_factor_rgb as libc::c_uint
            || (*new_b).src_factor_alpha as libc::c_uint
                != (*cache_b).src_factor_alpha as libc::c_uint
            || (*new_b).dst_factor_alpha as libc::c_uint
                != (*cache_b).dst_factor_alpha as libc::c_uint
        {
            (*cache_b).src_factor_rgb = (*new_b).src_factor_rgb;
            (*cache_b).dst_factor_rgb = (*new_b).dst_factor_rgb;
            (*cache_b).src_factor_alpha = (*new_b).src_factor_alpha;
            (*cache_b).dst_factor_alpha = (*new_b).dst_factor_alpha;
            glBlendFuncSeparate(
                _sg_gl_blend_factor((*new_b).src_factor_rgb),
                _sg_gl_blend_factor((*new_b).dst_factor_rgb),
                _sg_gl_blend_factor((*new_b).src_factor_alpha),
                _sg_gl_blend_factor((*new_b).dst_factor_alpha),
            );
        }
        if (*new_b).op_rgb as libc::c_uint != (*cache_b).op_rgb as libc::c_uint
            || (*new_b).op_alpha as libc::c_uint != (*cache_b).op_alpha as libc::c_uint
        {
            (*cache_b).op_rgb = (*new_b).op_rgb;
            (*cache_b).op_alpha = (*new_b).op_alpha;
            glBlendEquationSeparate(
                _sg_gl_blend_op((*new_b).op_rgb),
                _sg_gl_blend_op((*new_b).op_alpha),
            );
        }
        if (*new_b).color_write_mask as libc::c_int
            != (*cache_b).color_write_mask as libc::c_int
        {
            (*cache_b).color_write_mask = (*new_b).color_write_mask;
            glColorMask(
                ((*new_b).color_write_mask as libc::c_int & SG_COLORMASK_R as libc::c_int
                    != 0 as libc::c_int) as libc::c_int as GLboolean,
                ((*new_b).color_write_mask as libc::c_int & SG_COLORMASK_G as libc::c_int
                    != 0 as libc::c_int) as libc::c_int as GLboolean,
                ((*new_b).color_write_mask as libc::c_int & SG_COLORMASK_B as libc::c_int
                    != 0 as libc::c_int) as libc::c_int as GLboolean,
                ((*new_b).color_write_mask as libc::c_int & SG_COLORMASK_A as libc::c_int
                    != 0 as libc::c_int) as libc::c_int as GLboolean,
            );
        }
        if !((*new_b).blend_color[0 as libc::c_int as usize]
            - (*cache_b).blend_color[0 as libc::c_int as usize] > -0.0001f32
            && (*new_b).blend_color[0 as libc::c_int as usize]
                - (*cache_b).blend_color[0 as libc::c_int as usize] < 0.0001f32)
            || !((*new_b).blend_color[1 as libc::c_int as usize]
                - (*cache_b).blend_color[1 as libc::c_int as usize] > -0.0001f32
                && (*new_b).blend_color[1 as libc::c_int as usize]
                    - (*cache_b).blend_color[1 as libc::c_int as usize] < 0.0001f32)
            || !((*new_b).blend_color[2 as libc::c_int as usize]
                - (*cache_b).blend_color[2 as libc::c_int as usize] > -0.0001f32
                && (*new_b).blend_color[2 as libc::c_int as usize]
                    - (*cache_b).blend_color[2 as libc::c_int as usize] < 0.0001f32)
            || !((*new_b).blend_color[3 as libc::c_int as usize]
                - (*cache_b).blend_color[3 as libc::c_int as usize] > -0.0001f32
                && (*new_b).blend_color[3 as libc::c_int as usize]
                    - (*cache_b).blend_color[3 as libc::c_int as usize] < 0.0001f32)
        {
            let mut bc: *const libc::c_float = ((*new_b).blend_color).as_ptr();
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 4 as libc::c_int {
                (*cache_b).blend_color[i_0 as usize] = *bc.offset(i_0 as isize);
                i_0 += 1;
                i_0;
            }
            glBlendColor(
                *bc.offset(0 as libc::c_int as isize),
                *bc.offset(1 as libc::c_int as isize),
                *bc.offset(2 as libc::c_int as isize),
                *bc.offset(3 as libc::c_int as isize),
            );
        }
        let mut new_r: *const sg_rasterizer_state = &mut (*pip).rast;
        let mut cache_r: *mut sg_rasterizer_state = &mut _sg.gl.cache.rast;
        if (*new_r).cull_mode as libc::c_uint != (*cache_r).cull_mode as libc::c_uint {
            (*cache_r).cull_mode = (*new_r).cull_mode;
            if SG_CULLMODE_NONE as libc::c_int as libc::c_uint
                == (*new_r).cull_mode as libc::c_uint
            {
                glDisable(0xb44 as libc::c_int as GLenum);
            } else {
                glEnable(0xb44 as libc::c_int as GLenum);
                let mut gl_mode: GLenum = (if SG_CULLMODE_FRONT as libc::c_int
                    as libc::c_uint == (*new_r).cull_mode as libc::c_uint
                {
                    0x404 as libc::c_int
                } else {
                    0x405 as libc::c_int
                }) as GLenum;
                glCullFace(gl_mode);
            }
        }
        if (*new_r).face_winding as libc::c_uint
            != (*cache_r).face_winding as libc::c_uint
        {
            (*cache_r).face_winding = (*new_r).face_winding;
            let mut gl_winding: GLenum = (if SG_FACEWINDING_CW as libc::c_int
                as libc::c_uint == (*new_r).face_winding as libc::c_uint
            {
                0x900 as libc::c_int
            } else {
                0x901 as libc::c_int
            }) as GLenum;
            glFrontFace(gl_winding);
        }
        if (*new_r).alpha_to_coverage_enabled as libc::c_int
            != (*cache_r).alpha_to_coverage_enabled as libc::c_int
        {
            (*cache_r).alpha_to_coverage_enabled = (*new_r).alpha_to_coverage_enabled;
            if (*new_r).alpha_to_coverage_enabled {
                glEnable(0x809e as libc::c_int as GLenum);
            } else {
                glDisable(0x809e as libc::c_int as GLenum);
            }
        }
        if (*new_r).sample_count != (*cache_r).sample_count {
            (*cache_r).sample_count = (*new_r).sample_count;
            if (*new_r).sample_count > 1 as libc::c_int {
                glEnable(0x809d as libc::c_int as GLenum);
            } else {
                glDisable(0x809d as libc::c_int as GLenum);
            }
        }
        if !((*new_r).depth_bias - (*cache_r).depth_bias > -0.000001f32
            && (*new_r).depth_bias - (*cache_r).depth_bias < 0.000001f32)
            || !((*new_r).depth_bias_slope_scale - (*cache_r).depth_bias_slope_scale
                > -0.000001f32
                && (*new_r).depth_bias_slope_scale - (*cache_r).depth_bias_slope_scale
                    < 0.000001f32)
        {
            (*cache_r).depth_bias = (*new_r).depth_bias;
            (*cache_r).depth_bias_slope_scale = (*new_r).depth_bias_slope_scale;
            glPolygonOffset((*new_r).depth_bias_slope_scale, (*new_r).depth_bias);
            let mut po_enabled: bool = 1 as libc::c_int != 0;
            if (*new_r).depth_bias - 0.0f32 > -0.000001f32
                && (*new_r).depth_bias - 0.0f32 < 0.000001f32
                && ((*new_r).depth_bias_slope_scale - 0.0f32 > -0.000001f32
                    && (*new_r).depth_bias_slope_scale - 0.0f32 < 0.000001f32)
            {
                po_enabled = 0 as libc::c_int != 0;
            }
            if po_enabled as libc::c_int
                != _sg.gl.cache.polygon_offset_enabled as libc::c_int
            {
                _sg.gl.cache.polygon_offset_enabled = po_enabled;
                if po_enabled {
                    glEnable(0x8037 as libc::c_int as GLenum);
                } else {
                    glDisable(0x8037 as libc::c_int as GLenum);
                }
            }
        }
        glUseProgram((*(*pip).shader).gl_prog);
    }
}
pub unsafe extern "C" fn sg_apply_pipeline(mut pip_id: sg_pipeline) {
    _sg.bindings_valid = 0 as libc::c_int != 0;
    if !_sg_validate_apply_pipeline(pip_id) {
        _sg.next_draw_valid = 0 as libc::c_int != 0;
        return;
    }
    if !_sg.pass_valid {
        return;
    }
    _sg.cur_pipeline = pip_id;
    let mut pip: *mut _sg_pipeline_t = _sg_lookup_pipeline(&mut _sg.pools, pip_id.id);
    _sg
        .next_draw_valid = SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
        == (*pip).slot.state as libc::c_uint;
    _sg_apply_pipeline(pip);
}
unsafe extern "C" fn _sg_apply_scissor_rect(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut origin_top_left: bool,
) {
    y = if origin_top_left as libc::c_int != 0 {
        _sg.gl.cur_pass_height - (y + h)
    } else {
        y
    };
    glScissor(x, y, w, h);
}
pub unsafe extern "C" fn sg_apply_scissor_rect(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut origin_top_left: bool,
) {
    if !_sg.pass_valid {
        return;
    }
    _sg_apply_scissor_rect(x, y, width, height, origin_top_left);
}
unsafe extern "C" fn _sg_apply_viewport(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut origin_top_left: bool,
) {
    y = if origin_top_left as libc::c_int != 0 {
        _sg.gl.cur_pass_height - (y + h)
    } else {
        y
    };
    glViewport(x, y, w, h);
}
pub unsafe extern "C" fn sg_apply_viewport(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut origin_top_left: bool,
) {
    if !_sg.pass_valid {
        return;
    }
    _sg_apply_viewport(x, y, width, height, origin_top_left);
}
unsafe extern "C" fn _sg_validate_begin_pass(mut pass: *mut _sg_pass_t) -> bool {
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn sg_begin_pass(
    mut pass_id: sg_pass,
    mut pass_action: *const sg_pass_action,
) {
    _sg.cur_pass = pass_id;
    let mut pass: *mut _sg_pass_t = _sg_lookup_pass(&mut _sg.pools, pass_id.id);
    if !pass.is_null() && _sg_validate_begin_pass(pass) as libc::c_int != 0 {
        _sg.pass_valid = 1 as libc::c_int != 0;
        let mut pa: sg_pass_action = sg_pass_action {
            _start_canary: 0,
            colors: [sg_color_attachment_action {
                action: _SG_ACTION_DEFAULT,
                val: [0.; 4],
            }; 4],
            depth: sg_depth_attachment_action {
                action: _SG_ACTION_DEFAULT,
                val: 0.,
            },
            stencil: sg_stencil_attachment_action {
                action: _SG_ACTION_DEFAULT,
                val: 0,
            },
            _end_canary: 0,
        };
        _sg_resolve_default_pass_action(pass_action, &mut pa);
        let w: libc::c_int = (*(*pass).color_atts[0 as libc::c_int as usize].image)
            .width;
        let h: libc::c_int = (*(*pass).color_atts[0 as libc::c_int as usize].image)
            .height;
        _sg_begin_pass(pass, &mut pa, w, h);
    } else {
        _sg.pass_valid = 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn _sg_resolve_default_pass_action(
    mut from: *const sg_pass_action,
    mut to: *mut sg_pass_action,
) {
    *to = *from;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < SG_MAX_COLOR_ATTACHMENTS as libc::c_int {
        if (*to).colors[i as usize].action as libc::c_uint
            == _SG_ACTION_DEFAULT as libc::c_int as libc::c_uint
        {
            (*to).colors[i as usize].action = SG_ACTION_CLEAR;
            (*to).colors[i as usize].val[0 as libc::c_int as usize] = 0.5f32;
            (*to).colors[i as usize].val[1 as libc::c_int as usize] = 0.5f32;
            (*to).colors[i as usize].val[2 as libc::c_int as usize] = 0.5f32;
            (*to).colors[i as usize].val[3 as libc::c_int as usize] = 1.0f32;
        }
        i += 1;
        i;
    }
    if (*to).depth.action as libc::c_uint
        == _SG_ACTION_DEFAULT as libc::c_int as libc::c_uint
    {
        (*to).depth.action = SG_ACTION_CLEAR;
        (*to).depth.val = 1.0f32;
    }
    if (*to).stencil.action as libc::c_uint
        == _SG_ACTION_DEFAULT as libc::c_int as libc::c_uint
    {
        (*to).stencil.action = SG_ACTION_CLEAR;
        (*to).stencil.val = 0 as libc::c_int as uint8_t;
    }
}
unsafe extern "C" fn _sg_begin_pass(
    mut pass: *mut _sg_pass_t,
    mut action: *const sg_pass_action,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    _sg.gl.in_pass = 1 as libc::c_int != 0;
    _sg.gl.cur_pass = pass;
    if !pass.is_null() {
        _sg.gl.cur_pass_id.id = (*pass).slot.id;
    } else {
        _sg.gl.cur_pass_id.id = SG_INVALID_ID as libc::c_int as uint32_t;
    }
    _sg.gl.cur_pass_width = w;
    _sg.gl.cur_pass_height = h;
    if !pass.is_null() {
        glBindFramebuffer(0x8d40 as libc::c_int as GLenum, (*pass).gl_fb);
        if !_sg.gl.gles2 {
            let mut att: [GLenum; 4] = [
                0x8ce0 as libc::c_int as GLenum,
                0x8ce1 as libc::c_int as GLenum,
                0x8ce2 as libc::c_int as GLenum,
                0x8ce3 as libc::c_int as GLenum,
            ];
            let mut num_attrs: libc::c_int = 0 as libc::c_int;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < SG_MAX_COLOR_ATTACHMENTS as libc::c_int {
                if ((*pass).color_atts[num_attrs as usize].image).is_null() {
                    break;
                }
                num_attrs += 1;
                num_attrs;
                i += 1;
                i;
            }
            glDrawBuffers(num_attrs, att.as_mut_ptr());
        }
    } else {
        glBindFramebuffer(
            0x8d40 as libc::c_int as GLenum,
            (*_sg.gl.cur_context).default_framebuffer,
        );
    }
    glViewport(0 as libc::c_int, 0 as libc::c_int, w, h);
    glScissor(0 as libc::c_int, 0 as libc::c_int, w, h);
    let mut need_pip_cache_flush: bool = 0 as libc::c_int != 0;
    if _sg.gl.cache.blend.color_write_mask as libc::c_int
        != SG_COLORMASK_RGBA as libc::c_int
    {
        need_pip_cache_flush = 1 as libc::c_int != 0;
        _sg
            .gl
            .cache
            .blend
            .color_write_mask = SG_COLORMASK_RGBA as libc::c_int as uint8_t;
        glColorMask(
            1 as libc::c_int as GLboolean,
            1 as libc::c_int as GLboolean,
            1 as libc::c_int as GLboolean,
            1 as libc::c_int as GLboolean,
        );
    }
    if !_sg.gl.cache.ds.depth_write_enabled {
        need_pip_cache_flush = 1 as libc::c_int != 0;
        _sg.gl.cache.ds.depth_write_enabled = 1 as libc::c_int != 0;
        glDepthMask(1 as libc::c_int as GLboolean);
    }
    if _sg.gl.cache.ds.depth_compare_func as libc::c_uint
        != SG_COMPAREFUNC_ALWAYS as libc::c_int as libc::c_uint
    {
        need_pip_cache_flush = 1 as libc::c_int != 0;
        _sg.gl.cache.ds.depth_compare_func = SG_COMPAREFUNC_ALWAYS;
        glDepthFunc(0x207 as libc::c_int as GLenum);
    }
    if _sg.gl.cache.ds.stencil_write_mask as libc::c_int != 0xff as libc::c_int {
        need_pip_cache_flush = 1 as libc::c_int != 0;
        _sg.gl.cache.ds.stencil_write_mask = 0xff as libc::c_int as uint8_t;
        glStencilMask(0xff as libc::c_int as GLuint);
    }
    if need_pip_cache_flush {
        _sg.gl.cache.cur_pipeline = 0 as *mut _sg_pipeline_t;
        _sg.gl.cache.cur_pipeline_id.id = SG_INVALID_ID as libc::c_int as uint32_t;
    }
    let mut use_mrt_clear: bool = !pass.is_null();
    if _sg.gl.gles2 {
        use_mrt_clear = 0 as libc::c_int != 0;
    }
    if !use_mrt_clear {
        let mut clear_mask: GLbitfield = 0 as libc::c_int as GLbitfield;
        if (*action).colors[0 as libc::c_int as usize].action as libc::c_uint
            == SG_ACTION_CLEAR as libc::c_int as libc::c_uint
        {
            clear_mask |= 0x4000 as libc::c_int as libc::c_uint;
            let mut c: *const libc::c_float = ((*action)
                .colors[0 as libc::c_int as usize]
                .val)
                .as_ptr();
            glClearColor(
                *c.offset(0 as libc::c_int as isize),
                *c.offset(1 as libc::c_int as isize),
                *c.offset(2 as libc::c_int as isize),
                *c.offset(3 as libc::c_int as isize),
            );
        }
        if (*action).depth.action as libc::c_uint
            == SG_ACTION_CLEAR as libc::c_int as libc::c_uint
        {
            clear_mask |= 0x100 as libc::c_int as libc::c_uint;
            glClearDepth((*action).depth.val as GLclampd);
        }
        if (*action).stencil.action as libc::c_uint
            == SG_ACTION_CLEAR as libc::c_int as libc::c_uint
        {
            clear_mask |= 0x400 as libc::c_int as libc::c_uint;
            glClearStencil((*action).stencil.val as GLint);
        }
        if 0 as libc::c_int as libc::c_uint != clear_mask {
            glClear(clear_mask);
        }
    } else {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < SG_MAX_COLOR_ATTACHMENTS as libc::c_int {
            if ((*pass).color_atts[i_0 as usize].image).is_null() {
                break;
            }
            if (*action).colors[i_0 as usize].action as libc::c_uint
                == SG_ACTION_CLEAR as libc::c_int as libc::c_uint
            {
                glClearBufferfv(
                    0x1800 as libc::c_int as GLenum,
                    i_0,
                    ((*action).colors[i_0 as usize].val).as_ptr(),
                );
            }
            i_0 += 1;
            i_0;
        }
        if !((*pass).ds_att.image).is_null() {
            if (*action).depth.action as libc::c_uint
                == SG_ACTION_CLEAR as libc::c_int as libc::c_uint
                && (*action).stencil.action as libc::c_uint
                    == SG_ACTION_CLEAR as libc::c_int as libc::c_uint
            {
                glClearBufferfi(
                    0x84f9 as libc::c_int as GLenum,
                    0 as libc::c_int,
                    (*action).depth.val,
                    (*action).stencil.val as GLint,
                );
            } else if (*action).depth.action as libc::c_uint
                == SG_ACTION_CLEAR as libc::c_int as libc::c_uint
            {
                glClearBufferfv(
                    0x1801 as libc::c_int as GLenum,
                    0 as libc::c_int,
                    &(*action).depth.val,
                );
            } else if (*action).stencil.action as libc::c_uint
                == SG_ACTION_CLEAR as libc::c_int as libc::c_uint
            {
                let mut val: GLuint = (*action).stencil.val as GLuint;
                glClearBufferuiv(
                    0x1802 as libc::c_int as GLenum,
                    0 as libc::c_int,
                    &mut val,
                );
            }
        }
    };
}
pub unsafe extern "C" fn sg_begin_default_pass(
    mut pass_action: *const sg_pass_action,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut pa: sg_pass_action = sg_pass_action {
        _start_canary: 0,
        colors: [sg_color_attachment_action {
            action: _SG_ACTION_DEFAULT,
            val: [0.; 4],
        }; 4],
        depth: sg_depth_attachment_action {
            action: _SG_ACTION_DEFAULT,
            val: 0.,
        },
        stencil: sg_stencil_attachment_action {
            action: _SG_ACTION_DEFAULT,
            val: 0,
        },
        _end_canary: 0,
    };
    _sg_resolve_default_pass_action(pass_action, &mut pa);
    _sg.cur_pass.id = SG_INVALID_ID as libc::c_int as uint32_t;
    _sg.pass_valid = 1 as libc::c_int != 0;
    _sg_begin_pass(0 as *mut _sg_pass_t, &mut pa, width, height);
}
pub unsafe extern "C" fn sg_query_pass_state(mut pass_id: sg_pass) -> sg_resource_state {
    let mut pass: *mut _sg_pass_t = _sg_lookup_pass(&mut _sg.pools, pass_id.id);
    let mut res: sg_resource_state = (if !pass.is_null() {
        (*pass).slot.state as libc::c_uint
    } else {
        SG_RESOURCESTATE_INVALID as libc::c_int as libc::c_uint
    }) as sg_resource_state;
    return res;
}
pub unsafe extern "C" fn sg_query_pipeline_state(
    mut pip_id: sg_pipeline,
) -> sg_resource_state {
    let mut pip: *mut _sg_pipeline_t = _sg_lookup_pipeline(&mut _sg.pools, pip_id.id);
    let mut res: sg_resource_state = (if !pip.is_null() {
        (*pip).slot.state as libc::c_uint
    } else {
        SG_RESOURCESTATE_INVALID as libc::c_int as libc::c_uint
    }) as sg_resource_state;
    return res;
}
pub unsafe extern "C" fn sg_query_shader_state(
    mut shd_id: sg_shader,
) -> sg_resource_state {
    let mut shd: *mut _sg_shader_t = _sg_lookup_shader(&mut _sg.pools, shd_id.id);
    let mut res: sg_resource_state = (if !shd.is_null() {
        (*shd).slot.state as libc::c_uint
    } else {
        SG_RESOURCESTATE_INVALID as libc::c_int as libc::c_uint
    }) as sg_resource_state;
    return res;
}
pub unsafe extern "C" fn sg_query_image_state(
    mut img_id: sg_image,
) -> sg_resource_state {
    let mut img: *mut _sg_image_t = _sg_lookup_image(&mut _sg.pools, img_id.id);
    let mut res: sg_resource_state = (if !img.is_null() {
        (*img).slot.state as libc::c_uint
    } else {
        SG_RESOURCESTATE_INVALID as libc::c_int as libc::c_uint
    }) as sg_resource_state;
    return res;
}
pub unsafe extern "C" fn sg_query_buffer_state(
    mut buf_id: sg_buffer,
) -> sg_resource_state {
    let mut buf: *mut _sg_buffer_t = _sg_lookup_buffer(&mut _sg.pools, buf_id.id);
    let mut res: sg_resource_state = (if !buf.is_null() {
        (*buf).slot.state as libc::c_uint
    } else {
        SG_RESOURCESTATE_INVALID as libc::c_int as libc::c_uint
    }) as sg_resource_state;
    return res;
}
pub unsafe extern "C" fn sg_query_buffer_overflow(mut buf_id: sg_buffer) -> bool {
    let mut buf: *mut _sg_buffer_t = _sg_lookup_buffer(&mut _sg.pools, buf_id.id);
    let mut result: bool = if !buf.is_null() {
        (*buf).append_overflow as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
    return result;
}
unsafe extern "C" fn _sg_validate_append_buffer(
    mut buf: *const _sg_buffer_t,
    mut data: *const libc::c_void,
    mut size: libc::c_int,
) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_append_buffer(
    mut buf: *mut _sg_buffer_t,
    mut data_ptr: *const libc::c_void,
    mut data_size: libc::c_int,
    mut new_frame: bool,
) {
    if new_frame {
        (*buf).active_slot += 1;
        if (*buf).active_slot >= (*buf).num_slots {
            (*buf).active_slot = 0 as libc::c_int;
        }
    }
    let mut gl_tgt: GLenum = _sg_gl_buffer_target((*buf).type_0);
    let mut gl_buf: GLuint = (*buf).gl_buf[(*buf).active_slot as usize];
    _sg_gl_store_buffer_binding(gl_tgt);
    _sg_gl_bind_buffer(gl_tgt, gl_buf);
    glBufferSubData(
        gl_tgt,
        (*buf).append_pos as GLintptr,
        data_size as GLsizeiptr,
        data_ptr,
    );
    _sg_gl_restore_buffer_binding(gl_tgt);
}
pub unsafe extern "C" fn sg_append_buffer(
    mut buf_id: sg_buffer,
    mut data: *const libc::c_void,
    mut num_bytes: libc::c_int,
) -> libc::c_int {
    let mut buf: *mut _sg_buffer_t = _sg_lookup_buffer(&mut _sg.pools, buf_id.id);
    let mut result: libc::c_int = 0;
    if !buf.is_null() {
        if (*buf).append_frame_index != _sg.frame_index {
            (*buf).append_pos = 0 as libc::c_int;
            (*buf).append_overflow = 0 as libc::c_int != 0;
        }
        if (*buf).append_pos + num_bytes > (*buf).size {
            (*buf).append_overflow = 1 as libc::c_int != 0;
        }
        let start_pos: libc::c_int = (*buf).append_pos;
        if (*buf).slot.state as libc::c_uint
            == SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
        {
            if _sg_validate_append_buffer(buf, data, num_bytes) {
                if !(*buf).append_overflow && num_bytes > 0 as libc::c_int {
                    _sg_append_buffer(
                        buf,
                        data,
                        num_bytes,
                        (*buf).append_frame_index != _sg.frame_index,
                    );
                    (*buf).append_pos += num_bytes;
                    (*buf).append_frame_index = _sg.frame_index;
                }
            }
        }
        result = start_pos;
    } else {
        result = 0 as libc::c_int;
    }
    return result;
}
unsafe extern "C" fn _sg_validate_update_image(
    mut img: *const _sg_image_t,
    mut data: *const sg_image_content,
) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_update_image(
    mut img: *mut _sg_image_t,
    mut data: *const sg_image_content,
) {
    (*img).active_slot += 1;
    if (*img).active_slot >= (*img).num_slots {
        (*img).active_slot = 0 as libc::c_int;
    }
    _sg_gl_store_texture_binding(0 as libc::c_int);
    _sg_gl_bind_texture(
        0 as libc::c_int,
        (*img).gl_target,
        (*img).gl_tex[(*img).active_slot as usize],
    );
    let gl_img_format: GLenum = _sg_gl_teximage_format((*img).pixel_format);
    let gl_img_type: GLenum = _sg_gl_teximage_type((*img).pixel_format);
    let num_faces: libc::c_int = if (*img).type_0 as libc::c_uint
        == SG_IMAGETYPE_CUBE as libc::c_int as libc::c_uint
    {
        6 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let num_mips: libc::c_int = (*img).num_mipmaps;
    let mut face_index: libc::c_int = 0 as libc::c_int;
    while face_index < num_faces {
        let mut mip_index: libc::c_int = 0 as libc::c_int;
        while mip_index < num_mips {
            let mut gl_img_target: GLenum = (*img).gl_target;
            if SG_IMAGETYPE_CUBE as libc::c_int as libc::c_uint
                == (*img).type_0 as libc::c_uint
            {
                gl_img_target = _sg_gl_cubeface_target(face_index);
            }
            let mut data_ptr: *const libc::c_void = (*data)
                .subimage[face_index as usize][mip_index as usize]
                .ptr;
            let mut mip_width: libc::c_int = (*img).width >> mip_index;
            if mip_width == 0 as libc::c_int {
                mip_width = 1 as libc::c_int;
            }
            let mut mip_height: libc::c_int = (*img).height >> mip_index;
            if mip_height == 0 as libc::c_int {
                mip_height = 1 as libc::c_int;
            }
            if SG_IMAGETYPE_2D as libc::c_int as libc::c_uint
                == (*img).type_0 as libc::c_uint
                || SG_IMAGETYPE_CUBE as libc::c_int as libc::c_uint
                    == (*img).type_0 as libc::c_uint
            {
                glTexSubImage2D(
                    gl_img_target,
                    mip_index,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    mip_width,
                    mip_height,
                    gl_img_format,
                    gl_img_type,
                    data_ptr,
                );
            } else if !_sg.gl.gles2
                && (SG_IMAGETYPE_3D as libc::c_int as libc::c_uint
                    == (*img).type_0 as libc::c_uint
                    || SG_IMAGETYPE_ARRAY as libc::c_int as libc::c_uint
                        == (*img).type_0 as libc::c_uint)
            {
                let mut mip_depth: libc::c_int = (*img).depth >> mip_index;
                if mip_depth == 0 as libc::c_int {
                    mip_depth = 1 as libc::c_int;
                }
                glTexSubImage3D(
                    gl_img_target,
                    mip_index,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    mip_width,
                    mip_height,
                    mip_depth,
                    gl_img_format,
                    gl_img_type,
                    data_ptr,
                );
            }
            mip_index += 1;
            mip_index;
        }
        face_index += 1;
        face_index;
    }
    _sg_gl_restore_texture_binding(0 as libc::c_int);
}
pub unsafe extern "C" fn sg_update_image(
    mut img_id: sg_image,
    mut data: *const sg_image_content,
) {
    let mut img: *mut _sg_image_t = _sg_lookup_image(&mut _sg.pools, img_id.id);
    if !img.is_null()
        && (*img).slot.state as libc::c_uint
            == SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
    {
        if _sg_validate_update_image(img, data) {
            _sg_update_image(img, data);
            (*img).upd_frame_index = _sg.frame_index;
        }
    }
}
unsafe extern "C" fn _sg_validate_update_buffer(
    mut buf: *const _sg_buffer_t,
    mut data: *const libc::c_void,
    mut size: libc::c_int,
) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_update_buffer(
    mut buf: *mut _sg_buffer_t,
    mut data_ptr: *const libc::c_void,
    mut data_size: libc::c_int,
) {
    (*buf).active_slot += 1;
    if (*buf).active_slot >= (*buf).num_slots {
        (*buf).active_slot = 0 as libc::c_int;
    }
    let mut gl_tgt: GLenum = _sg_gl_buffer_target((*buf).type_0);
    let mut gl_buf: GLuint = (*buf).gl_buf[(*buf).active_slot as usize];
    _sg_gl_store_buffer_binding(gl_tgt);
    _sg_gl_bind_buffer(gl_tgt, gl_buf);
    glBufferSubData(
        gl_tgt,
        0 as libc::c_int as GLintptr,
        data_size as GLsizeiptr,
        data_ptr,
    );
    _sg_gl_restore_buffer_binding(gl_tgt);
}
pub unsafe extern "C" fn sg_update_buffer(
    mut buf_id: sg_buffer,
    mut data: *const libc::c_void,
    mut num_bytes: libc::c_int,
) {
    let mut buf: *mut _sg_buffer_t = _sg_lookup_buffer(&mut _sg.pools, buf_id.id);
    if num_bytes > 0 as libc::c_int && !buf.is_null()
        && (*buf).slot.state as libc::c_uint
            == SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
    {
        if _sg_validate_update_buffer(buf, data, num_bytes) {
            _sg_update_buffer(buf, data, num_bytes);
            (*buf).update_frame_index = _sg.frame_index;
        }
    }
}
unsafe extern "C" fn _sg_reset_pass(mut pass: *mut _sg_pass_t) {
    memset(
        pass as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_sg_pass_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn sg_destroy_pass(mut pass_id: sg_pass) {
    let mut pass: *mut _sg_pass_t = _sg_lookup_pass(&mut _sg.pools, pass_id.id);
    if !pass.is_null() {
        if (*pass).slot.ctx_id == _sg.active_context.id {
            _sg_destroy_pass(pass);
            _sg_reset_pass(pass);
            _sg_pool_free_index(&mut _sg.pools.pass_pool, _sg_slot_index(pass_id.id));
        }
    }
}
unsafe extern "C" fn _sg_reset_pipeline(mut pip: *mut _sg_pipeline_t) {
    memset(
        pip as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_sg_pipeline_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn sg_destroy_pipeline(mut pip_id: sg_pipeline) {
    let mut pip: *mut _sg_pipeline_t = _sg_lookup_pipeline(&mut _sg.pools, pip_id.id);
    if !pip.is_null() {
        if (*pip).slot.ctx_id == _sg.active_context.id {
            _sg_destroy_pipeline(pip);
            _sg_reset_pipeline(pip);
            _sg_pool_free_index(&mut _sg.pools.pipeline_pool, _sg_slot_index(pip_id.id));
        }
    }
}
unsafe extern "C" fn _sg_reset_shader(mut shd: *mut _sg_shader_t) {
    memset(
        shd as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_sg_shader_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn sg_destroy_shader(mut shd_id: sg_shader) {
    let mut shd: *mut _sg_shader_t = _sg_lookup_shader(&mut _sg.pools, shd_id.id);
    if !shd.is_null() {
        if (*shd).slot.ctx_id == _sg.active_context.id {
            _sg_destroy_shader(shd);
            _sg_reset_shader(shd);
            _sg_pool_free_index(&mut _sg.pools.shader_pool, _sg_slot_index(shd_id.id));
        }
    }
}
unsafe extern "C" fn _sg_reset_image(mut img: *mut _sg_image_t) {
    memset(
        img as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_sg_image_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn sg_destroy_image(mut img_id: sg_image) {
    let mut img: *mut _sg_image_t = _sg_lookup_image(&mut _sg.pools, img_id.id);
    if !img.is_null() {
        if (*img).slot.ctx_id == _sg.active_context.id {
            _sg_destroy_image(img);
            _sg_reset_image(img);
            _sg_pool_free_index(&mut _sg.pools.image_pool, _sg_slot_index(img_id.id));
        }
    }
}
unsafe extern "C" fn _sg_reset_buffer(mut buf: *mut _sg_buffer_t) {
    memset(
        buf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_sg_buffer_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn _sg_pool_free_index(
    mut pool: *mut _sg_pool_t,
    mut slot_index: libc::c_int,
) {
    let fresh0 = (*pool).queue_top;
    (*pool).queue_top = (*pool).queue_top + 1;
    *((*pool).free_queue).offset(fresh0 as isize) = slot_index;
}
pub unsafe extern "C" fn sg_destroy_buffer(mut buf_id: sg_buffer) {
    let mut buf: *mut _sg_buffer_t = _sg_lookup_buffer(&mut _sg.pools, buf_id.id);
    if !buf.is_null() {
        if (*buf).slot.ctx_id == _sg.active_context.id {
            _sg_destroy_buffer(buf);
            _sg_reset_buffer(buf);
            _sg_pool_free_index(&mut _sg.pools.buffer_pool, _sg_slot_index(buf_id.id));
        }
    }
}
unsafe extern "C" fn _sg_validate_pass_desc(mut desc: *const sg_pass_desc) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_is_depth_stencil_format(mut fmt: sg_pixel_format) -> bool {
    return SG_PIXELFORMAT_DEPTHSTENCIL as libc::c_int as libc::c_uint
        == fmt as libc::c_uint;
}
unsafe extern "C" fn _sg_create_pass(
    mut pass: *mut _sg_pass_t,
    mut att_images: *mut *mut _sg_image_t,
    mut desc: *const sg_pass_desc,
) -> sg_resource_state {
    let mut att_desc: *const sg_attachment_desc = 0 as *const sg_attachment_desc;
    let mut att: *mut _sg_attachment_t = 0 as *mut _sg_attachment_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < SG_MAX_COLOR_ATTACHMENTS as libc::c_int {
        att_desc = &*((*desc).color_attachments).as_ptr().offset(i as isize)
            as *const sg_attachment_desc;
        if (*att_desc).image.id != SG_INVALID_ID as libc::c_int as libc::c_uint {
            (*pass).num_color_atts += 1;
            (*pass).num_color_atts;
            att = &mut *((*pass).color_atts).as_mut_ptr().offset(i as isize)
                as *mut _sg_attachment_t;
            (*att).image = *att_images.offset(i as isize);
            (*att).image_id = (*att_desc).image;
            (*att).mip_level = (*att_desc).mip_level;
            (*att).slice = (*att_desc).c2rust_unnamed.slice;
        }
        i += 1;
        i;
    }
    att_desc = &(*desc).depth_stencil_attachment;
    let ds_img_index: libc::c_int = SG_MAX_COLOR_ATTACHMENTS as libc::c_int;
    if (*att_desc).image.id != SG_INVALID_ID as libc::c_int as libc::c_uint {
        att = &mut (*pass).ds_att;
        (*att).image = *att_images.offset(ds_img_index as isize);
        (*att).image_id = (*att_desc).image;
        (*att).mip_level = (*att_desc).mip_level;
        (*att).slice = (*att_desc).c2rust_unnamed.slice;
    }
    let mut gl_orig_fb: GLuint = 0;
    glGetIntegerv(
        0x8ca6 as libc::c_int as GLenum,
        &mut gl_orig_fb as *mut GLuint as *mut GLint,
    );
    glGenFramebuffers(1 as libc::c_int, &mut (*pass).gl_fb);
    glBindFramebuffer(0x8d40 as libc::c_int as GLenum, (*pass).gl_fb);
    let is_msaa: bool = 0 as libc::c_int as libc::c_uint
        != (**att_images.offset(0 as libc::c_int as isize)).gl_msaa_render_buffer;
    if is_msaa {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < SG_MAX_COLOR_ATTACHMENTS as libc::c_int {
            let mut att_img: *const _sg_image_t = (*pass).color_atts[i_0 as usize].image;
            if !att_img.is_null() {
                let gl_render_buffer: GLuint = (*att_img).gl_msaa_render_buffer;
                glFramebufferRenderbuffer(
                    0x8d40 as libc::c_int as GLenum,
                    (0x8ce0 as libc::c_int + i_0) as GLenum,
                    0x8d41 as libc::c_int as GLenum,
                    gl_render_buffer,
                );
            }
            i_0 += 1;
            i_0;
        }
    } else {
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < SG_MAX_COLOR_ATTACHMENTS as libc::c_int {
            let mut att_img_0: *const _sg_image_t = (*pass)
                .color_atts[i_1 as usize]
                .image;
            let mip_level: libc::c_int = (*pass).color_atts[i_1 as usize].mip_level;
            let slice: libc::c_int = (*pass).color_atts[i_1 as usize].slice;
            if !att_img_0.is_null() {
                let gl_tex: GLuint = (*att_img_0).gl_tex[0 as libc::c_int as usize];
                let gl_att: GLenum = (0x8ce0 as libc::c_int + i_1) as GLenum;
                match (*att_img_0).type_0 as libc::c_uint {
                    1 => {
                        glFramebufferTexture2D(
                            0x8d40 as libc::c_int as GLenum,
                            gl_att,
                            0xde1 as libc::c_int as GLenum,
                            gl_tex,
                            mip_level,
                        );
                    }
                    2 => {
                        glFramebufferTexture2D(
                            0x8d40 as libc::c_int as GLenum,
                            gl_att,
                            _sg_gl_cubeface_target(slice),
                            gl_tex,
                            mip_level,
                        );
                    }
                    _ => {
                        if !_sg.gl.gles2 {
                            glFramebufferTextureLayer(
                                0x8d40 as libc::c_int as GLenum,
                                gl_att,
                                gl_tex,
                                mip_level,
                                slice,
                            );
                        }
                    }
                }
            }
            i_1 += 1;
            i_1;
        }
    }
    if !((*pass).ds_att.image).is_null() {
        let gl_render_buffer_0: GLuint = (*(*pass).ds_att.image).gl_depth_render_buffer;
        glFramebufferRenderbuffer(
            0x8d40 as libc::c_int as GLenum,
            0x8d00 as libc::c_int as GLenum,
            0x8d41 as libc::c_int as GLenum,
            gl_render_buffer_0,
        );
        if _sg_is_depth_stencil_format((*(*pass).ds_att.image).pixel_format) {
            glFramebufferRenderbuffer(
                0x8d40 as libc::c_int as GLenum,
                0x8d20 as libc::c_int as GLenum,
                0x8d41 as libc::c_int as GLenum,
                gl_render_buffer_0,
            );
        }
    }
    if glCheckFramebufferStatus(0x8d40 as libc::c_int as GLenum)
        != 0x8cd5 as libc::c_int as libc::c_uint
    {
        return SG_RESOURCESTATE_FAILED;
    }
    if is_msaa {
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < SG_MAX_COLOR_ATTACHMENTS as libc::c_int {
            att = &mut *((*pass).color_atts).as_mut_ptr().offset(i_2 as isize)
                as *mut _sg_attachment_t;
            if !((*att).image).is_null() {
                glGenFramebuffers(1 as libc::c_int, &mut (*att).gl_msaa_resolve_buffer);
                glBindFramebuffer(
                    0x8d40 as libc::c_int as GLenum,
                    (*att).gl_msaa_resolve_buffer,
                );
                let gl_tex_0: GLuint = (*(*att).image).gl_tex[0 as libc::c_int as usize];
                match (*(*att).image).type_0 as libc::c_uint {
                    1 => {
                        glFramebufferTexture2D(
                            0x8d40 as libc::c_int as GLenum,
                            0x8ce0 as libc::c_int as GLenum,
                            0xde1 as libc::c_int as GLenum,
                            gl_tex_0,
                            (*att).mip_level,
                        );
                    }
                    2 => {
                        glFramebufferTexture2D(
                            0x8d40 as libc::c_int as GLenum,
                            0x8ce0 as libc::c_int as GLenum,
                            _sg_gl_cubeface_target((*att).slice),
                            gl_tex_0,
                            (*att).mip_level,
                        );
                    }
                    _ => {
                        if !_sg.gl.gles2 {
                            glFramebufferTextureLayer(
                                0x8d40 as libc::c_int as GLenum,
                                0x8ce0 as libc::c_int as GLenum,
                                gl_tex_0,
                                (*att).mip_level,
                                (*att).slice,
                            );
                        }
                    }
                }
                if glCheckFramebufferStatus(0x8d40 as libc::c_int as GLenum)
                    != 0x8cd5 as libc::c_int as libc::c_uint
                {
                    return SG_RESOURCESTATE_FAILED;
                }
            }
            i_2 += 1;
            i_2;
        }
    }
    glBindFramebuffer(0x8d40 as libc::c_int as GLenum, gl_orig_fb);
    return SG_RESOURCESTATE_VALID;
}
unsafe extern "C" fn _sg_pass_at(
    mut p: *const _sg_pools_t,
    mut pass_id: uint32_t,
) -> *mut _sg_pass_t {
    let mut slot_index: libc::c_int = _sg_slot_index(pass_id);
    return &mut *((*p).passes).offset(slot_index as isize) as *mut _sg_pass_t;
}
unsafe extern "C" fn _sg_lookup_pass(
    mut p: *const _sg_pools_t,
    mut pass_id: uint32_t,
) -> *mut _sg_pass_t {
    if SG_INVALID_ID as libc::c_int as libc::c_uint != pass_id {
        let mut pass: *mut _sg_pass_t = _sg_pass_at(p, pass_id);
        if (*pass).slot.id == pass_id {
            return pass;
        }
    }
    return 0 as *mut _sg_pass_t;
}
unsafe extern "C" fn _sg_init_pass(mut pass_id: sg_pass, mut desc: *const sg_pass_desc) {
    let mut pass: *mut _sg_pass_t = _sg_lookup_pass(&mut _sg.pools, pass_id.id);
    (*pass).slot.ctx_id = _sg.active_context.id;
    if _sg_validate_pass_desc(desc) {
        let mut att_imgs: [*mut _sg_image_t; 5] = [0 as *mut _sg_image_t; 5];
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < SG_MAX_COLOR_ATTACHMENTS as libc::c_int {
            if (*desc).color_attachments[i as usize].image.id != 0 {
                att_imgs[i
                    as usize] = _sg_lookup_image(
                    &mut _sg.pools,
                    (*desc).color_attachments[i as usize].image.id,
                );
            } else {
                att_imgs[i as usize] = 0 as *mut _sg_image_t;
            }
            i += 1;
            i;
        }
        let ds_att_index: libc::c_int = SG_MAX_COLOR_ATTACHMENTS as libc::c_int;
        if (*desc).depth_stencil_attachment.image.id != 0 {
            att_imgs[ds_att_index
                as usize] = _sg_lookup_image(
                &mut _sg.pools,
                (*desc).depth_stencil_attachment.image.id,
            );
        } else {
            att_imgs[ds_att_index as usize] = 0 as *mut _sg_image_t;
        }
        (*pass).slot.state = _sg_create_pass(pass, att_imgs.as_mut_ptr(), desc);
    } else {
        (*pass).slot.state = SG_RESOURCESTATE_FAILED;
    };
}
unsafe extern "C" fn _sg_pass_desc_defaults(
    mut desc: *const sg_pass_desc,
) -> sg_pass_desc {
    let mut def: sg_pass_desc = *desc;
    return def;
}
unsafe extern "C" fn _sg_alloc_pass() -> sg_pass {
    let mut res: sg_pass = sg_pass { id: 0 };
    let mut slot_index: libc::c_int = _sg_pool_alloc_index(&mut _sg.pools.pass_pool);
    if 0 as libc::c_int != slot_index {
        res
            .id = _sg_slot_alloc(
            &mut _sg.pools.pass_pool,
            &mut (*(_sg.pools.passes).offset(slot_index as isize)).slot,
            slot_index,
        );
    } else {
        res.id = SG_INVALID_ID as libc::c_int as uint32_t;
    }
    return res;
}
pub unsafe extern "C" fn sg_make_pass(mut desc: *const sg_pass_desc) -> sg_pass {
    let mut desc_def: sg_pass_desc = _sg_pass_desc_defaults(desc);
    let mut pass_id: sg_pass = _sg_alloc_pass();
    if pass_id.id != SG_INVALID_ID as libc::c_int as libc::c_uint {
        _sg_init_pass(pass_id, &mut desc_def);
    }
    return pass_id;
}
unsafe extern "C" fn _sg_validate_pipeline_desc(
    mut desc: *const sg_pipeline_desc,
) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_strempty(mut str: *const _sg_str_t) -> bool {
    return 0 as libc::c_int == (*str).buf[0 as libc::c_int as usize] as libc::c_int;
}
unsafe extern "C" fn _sg_strptr(mut str: *const _sg_str_t) -> *const libc::c_char {
    return &*((*str).buf).as_ptr().offset(0 as libc::c_int as isize)
        as *const libc::c_char;
}
unsafe extern "C" fn _sg_gl_vertexformat_size(mut fmt: sg_vertex_format) -> GLint {
    match fmt as libc::c_uint {
        1 => return 1 as libc::c_int,
        2 => return 2 as libc::c_int,
        3 => return 3 as libc::c_int,
        4 => return 4 as libc::c_int,
        5 => return 4 as libc::c_int,
        6 => return 4 as libc::c_int,
        7 => return 4 as libc::c_int,
        8 => return 4 as libc::c_int,
        9 => return 2 as libc::c_int,
        10 => return 2 as libc::c_int,
        11 => return 4 as libc::c_int,
        12 => return 4 as libc::c_int,
        13 => return 4 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn _sg_gl_vertexformat_type(mut fmt: sg_vertex_format) -> GLenum {
    match fmt as libc::c_uint {
        1 | 2 | 3 | 4 => return 0x1406 as libc::c_int as GLenum,
        5 | 6 => return 0x1400 as libc::c_int as GLenum,
        7 | 8 => return 0x1401 as libc::c_int as GLenum,
        9 | 10 | 11 | 12 => return 0x1402 as libc::c_int as GLenum,
        13 => return 0x8368 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_vertexformat_normalized(
    mut fmt: sg_vertex_format,
) -> GLboolean {
    match fmt as libc::c_uint {
        6 | 8 | 10 | 12 | 13 => return 1 as libc::c_int as GLboolean,
        _ => return 0 as libc::c_int as GLboolean,
    };
}
unsafe extern "C" fn _sg_create_pipeline(
    mut pip: *mut _sg_pipeline_t,
    mut shd: *mut _sg_shader_t,
    mut desc: *const sg_pipeline_desc,
) -> sg_resource_state {
    (*pip).shader = shd;
    (*pip).shader_id = (*desc).shader;
    (*pip).primitive_type = (*desc).primitive_type;
    (*pip).index_type = (*desc).index_type;
    (*pip).color_attachment_count = (*desc).blend.color_attachment_count;
    (*pip).color_format = (*desc).blend.color_format;
    (*pip).depth_format = (*desc).blend.depth_format;
    (*pip).sample_count = (*desc).rasterizer.sample_count;
    (*pip).depth_stencil = (*desc).depth_stencil;
    (*pip).blend = (*desc).blend;
    (*pip).rast = (*desc).rasterizer;
    let mut attr_index: libc::c_int = 0 as libc::c_int;
    while attr_index < SG_MAX_VERTEX_ATTRIBUTES as libc::c_int {
        (*pip).gl_attrs[attr_index as usize].vb_index = -(1 as libc::c_int) as int8_t;
        attr_index += 1;
        attr_index;
    }
    let mut attr_index_0: libc::c_int = 0 as libc::c_int;
    while attr_index_0 < SG_MAX_VERTEX_ATTRIBUTES as libc::c_int {
        let mut a_desc: *const sg_vertex_attr_desc = &*((*desc).layout.attrs)
            .as_ptr()
            .offset(attr_index_0 as isize) as *const sg_vertex_attr_desc;
        if (*a_desc).format as libc::c_uint
            == SG_VERTEXFORMAT_INVALID as libc::c_int as libc::c_uint
        {
            break;
        }
        let mut l_desc: *const sg_buffer_layout_desc = &*((*desc).layout.buffers)
            .as_ptr()
            .offset((*a_desc).buffer_index as isize) as *const sg_buffer_layout_desc;
        let step_func: sg_vertex_step = (*l_desc).step_func;
        let step_rate: libc::c_int = (*l_desc).step_rate;
        let mut attr_loc: GLint = attr_index_0;
        if !_sg_strempty(
            &mut (*((*shd).attrs).as_mut_ptr().offset(attr_index_0 as isize)).name,
        ) {
            attr_loc = glGetAttribLocation(
                (*(*pip).shader).gl_prog,
                _sg_strptr(
                    &mut (*((*shd).attrs).as_mut_ptr().offset(attr_index_0 as isize))
                        .name,
                ),
            );
        }
        if attr_loc != -(1 as libc::c_int) {
            let mut gl_attr: *mut _sg_gl_attr_t = &mut *((*pip).gl_attrs)
                .as_mut_ptr()
                .offset(attr_loc as isize) as *mut _sg_gl_attr_t;
            (*gl_attr).vb_index = (*a_desc).buffer_index as int8_t;
            if step_func as libc::c_uint
                == SG_VERTEXSTEP_PER_VERTEX as libc::c_int as libc::c_uint
            {
                (*gl_attr).divisor = 0 as libc::c_int as int8_t;
            } else {
                (*gl_attr).divisor = step_rate as int8_t;
            }
            (*gl_attr).stride = (*l_desc).stride as uint8_t;
            (*gl_attr).offset = (*a_desc).offset;
            (*gl_attr).size = _sg_gl_vertexformat_size((*a_desc).format) as uint8_t;
            (*gl_attr).type_0 = _sg_gl_vertexformat_type((*a_desc).format);
            (*gl_attr).normalized = _sg_gl_vertexformat_normalized((*a_desc).format);
            (*pip)
                .vertex_layout_valid[(*a_desc).buffer_index
                as usize] = 1 as libc::c_int != 0;
        }
        attr_index_0 += 1;
        attr_index_0;
    }
    return SG_RESOURCESTATE_VALID;
}
unsafe extern "C" fn _sg_pipeline_at(
    mut p: *const _sg_pools_t,
    mut pip_id: uint32_t,
) -> *mut _sg_pipeline_t {
    let mut slot_index: libc::c_int = _sg_slot_index(pip_id);
    return &mut *((*p).pipelines).offset(slot_index as isize) as *mut _sg_pipeline_t;
}
unsafe extern "C" fn _sg_lookup_pipeline(
    mut p: *const _sg_pools_t,
    mut pip_id: uint32_t,
) -> *mut _sg_pipeline_t {
    if SG_INVALID_ID as libc::c_int as libc::c_uint != pip_id {
        let mut pip: *mut _sg_pipeline_t = _sg_pipeline_at(p, pip_id);
        if (*pip).slot.id == pip_id {
            return pip;
        }
    }
    return 0 as *mut _sg_pipeline_t;
}
unsafe extern "C" fn _sg_init_pipeline(
    mut pip_id: sg_pipeline,
    mut desc: *const sg_pipeline_desc,
) {
    let mut pip: *mut _sg_pipeline_t = _sg_lookup_pipeline(&mut _sg.pools, pip_id.id);
    (*pip).slot.ctx_id = _sg.active_context.id;
    if _sg_validate_pipeline_desc(desc) {
        let mut shd: *mut _sg_shader_t = _sg_lookup_shader(
            &mut _sg.pools,
            (*desc).shader.id,
        );
        (*pip).slot.state = _sg_create_pipeline(pip, shd, desc);
    } else {
        (*pip).slot.state = SG_RESOURCESTATE_FAILED;
    };
}
unsafe extern "C" fn _sg_vertexformat_bytesize(
    mut fmt: sg_vertex_format,
) -> libc::c_int {
    match fmt as libc::c_uint {
        1 => return 4 as libc::c_int,
        2 => return 8 as libc::c_int,
        3 => return 12 as libc::c_int,
        4 => return 16 as libc::c_int,
        5 => return 4 as libc::c_int,
        6 => return 4 as libc::c_int,
        7 => return 4 as libc::c_int,
        8 => return 4 as libc::c_int,
        9 => return 4 as libc::c_int,
        10 => return 4 as libc::c_int,
        11 => return 8 as libc::c_int,
        12 => return 8 as libc::c_int,
        13 => return 4 as libc::c_int,
        0 => return 0 as libc::c_int,
        _ => return -(1 as libc::c_int),
    };
}
unsafe extern "C" fn _sg_pipeline_desc_defaults(
    mut desc: *const sg_pipeline_desc,
) -> sg_pipeline_desc {
    let mut def: sg_pipeline_desc = *desc;
    def
        .primitive_type = (if def.primitive_type as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_PRIMITIVETYPE_TRIANGLES as libc::c_int as libc::c_uint
    } else {
        def.primitive_type as libc::c_uint
    }) as sg_primitive_type;
    def
        .index_type = (if def.index_type as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_INDEXTYPE_NONE as libc::c_int as libc::c_uint
    } else {
        def.index_type as libc::c_uint
    }) as sg_index_type;
    def
        .depth_stencil
        .stencil_front
        .fail_op = (if def.depth_stencil.stencil_front.fail_op as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_STENCILOP_KEEP as libc::c_int as libc::c_uint
    } else {
        def.depth_stencil.stencil_front.fail_op as libc::c_uint
    }) as sg_stencil_op;
    def
        .depth_stencil
        .stencil_front
        .depth_fail_op = (if def.depth_stencil.stencil_front.depth_fail_op
        as libc::c_uint == 0 as libc::c_int as libc::c_uint
    {
        SG_STENCILOP_KEEP as libc::c_int as libc::c_uint
    } else {
        def.depth_stencil.stencil_front.depth_fail_op as libc::c_uint
    }) as sg_stencil_op;
    def
        .depth_stencil
        .stencil_front
        .pass_op = (if def.depth_stencil.stencil_front.pass_op as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_STENCILOP_KEEP as libc::c_int as libc::c_uint
    } else {
        def.depth_stencil.stencil_front.pass_op as libc::c_uint
    }) as sg_stencil_op;
    def
        .depth_stencil
        .stencil_front
        .compare_func = (if def.depth_stencil.stencil_front.compare_func as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_COMPAREFUNC_ALWAYS as libc::c_int as libc::c_uint
    } else {
        def.depth_stencil.stencil_front.compare_func as libc::c_uint
    }) as sg_compare_func;
    def
        .depth_stencil
        .stencil_back
        .fail_op = (if def.depth_stencil.stencil_back.fail_op as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_STENCILOP_KEEP as libc::c_int as libc::c_uint
    } else {
        def.depth_stencil.stencil_back.fail_op as libc::c_uint
    }) as sg_stencil_op;
    def
        .depth_stencil
        .stencil_back
        .depth_fail_op = (if def.depth_stencil.stencil_back.depth_fail_op as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_STENCILOP_KEEP as libc::c_int as libc::c_uint
    } else {
        def.depth_stencil.stencil_back.depth_fail_op as libc::c_uint
    }) as sg_stencil_op;
    def
        .depth_stencil
        .stencil_back
        .pass_op = (if def.depth_stencil.stencil_back.pass_op as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_STENCILOP_KEEP as libc::c_int as libc::c_uint
    } else {
        def.depth_stencil.stencil_back.pass_op as libc::c_uint
    }) as sg_stencil_op;
    def
        .depth_stencil
        .stencil_back
        .compare_func = (if def.depth_stencil.stencil_back.compare_func as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_COMPAREFUNC_ALWAYS as libc::c_int as libc::c_uint
    } else {
        def.depth_stencil.stencil_back.compare_func as libc::c_uint
    }) as sg_compare_func;
    def
        .depth_stencil
        .depth_compare_func = (if def.depth_stencil.depth_compare_func as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_COMPAREFUNC_ALWAYS as libc::c_int as libc::c_uint
    } else {
        def.depth_stencil.depth_compare_func as libc::c_uint
    }) as sg_compare_func;
    def
        .blend
        .src_factor_rgb = (if def.blend.src_factor_rgb as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_BLENDFACTOR_ONE as libc::c_int as libc::c_uint
    } else {
        def.blend.src_factor_rgb as libc::c_uint
    }) as sg_blend_factor;
    def
        .blend
        .dst_factor_rgb = (if def.blend.dst_factor_rgb as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_BLENDFACTOR_ZERO as libc::c_int as libc::c_uint
    } else {
        def.blend.dst_factor_rgb as libc::c_uint
    }) as sg_blend_factor;
    def
        .blend
        .op_rgb = (if def.blend.op_rgb as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_BLENDOP_ADD as libc::c_int as libc::c_uint
    } else {
        def.blend.op_rgb as libc::c_uint
    }) as sg_blend_op;
    def
        .blend
        .src_factor_alpha = (if def.blend.src_factor_alpha as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_BLENDFACTOR_ONE as libc::c_int as libc::c_uint
    } else {
        def.blend.src_factor_alpha as libc::c_uint
    }) as sg_blend_factor;
    def
        .blend
        .dst_factor_alpha = (if def.blend.dst_factor_alpha as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_BLENDFACTOR_ZERO as libc::c_int as libc::c_uint
    } else {
        def.blend.dst_factor_alpha as libc::c_uint
    }) as sg_blend_factor;
    def
        .blend
        .op_alpha = (if def.blend.op_alpha as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_BLENDOP_ADD as libc::c_int as libc::c_uint
    } else {
        def.blend.op_alpha as libc::c_uint
    }) as sg_blend_op;
    if def.blend.color_write_mask as libc::c_int == SG_COLORMASK_NONE as libc::c_int {
        def.blend.color_write_mask = 0 as libc::c_int as uint8_t;
    } else {
        def
            .blend
            .color_write_mask = (if def.blend.color_write_mask as sg_color_mask
            as libc::c_uint == 0 as libc::c_int as libc::c_uint
        {
            SG_COLORMASK_RGBA as libc::c_int as libc::c_uint
        } else {
            def.blend.color_write_mask as sg_color_mask as libc::c_uint
        }) as uint8_t;
    }
    def
        .blend
        .color_attachment_count = if def.blend.color_attachment_count == 0 as libc::c_int
    {
        1 as libc::c_int
    } else {
        def.blend.color_attachment_count
    };
    def
        .blend
        .color_format = (if def.blend.color_format as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_PIXELFORMAT_RGBA8 as libc::c_int as libc::c_uint
    } else {
        def.blend.color_format as libc::c_uint
    }) as sg_pixel_format;
    def
        .blend
        .depth_format = (if def.blend.depth_format as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_PIXELFORMAT_DEPTHSTENCIL as libc::c_int as libc::c_uint
    } else {
        def.blend.depth_format as libc::c_uint
    }) as sg_pixel_format;
    def
        .rasterizer
        .cull_mode = (if def.rasterizer.cull_mode as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_CULLMODE_NONE as libc::c_int as libc::c_uint
    } else {
        def.rasterizer.cull_mode as libc::c_uint
    }) as sg_cull_mode;
    def
        .rasterizer
        .face_winding = (if def.rasterizer.face_winding as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_FACEWINDING_CW as libc::c_int as libc::c_uint
    } else {
        def.rasterizer.face_winding as libc::c_uint
    }) as sg_face_winding;
    def
        .rasterizer
        .sample_count = if def.rasterizer.sample_count == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        def.rasterizer.sample_count
    };
    let mut attr_index: libc::c_int = 0 as libc::c_int;
    while attr_index < SG_MAX_VERTEX_ATTRIBUTES as libc::c_int {
        let mut a_desc: *mut sg_vertex_attr_desc = &mut *(def.layout.attrs)
            .as_mut_ptr()
            .offset(attr_index as isize) as *mut sg_vertex_attr_desc;
        if (*a_desc).format as libc::c_uint
            == SG_VERTEXFORMAT_INVALID as libc::c_int as libc::c_uint
        {
            break;
        }
        let mut b_desc: *mut sg_buffer_layout_desc = &mut *(def.layout.buffers)
            .as_mut_ptr()
            .offset((*a_desc).buffer_index as isize) as *mut sg_buffer_layout_desc;
        (*b_desc)
            .step_func = (if (*b_desc).step_func as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            SG_VERTEXSTEP_PER_VERTEX as libc::c_int as libc::c_uint
        } else {
            (*b_desc).step_func as libc::c_uint
        }) as sg_vertex_step;
        (*b_desc)
            .step_rate = if (*b_desc).step_rate == 0 as libc::c_int {
            1 as libc::c_int
        } else {
            (*b_desc).step_rate
        };
        attr_index += 1;
        attr_index;
    }
    let mut auto_offset: [libc::c_int; 4] = [0; 4];
    memset(
        auto_offset.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong,
    );
    let mut use_auto_offset: bool = 1 as libc::c_int != 0;
    let mut attr_index_0: libc::c_int = 0 as libc::c_int;
    while attr_index_0 < SG_MAX_VERTEX_ATTRIBUTES as libc::c_int {
        if def.layout.attrs[attr_index_0 as usize].offset != 0 as libc::c_int {
            use_auto_offset = 0 as libc::c_int != 0;
        }
        attr_index_0 += 1;
        attr_index_0;
    }
    let mut attr_index_1: libc::c_int = 0 as libc::c_int;
    while attr_index_1 < SG_MAX_VERTEX_ATTRIBUTES as libc::c_int {
        let mut a_desc_0: *mut sg_vertex_attr_desc = &mut *(def.layout.attrs)
            .as_mut_ptr()
            .offset(attr_index_1 as isize) as *mut sg_vertex_attr_desc;
        if (*a_desc_0).format as libc::c_uint
            == SG_VERTEXFORMAT_INVALID as libc::c_int as libc::c_uint
        {
            break;
        }
        if use_auto_offset {
            (*a_desc_0).offset = auto_offset[(*a_desc_0).buffer_index as usize];
        }
        auto_offset[(*a_desc_0).buffer_index as usize]
            += _sg_vertexformat_bytesize((*a_desc_0).format);
        attr_index_1 += 1;
        attr_index_1;
    }
    let mut buf_index: libc::c_int = 0 as libc::c_int;
    while buf_index < SG_MAX_SHADERSTAGE_BUFFERS as libc::c_int {
        let mut l_desc: *mut sg_buffer_layout_desc = &mut *(def.layout.buffers)
            .as_mut_ptr()
            .offset(buf_index as isize) as *mut sg_buffer_layout_desc;
        if (*l_desc).stride == 0 as libc::c_int {
            (*l_desc).stride = auto_offset[buf_index as usize];
        }
        buf_index += 1;
        buf_index;
    }
    return def;
}
unsafe extern "C" fn _sg_alloc_pipeline() -> sg_pipeline {
    let mut res: sg_pipeline = sg_pipeline { id: 0 };
    let mut slot_index: libc::c_int = _sg_pool_alloc_index(&mut _sg.pools.pipeline_pool);
    if 0 as libc::c_int != slot_index {
        res
            .id = _sg_slot_alloc(
            &mut _sg.pools.pipeline_pool,
            &mut (*(_sg.pools.pipelines).offset(slot_index as isize)).slot,
            slot_index,
        );
    } else {
        res.id = SG_INVALID_ID as libc::c_int as uint32_t;
    }
    return res;
}
pub unsafe extern "C" fn sg_make_pipeline(
    mut desc: *const sg_pipeline_desc,
) -> sg_pipeline {
    let mut desc_def: sg_pipeline_desc = _sg_pipeline_desc_defaults(desc);
    let mut pip_id: sg_pipeline = _sg_alloc_pipeline();
    if pip_id.id != SG_INVALID_ID as libc::c_int as libc::c_uint {
        _sg_init_pipeline(pip_id, &mut desc_def);
    }
    return pip_id;
}
unsafe extern "C" fn _sg_validate_shader_desc(mut desc: *const sg_shader_desc) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_strcpy(mut dst: *mut _sg_str_t, mut src: *const libc::c_char) {
    if !src.is_null() {
        strncpy(
            ((*dst).buf).as_mut_ptr(),
            src,
            _SG_STRING_SIZE as libc::c_int as libc::c_ulong,
        );
        (*dst)
            .buf[(_SG_STRING_SIZE as libc::c_int - 1 as libc::c_int)
            as usize] = 0 as libc::c_int as libc::c_char;
    } else {
        memset(
            ((*dst).buf).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            _SG_STRING_SIZE as libc::c_int as libc::c_ulong,
        );
    };
}
unsafe extern "C" fn _sg_gl_shader_stage(mut stage: sg_shader_stage) -> GLenum {
    match stage as libc::c_uint {
        0 => return 0x8b31 as libc::c_int as GLenum,
        1 => return 0x8b30 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_compile_shader(
    mut stage: sg_shader_stage,
    mut src: *const libc::c_char,
) -> GLuint {
    let mut gl_shd: GLuint = glCreateShader(_sg_gl_shader_stage(stage));
    glShaderSource(
        gl_shd,
        1 as libc::c_int,
        &mut src as *mut *const libc::c_char as *const *const GLchar,
        0 as *const GLint,
    );
    glCompileShader(gl_shd);
    let mut compile_status: GLint = 0 as libc::c_int;
    glGetShaderiv(gl_shd, 0x8b81 as libc::c_int as GLenum, &mut compile_status);
    if compile_status == 0 {
        let mut log_len: GLint = 0 as libc::c_int;
        glGetShaderiv(gl_shd, 0x8b84 as libc::c_int as GLenum, &mut log_len);
        if log_len > 0 as libc::c_int {
            let mut log_buf: *mut GLchar = malloc(log_len as libc::c_ulong)
                as *mut GLchar;
            glGetShaderInfoLog(gl_shd, log_len, &mut log_len, log_buf);
            free(log_buf as *mut libc::c_void);
        }
        glDeleteShader(gl_shd);
        gl_shd = 0 as libc::c_int as GLuint;
    }
    return gl_shd;
}
unsafe extern "C" fn _sg_uniform_size(
    mut type_0: sg_uniform_type,
    mut count: libc::c_int,
) -> libc::c_int {
    match type_0 as libc::c_uint {
        0 => return 0 as libc::c_int,
        1 => return 4 as libc::c_int * count,
        2 => return 8 as libc::c_int * count,
        3 => return 12 as libc::c_int * count,
        4 => return 16 as libc::c_int * count,
        5 => return 64 as libc::c_int * count,
        _ => return -(1 as libc::c_int),
    };
}
unsafe extern "C" fn _sg_create_shader(
    mut shd: *mut _sg_shader_t,
    mut desc: *const sg_shader_desc,
) -> sg_resource_state {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < SG_MAX_VERTEX_ATTRIBUTES as libc::c_int {
        _sg_strcpy(
            &mut (*((*shd).attrs).as_mut_ptr().offset(i as isize)).name,
            (*desc).attrs[i as usize].name,
        );
        i += 1;
        i;
    }
    let mut gl_vs: GLuint = _sg_gl_compile_shader(SG_SHADERSTAGE_VS, (*desc).vs.source);
    let mut gl_fs: GLuint = _sg_gl_compile_shader(SG_SHADERSTAGE_FS, (*desc).fs.source);
    if !(gl_vs != 0 && gl_fs != 0) {
        return SG_RESOURCESTATE_FAILED;
    }
    let mut gl_prog: GLuint = glCreateProgram();
    glAttachShader(gl_prog, gl_vs);
    glAttachShader(gl_prog, gl_fs);
    glLinkProgram(gl_prog);
    glDeleteShader(gl_vs);
    glDeleteShader(gl_fs);
    let mut link_status: GLint = 0;
    glGetProgramiv(gl_prog, 0x8b82 as libc::c_int as GLenum, &mut link_status);
    if link_status == 0 {
        let mut log_len: GLint = 0 as libc::c_int;
        glGetProgramiv(gl_prog, 0x8b84 as libc::c_int as GLenum, &mut log_len);
        if log_len > 0 as libc::c_int {
            let mut log_buf: *mut GLchar = malloc(log_len as libc::c_ulong)
                as *mut GLchar;
            glGetProgramInfoLog(gl_prog, log_len, &mut log_len, log_buf);
            free(log_buf as *mut libc::c_void);
        }
        glDeleteProgram(gl_prog);
        return SG_RESOURCESTATE_FAILED;
    }
    (*shd).gl_prog = gl_prog;
    let mut stage_index: libc::c_int = 0 as libc::c_int;
    while stage_index < SG_NUM_SHADER_STAGES as libc::c_int {
        let mut stage_desc: *const sg_shader_stage_desc = if stage_index
            == SG_SHADERSTAGE_VS as libc::c_int
        {
            &(*desc).vs
        } else {
            &(*desc).fs
        };
        let mut stage: *mut _sg_shader_stage_t = &mut *((*shd).stage)
            .as_mut_ptr()
            .offset(stage_index as isize) as *mut _sg_shader_stage_t;
        let mut ub_index: libc::c_int = 0 as libc::c_int;
        while ub_index < SG_MAX_SHADERSTAGE_UBS as libc::c_int {
            let mut ub_desc: *const sg_shader_uniform_block_desc = &*((*stage_desc)
                .uniform_blocks)
                .as_ptr()
                .offset(ub_index as isize) as *const sg_shader_uniform_block_desc;
            if 0 as libc::c_int == (*ub_desc).size {
                break;
            }
            let mut ub: *mut _sg_uniform_block_t = &mut *((*stage).uniform_blocks)
                .as_mut_ptr()
                .offset(ub_index as isize) as *mut _sg_uniform_block_t;
            (*ub).size = (*ub_desc).size;
            let mut cur_uniform_offset: libc::c_int = 0 as libc::c_int;
            let mut u_index: libc::c_int = 0 as libc::c_int;
            while u_index < SG_MAX_UB_MEMBERS as libc::c_int {
                let mut u_desc: *const sg_shader_uniform_desc = &*((*ub_desc).uniforms)
                    .as_ptr()
                    .offset(u_index as isize) as *const sg_shader_uniform_desc;
                if (*u_desc).type_0 as libc::c_uint
                    == SG_UNIFORMTYPE_INVALID as libc::c_int as libc::c_uint
                {
                    break;
                }
                let mut u: *mut _sg_uniform_t = &mut *((*ub).uniforms)
                    .as_mut_ptr()
                    .offset(u_index as isize) as *mut _sg_uniform_t;
                (*u).type_0 = (*u_desc).type_0;
                (*u).count = (*u_desc).array_count as uint8_t;
                (*u).offset = cur_uniform_offset as uint16_t;
                cur_uniform_offset
                    += _sg_uniform_size((*u).type_0, (*u).count as libc::c_int);
                if !((*u_desc).name).is_null() {
                    (*u).gl_loc = glGetUniformLocation(gl_prog, (*u_desc).name);
                } else {
                    (*u).gl_loc = u_index;
                }
                (*ub).num_uniforms += 1;
                (*ub).num_uniforms;
                u_index += 1;
                u_index;
            }
            (*stage).num_uniform_blocks += 1;
            (*stage).num_uniform_blocks;
            ub_index += 1;
            ub_index;
        }
        stage_index += 1;
        stage_index;
    }
    let mut gl_tex_slot: libc::c_int = 0 as libc::c_int;
    let mut stage_index_0: libc::c_int = 0 as libc::c_int;
    while stage_index_0 < SG_NUM_SHADER_STAGES as libc::c_int {
        let mut stage_desc_0: *const sg_shader_stage_desc = if stage_index_0
            == SG_SHADERSTAGE_VS as libc::c_int
        {
            &(*desc).vs
        } else {
            &(*desc).fs
        };
        let mut stage_0: *mut _sg_shader_stage_t = &mut *((*shd).stage)
            .as_mut_ptr()
            .offset(stage_index_0 as isize) as *mut _sg_shader_stage_t;
        let mut img_index: libc::c_int = 0 as libc::c_int;
        while img_index < SG_MAX_SHADERSTAGE_IMAGES as libc::c_int {
            let mut img_desc: *const sg_shader_image_desc = &*((*stage_desc_0).images)
                .as_ptr()
                .offset(img_index as isize) as *const sg_shader_image_desc;
            if (*img_desc).type_0 as libc::c_uint
                == _SG_IMAGETYPE_DEFAULT as libc::c_int as libc::c_uint
            {
                break;
            }
            let mut img: *mut _sg_shader_image_t = &mut *((*stage_0).images)
                .as_mut_ptr()
                .offset(img_index as isize) as *mut _sg_shader_image_t;
            (*img).type_0 = (*img_desc).type_0;
            (*img).gl_loc = img_index;
            if !((*img_desc).name).is_null() {
                (*img).gl_loc = glGetUniformLocation(gl_prog, (*img_desc).name);
            }
            if (*img).gl_loc != -(1 as libc::c_int) {
                let fresh1 = gl_tex_slot;
                gl_tex_slot = gl_tex_slot + 1;
                (*img).gl_tex_slot = fresh1;
            } else {
                (*img).gl_tex_slot = -(1 as libc::c_int);
            }
            (*stage_0).num_images += 1;
            (*stage_0).num_images;
            img_index += 1;
            img_index;
        }
        stage_index_0 += 1;
        stage_index_0;
    }
    return SG_RESOURCESTATE_VALID;
}
unsafe extern "C" fn _sg_shader_at(
    mut p: *const _sg_pools_t,
    mut shd_id: uint32_t,
) -> *mut _sg_shader_t {
    let mut slot_index: libc::c_int = _sg_slot_index(shd_id);
    return &mut *((*p).shaders).offset(slot_index as isize) as *mut _sg_shader_t;
}
unsafe extern "C" fn _sg_lookup_shader(
    mut p: *const _sg_pools_t,
    mut shd_id: uint32_t,
) -> *mut _sg_shader_t {
    if SG_INVALID_ID as libc::c_int as libc::c_uint != shd_id {
        let mut shd: *mut _sg_shader_t = _sg_shader_at(p, shd_id);
        if (*shd).slot.id == shd_id {
            return shd;
        }
    }
    return 0 as *mut _sg_shader_t;
}
unsafe extern "C" fn _sg_init_shader(
    mut shd_id: sg_shader,
    mut desc: *const sg_shader_desc,
) {
    let mut shd: *mut _sg_shader_t = _sg_lookup_shader(&mut _sg.pools, shd_id.id);
    (*shd).slot.ctx_id = _sg.active_context.id;
    if _sg_validate_shader_desc(desc) {
        (*shd).slot.state = _sg_create_shader(shd, desc);
    } else {
        (*shd).slot.state = SG_RESOURCESTATE_FAILED;
    };
}
unsafe extern "C" fn _sg_shader_desc_defaults(
    mut desc: *const sg_shader_desc,
) -> sg_shader_desc {
    let mut def: sg_shader_desc = *desc;
    def
        .vs
        .entry = if (def.vs.entry).is_null() {
        b"main\0" as *const u8 as *const libc::c_char
    } else {
        def.vs.entry
    };
    def
        .fs
        .entry = if (def.fs.entry).is_null() {
        b"main\0" as *const u8 as *const libc::c_char
    } else {
        def.fs.entry
    };
    let mut stage_index: libc::c_int = 0 as libc::c_int;
    while stage_index < SG_NUM_SHADER_STAGES as libc::c_int {
        let mut stage_desc: *mut sg_shader_stage_desc = if stage_index
            == SG_SHADERSTAGE_VS as libc::c_int
        {
            &mut def.vs
        } else {
            &mut def.fs
        };
        let mut ub_index: libc::c_int = 0 as libc::c_int;
        while ub_index < SG_MAX_SHADERSTAGE_UBS as libc::c_int {
            let mut ub_desc: *mut sg_shader_uniform_block_desc = &mut *((*stage_desc)
                .uniform_blocks)
                .as_mut_ptr()
                .offset(ub_index as isize) as *mut sg_shader_uniform_block_desc;
            if 0 as libc::c_int == (*ub_desc).size {
                break;
            }
            let mut u_index: libc::c_int = 0 as libc::c_int;
            while u_index < SG_MAX_UB_MEMBERS as libc::c_int {
                let mut u_desc: *mut sg_shader_uniform_desc = &mut *((*ub_desc).uniforms)
                    .as_mut_ptr()
                    .offset(u_index as isize) as *mut sg_shader_uniform_desc;
                if (*u_desc).type_0 as libc::c_uint
                    == SG_UNIFORMTYPE_INVALID as libc::c_int as libc::c_uint
                {
                    break;
                }
                (*u_desc)
                    .array_count = if (*u_desc).array_count == 0 as libc::c_int {
                    1 as libc::c_int
                } else {
                    (*u_desc).array_count
                };
                u_index += 1;
                u_index;
            }
            ub_index += 1;
            ub_index;
        }
        stage_index += 1;
        stage_index;
    }
    return def;
}
unsafe extern "C" fn _sg_alloc_shader() -> sg_shader {
    let mut res: sg_shader = sg_shader { id: 0 };
    let mut slot_index: libc::c_int = _sg_pool_alloc_index(&mut _sg.pools.shader_pool);
    if 0 as libc::c_int != slot_index {
        res
            .id = _sg_slot_alloc(
            &mut _sg.pools.shader_pool,
            &mut (*(_sg.pools.shaders).offset(slot_index as isize)).slot,
            slot_index,
        );
    } else {
        res.id = SG_INVALID_ID as libc::c_int as uint32_t;
    }
    return res;
}
pub unsafe extern "C" fn sg_make_shader(mut desc: *const sg_shader_desc) -> sg_shader {
    let mut desc_def: sg_shader_desc = _sg_shader_desc_defaults(desc);
    let mut shd_id: sg_shader = _sg_alloc_shader();
    if shd_id.id != SG_INVALID_ID as libc::c_int as libc::c_uint {
        _sg_init_shader(shd_id, &mut desc_def);
    }
    return shd_id;
}
unsafe extern "C" fn _sg_validate_image_desc(mut desc: *const sg_image_desc) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_gl_supported_texture_format(mut fmt: sg_pixel_format) -> bool {
    match fmt as libc::c_uint {
        13 | 14 | 15 => {
            return _sg
                .gl
                .features[SG_FEATURE_TEXTURE_COMPRESSION_DXT as libc::c_int as usize];
        }
        18 | 19 | 20 | 21 => {
            return _sg
                .gl
                .features[SG_FEATURE_TEXTURE_COMPRESSION_PVRTC as libc::c_int as usize];
        }
        22 | 23 => {
            return _sg
                .gl
                .features[SG_FEATURE_TEXTURE_COMPRESSION_ETC2 as libc::c_int as usize];
        }
        _ => return 1 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
unsafe extern "C" fn _sapp_fail(mut msg: *const libc::c_char) {
    if (_sapp.desc.fail_cb).is_some() {
        (_sapp.desc.fail_cb).unwrap()(msg);
    } else if (_sapp.desc.fail_userdata_cb).is_some() {
        (_sapp.desc.fail_userdata_cb).unwrap()(msg, _sapp.desc.user_data);
    }
    abort();
}
unsafe extern "C" fn _sapp_call_init() {
    if (_sapp.desc.init_cb).is_some() {
        (_sapp.desc.init_cb).unwrap()();
    } else if (_sapp.desc.init_userdata_cb).is_some() {
        (_sapp.desc.init_userdata_cb).unwrap()(_sapp.desc.user_data);
    }
    _sapp.init_called = 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sapp_call_frame() {
    if _sapp.init_called as libc::c_int != 0 && !_sapp.cleanup_called {
        if (_sapp.desc.frame_cb).is_some() {
            (_sapp.desc.frame_cb).unwrap()();
        } else if (_sapp.desc.frame_userdata_cb).is_some() {
            (_sapp.desc.frame_userdata_cb).unwrap()(_sapp.desc.user_data);
        }
    }
}
unsafe extern "C" fn _sapp_call_cleanup() {
    if (_sapp.desc.cleanup_cb).is_some() {
        (_sapp.desc.cleanup_cb).unwrap()();
    } else if (_sapp.desc.cleanup_userdata_cb).is_some() {
        (_sapp.desc.cleanup_userdata_cb).unwrap()(_sapp.desc.user_data);
    }
    _sapp.cleanup_called = 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sapp_call_event(mut e: *const sapp_event) {
    if (_sapp.desc.event_cb).is_some() {
        (_sapp.desc.event_cb).unwrap()(e);
    } else if (_sapp.desc.event_userdata_cb).is_some() {
        (_sapp.desc.event_userdata_cb).unwrap()(e, _sapp.desc.user_data);
    }
}
unsafe extern "C" fn _sapp_strcpy(
    mut src: *const libc::c_char,
    mut dst: *mut libc::c_char,
    mut max_len: libc::c_int,
) {
    let end: *mut libc::c_char = &mut *dst.offset((max_len - 1 as libc::c_int) as isize)
        as *mut libc::c_char;
    let mut c: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < max_len {
        c = *src;
        if c as libc::c_int != 0 as libc::c_int {
            src = src.offset(1);
            src;
        }
        let fresh2 = dst;
        dst = dst.offset(1);
        *fresh2 = c;
        i += 1;
        i;
    }
    if c as libc::c_int != 0 as libc::c_int {
        *end = 0 as libc::c_int as libc::c_char;
    }
}
unsafe extern "C" fn _sapp_init_state(mut desc: *const sapp_desc) {
    memset(
        &mut _sapp as *mut _sapp_state as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_sapp_state>() as libc::c_ulong,
    );
    _sapp.desc = *desc;
    _sapp.first_frame = 1 as libc::c_int != 0;
    _sapp
        .window_width = if _sapp.desc.width == 0 as libc::c_int {
        640 as libc::c_int
    } else {
        _sapp.desc.width
    };
    _sapp
        .window_height = if _sapp.desc.height == 0 as libc::c_int {
        480 as libc::c_int
    } else {
        _sapp.desc.height
    };
    _sapp.framebuffer_width = _sapp.window_width;
    _sapp.framebuffer_height = _sapp.window_height;
    _sapp
        .sample_count = if _sapp.desc.sample_count == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        _sapp.desc.sample_count
    };
    _sapp
        .swap_interval = if _sapp.desc.swap_interval == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        _sapp.desc.swap_interval
    };
    _sapp
        .html5_canvas_name = if (_sapp.desc.html5_canvas_name).is_null() {
        b"canvas\0" as *const u8 as *const libc::c_char
    } else {
        _sapp.desc.html5_canvas_name
    };
    _sapp.html5_canvas_resize = _sapp.desc.html5_canvas_resize;
    if !(_sapp.desc.window_title).is_null() {
        _sapp_strcpy(
            _sapp.desc.window_title,
            (_sapp.window_title).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        );
    } else {
        _sapp_strcpy(
            b"sokol_app\0" as *const u8 as *const libc::c_char,
            (_sapp.window_title).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        );
    }
    _sapp.dpi_scale = 1.0f32;
}
unsafe extern "C" fn _sapp_init_event(mut type_0: sapp_event_type) {
    memset(
        &mut _sapp.event as *mut sapp_event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sapp_event>() as libc::c_ulong,
    );
    _sapp.event.type_0 = type_0;
    _sapp.event.frame_count = _sapp.frame_count;
    _sapp.event.mouse_button = SAPP_MOUSEBUTTON_INVALID;
    _sapp.event.window_width = _sapp.window_width;
    _sapp.event.window_height = _sapp.window_height;
    _sapp.event.framebuffer_width = _sapp.framebuffer_width;
    _sapp.event.framebuffer_height = _sapp.framebuffer_height;
}
unsafe extern "C" fn _sapp_events_enabled() -> bool {
    return ((_sapp.desc.event_cb).is_some() || (_sapp.desc.event_userdata_cb).is_some())
        && _sapp.init_called as libc::c_int != 0;
}
unsafe extern "C" fn _sapp_frame() {
    if _sapp.first_frame {
        _sapp.first_frame = 0 as libc::c_int != 0;
        _sapp_call_init();
    }
    _sapp_call_frame();
    _sapp.frame_count = (_sapp.frame_count).wrapping_add(1);
    _sapp.frame_count;
}
unsafe extern "C" fn _sapp_gl_init_fbconfig(mut fbconfig: *mut _sapp_gl_fbconfig) {
    memset(
        fbconfig as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_sapp_gl_fbconfig>() as libc::c_ulong,
    );
    (*fbconfig).red_bits = -(1 as libc::c_int);
    (*fbconfig).green_bits = -(1 as libc::c_int);
    (*fbconfig).blue_bits = -(1 as libc::c_int);
    (*fbconfig).alpha_bits = -(1 as libc::c_int);
    (*fbconfig).depth_bits = -(1 as libc::c_int);
    (*fbconfig).stencil_bits = -(1 as libc::c_int);
    (*fbconfig).samples = -(1 as libc::c_int);
}
unsafe extern "C" fn _sapp_gl_choose_fbconfig(
    mut desired: *const _sapp_gl_fbconfig,
    mut alternatives: *const _sapp_gl_fbconfig,
    mut count: libc::c_uint,
) -> *const _sapp_gl_fbconfig {
    let mut i: libc::c_uint = 0;
    let mut missing: libc::c_uint = 0;
    let mut least_missing: libc::c_uint = 1000000 as libc::c_int as libc::c_uint;
    let mut color_diff: libc::c_uint = 0;
    let mut least_color_diff: libc::c_uint = 10000000 as libc::c_int as libc::c_uint;
    let mut extra_diff: libc::c_uint = 0;
    let mut least_extra_diff: libc::c_uint = 10000000 as libc::c_int as libc::c_uint;
    let mut current: *const _sapp_gl_fbconfig = 0 as *const _sapp_gl_fbconfig;
    let mut closest: *const _sapp_gl_fbconfig = 0 as *const _sapp_gl_fbconfig;
    i = 0 as libc::c_int as libc::c_uint;
    while i < count {
        current = alternatives.offset(i as isize);
        if !((*desired).doublebuffer as libc::c_int
            != (*current).doublebuffer as libc::c_int)
        {
            missing = 0 as libc::c_int as libc::c_uint;
            if (*desired).alpha_bits > 0 as libc::c_int
                && (*current).alpha_bits == 0 as libc::c_int
            {
                missing = missing.wrapping_add(1);
                missing;
            }
            if (*desired).depth_bits > 0 as libc::c_int
                && (*current).depth_bits == 0 as libc::c_int
            {
                missing = missing.wrapping_add(1);
                missing;
            }
            if (*desired).stencil_bits > 0 as libc::c_int
                && (*current).stencil_bits == 0 as libc::c_int
            {
                missing = missing.wrapping_add(1);
                missing;
            }
            if (*desired).samples > 0 as libc::c_int
                && (*current).samples == 0 as libc::c_int
            {
                missing = missing.wrapping_add(1);
                missing;
            }
            color_diff = 0 as libc::c_int as libc::c_uint;
            if (*desired).red_bits != -(1 as libc::c_int) {
                color_diff = color_diff
                    .wrapping_add(
                        (((*desired).red_bits - (*current).red_bits)
                            * ((*desired).red_bits - (*current).red_bits))
                            as libc::c_uint,
                    );
            }
            if (*desired).green_bits != -(1 as libc::c_int) {
                color_diff = color_diff
                    .wrapping_add(
                        (((*desired).green_bits - (*current).green_bits)
                            * ((*desired).green_bits - (*current).green_bits))
                            as libc::c_uint,
                    );
            }
            if (*desired).blue_bits != -(1 as libc::c_int) {
                color_diff = color_diff
                    .wrapping_add(
                        (((*desired).blue_bits - (*current).blue_bits)
                            * ((*desired).blue_bits - (*current).blue_bits))
                            as libc::c_uint,
                    );
            }
            extra_diff = 0 as libc::c_int as libc::c_uint;
            if (*desired).alpha_bits != -(1 as libc::c_int) {
                extra_diff = extra_diff
                    .wrapping_add(
                        (((*desired).alpha_bits - (*current).alpha_bits)
                            * ((*desired).alpha_bits - (*current).alpha_bits))
                            as libc::c_uint,
                    );
            }
            if (*desired).depth_bits != -(1 as libc::c_int) {
                extra_diff = extra_diff
                    .wrapping_add(
                        (((*desired).depth_bits - (*current).depth_bits)
                            * ((*desired).depth_bits - (*current).depth_bits))
                            as libc::c_uint,
                    );
            }
            if (*desired).stencil_bits != -(1 as libc::c_int) {
                extra_diff = extra_diff
                    .wrapping_add(
                        (((*desired).stencil_bits - (*current).stencil_bits)
                            * ((*desired).stencil_bits - (*current).stencil_bits))
                            as libc::c_uint,
                    );
            }
            if (*desired).samples != -(1 as libc::c_int) {
                extra_diff = extra_diff
                    .wrapping_add(
                        (((*desired).samples - (*current).samples)
                            * ((*desired).samples - (*current).samples)) as libc::c_uint,
                    );
            }
            if missing < least_missing {
                closest = current;
            } else if missing == least_missing {
                if color_diff < least_color_diff
                    || color_diff == least_color_diff && extra_diff < least_extra_diff
                {
                    closest = current;
                }
            }
            if current == closest {
                least_missing = missing;
                least_color_diff = color_diff;
                least_extra_diff = extra_diff;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return closest;
}
unsafe extern "C" fn _sg_is_valid_rendertarget_depth_format(
    mut fmt: sg_pixel_format,
) -> bool {
    match fmt as libc::c_uint {
        16 | 17 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn _sg_gl_depth_attachment_format(mut fmt: sg_pixel_format) -> GLenum {
    match fmt as libc::c_uint {
        16 => return 0x81a5 as libc::c_int as GLenum,
        17 => return 0x88f0 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_texture_target(mut t: sg_image_type) -> GLenum {
    match t as libc::c_uint {
        1 => return 0xde1 as libc::c_int as GLenum,
        2 => return 0x8513 as libc::c_int as GLenum,
        3 => return 0x806f as libc::c_int as GLenum,
        4 => return 0x8c1a as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_store_texture_binding(mut slot_index: libc::c_int) {
    _sg.gl.cache.stored_texture = _sg.gl.cache.textures[slot_index as usize];
}
unsafe extern "C" fn _sg_gl_filter(mut f: sg_filter) -> GLenum {
    match f as libc::c_uint {
        1 => return 0x2600 as libc::c_int as GLenum,
        2 => return 0x2601 as libc::c_int as GLenum,
        3 => return 0x2700 as libc::c_int as GLenum,
        4 => return 0x2702 as libc::c_int as GLenum,
        5 => return 0x2701 as libc::c_int as GLenum,
        6 => return 0x2703 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_wrap(mut w: sg_wrap) -> GLenum {
    match w as libc::c_uint {
        2 => return 0x812f as libc::c_int as GLenum,
        1 => return 0x2901 as libc::c_int as GLenum,
        3 => return 0x8370 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_cubeface_target(mut face_index: libc::c_int) -> GLenum {
    match face_index {
        0 => return 0x8515 as libc::c_int as GLenum,
        1 => return 0x8516 as libc::c_int as GLenum,
        2 => return 0x8517 as libc::c_int as GLenum,
        3 => return 0x8518 as libc::c_int as GLenum,
        4 => return 0x8519 as libc::c_int as GLenum,
        5 => return 0x851a as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_is_compressed_pixel_format(mut fmt: sg_pixel_format) -> bool {
    match fmt as libc::c_uint {
        13 | 14 | 15 | 18 | 19 | 20 | 21 | 22 | 23 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn _sg_gl_teximage_internal_format(
    mut fmt: sg_pixel_format,
) -> GLenum {
    if _sg.gl.gles2 {
        return _sg_gl_teximage_format(fmt)
    } else {
        match fmt as libc::c_uint {
            1 => return 0 as libc::c_int as GLenum,
            2 => return 0x8058 as libc::c_int as GLenum,
            3 => return 0x8051 as libc::c_int as GLenum,
            4 => return 0x8056 as libc::c_int as GLenum,
            5 => return 0x8050 as libc::c_int as GLenum,
            6 => return 0x8057 as libc::c_int as GLenum,
            7 => return 0x8059 as libc::c_int as GLenum,
            8 => return 0x8814 as libc::c_int as GLenum,
            9 => return 0x881a as libc::c_int as GLenum,
            10 => return 0x822e as libc::c_int as GLenum,
            11 => return 0x822d as libc::c_int as GLenum,
            12 => return 0x8229 as libc::c_int as GLenum,
            16 => return 0x81a5 as libc::c_int as GLenum,
            17 => return 0x88f0 as libc::c_int as GLenum,
            13 => return 0x83f1 as libc::c_int as GLenum,
            14 => return 0x83f2 as libc::c_int as GLenum,
            15 => return 0x83f3 as libc::c_int as GLenum,
            18 => return 0x8c01 as libc::c_int as GLenum,
            19 => return 0x8c00 as libc::c_int as GLenum,
            20 => return 0x8c03 as libc::c_int as GLenum,
            21 => return 0x8c02 as libc::c_int as GLenum,
            22 => return 0x9274 as libc::c_int as GLenum,
            23 => return 0x9275 as libc::c_int as GLenum,
            _ => return 0 as libc::c_int as GLenum,
        }
    };
}
unsafe extern "C" fn _sg_gl_teximage_format(mut fmt: sg_pixel_format) -> GLenum {
    match fmt as libc::c_uint {
        1 => return 0 as libc::c_int as GLenum,
        2 | 6 | 4 | 8 | 9 | 7 => return 0x1908 as libc::c_int as GLenum,
        3 | 5 => return 0x1907 as libc::c_int as GLenum,
        12 | 10 | 11 => {
            if _sg.gl.gles2 {
                return 0x1909 as libc::c_int as GLenum
            } else {
                return 0x1903 as libc::c_int as GLenum
            }
        }
        16 => return 0x1902 as libc::c_int as GLenum,
        17 => return 0x84f9 as libc::c_int as GLenum,
        13 => return 0x83f1 as libc::c_int as GLenum,
        14 => return 0x83f2 as libc::c_int as GLenum,
        15 => return 0x83f3 as libc::c_int as GLenum,
        18 => return 0x8c01 as libc::c_int as GLenum,
        19 => return 0x8c00 as libc::c_int as GLenum,
        20 => return 0x8c03 as libc::c_int as GLenum,
        21 => return 0x8c02 as libc::c_int as GLenum,
        22 => return 0x9274 as libc::c_int as GLenum,
        23 => return 0x9275 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
static mut _sapp_x11_quit_requested: bool = false;
static mut _sapp_x11_display: *mut Display = 0 as *const Display as *mut Display;
static mut _sapp_x11_screen: libc::c_int = 0;
static mut _sapp_x11_root: Window = 0;
static mut _sapp_x11_colormap: Colormap = 0;
static mut _sapp_x11_window: Window = 0;
static mut _sapp_x11_dpi: libc::c_float = 0.;
static mut _sapp_x11_window_state: libc::c_int = 0;
static mut _sapp_x11_error_code: libc::c_uchar = 0;
static mut _sapp_glx_libgl: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut _sapp_glx_major: libc::c_int = 0;
static mut _sapp_glx_minor: libc::c_int = 0;
static mut _sapp_glx_eventbase: libc::c_int = 0;
static mut _sapp_glx_errorbase: libc::c_int = 0;
static mut _sapp_glx_ctx: GLXContext = 0 as *const __GLXcontext as *mut __GLXcontext;
static mut _sapp_glx_window: GLXWindow = 0;
static mut _sapp_x11_UTF8_STRING: Atom = 0;
static mut _sapp_x11_WM_PROTOCOLS: Atom = 0;
static mut _sapp_x11_WM_DELETE_WINDOW: Atom = 0;
static mut _sapp_x11_WM_STATE: Atom = 0;
static mut _sapp_x11_NET_WM_NAME: Atom = 0;
static mut _sapp_x11_NET_WM_ICON_NAME: Atom = 0;
static mut _sapp_glx_GetFBConfigs: PFNGLXGETFBCONFIGSPROC = None;
static mut _sapp_glx_GetFBConfigAttrib: PFNGLXGETFBCONFIGATTRIBPROC = None;
static mut _sapp_glx_GetClientString: PFNGLXGETCLIENTSTRINGPROC = None;
static mut _sapp_glx_QueryExtension: PFNGLXQUERYEXTENSIONPROC = None;
static mut _sapp_glx_QueryVersion: PFNGLXQUERYVERSIONPROC = None;
static mut _sapp_glx_DestroyContext: PFNGLXDESTROYCONTEXTPROC = None;
static mut _sapp_glx_MakeCurrent: PFNGLXMAKECURRENTPROC = None;
static mut _sapp_glx_SwapBuffers: PFNGLXSWAPBUFFERSPROC = None;
static mut _sapp_glx_QueryExtensionsString: PFNGLXQUERYEXTENSIONSSTRINGPROC = None;
static mut _sapp_glx_CreateNewContext: PFNGLXCREATENEWCONTEXTPROC = None;
static mut _sapp_glx_GetVisualFromFBConfig: PFNGLXGETVISUALFROMFBCONFIGPROC = None;
static mut _sapp_glx_CreateWindow: PFNGLXCREATEWINDOWPROC = None;
static mut _sapp_glx_DestroyWindow: PFNGLXDESTROYWINDOWPROC = None;
static mut _sapp_glx_GetProcAddress: PFNGLXGETPROCADDRESSPROC = None;
static mut _sapp_glx_GetProcAddressARB: PFNGLXGETPROCADDRESSPROC = None;
static mut _sapp_glx_SwapIntervalEXT: PFNGLXSWAPINTERVALEXTPROC = None;
static mut _sapp_glx_SwapIntervalMESA: PFNGLXSWAPINTERVALMESAPROC = None;
static mut _sapp_glx_CreateContextAttribsARB: PFNGLXCREATECONTEXTATTRIBSARBPROC = None;
static mut _sapp_glx_EXT_swap_control: bool = false;
static mut _sapp_glx_MESA_swap_control: bool = false;
static mut _sapp_glx_ARB_multisample: bool = false;
static mut _sapp_glx_ARB_framebuffer_sRGB: bool = false;
static mut _sapp_glx_EXT_framebuffer_sRGB: bool = false;
static mut _sapp_glx_ARB_create_context: bool = false;
static mut _sapp_glx_ARB_create_context_profile: bool = false;
static mut _sapp_x11_keysymtab: [_sapp_x11_codepair; 828] = [
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1a1 as libc::c_int as uint16_t,
            ucs: 0x104 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1a2 as libc::c_int as uint16_t,
            ucs: 0x2d8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1a3 as libc::c_int as uint16_t,
            ucs: 0x141 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1a5 as libc::c_int as uint16_t,
            ucs: 0x13d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1a6 as libc::c_int as uint16_t,
            ucs: 0x15a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1a9 as libc::c_int as uint16_t,
            ucs: 0x160 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1aa as libc::c_int as uint16_t,
            ucs: 0x15e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1ab as libc::c_int as uint16_t,
            ucs: 0x164 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1ac as libc::c_int as uint16_t,
            ucs: 0x179 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1ae as libc::c_int as uint16_t,
            ucs: 0x17d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1af as libc::c_int as uint16_t,
            ucs: 0x17b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1b1 as libc::c_int as uint16_t,
            ucs: 0x105 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1b2 as libc::c_int as uint16_t,
            ucs: 0x2db as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1b3 as libc::c_int as uint16_t,
            ucs: 0x142 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1b5 as libc::c_int as uint16_t,
            ucs: 0x13e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1b6 as libc::c_int as uint16_t,
            ucs: 0x15b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1b7 as libc::c_int as uint16_t,
            ucs: 0x2c7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1b9 as libc::c_int as uint16_t,
            ucs: 0x161 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1ba as libc::c_int as uint16_t,
            ucs: 0x15f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1bb as libc::c_int as uint16_t,
            ucs: 0x165 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1bc as libc::c_int as uint16_t,
            ucs: 0x17a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1bd as libc::c_int as uint16_t,
            ucs: 0x2dd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1be as libc::c_int as uint16_t,
            ucs: 0x17e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1bf as libc::c_int as uint16_t,
            ucs: 0x17c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1c0 as libc::c_int as uint16_t,
            ucs: 0x154 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1c3 as libc::c_int as uint16_t,
            ucs: 0x102 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1c5 as libc::c_int as uint16_t,
            ucs: 0x139 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1c6 as libc::c_int as uint16_t,
            ucs: 0x106 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1c8 as libc::c_int as uint16_t,
            ucs: 0x10c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1ca as libc::c_int as uint16_t,
            ucs: 0x118 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1cc as libc::c_int as uint16_t,
            ucs: 0x11a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1cf as libc::c_int as uint16_t,
            ucs: 0x10e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1d0 as libc::c_int as uint16_t,
            ucs: 0x110 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1d1 as libc::c_int as uint16_t,
            ucs: 0x143 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1d2 as libc::c_int as uint16_t,
            ucs: 0x147 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1d5 as libc::c_int as uint16_t,
            ucs: 0x150 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1d8 as libc::c_int as uint16_t,
            ucs: 0x158 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1d9 as libc::c_int as uint16_t,
            ucs: 0x16e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1db as libc::c_int as uint16_t,
            ucs: 0x170 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1de as libc::c_int as uint16_t,
            ucs: 0x162 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1e0 as libc::c_int as uint16_t,
            ucs: 0x155 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1e3 as libc::c_int as uint16_t,
            ucs: 0x103 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1e5 as libc::c_int as uint16_t,
            ucs: 0x13a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1e6 as libc::c_int as uint16_t,
            ucs: 0x107 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1e8 as libc::c_int as uint16_t,
            ucs: 0x10d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1ea as libc::c_int as uint16_t,
            ucs: 0x119 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1ec as libc::c_int as uint16_t,
            ucs: 0x11b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1ef as libc::c_int as uint16_t,
            ucs: 0x10f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1f0 as libc::c_int as uint16_t,
            ucs: 0x111 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1f1 as libc::c_int as uint16_t,
            ucs: 0x144 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1f2 as libc::c_int as uint16_t,
            ucs: 0x148 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1f5 as libc::c_int as uint16_t,
            ucs: 0x151 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1f8 as libc::c_int as uint16_t,
            ucs: 0x159 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1f9 as libc::c_int as uint16_t,
            ucs: 0x16f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1fb as libc::c_int as uint16_t,
            ucs: 0x171 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1fe as libc::c_int as uint16_t,
            ucs: 0x163 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x1ff as libc::c_int as uint16_t,
            ucs: 0x2d9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2a1 as libc::c_int as uint16_t,
            ucs: 0x126 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2a6 as libc::c_int as uint16_t,
            ucs: 0x124 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2a9 as libc::c_int as uint16_t,
            ucs: 0x130 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2ab as libc::c_int as uint16_t,
            ucs: 0x11e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2ac as libc::c_int as uint16_t,
            ucs: 0x134 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2b1 as libc::c_int as uint16_t,
            ucs: 0x127 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2b6 as libc::c_int as uint16_t,
            ucs: 0x125 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2b9 as libc::c_int as uint16_t,
            ucs: 0x131 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2bb as libc::c_int as uint16_t,
            ucs: 0x11f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2bc as libc::c_int as uint16_t,
            ucs: 0x135 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2c5 as libc::c_int as uint16_t,
            ucs: 0x10a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2c6 as libc::c_int as uint16_t,
            ucs: 0x108 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2d5 as libc::c_int as uint16_t,
            ucs: 0x120 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2d8 as libc::c_int as uint16_t,
            ucs: 0x11c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2dd as libc::c_int as uint16_t,
            ucs: 0x16c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2de as libc::c_int as uint16_t,
            ucs: 0x15c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2e5 as libc::c_int as uint16_t,
            ucs: 0x10b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2e6 as libc::c_int as uint16_t,
            ucs: 0x109 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2f5 as libc::c_int as uint16_t,
            ucs: 0x121 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2f8 as libc::c_int as uint16_t,
            ucs: 0x11d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2fd as libc::c_int as uint16_t,
            ucs: 0x16d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x2fe as libc::c_int as uint16_t,
            ucs: 0x15d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3a2 as libc::c_int as uint16_t,
            ucs: 0x138 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3a3 as libc::c_int as uint16_t,
            ucs: 0x156 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3a5 as libc::c_int as uint16_t,
            ucs: 0x128 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3a6 as libc::c_int as uint16_t,
            ucs: 0x13b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3aa as libc::c_int as uint16_t,
            ucs: 0x112 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3ab as libc::c_int as uint16_t,
            ucs: 0x122 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3ac as libc::c_int as uint16_t,
            ucs: 0x166 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3b3 as libc::c_int as uint16_t,
            ucs: 0x157 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3b5 as libc::c_int as uint16_t,
            ucs: 0x129 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3b6 as libc::c_int as uint16_t,
            ucs: 0x13c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3ba as libc::c_int as uint16_t,
            ucs: 0x113 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3bb as libc::c_int as uint16_t,
            ucs: 0x123 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3bc as libc::c_int as uint16_t,
            ucs: 0x167 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3bd as libc::c_int as uint16_t,
            ucs: 0x14a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3bf as libc::c_int as uint16_t,
            ucs: 0x14b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3c0 as libc::c_int as uint16_t,
            ucs: 0x100 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3c7 as libc::c_int as uint16_t,
            ucs: 0x12e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3cc as libc::c_int as uint16_t,
            ucs: 0x116 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3cf as libc::c_int as uint16_t,
            ucs: 0x12a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3d1 as libc::c_int as uint16_t,
            ucs: 0x145 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3d2 as libc::c_int as uint16_t,
            ucs: 0x14c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3d3 as libc::c_int as uint16_t,
            ucs: 0x136 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3d9 as libc::c_int as uint16_t,
            ucs: 0x172 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3dd as libc::c_int as uint16_t,
            ucs: 0x168 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3de as libc::c_int as uint16_t,
            ucs: 0x16a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3e0 as libc::c_int as uint16_t,
            ucs: 0x101 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3e7 as libc::c_int as uint16_t,
            ucs: 0x12f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3ec as libc::c_int as uint16_t,
            ucs: 0x117 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3ef as libc::c_int as uint16_t,
            ucs: 0x12b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3f1 as libc::c_int as uint16_t,
            ucs: 0x146 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3f2 as libc::c_int as uint16_t,
            ucs: 0x14d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3f3 as libc::c_int as uint16_t,
            ucs: 0x137 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3f9 as libc::c_int as uint16_t,
            ucs: 0x173 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3fd as libc::c_int as uint16_t,
            ucs: 0x169 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x3fe as libc::c_int as uint16_t,
            ucs: 0x16b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x47e as libc::c_int as uint16_t,
            ucs: 0x203e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4a1 as libc::c_int as uint16_t,
            ucs: 0x3002 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4a2 as libc::c_int as uint16_t,
            ucs: 0x300c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4a3 as libc::c_int as uint16_t,
            ucs: 0x300d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4a4 as libc::c_int as uint16_t,
            ucs: 0x3001 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4a5 as libc::c_int as uint16_t,
            ucs: 0x30fb as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4a6 as libc::c_int as uint16_t,
            ucs: 0x30f2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4a7 as libc::c_int as uint16_t,
            ucs: 0x30a1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4a8 as libc::c_int as uint16_t,
            ucs: 0x30a3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4a9 as libc::c_int as uint16_t,
            ucs: 0x30a5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4aa as libc::c_int as uint16_t,
            ucs: 0x30a7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4ab as libc::c_int as uint16_t,
            ucs: 0x30a9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4ac as libc::c_int as uint16_t,
            ucs: 0x30e3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4ad as libc::c_int as uint16_t,
            ucs: 0x30e5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4ae as libc::c_int as uint16_t,
            ucs: 0x30e7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4af as libc::c_int as uint16_t,
            ucs: 0x30c3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4b0 as libc::c_int as uint16_t,
            ucs: 0x30fc as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4b1 as libc::c_int as uint16_t,
            ucs: 0x30a2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4b2 as libc::c_int as uint16_t,
            ucs: 0x30a4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4b3 as libc::c_int as uint16_t,
            ucs: 0x30a6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4b4 as libc::c_int as uint16_t,
            ucs: 0x30a8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4b5 as libc::c_int as uint16_t,
            ucs: 0x30aa as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4b6 as libc::c_int as uint16_t,
            ucs: 0x30ab as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4b7 as libc::c_int as uint16_t,
            ucs: 0x30ad as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4b8 as libc::c_int as uint16_t,
            ucs: 0x30af as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4b9 as libc::c_int as uint16_t,
            ucs: 0x30b1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4ba as libc::c_int as uint16_t,
            ucs: 0x30b3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4bb as libc::c_int as uint16_t,
            ucs: 0x30b5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4bc as libc::c_int as uint16_t,
            ucs: 0x30b7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4bd as libc::c_int as uint16_t,
            ucs: 0x30b9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4be as libc::c_int as uint16_t,
            ucs: 0x30bb as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4bf as libc::c_int as uint16_t,
            ucs: 0x30bd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4c0 as libc::c_int as uint16_t,
            ucs: 0x30bf as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4c1 as libc::c_int as uint16_t,
            ucs: 0x30c1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4c2 as libc::c_int as uint16_t,
            ucs: 0x30c4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4c3 as libc::c_int as uint16_t,
            ucs: 0x30c6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4c4 as libc::c_int as uint16_t,
            ucs: 0x30c8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4c5 as libc::c_int as uint16_t,
            ucs: 0x30ca as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4c6 as libc::c_int as uint16_t,
            ucs: 0x30cb as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4c7 as libc::c_int as uint16_t,
            ucs: 0x30cc as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4c8 as libc::c_int as uint16_t,
            ucs: 0x30cd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4c9 as libc::c_int as uint16_t,
            ucs: 0x30ce as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4ca as libc::c_int as uint16_t,
            ucs: 0x30cf as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4cb as libc::c_int as uint16_t,
            ucs: 0x30d2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4cc as libc::c_int as uint16_t,
            ucs: 0x30d5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4cd as libc::c_int as uint16_t,
            ucs: 0x30d8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4ce as libc::c_int as uint16_t,
            ucs: 0x30db as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4cf as libc::c_int as uint16_t,
            ucs: 0x30de as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4d0 as libc::c_int as uint16_t,
            ucs: 0x30df as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4d1 as libc::c_int as uint16_t,
            ucs: 0x30e0 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4d2 as libc::c_int as uint16_t,
            ucs: 0x30e1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4d3 as libc::c_int as uint16_t,
            ucs: 0x30e2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4d4 as libc::c_int as uint16_t,
            ucs: 0x30e4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4d5 as libc::c_int as uint16_t,
            ucs: 0x30e6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4d6 as libc::c_int as uint16_t,
            ucs: 0x30e8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4d7 as libc::c_int as uint16_t,
            ucs: 0x30e9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4d8 as libc::c_int as uint16_t,
            ucs: 0x30ea as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4d9 as libc::c_int as uint16_t,
            ucs: 0x30eb as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4da as libc::c_int as uint16_t,
            ucs: 0x30ec as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4db as libc::c_int as uint16_t,
            ucs: 0x30ed as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4dc as libc::c_int as uint16_t,
            ucs: 0x30ef as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4dd as libc::c_int as uint16_t,
            ucs: 0x30f3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4de as libc::c_int as uint16_t,
            ucs: 0x309b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x4df as libc::c_int as uint16_t,
            ucs: 0x309c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5ac as libc::c_int as uint16_t,
            ucs: 0x60c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5bb as libc::c_int as uint16_t,
            ucs: 0x61b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5bf as libc::c_int as uint16_t,
            ucs: 0x61f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5c1 as libc::c_int as uint16_t,
            ucs: 0x621 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5c2 as libc::c_int as uint16_t,
            ucs: 0x622 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5c3 as libc::c_int as uint16_t,
            ucs: 0x623 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5c4 as libc::c_int as uint16_t,
            ucs: 0x624 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5c5 as libc::c_int as uint16_t,
            ucs: 0x625 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5c6 as libc::c_int as uint16_t,
            ucs: 0x626 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5c7 as libc::c_int as uint16_t,
            ucs: 0x627 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5c8 as libc::c_int as uint16_t,
            ucs: 0x628 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5c9 as libc::c_int as uint16_t,
            ucs: 0x629 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5ca as libc::c_int as uint16_t,
            ucs: 0x62a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5cb as libc::c_int as uint16_t,
            ucs: 0x62b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5cc as libc::c_int as uint16_t,
            ucs: 0x62c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5cd as libc::c_int as uint16_t,
            ucs: 0x62d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5ce as libc::c_int as uint16_t,
            ucs: 0x62e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5cf as libc::c_int as uint16_t,
            ucs: 0x62f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5d0 as libc::c_int as uint16_t,
            ucs: 0x630 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5d1 as libc::c_int as uint16_t,
            ucs: 0x631 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5d2 as libc::c_int as uint16_t,
            ucs: 0x632 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5d3 as libc::c_int as uint16_t,
            ucs: 0x633 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5d4 as libc::c_int as uint16_t,
            ucs: 0x634 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5d5 as libc::c_int as uint16_t,
            ucs: 0x635 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5d6 as libc::c_int as uint16_t,
            ucs: 0x636 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5d7 as libc::c_int as uint16_t,
            ucs: 0x637 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5d8 as libc::c_int as uint16_t,
            ucs: 0x638 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5d9 as libc::c_int as uint16_t,
            ucs: 0x639 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5da as libc::c_int as uint16_t,
            ucs: 0x63a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5e0 as libc::c_int as uint16_t,
            ucs: 0x640 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5e1 as libc::c_int as uint16_t,
            ucs: 0x641 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5e2 as libc::c_int as uint16_t,
            ucs: 0x642 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5e3 as libc::c_int as uint16_t,
            ucs: 0x643 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5e4 as libc::c_int as uint16_t,
            ucs: 0x644 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5e5 as libc::c_int as uint16_t,
            ucs: 0x645 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5e6 as libc::c_int as uint16_t,
            ucs: 0x646 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5e7 as libc::c_int as uint16_t,
            ucs: 0x647 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5e8 as libc::c_int as uint16_t,
            ucs: 0x648 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5e9 as libc::c_int as uint16_t,
            ucs: 0x649 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5ea as libc::c_int as uint16_t,
            ucs: 0x64a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5eb as libc::c_int as uint16_t,
            ucs: 0x64b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5ec as libc::c_int as uint16_t,
            ucs: 0x64c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5ed as libc::c_int as uint16_t,
            ucs: 0x64d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5ee as libc::c_int as uint16_t,
            ucs: 0x64e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5ef as libc::c_int as uint16_t,
            ucs: 0x64f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5f0 as libc::c_int as uint16_t,
            ucs: 0x650 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5f1 as libc::c_int as uint16_t,
            ucs: 0x651 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x5f2 as libc::c_int as uint16_t,
            ucs: 0x652 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6a1 as libc::c_int as uint16_t,
            ucs: 0x452 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6a2 as libc::c_int as uint16_t,
            ucs: 0x453 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6a3 as libc::c_int as uint16_t,
            ucs: 0x451 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6a4 as libc::c_int as uint16_t,
            ucs: 0x454 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6a5 as libc::c_int as uint16_t,
            ucs: 0x455 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6a6 as libc::c_int as uint16_t,
            ucs: 0x456 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6a7 as libc::c_int as uint16_t,
            ucs: 0x457 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6a8 as libc::c_int as uint16_t,
            ucs: 0x458 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6a9 as libc::c_int as uint16_t,
            ucs: 0x459 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6aa as libc::c_int as uint16_t,
            ucs: 0x45a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ab as libc::c_int as uint16_t,
            ucs: 0x45b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ac as libc::c_int as uint16_t,
            ucs: 0x45c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ae as libc::c_int as uint16_t,
            ucs: 0x45e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6af as libc::c_int as uint16_t,
            ucs: 0x45f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6b0 as libc::c_int as uint16_t,
            ucs: 0x2116 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6b1 as libc::c_int as uint16_t,
            ucs: 0x402 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6b2 as libc::c_int as uint16_t,
            ucs: 0x403 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6b3 as libc::c_int as uint16_t,
            ucs: 0x401 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6b4 as libc::c_int as uint16_t,
            ucs: 0x404 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6b5 as libc::c_int as uint16_t,
            ucs: 0x405 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6b6 as libc::c_int as uint16_t,
            ucs: 0x406 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6b7 as libc::c_int as uint16_t,
            ucs: 0x407 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6b8 as libc::c_int as uint16_t,
            ucs: 0x408 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6b9 as libc::c_int as uint16_t,
            ucs: 0x409 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ba as libc::c_int as uint16_t,
            ucs: 0x40a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6bb as libc::c_int as uint16_t,
            ucs: 0x40b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6bc as libc::c_int as uint16_t,
            ucs: 0x40c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6be as libc::c_int as uint16_t,
            ucs: 0x40e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6bf as libc::c_int as uint16_t,
            ucs: 0x40f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6c0 as libc::c_int as uint16_t,
            ucs: 0x44e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6c1 as libc::c_int as uint16_t,
            ucs: 0x430 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6c2 as libc::c_int as uint16_t,
            ucs: 0x431 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6c3 as libc::c_int as uint16_t,
            ucs: 0x446 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6c4 as libc::c_int as uint16_t,
            ucs: 0x434 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6c5 as libc::c_int as uint16_t,
            ucs: 0x435 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6c6 as libc::c_int as uint16_t,
            ucs: 0x444 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6c7 as libc::c_int as uint16_t,
            ucs: 0x433 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6c8 as libc::c_int as uint16_t,
            ucs: 0x445 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6c9 as libc::c_int as uint16_t,
            ucs: 0x438 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ca as libc::c_int as uint16_t,
            ucs: 0x439 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6cb as libc::c_int as uint16_t,
            ucs: 0x43a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6cc as libc::c_int as uint16_t,
            ucs: 0x43b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6cd as libc::c_int as uint16_t,
            ucs: 0x43c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ce as libc::c_int as uint16_t,
            ucs: 0x43d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6cf as libc::c_int as uint16_t,
            ucs: 0x43e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6d0 as libc::c_int as uint16_t,
            ucs: 0x43f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6d1 as libc::c_int as uint16_t,
            ucs: 0x44f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6d2 as libc::c_int as uint16_t,
            ucs: 0x440 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6d3 as libc::c_int as uint16_t,
            ucs: 0x441 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6d4 as libc::c_int as uint16_t,
            ucs: 0x442 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6d5 as libc::c_int as uint16_t,
            ucs: 0x443 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6d6 as libc::c_int as uint16_t,
            ucs: 0x436 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6d7 as libc::c_int as uint16_t,
            ucs: 0x432 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6d8 as libc::c_int as uint16_t,
            ucs: 0x44c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6d9 as libc::c_int as uint16_t,
            ucs: 0x44b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6da as libc::c_int as uint16_t,
            ucs: 0x437 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6db as libc::c_int as uint16_t,
            ucs: 0x448 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6dc as libc::c_int as uint16_t,
            ucs: 0x44d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6dd as libc::c_int as uint16_t,
            ucs: 0x449 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6de as libc::c_int as uint16_t,
            ucs: 0x447 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6df as libc::c_int as uint16_t,
            ucs: 0x44a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6e0 as libc::c_int as uint16_t,
            ucs: 0x42e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6e1 as libc::c_int as uint16_t,
            ucs: 0x410 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6e2 as libc::c_int as uint16_t,
            ucs: 0x411 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6e3 as libc::c_int as uint16_t,
            ucs: 0x426 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6e4 as libc::c_int as uint16_t,
            ucs: 0x414 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6e5 as libc::c_int as uint16_t,
            ucs: 0x415 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6e6 as libc::c_int as uint16_t,
            ucs: 0x424 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6e7 as libc::c_int as uint16_t,
            ucs: 0x413 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6e8 as libc::c_int as uint16_t,
            ucs: 0x425 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6e9 as libc::c_int as uint16_t,
            ucs: 0x418 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ea as libc::c_int as uint16_t,
            ucs: 0x419 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6eb as libc::c_int as uint16_t,
            ucs: 0x41a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ec as libc::c_int as uint16_t,
            ucs: 0x41b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ed as libc::c_int as uint16_t,
            ucs: 0x41c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ee as libc::c_int as uint16_t,
            ucs: 0x41d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ef as libc::c_int as uint16_t,
            ucs: 0x41e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6f0 as libc::c_int as uint16_t,
            ucs: 0x41f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6f1 as libc::c_int as uint16_t,
            ucs: 0x42f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6f2 as libc::c_int as uint16_t,
            ucs: 0x420 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6f3 as libc::c_int as uint16_t,
            ucs: 0x421 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6f4 as libc::c_int as uint16_t,
            ucs: 0x422 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6f5 as libc::c_int as uint16_t,
            ucs: 0x423 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6f6 as libc::c_int as uint16_t,
            ucs: 0x416 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6f7 as libc::c_int as uint16_t,
            ucs: 0x412 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6f8 as libc::c_int as uint16_t,
            ucs: 0x42c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6f9 as libc::c_int as uint16_t,
            ucs: 0x42b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6fa as libc::c_int as uint16_t,
            ucs: 0x417 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6fb as libc::c_int as uint16_t,
            ucs: 0x428 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6fc as libc::c_int as uint16_t,
            ucs: 0x42d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6fd as libc::c_int as uint16_t,
            ucs: 0x429 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6fe as libc::c_int as uint16_t,
            ucs: 0x427 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x6ff as libc::c_int as uint16_t,
            ucs: 0x42a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7a1 as libc::c_int as uint16_t,
            ucs: 0x386 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7a2 as libc::c_int as uint16_t,
            ucs: 0x388 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7a3 as libc::c_int as uint16_t,
            ucs: 0x389 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7a4 as libc::c_int as uint16_t,
            ucs: 0x38a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7a5 as libc::c_int as uint16_t,
            ucs: 0x3aa as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7a7 as libc::c_int as uint16_t,
            ucs: 0x38c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7a8 as libc::c_int as uint16_t,
            ucs: 0x38e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7a9 as libc::c_int as uint16_t,
            ucs: 0x3ab as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7ab as libc::c_int as uint16_t,
            ucs: 0x38f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7ae as libc::c_int as uint16_t,
            ucs: 0x385 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7af as libc::c_int as uint16_t,
            ucs: 0x2015 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7b1 as libc::c_int as uint16_t,
            ucs: 0x3ac as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7b2 as libc::c_int as uint16_t,
            ucs: 0x3ad as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7b3 as libc::c_int as uint16_t,
            ucs: 0x3ae as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7b4 as libc::c_int as uint16_t,
            ucs: 0x3af as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7b5 as libc::c_int as uint16_t,
            ucs: 0x3ca as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7b6 as libc::c_int as uint16_t,
            ucs: 0x390 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7b7 as libc::c_int as uint16_t,
            ucs: 0x3cc as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7b8 as libc::c_int as uint16_t,
            ucs: 0x3cd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7b9 as libc::c_int as uint16_t,
            ucs: 0x3cb as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7ba as libc::c_int as uint16_t,
            ucs: 0x3b0 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7bb as libc::c_int as uint16_t,
            ucs: 0x3ce as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7c1 as libc::c_int as uint16_t,
            ucs: 0x391 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7c2 as libc::c_int as uint16_t,
            ucs: 0x392 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7c3 as libc::c_int as uint16_t,
            ucs: 0x393 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7c4 as libc::c_int as uint16_t,
            ucs: 0x394 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7c5 as libc::c_int as uint16_t,
            ucs: 0x395 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7c6 as libc::c_int as uint16_t,
            ucs: 0x396 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7c7 as libc::c_int as uint16_t,
            ucs: 0x397 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7c8 as libc::c_int as uint16_t,
            ucs: 0x398 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7c9 as libc::c_int as uint16_t,
            ucs: 0x399 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7ca as libc::c_int as uint16_t,
            ucs: 0x39a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7cb as libc::c_int as uint16_t,
            ucs: 0x39b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7cc as libc::c_int as uint16_t,
            ucs: 0x39c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7cd as libc::c_int as uint16_t,
            ucs: 0x39d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7ce as libc::c_int as uint16_t,
            ucs: 0x39e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7cf as libc::c_int as uint16_t,
            ucs: 0x39f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7d0 as libc::c_int as uint16_t,
            ucs: 0x3a0 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7d1 as libc::c_int as uint16_t,
            ucs: 0x3a1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7d2 as libc::c_int as uint16_t,
            ucs: 0x3a3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7d4 as libc::c_int as uint16_t,
            ucs: 0x3a4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7d5 as libc::c_int as uint16_t,
            ucs: 0x3a5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7d6 as libc::c_int as uint16_t,
            ucs: 0x3a6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7d7 as libc::c_int as uint16_t,
            ucs: 0x3a7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7d8 as libc::c_int as uint16_t,
            ucs: 0x3a8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7d9 as libc::c_int as uint16_t,
            ucs: 0x3a9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7e1 as libc::c_int as uint16_t,
            ucs: 0x3b1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7e2 as libc::c_int as uint16_t,
            ucs: 0x3b2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7e3 as libc::c_int as uint16_t,
            ucs: 0x3b3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7e4 as libc::c_int as uint16_t,
            ucs: 0x3b4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7e5 as libc::c_int as uint16_t,
            ucs: 0x3b5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7e6 as libc::c_int as uint16_t,
            ucs: 0x3b6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7e7 as libc::c_int as uint16_t,
            ucs: 0x3b7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7e8 as libc::c_int as uint16_t,
            ucs: 0x3b8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7e9 as libc::c_int as uint16_t,
            ucs: 0x3b9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7ea as libc::c_int as uint16_t,
            ucs: 0x3ba as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7eb as libc::c_int as uint16_t,
            ucs: 0x3bb as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7ec as libc::c_int as uint16_t,
            ucs: 0x3bc as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7ed as libc::c_int as uint16_t,
            ucs: 0x3bd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7ee as libc::c_int as uint16_t,
            ucs: 0x3be as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7ef as libc::c_int as uint16_t,
            ucs: 0x3bf as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7f0 as libc::c_int as uint16_t,
            ucs: 0x3c0 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7f1 as libc::c_int as uint16_t,
            ucs: 0x3c1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7f2 as libc::c_int as uint16_t,
            ucs: 0x3c3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7f3 as libc::c_int as uint16_t,
            ucs: 0x3c2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7f4 as libc::c_int as uint16_t,
            ucs: 0x3c4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7f5 as libc::c_int as uint16_t,
            ucs: 0x3c5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7f6 as libc::c_int as uint16_t,
            ucs: 0x3c6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7f7 as libc::c_int as uint16_t,
            ucs: 0x3c7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7f8 as libc::c_int as uint16_t,
            ucs: 0x3c8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x7f9 as libc::c_int as uint16_t,
            ucs: 0x3c9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8a1 as libc::c_int as uint16_t,
            ucs: 0x23b7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8a2 as libc::c_int as uint16_t,
            ucs: 0x250c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8a3 as libc::c_int as uint16_t,
            ucs: 0x2500 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8a4 as libc::c_int as uint16_t,
            ucs: 0x2320 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8a5 as libc::c_int as uint16_t,
            ucs: 0x2321 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8a6 as libc::c_int as uint16_t,
            ucs: 0x2502 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8a7 as libc::c_int as uint16_t,
            ucs: 0x23a1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8a8 as libc::c_int as uint16_t,
            ucs: 0x23a3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8a9 as libc::c_int as uint16_t,
            ucs: 0x23a4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8aa as libc::c_int as uint16_t,
            ucs: 0x23a6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8ab as libc::c_int as uint16_t,
            ucs: 0x239b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8ac as libc::c_int as uint16_t,
            ucs: 0x239d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8ad as libc::c_int as uint16_t,
            ucs: 0x239e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8ae as libc::c_int as uint16_t,
            ucs: 0x23a0 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8af as libc::c_int as uint16_t,
            ucs: 0x23a8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8b0 as libc::c_int as uint16_t,
            ucs: 0x23ac as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8bc as libc::c_int as uint16_t,
            ucs: 0x2264 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8bd as libc::c_int as uint16_t,
            ucs: 0x2260 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8be as libc::c_int as uint16_t,
            ucs: 0x2265 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8bf as libc::c_int as uint16_t,
            ucs: 0x222b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8c0 as libc::c_int as uint16_t,
            ucs: 0x2234 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8c1 as libc::c_int as uint16_t,
            ucs: 0x221d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8c2 as libc::c_int as uint16_t,
            ucs: 0x221e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8c5 as libc::c_int as uint16_t,
            ucs: 0x2207 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8c8 as libc::c_int as uint16_t,
            ucs: 0x223c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8c9 as libc::c_int as uint16_t,
            ucs: 0x2243 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8cd as libc::c_int as uint16_t,
            ucs: 0x21d4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8ce as libc::c_int as uint16_t,
            ucs: 0x21d2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8cf as libc::c_int as uint16_t,
            ucs: 0x2261 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8d6 as libc::c_int as uint16_t,
            ucs: 0x221a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8da as libc::c_int as uint16_t,
            ucs: 0x2282 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8db as libc::c_int as uint16_t,
            ucs: 0x2283 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8dc as libc::c_int as uint16_t,
            ucs: 0x2229 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8dd as libc::c_int as uint16_t,
            ucs: 0x222a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8de as libc::c_int as uint16_t,
            ucs: 0x2227 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8df as libc::c_int as uint16_t,
            ucs: 0x2228 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8ef as libc::c_int as uint16_t,
            ucs: 0x2202 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8f6 as libc::c_int as uint16_t,
            ucs: 0x192 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8fb as libc::c_int as uint16_t,
            ucs: 0x2190 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8fc as libc::c_int as uint16_t,
            ucs: 0x2191 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8fd as libc::c_int as uint16_t,
            ucs: 0x2192 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x8fe as libc::c_int as uint16_t,
            ucs: 0x2193 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9e0 as libc::c_int as uint16_t,
            ucs: 0x25c6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9e1 as libc::c_int as uint16_t,
            ucs: 0x2592 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9e2 as libc::c_int as uint16_t,
            ucs: 0x2409 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9e3 as libc::c_int as uint16_t,
            ucs: 0x240c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9e4 as libc::c_int as uint16_t,
            ucs: 0x240d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9e5 as libc::c_int as uint16_t,
            ucs: 0x240a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9e8 as libc::c_int as uint16_t,
            ucs: 0x2424 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9e9 as libc::c_int as uint16_t,
            ucs: 0x240b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9ea as libc::c_int as uint16_t,
            ucs: 0x2518 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9eb as libc::c_int as uint16_t,
            ucs: 0x2510 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9ec as libc::c_int as uint16_t,
            ucs: 0x250c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9ed as libc::c_int as uint16_t,
            ucs: 0x2514 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9ee as libc::c_int as uint16_t,
            ucs: 0x253c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9ef as libc::c_int as uint16_t,
            ucs: 0x23ba as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9f0 as libc::c_int as uint16_t,
            ucs: 0x23bb as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9f1 as libc::c_int as uint16_t,
            ucs: 0x2500 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9f2 as libc::c_int as uint16_t,
            ucs: 0x23bc as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9f3 as libc::c_int as uint16_t,
            ucs: 0x23bd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9f4 as libc::c_int as uint16_t,
            ucs: 0x251c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9f5 as libc::c_int as uint16_t,
            ucs: 0x2524 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9f6 as libc::c_int as uint16_t,
            ucs: 0x2534 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9f7 as libc::c_int as uint16_t,
            ucs: 0x252c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x9f8 as libc::c_int as uint16_t,
            ucs: 0x2502 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaa1 as libc::c_int as uint16_t,
            ucs: 0x2003 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaa2 as libc::c_int as uint16_t,
            ucs: 0x2002 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaa3 as libc::c_int as uint16_t,
            ucs: 0x2004 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaa4 as libc::c_int as uint16_t,
            ucs: 0x2005 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaa5 as libc::c_int as uint16_t,
            ucs: 0x2007 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaa6 as libc::c_int as uint16_t,
            ucs: 0x2008 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaa7 as libc::c_int as uint16_t,
            ucs: 0x2009 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaa8 as libc::c_int as uint16_t,
            ucs: 0x200a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaa9 as libc::c_int as uint16_t,
            ucs: 0x2014 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaaa as libc::c_int as uint16_t,
            ucs: 0x2013 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaae as libc::c_int as uint16_t,
            ucs: 0x2026 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaaf as libc::c_int as uint16_t,
            ucs: 0x2025 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xab0 as libc::c_int as uint16_t,
            ucs: 0x2153 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xab1 as libc::c_int as uint16_t,
            ucs: 0x2154 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xab2 as libc::c_int as uint16_t,
            ucs: 0x2155 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xab3 as libc::c_int as uint16_t,
            ucs: 0x2156 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xab4 as libc::c_int as uint16_t,
            ucs: 0x2157 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xab5 as libc::c_int as uint16_t,
            ucs: 0x2158 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xab6 as libc::c_int as uint16_t,
            ucs: 0x2159 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xab7 as libc::c_int as uint16_t,
            ucs: 0x215a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xab8 as libc::c_int as uint16_t,
            ucs: 0x2105 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xabb as libc::c_int as uint16_t,
            ucs: 0x2012 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xabc as libc::c_int as uint16_t,
            ucs: 0x2329 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xabe as libc::c_int as uint16_t,
            ucs: 0x232a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xac3 as libc::c_int as uint16_t,
            ucs: 0x215b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xac4 as libc::c_int as uint16_t,
            ucs: 0x215c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xac5 as libc::c_int as uint16_t,
            ucs: 0x215d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xac6 as libc::c_int as uint16_t,
            ucs: 0x215e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xac9 as libc::c_int as uint16_t,
            ucs: 0x2122 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaca as libc::c_int as uint16_t,
            ucs: 0x2613 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xacc as libc::c_int as uint16_t,
            ucs: 0x25c1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xacd as libc::c_int as uint16_t,
            ucs: 0x25b7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xace as libc::c_int as uint16_t,
            ucs: 0x25cb as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xacf as libc::c_int as uint16_t,
            ucs: 0x25af as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xad0 as libc::c_int as uint16_t,
            ucs: 0x2018 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xad1 as libc::c_int as uint16_t,
            ucs: 0x2019 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xad2 as libc::c_int as uint16_t,
            ucs: 0x201c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xad3 as libc::c_int as uint16_t,
            ucs: 0x201d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xad4 as libc::c_int as uint16_t,
            ucs: 0x211e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xad6 as libc::c_int as uint16_t,
            ucs: 0x2032 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xad7 as libc::c_int as uint16_t,
            ucs: 0x2033 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xad9 as libc::c_int as uint16_t,
            ucs: 0x271d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xadb as libc::c_int as uint16_t,
            ucs: 0x25ac as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xadc as libc::c_int as uint16_t,
            ucs: 0x25c0 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xadd as libc::c_int as uint16_t,
            ucs: 0x25b6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xade as libc::c_int as uint16_t,
            ucs: 0x25cf as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xadf as libc::c_int as uint16_t,
            ucs: 0x25ae as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xae0 as libc::c_int as uint16_t,
            ucs: 0x25e6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xae1 as libc::c_int as uint16_t,
            ucs: 0x25ab as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xae2 as libc::c_int as uint16_t,
            ucs: 0x25ad as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xae3 as libc::c_int as uint16_t,
            ucs: 0x25b3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xae4 as libc::c_int as uint16_t,
            ucs: 0x25bd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xae5 as libc::c_int as uint16_t,
            ucs: 0x2606 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xae6 as libc::c_int as uint16_t,
            ucs: 0x2022 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xae7 as libc::c_int as uint16_t,
            ucs: 0x25aa as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xae8 as libc::c_int as uint16_t,
            ucs: 0x25b2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xae9 as libc::c_int as uint16_t,
            ucs: 0x25bc as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaea as libc::c_int as uint16_t,
            ucs: 0x261c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaeb as libc::c_int as uint16_t,
            ucs: 0x261e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaec as libc::c_int as uint16_t,
            ucs: 0x2663 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaed as libc::c_int as uint16_t,
            ucs: 0x2666 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaee as libc::c_int as uint16_t,
            ucs: 0x2665 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaf0 as libc::c_int as uint16_t,
            ucs: 0x2720 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaf1 as libc::c_int as uint16_t,
            ucs: 0x2020 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaf2 as libc::c_int as uint16_t,
            ucs: 0x2021 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaf3 as libc::c_int as uint16_t,
            ucs: 0x2713 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaf4 as libc::c_int as uint16_t,
            ucs: 0x2717 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaf5 as libc::c_int as uint16_t,
            ucs: 0x266f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaf6 as libc::c_int as uint16_t,
            ucs: 0x266d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaf7 as libc::c_int as uint16_t,
            ucs: 0x2642 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaf8 as libc::c_int as uint16_t,
            ucs: 0x2640 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xaf9 as libc::c_int as uint16_t,
            ucs: 0x260e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xafa as libc::c_int as uint16_t,
            ucs: 0x2315 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xafb as libc::c_int as uint16_t,
            ucs: 0x2117 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xafc as libc::c_int as uint16_t,
            ucs: 0x2038 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xafd as libc::c_int as uint16_t,
            ucs: 0x201a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xafe as libc::c_int as uint16_t,
            ucs: 0x201e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xba3 as libc::c_int as uint16_t,
            ucs: 0x3c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xba6 as libc::c_int as uint16_t,
            ucs: 0x3e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xba8 as libc::c_int as uint16_t,
            ucs: 0x2228 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xba9 as libc::c_int as uint16_t,
            ucs: 0x2227 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbc0 as libc::c_int as uint16_t,
            ucs: 0xaf as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbc2 as libc::c_int as uint16_t,
            ucs: 0x22a5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbc3 as libc::c_int as uint16_t,
            ucs: 0x2229 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbc4 as libc::c_int as uint16_t,
            ucs: 0x230a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbc6 as libc::c_int as uint16_t,
            ucs: 0x5f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbca as libc::c_int as uint16_t,
            ucs: 0x2218 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbcc as libc::c_int as uint16_t,
            ucs: 0x2395 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbce as libc::c_int as uint16_t,
            ucs: 0x22a4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbcf as libc::c_int as uint16_t,
            ucs: 0x25cb as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbd3 as libc::c_int as uint16_t,
            ucs: 0x2308 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbd6 as libc::c_int as uint16_t,
            ucs: 0x222a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbd8 as libc::c_int as uint16_t,
            ucs: 0x2283 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbda as libc::c_int as uint16_t,
            ucs: 0x2282 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbdc as libc::c_int as uint16_t,
            ucs: 0x22a2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xbfc as libc::c_int as uint16_t,
            ucs: 0x22a3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcdf as libc::c_int as uint16_t,
            ucs: 0x2017 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xce0 as libc::c_int as uint16_t,
            ucs: 0x5d0 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xce1 as libc::c_int as uint16_t,
            ucs: 0x5d1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xce2 as libc::c_int as uint16_t,
            ucs: 0x5d2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xce3 as libc::c_int as uint16_t,
            ucs: 0x5d3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xce4 as libc::c_int as uint16_t,
            ucs: 0x5d4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xce5 as libc::c_int as uint16_t,
            ucs: 0x5d5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xce6 as libc::c_int as uint16_t,
            ucs: 0x5d6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xce7 as libc::c_int as uint16_t,
            ucs: 0x5d7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xce8 as libc::c_int as uint16_t,
            ucs: 0x5d8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xce9 as libc::c_int as uint16_t,
            ucs: 0x5d9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcea as libc::c_int as uint16_t,
            ucs: 0x5da as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xceb as libc::c_int as uint16_t,
            ucs: 0x5db as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcec as libc::c_int as uint16_t,
            ucs: 0x5dc as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xced as libc::c_int as uint16_t,
            ucs: 0x5dd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcee as libc::c_int as uint16_t,
            ucs: 0x5de as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcef as libc::c_int as uint16_t,
            ucs: 0x5df as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcf0 as libc::c_int as uint16_t,
            ucs: 0x5e0 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcf1 as libc::c_int as uint16_t,
            ucs: 0x5e1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcf2 as libc::c_int as uint16_t,
            ucs: 0x5e2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcf3 as libc::c_int as uint16_t,
            ucs: 0x5e3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcf4 as libc::c_int as uint16_t,
            ucs: 0x5e4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcf5 as libc::c_int as uint16_t,
            ucs: 0x5e5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcf6 as libc::c_int as uint16_t,
            ucs: 0x5e6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcf7 as libc::c_int as uint16_t,
            ucs: 0x5e7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcf8 as libc::c_int as uint16_t,
            ucs: 0x5e8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcf9 as libc::c_int as uint16_t,
            ucs: 0x5e9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xcfa as libc::c_int as uint16_t,
            ucs: 0x5ea as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xda1 as libc::c_int as uint16_t,
            ucs: 0xe01 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xda2 as libc::c_int as uint16_t,
            ucs: 0xe02 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xda3 as libc::c_int as uint16_t,
            ucs: 0xe03 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xda4 as libc::c_int as uint16_t,
            ucs: 0xe04 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xda5 as libc::c_int as uint16_t,
            ucs: 0xe05 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xda6 as libc::c_int as uint16_t,
            ucs: 0xe06 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xda7 as libc::c_int as uint16_t,
            ucs: 0xe07 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xda8 as libc::c_int as uint16_t,
            ucs: 0xe08 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xda9 as libc::c_int as uint16_t,
            ucs: 0xe09 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdaa as libc::c_int as uint16_t,
            ucs: 0xe0a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdab as libc::c_int as uint16_t,
            ucs: 0xe0b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdac as libc::c_int as uint16_t,
            ucs: 0xe0c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdad as libc::c_int as uint16_t,
            ucs: 0xe0d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdae as libc::c_int as uint16_t,
            ucs: 0xe0e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdaf as libc::c_int as uint16_t,
            ucs: 0xe0f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdb0 as libc::c_int as uint16_t,
            ucs: 0xe10 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdb1 as libc::c_int as uint16_t,
            ucs: 0xe11 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdb2 as libc::c_int as uint16_t,
            ucs: 0xe12 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdb3 as libc::c_int as uint16_t,
            ucs: 0xe13 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdb4 as libc::c_int as uint16_t,
            ucs: 0xe14 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdb5 as libc::c_int as uint16_t,
            ucs: 0xe15 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdb6 as libc::c_int as uint16_t,
            ucs: 0xe16 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdb7 as libc::c_int as uint16_t,
            ucs: 0xe17 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdb8 as libc::c_int as uint16_t,
            ucs: 0xe18 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdb9 as libc::c_int as uint16_t,
            ucs: 0xe19 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdba as libc::c_int as uint16_t,
            ucs: 0xe1a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdbb as libc::c_int as uint16_t,
            ucs: 0xe1b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdbc as libc::c_int as uint16_t,
            ucs: 0xe1c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdbd as libc::c_int as uint16_t,
            ucs: 0xe1d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdbe as libc::c_int as uint16_t,
            ucs: 0xe1e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdbf as libc::c_int as uint16_t,
            ucs: 0xe1f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdc0 as libc::c_int as uint16_t,
            ucs: 0xe20 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdc1 as libc::c_int as uint16_t,
            ucs: 0xe21 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdc2 as libc::c_int as uint16_t,
            ucs: 0xe22 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdc3 as libc::c_int as uint16_t,
            ucs: 0xe23 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdc4 as libc::c_int as uint16_t,
            ucs: 0xe24 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdc5 as libc::c_int as uint16_t,
            ucs: 0xe25 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdc6 as libc::c_int as uint16_t,
            ucs: 0xe26 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdc7 as libc::c_int as uint16_t,
            ucs: 0xe27 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdc8 as libc::c_int as uint16_t,
            ucs: 0xe28 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdc9 as libc::c_int as uint16_t,
            ucs: 0xe29 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdca as libc::c_int as uint16_t,
            ucs: 0xe2a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdcb as libc::c_int as uint16_t,
            ucs: 0xe2b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdcc as libc::c_int as uint16_t,
            ucs: 0xe2c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdcd as libc::c_int as uint16_t,
            ucs: 0xe2d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdce as libc::c_int as uint16_t,
            ucs: 0xe2e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdcf as libc::c_int as uint16_t,
            ucs: 0xe2f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdd0 as libc::c_int as uint16_t,
            ucs: 0xe30 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdd1 as libc::c_int as uint16_t,
            ucs: 0xe31 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdd2 as libc::c_int as uint16_t,
            ucs: 0xe32 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdd3 as libc::c_int as uint16_t,
            ucs: 0xe33 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdd4 as libc::c_int as uint16_t,
            ucs: 0xe34 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdd5 as libc::c_int as uint16_t,
            ucs: 0xe35 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdd6 as libc::c_int as uint16_t,
            ucs: 0xe36 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdd7 as libc::c_int as uint16_t,
            ucs: 0xe37 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdd8 as libc::c_int as uint16_t,
            ucs: 0xe38 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdd9 as libc::c_int as uint16_t,
            ucs: 0xe39 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdda as libc::c_int as uint16_t,
            ucs: 0xe3a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xddf as libc::c_int as uint16_t,
            ucs: 0xe3f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xde0 as libc::c_int as uint16_t,
            ucs: 0xe40 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xde1 as libc::c_int as uint16_t,
            ucs: 0xe41 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xde2 as libc::c_int as uint16_t,
            ucs: 0xe42 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xde3 as libc::c_int as uint16_t,
            ucs: 0xe43 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xde4 as libc::c_int as uint16_t,
            ucs: 0xe44 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xde5 as libc::c_int as uint16_t,
            ucs: 0xe45 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xde6 as libc::c_int as uint16_t,
            ucs: 0xe46 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xde7 as libc::c_int as uint16_t,
            ucs: 0xe47 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xde8 as libc::c_int as uint16_t,
            ucs: 0xe48 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xde9 as libc::c_int as uint16_t,
            ucs: 0xe49 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdea as libc::c_int as uint16_t,
            ucs: 0xe4a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdeb as libc::c_int as uint16_t,
            ucs: 0xe4b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdec as libc::c_int as uint16_t,
            ucs: 0xe4c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xded as libc::c_int as uint16_t,
            ucs: 0xe4d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdf0 as libc::c_int as uint16_t,
            ucs: 0xe50 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdf1 as libc::c_int as uint16_t,
            ucs: 0xe51 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdf2 as libc::c_int as uint16_t,
            ucs: 0xe52 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdf3 as libc::c_int as uint16_t,
            ucs: 0xe53 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdf4 as libc::c_int as uint16_t,
            ucs: 0xe54 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdf5 as libc::c_int as uint16_t,
            ucs: 0xe55 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdf6 as libc::c_int as uint16_t,
            ucs: 0xe56 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdf7 as libc::c_int as uint16_t,
            ucs: 0xe57 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdf8 as libc::c_int as uint16_t,
            ucs: 0xe58 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xdf9 as libc::c_int as uint16_t,
            ucs: 0xe59 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xea1 as libc::c_int as uint16_t,
            ucs: 0x3131 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xea2 as libc::c_int as uint16_t,
            ucs: 0x3132 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xea3 as libc::c_int as uint16_t,
            ucs: 0x3133 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xea4 as libc::c_int as uint16_t,
            ucs: 0x3134 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xea5 as libc::c_int as uint16_t,
            ucs: 0x3135 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xea6 as libc::c_int as uint16_t,
            ucs: 0x3136 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xea7 as libc::c_int as uint16_t,
            ucs: 0x3137 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xea8 as libc::c_int as uint16_t,
            ucs: 0x3138 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xea9 as libc::c_int as uint16_t,
            ucs: 0x3139 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeaa as libc::c_int as uint16_t,
            ucs: 0x313a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeab as libc::c_int as uint16_t,
            ucs: 0x313b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeac as libc::c_int as uint16_t,
            ucs: 0x313c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xead as libc::c_int as uint16_t,
            ucs: 0x313d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeae as libc::c_int as uint16_t,
            ucs: 0x313e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeaf as libc::c_int as uint16_t,
            ucs: 0x313f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeb0 as libc::c_int as uint16_t,
            ucs: 0x3140 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeb1 as libc::c_int as uint16_t,
            ucs: 0x3141 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeb2 as libc::c_int as uint16_t,
            ucs: 0x3142 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeb3 as libc::c_int as uint16_t,
            ucs: 0x3143 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeb4 as libc::c_int as uint16_t,
            ucs: 0x3144 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeb5 as libc::c_int as uint16_t,
            ucs: 0x3145 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeb6 as libc::c_int as uint16_t,
            ucs: 0x3146 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeb7 as libc::c_int as uint16_t,
            ucs: 0x3147 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeb8 as libc::c_int as uint16_t,
            ucs: 0x3148 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeb9 as libc::c_int as uint16_t,
            ucs: 0x3149 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeba as libc::c_int as uint16_t,
            ucs: 0x314a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xebb as libc::c_int as uint16_t,
            ucs: 0x314b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xebc as libc::c_int as uint16_t,
            ucs: 0x314c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xebd as libc::c_int as uint16_t,
            ucs: 0x314d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xebe as libc::c_int as uint16_t,
            ucs: 0x314e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xebf as libc::c_int as uint16_t,
            ucs: 0x314f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xec0 as libc::c_int as uint16_t,
            ucs: 0x3150 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xec1 as libc::c_int as uint16_t,
            ucs: 0x3151 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xec2 as libc::c_int as uint16_t,
            ucs: 0x3152 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xec3 as libc::c_int as uint16_t,
            ucs: 0x3153 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xec4 as libc::c_int as uint16_t,
            ucs: 0x3154 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xec5 as libc::c_int as uint16_t,
            ucs: 0x3155 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xec6 as libc::c_int as uint16_t,
            ucs: 0x3156 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xec7 as libc::c_int as uint16_t,
            ucs: 0x3157 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xec8 as libc::c_int as uint16_t,
            ucs: 0x3158 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xec9 as libc::c_int as uint16_t,
            ucs: 0x3159 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeca as libc::c_int as uint16_t,
            ucs: 0x315a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xecb as libc::c_int as uint16_t,
            ucs: 0x315b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xecc as libc::c_int as uint16_t,
            ucs: 0x315c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xecd as libc::c_int as uint16_t,
            ucs: 0x315d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xece as libc::c_int as uint16_t,
            ucs: 0x315e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xecf as libc::c_int as uint16_t,
            ucs: 0x315f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xed0 as libc::c_int as uint16_t,
            ucs: 0x3160 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xed1 as libc::c_int as uint16_t,
            ucs: 0x3161 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xed2 as libc::c_int as uint16_t,
            ucs: 0x3162 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xed3 as libc::c_int as uint16_t,
            ucs: 0x3163 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xed4 as libc::c_int as uint16_t,
            ucs: 0x11a8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xed5 as libc::c_int as uint16_t,
            ucs: 0x11a9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xed6 as libc::c_int as uint16_t,
            ucs: 0x11aa as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xed7 as libc::c_int as uint16_t,
            ucs: 0x11ab as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xed8 as libc::c_int as uint16_t,
            ucs: 0x11ac as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xed9 as libc::c_int as uint16_t,
            ucs: 0x11ad as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeda as libc::c_int as uint16_t,
            ucs: 0x11ae as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xedb as libc::c_int as uint16_t,
            ucs: 0x11af as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xedc as libc::c_int as uint16_t,
            ucs: 0x11b0 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xedd as libc::c_int as uint16_t,
            ucs: 0x11b1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xede as libc::c_int as uint16_t,
            ucs: 0x11b2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xedf as libc::c_int as uint16_t,
            ucs: 0x11b3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xee0 as libc::c_int as uint16_t,
            ucs: 0x11b4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xee1 as libc::c_int as uint16_t,
            ucs: 0x11b5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xee2 as libc::c_int as uint16_t,
            ucs: 0x11b6 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xee3 as libc::c_int as uint16_t,
            ucs: 0x11b7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xee4 as libc::c_int as uint16_t,
            ucs: 0x11b8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xee5 as libc::c_int as uint16_t,
            ucs: 0x11b9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xee6 as libc::c_int as uint16_t,
            ucs: 0x11ba as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xee7 as libc::c_int as uint16_t,
            ucs: 0x11bb as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xee8 as libc::c_int as uint16_t,
            ucs: 0x11bc as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xee9 as libc::c_int as uint16_t,
            ucs: 0x11bd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeea as libc::c_int as uint16_t,
            ucs: 0x11be as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeeb as libc::c_int as uint16_t,
            ucs: 0x11bf as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeec as libc::c_int as uint16_t,
            ucs: 0x11c0 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeed as libc::c_int as uint16_t,
            ucs: 0x11c1 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeee as libc::c_int as uint16_t,
            ucs: 0x11c2 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeef as libc::c_int as uint16_t,
            ucs: 0x316d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xef0 as libc::c_int as uint16_t,
            ucs: 0x3171 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xef1 as libc::c_int as uint16_t,
            ucs: 0x3178 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xef2 as libc::c_int as uint16_t,
            ucs: 0x317f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xef3 as libc::c_int as uint16_t,
            ucs: 0x3181 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xef4 as libc::c_int as uint16_t,
            ucs: 0x3184 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xef5 as libc::c_int as uint16_t,
            ucs: 0x3186 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xef6 as libc::c_int as uint16_t,
            ucs: 0x318d as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xef7 as libc::c_int as uint16_t,
            ucs: 0x318e as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xef8 as libc::c_int as uint16_t,
            ucs: 0x11eb as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xef9 as libc::c_int as uint16_t,
            ucs: 0x11f0 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xefa as libc::c_int as uint16_t,
            ucs: 0x11f9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xeff as libc::c_int as uint16_t,
            ucs: 0x20a9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x13a4 as libc::c_int as uint16_t,
            ucs: 0x20ac as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x13bc as libc::c_int as uint16_t,
            ucs: 0x152 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x13bd as libc::c_int as uint16_t,
            ucs: 0x153 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x13be as libc::c_int as uint16_t,
            ucs: 0x178 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0x20ac as libc::c_int as uint16_t,
            ucs: 0x20ac as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe50 as libc::c_int as uint16_t,
            ucs: '`' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe51 as libc::c_int as uint16_t,
            ucs: 0xb4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe52 as libc::c_int as uint16_t,
            ucs: '^' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe53 as libc::c_int as uint16_t,
            ucs: '~' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe54 as libc::c_int as uint16_t,
            ucs: 0xaf as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe55 as libc::c_int as uint16_t,
            ucs: 0x2d8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe56 as libc::c_int as uint16_t,
            ucs: 0x2d9 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe57 as libc::c_int as uint16_t,
            ucs: 0xa8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe58 as libc::c_int as uint16_t,
            ucs: 0x2da as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe59 as libc::c_int as uint16_t,
            ucs: 0x2dd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe5a as libc::c_int as uint16_t,
            ucs: 0x2c7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe5b as libc::c_int as uint16_t,
            ucs: 0xb8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe5c as libc::c_int as uint16_t,
            ucs: 0x2db as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe5d as libc::c_int as uint16_t,
            ucs: 0x37a as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe5e as libc::c_int as uint16_t,
            ucs: 0x309b as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe5f as libc::c_int as uint16_t,
            ucs: 0x309c as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe63 as libc::c_int as uint16_t,
            ucs: '/' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe64 as libc::c_int as uint16_t,
            ucs: 0x2bc as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe65 as libc::c_int as uint16_t,
            ucs: 0x2bd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe66 as libc::c_int as uint16_t,
            ucs: 0x2f5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe67 as libc::c_int as uint16_t,
            ucs: 0x2f3 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe68 as libc::c_int as uint16_t,
            ucs: 0x2cd as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe69 as libc::c_int as uint16_t,
            ucs: 0xa788 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe6a as libc::c_int as uint16_t,
            ucs: 0x2f7 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe6e as libc::c_int as uint16_t,
            ucs: ',' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe6f as libc::c_int as uint16_t,
            ucs: 0xa4 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe80 as libc::c_int as uint16_t,
            ucs: 'a' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe81 as libc::c_int as uint16_t,
            ucs: 'A' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe82 as libc::c_int as uint16_t,
            ucs: 'e' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe83 as libc::c_int as uint16_t,
            ucs: 'E' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe84 as libc::c_int as uint16_t,
            ucs: 'i' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe85 as libc::c_int as uint16_t,
            ucs: 'I' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe86 as libc::c_int as uint16_t,
            ucs: 'o' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe87 as libc::c_int as uint16_t,
            ucs: 'O' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe88 as libc::c_int as uint16_t,
            ucs: 'u' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe89 as libc::c_int as uint16_t,
            ucs: 'U' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe8a as libc::c_int as uint16_t,
            ucs: 0x259 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe8b as libc::c_int as uint16_t,
            ucs: 0x18f as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe8c as libc::c_int as uint16_t,
            ucs: 0xb5 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe90 as libc::c_int as uint16_t,
            ucs: '_' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe91 as libc::c_int as uint16_t,
            ucs: 0x2c8 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xfe92 as libc::c_int as uint16_t,
            ucs: 0x2cc as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xff80 as libc::c_int as uint16_t,
            ucs: ' ' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xff95 as libc::c_int as uint16_t,
            ucs: 0x37 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xff96 as libc::c_int as uint16_t,
            ucs: 0x34 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xff97 as libc::c_int as uint16_t,
            ucs: 0x38 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xff98 as libc::c_int as uint16_t,
            ucs: 0x36 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xff99 as libc::c_int as uint16_t,
            ucs: 0x32 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xff9a as libc::c_int as uint16_t,
            ucs: 0x39 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xff9b as libc::c_int as uint16_t,
            ucs: 0x33 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xff9c as libc::c_int as uint16_t,
            ucs: 0x31 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xff9d as libc::c_int as uint16_t,
            ucs: 0x35 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xff9e as libc::c_int as uint16_t,
            ucs: 0x30 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffaa as libc::c_int as uint16_t,
            ucs: '*' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffab as libc::c_int as uint16_t,
            ucs: '+' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffac as libc::c_int as uint16_t,
            ucs: ',' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffad as libc::c_int as uint16_t,
            ucs: '-' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffae as libc::c_int as uint16_t,
            ucs: '.' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffaf as libc::c_int as uint16_t,
            ucs: '/' as i32 as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffb0 as libc::c_int as uint16_t,
            ucs: 0x30 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffb1 as libc::c_int as uint16_t,
            ucs: 0x31 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffb2 as libc::c_int as uint16_t,
            ucs: 0x32 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffb3 as libc::c_int as uint16_t,
            ucs: 0x33 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffb4 as libc::c_int as uint16_t,
            ucs: 0x34 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffb5 as libc::c_int as uint16_t,
            ucs: 0x35 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffb6 as libc::c_int as uint16_t,
            ucs: 0x36 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffb7 as libc::c_int as uint16_t,
            ucs: 0x37 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffb8 as libc::c_int as uint16_t,
            ucs: 0x38 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffb9 as libc::c_int as uint16_t,
            ucs: 0x39 as libc::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = _sapp_x11_codepair {
            keysym: 0xffbd as libc::c_int as uint16_t,
            ucs: '=' as i32 as uint16_t,
        };
        init
    },
];
unsafe extern "C" fn _sapp_x11_error_handler(
    mut display: *mut Display,
    mut event: *mut XErrorEvent,
) -> libc::c_int {
    _sapp_x11_error_code = (*event).error_code;
    return 0 as libc::c_int;
}
unsafe extern "C" fn _sapp_x11_grab_error_handler() {
    _sapp_x11_error_code = 0 as libc::c_int as libc::c_uchar;
    XSetErrorHandler(
        Some(
            _sapp_x11_error_handler
                as unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn _sapp_x11_release_error_handler() {
    XSync(_sapp_x11_display, 0 as libc::c_int);
    XSetErrorHandler(None);
}
unsafe extern "C" fn _sapp_x11_init_extensions() {
    _sapp_x11_UTF8_STRING = XInternAtom(
        _sapp_x11_display,
        b"UTF8_STRING\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    _sapp_x11_WM_PROTOCOLS = XInternAtom(
        _sapp_x11_display,
        b"WM_PROTOCOLS\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    _sapp_x11_WM_DELETE_WINDOW = XInternAtom(
        _sapp_x11_display,
        b"WM_DELETE_WINDOW\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    _sapp_x11_WM_STATE = XInternAtom(
        _sapp_x11_display,
        b"WM_STATE\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    _sapp_x11_NET_WM_NAME = XInternAtom(
        _sapp_x11_display,
        b"_NET_WM_NAME\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    _sapp_x11_NET_WM_ICON_NAME = XInternAtom(
        _sapp_x11_display,
        b"_NET_WM_ICON_NAME\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn _sapp_x11_query_system_dpi() {
    let mut rms: *mut libc::c_char = XResourceManagerString(_sapp_x11_display);
    if !rms.is_null() {
        let mut db: XrmDatabase = XrmGetStringDatabase(rms);
        if !db.is_null() {
            let mut value: XrmValue = XrmValue {
                size: 0,
                addr: 0 as *mut libc::c_char,
            };
            let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
            if XrmGetResource(
                db,
                b"Xft.dpi\0" as *const u8 as *const libc::c_char,
                b"Xft.Dpi\0" as *const u8 as *const libc::c_char,
                &mut type_0,
                &mut value,
            ) != 0
            {
                if !type_0.is_null()
                    && strcmp(type_0, b"String\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    _sapp_x11_dpi = atof(value.addr as *const libc::c_char)
                        as libc::c_float;
                }
            }
            XrmDestroyDatabase(db);
        }
    }
}
unsafe extern "C" fn _sapp_glx_has_ext(
    mut ext: *const libc::c_char,
    mut extensions: *const libc::c_char,
) -> bool {
    let mut start: *const libc::c_char = extensions;
    loop {
        let mut where_0: *const libc::c_char = strstr(start, ext);
        if where_0.is_null() {
            return 0 as libc::c_int != 0;
        }
        let mut terminator: *const libc::c_char = where_0.offset(strlen(ext) as isize);
        if where_0 == start
            || *where_0.offset(-(1 as libc::c_int as isize)) as libc::c_int == ' ' as i32
        {
            if *terminator as libc::c_int == ' ' as i32
                || *terminator as libc::c_int == '\0' as i32
            {
                break;
            }
        }
        start = terminator;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sapp_glx_extsupported(
    mut ext: *const libc::c_char,
    mut extensions: *const libc::c_char,
) -> bool {
    if !extensions.is_null() {
        return _sapp_glx_has_ext(ext, extensions)
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn _sapp_glx_getprocaddr(
    mut procname: *const libc::c_char,
) -> *mut libc::c_void {
    if _sapp_glx_GetProcAddress.is_some() {
        return ::std::mem::transmute::<
            __GLXextproc,
            *mut libc::c_void,
        >(_sapp_glx_GetProcAddress.unwrap()(procname as *const GLubyte))
    } else if _sapp_glx_GetProcAddressARB.is_some() {
        return ::std::mem::transmute::<
            __GLXextproc,
            *mut libc::c_void,
        >(_sapp_glx_GetProcAddressARB.unwrap()(procname as *const GLubyte))
    } else {
        return dlsym(_sapp_glx_libgl, procname)
    };
}
unsafe extern "C" fn _sapp_glx_init() {
    let mut sonames: [*const libc::c_char; 3] = [
        b"libGL.so.1\0" as *const u8 as *const libc::c_char,
        b"libGL.so\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(sonames[i as usize]).is_null() {
        _sapp_glx_libgl = dlopen(
            sonames[i as usize],
            0x1 as libc::c_int | 0x100 as libc::c_int,
        );
        if !_sapp_glx_libgl.is_null() {
            break;
        }
        i += 1;
        i;
    }
    if _sapp_glx_libgl.is_null() {
        _sapp_fail(b"GLX: failed to load libGL\0" as *const u8 as *const libc::c_char);
    }
    _sapp_glx_GetFBConfigs = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXGETFBCONFIGSPROC,
    >(dlsym(_sapp_glx_libgl, b"glXGetFBConfigs\0" as *const u8 as *const libc::c_char));
    _sapp_glx_GetFBConfigAttrib = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXGETFBCONFIGATTRIBPROC,
    >(
        dlsym(
            _sapp_glx_libgl,
            b"glXGetFBConfigAttrib\0" as *const u8 as *const libc::c_char,
        ),
    );
    _sapp_glx_GetClientString = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXGETCLIENTSTRINGPROC,
    >(
        dlsym(
            _sapp_glx_libgl,
            b"glXGetClientString\0" as *const u8 as *const libc::c_char,
        ),
    );
    _sapp_glx_QueryExtension = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXQUERYEXTENSIONPROC,
    >(
        dlsym(
            _sapp_glx_libgl,
            b"glXQueryExtension\0" as *const u8 as *const libc::c_char,
        ),
    );
    _sapp_glx_QueryVersion = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXQUERYVERSIONPROC,
    >(dlsym(_sapp_glx_libgl, b"glXQueryVersion\0" as *const u8 as *const libc::c_char));
    _sapp_glx_DestroyContext = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXDESTROYCONTEXTPROC,
    >(
        dlsym(
            _sapp_glx_libgl,
            b"glXDestroyContext\0" as *const u8 as *const libc::c_char,
        ),
    );
    _sapp_glx_MakeCurrent = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXMAKECURRENTPROC,
    >(dlsym(_sapp_glx_libgl, b"glXMakeCurrent\0" as *const u8 as *const libc::c_char));
    _sapp_glx_SwapBuffers = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXSWAPBUFFERSPROC,
    >(dlsym(_sapp_glx_libgl, b"glXSwapBuffers\0" as *const u8 as *const libc::c_char));
    _sapp_glx_QueryExtensionsString = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXQUERYEXTENSIONSSTRINGPROC,
    >(
        dlsym(
            _sapp_glx_libgl,
            b"glXQueryExtensionsString\0" as *const u8 as *const libc::c_char,
        ),
    );
    _sapp_glx_CreateNewContext = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXCREATENEWCONTEXTPROC,
    >(
        dlsym(
            _sapp_glx_libgl,
            b"glXCreateNewContext\0" as *const u8 as *const libc::c_char,
        ),
    );
    _sapp_glx_CreateWindow = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXCREATEWINDOWPROC,
    >(dlsym(_sapp_glx_libgl, b"glXCreateWindow\0" as *const u8 as *const libc::c_char));
    _sapp_glx_DestroyWindow = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXDESTROYWINDOWPROC,
    >(dlsym(_sapp_glx_libgl, b"glXDestroyWindow\0" as *const u8 as *const libc::c_char));
    _sapp_glx_GetProcAddress = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXGETPROCADDRESSPROC,
    >(
        dlsym(
            _sapp_glx_libgl,
            b"glXGetProcAddress\0" as *const u8 as *const libc::c_char,
        ),
    );
    _sapp_glx_GetProcAddressARB = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXGETPROCADDRESSPROC,
    >(
        dlsym(
            _sapp_glx_libgl,
            b"glXGetProcAddressARB\0" as *const u8 as *const libc::c_char,
        ),
    );
    _sapp_glx_GetVisualFromFBConfig = ::std::mem::transmute::<
        *mut libc::c_void,
        PFNGLXGETVISUALFROMFBCONFIGPROC,
    >(
        dlsym(
            _sapp_glx_libgl,
            b"glXGetVisualFromFBConfig\0" as *const u8 as *const libc::c_char,
        ),
    );
    if _sapp_glx_GetFBConfigs.is_none() || _sapp_glx_GetFBConfigAttrib.is_none()
        || _sapp_glx_GetClientString.is_none() || _sapp_glx_QueryExtension.is_none()
        || _sapp_glx_QueryVersion.is_none() || _sapp_glx_DestroyContext.is_none()
        || _sapp_glx_MakeCurrent.is_none() || _sapp_glx_SwapBuffers.is_none()
        || _sapp_glx_QueryExtensionsString.is_none()
        || _sapp_glx_CreateNewContext.is_none() || _sapp_glx_CreateWindow.is_none()
        || _sapp_glx_DestroyWindow.is_none() || _sapp_glx_GetProcAddress.is_none()
        || _sapp_glx_GetProcAddressARB.is_none()
        || _sapp_glx_GetVisualFromFBConfig.is_none()
    {
        _sapp_fail(
            b"GLX: failed to load required entry points\0" as *const u8
                as *const libc::c_char,
        );
    }
    if _sapp_glx_QueryExtension
        .unwrap()(_sapp_x11_display, &mut _sapp_glx_errorbase, &mut _sapp_glx_eventbase)
        == 0
    {
        _sapp_fail(
            b"GLX: GLX extension not found\0" as *const u8 as *const libc::c_char,
        );
    }
    if _sapp_glx_QueryVersion
        .unwrap()(_sapp_x11_display, &mut _sapp_glx_major, &mut _sapp_glx_minor) == 0
    {
        _sapp_fail(
            b"GLX: Failed to query GLX version\0" as *const u8 as *const libc::c_char,
        );
    }
    if _sapp_glx_major == 1 as libc::c_int && _sapp_glx_minor < 3 as libc::c_int {
        _sapp_fail(
            b"GLX: GLX version 1.3 is required\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut exts: *const libc::c_char = _sapp_glx_QueryExtensionsString
        .unwrap()(_sapp_x11_display, _sapp_x11_screen);
    if _sapp_glx_extsupported(
        b"GLX_EXT_swap_control\0" as *const u8 as *const libc::c_char,
        exts,
    ) {
        _sapp_glx_SwapIntervalEXT = ::std::mem::transmute::<
            *mut libc::c_void,
            PFNGLXSWAPINTERVALEXTPROC,
        >(
            _sapp_glx_getprocaddr(
                b"glXSwapIntervalEXT\0" as *const u8 as *const libc::c_char,
            ),
        );
        _sapp_glx_EXT_swap_control = _sapp_glx_SwapIntervalEXT.is_some();
    }
    if _sapp_glx_extsupported(
        b"GLX_MESA_swap_control\0" as *const u8 as *const libc::c_char,
        exts,
    ) {
        _sapp_glx_SwapIntervalMESA = ::std::mem::transmute::<
            *mut libc::c_void,
            PFNGLXSWAPINTERVALMESAPROC,
        >(
            _sapp_glx_getprocaddr(
                b"glXSwapIntervalMESA\0" as *const u8 as *const libc::c_char,
            ),
        );
        _sapp_glx_MESA_swap_control = _sapp_glx_SwapIntervalMESA.is_some();
    }
    _sapp_glx_ARB_multisample = _sapp_glx_extsupported(
        b"GLX_ARB_multisample\0" as *const u8 as *const libc::c_char,
        exts,
    );
    _sapp_glx_ARB_framebuffer_sRGB = _sapp_glx_extsupported(
        b"GLX_ARB_framebuffer_sRGB\0" as *const u8 as *const libc::c_char,
        exts,
    );
    _sapp_glx_EXT_framebuffer_sRGB = _sapp_glx_extsupported(
        b"GLX_EXT_framebuffer_sRGB\0" as *const u8 as *const libc::c_char,
        exts,
    );
    if _sapp_glx_extsupported(
        b"GLX_ARB_create_context\0" as *const u8 as *const libc::c_char,
        exts,
    ) {
        _sapp_glx_CreateContextAttribsARB = ::std::mem::transmute::<
            *mut libc::c_void,
            PFNGLXCREATECONTEXTATTRIBSARBPROC,
        >(
            _sapp_glx_getprocaddr(
                b"glXCreateContextAttribsARB\0" as *const u8 as *const libc::c_char,
            ),
        );
        _sapp_glx_ARB_create_context = _sapp_glx_CreateContextAttribsARB.is_some();
    }
    _sapp_glx_ARB_create_context_profile = _sapp_glx_extsupported(
        b"GLX_ARB_create_context_profile\0" as *const u8 as *const libc::c_char,
        exts,
    );
}
unsafe extern "C" fn _sapp_glx_attrib(
    mut fbconfig: GLXFBConfig,
    mut attrib: libc::c_int,
) -> libc::c_int {
    let mut value: libc::c_int = 0;
    _sapp_glx_GetFBConfigAttrib
        .unwrap()(_sapp_x11_display, fbconfig, attrib, &mut value);
    return value;
}
unsafe extern "C" fn _sapp_glx_choosefbconfig() -> GLXFBConfig {
    let mut native_configs: *mut GLXFBConfig = 0 as *mut GLXFBConfig;
    let mut usable_configs: *mut _sapp_gl_fbconfig = 0 as *mut _sapp_gl_fbconfig;
    let mut closest: *const _sapp_gl_fbconfig = 0 as *const _sapp_gl_fbconfig;
    let mut i: libc::c_int = 0;
    let mut native_count: libc::c_int = 0;
    let mut usable_count: libc::c_int = 0;
    let mut vendor: *const libc::c_char = 0 as *const libc::c_char;
    let mut trust_window_bit: bool = 1 as libc::c_int != 0;
    vendor = _sapp_glx_GetClientString.unwrap()(_sapp_x11_display, 1 as libc::c_int);
    if !vendor.is_null()
        && strcmp(vendor, b"Chromium\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        trust_window_bit = 0 as libc::c_int != 0;
    }
    native_configs = _sapp_glx_GetFBConfigs
        .unwrap()(_sapp_x11_display, _sapp_x11_screen, &mut native_count);
    if native_configs.is_null() || native_count == 0 {
        _sapp_fail(
            b"GLX: No GLXFBConfigs returned\0" as *const u8 as *const libc::c_char,
        );
    }
    usable_configs = calloc(
        native_count as libc::c_ulong,
        ::std::mem::size_of::<_sapp_gl_fbconfig>() as libc::c_ulong,
    ) as *mut _sapp_gl_fbconfig;
    usable_count = 0 as libc::c_int;
    let mut current_block_25: u64;
    i = 0 as libc::c_int;
    while i < native_count {
        let n: GLXFBConfig = *native_configs.offset(i as isize);
        let mut u: *mut _sapp_gl_fbconfig = usable_configs.offset(usable_count as isize);
        _sapp_gl_init_fbconfig(u);
        if !(0 as libc::c_int
            == _sapp_glx_attrib(n, 0x8011 as libc::c_int) & 0x1 as libc::c_int)
        {
            if 0 as libc::c_int
                == _sapp_glx_attrib(n, 0x8010 as libc::c_int) & 0x1 as libc::c_int
            {
                if trust_window_bit {
                    current_block_25 = 1917311967535052937;
                } else {
                    current_block_25 = 5143058163439228106;
                }
            } else {
                current_block_25 = 5143058163439228106;
            }
            match current_block_25 {
                1917311967535052937 => {}
                _ => {
                    (*u).red_bits = _sapp_glx_attrib(n, 8 as libc::c_int);
                    (*u).green_bits = _sapp_glx_attrib(n, 9 as libc::c_int);
                    (*u).blue_bits = _sapp_glx_attrib(n, 10 as libc::c_int);
                    (*u).alpha_bits = _sapp_glx_attrib(n, 11 as libc::c_int);
                    (*u).depth_bits = _sapp_glx_attrib(n, 12 as libc::c_int);
                    (*u).stencil_bits = _sapp_glx_attrib(n, 13 as libc::c_int);
                    if _sapp_glx_attrib(n, 5 as libc::c_int) != 0 {
                        (*u).doublebuffer = 1 as libc::c_int != 0;
                    }
                    if _sapp_glx_ARB_multisample {
                        (*u).samples = _sapp_glx_attrib(n, 0x186a1 as libc::c_int);
                    }
                    (*u).handle = n as uintptr_t;
                    usable_count += 1;
                    usable_count;
                }
            }
        }
        i += 1;
        i;
    }
    let mut desired: _sapp_gl_fbconfig = _sapp_gl_fbconfig {
        red_bits: 0,
        green_bits: 0,
        blue_bits: 0,
        alpha_bits: 0,
        depth_bits: 0,
        stencil_bits: 0,
        samples: 0,
        doublebuffer: false,
        handle: 0,
    };
    _sapp_gl_init_fbconfig(&mut desired);
    desired.red_bits = 8 as libc::c_int;
    desired.green_bits = 8 as libc::c_int;
    desired.blue_bits = 8 as libc::c_int;
    desired.alpha_bits = 8 as libc::c_int;
    desired.depth_bits = 24 as libc::c_int;
    desired.stencil_bits = 8 as libc::c_int;
    desired.doublebuffer = 1 as libc::c_int != 0;
    desired
        .samples = if _sapp.sample_count > 1 as libc::c_int {
        _sapp.sample_count
    } else {
        0 as libc::c_int
    };
    closest = _sapp_gl_choose_fbconfig(
        &mut desired,
        usable_configs,
        usable_count as libc::c_uint,
    );
    let mut result: GLXFBConfig = 0 as GLXFBConfig;
    if !closest.is_null() {
        result = (*closest).handle as GLXFBConfig;
    }
    XFree(native_configs as *mut libc::c_void);
    free(usable_configs as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn _sapp_glx_choose_visual(
    mut visual: *mut *mut Visual,
    mut depth: *mut libc::c_int,
) {
    let mut native: GLXFBConfig = _sapp_glx_choosefbconfig();
    if native.is_null() {
        _sapp_fail(
            b"GLX: Failed to find a suitable GLXFBConfig\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut result: *mut XVisualInfo = _sapp_glx_GetVisualFromFBConfig
        .unwrap()(_sapp_x11_display, native);
    if result.is_null() {
        _sapp_fail(
            b"GLX: Failed to retrieve Visual for GLXFBConfig\0" as *const u8
                as *const libc::c_char,
        );
    }
    *visual = (*result).visual;
    *depth = (*result).depth;
    XFree(result as *mut libc::c_void);
}
unsafe extern "C" fn _sapp_glx_create_context() {
    let mut native: GLXFBConfig = _sapp_glx_choosefbconfig();
    if native.is_null() {
        _sapp_fail(
            b"GLX: Failed to find a suitable GLXFBConfig (2)\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(_sapp_glx_ARB_create_context as libc::c_int != 0
        && _sapp_glx_ARB_create_context_profile as libc::c_int != 0)
    {
        _sapp_fail(
            b"GLX: ARB_create_context and ARB_create_context_profile required\0"
                as *const u8 as *const libc::c_char,
        );
    }
    _sapp_x11_grab_error_handler();
    let attribs: [libc::c_int; 10] = [
        0x2091 as libc::c_int,
        3 as libc::c_int,
        0x2092 as libc::c_int,
        3 as libc::c_int,
        0x9126 as libc::c_int,
        0x1 as libc::c_int,
        0x2094 as libc::c_int,
        0x2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ];
    _sapp_glx_ctx = _sapp_glx_CreateContextAttribsARB
        .unwrap()(
        _sapp_x11_display,
        native,
        0 as GLXContext,
        1 as libc::c_int,
        attribs.as_ptr(),
    );
    if _sapp_glx_ctx.is_null() {
        _sapp_fail(
            b"GLX: failed to create GL context\0" as *const u8 as *const libc::c_char,
        );
    }
    _sapp_x11_release_error_handler();
    _sapp_glx_window = _sapp_glx_CreateWindow
        .unwrap()(_sapp_x11_display, native, _sapp_x11_window, 0 as *const libc::c_int);
    if _sapp_glx_window == 0 {
        _sapp_fail(
            b"GLX: failed to create window\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn _sapp_glx_destroy_context() {
    if _sapp_glx_window != 0 {
        _sapp_glx_DestroyWindow.unwrap()(_sapp_x11_display, _sapp_glx_window);
        _sapp_glx_window = 0 as libc::c_int as GLXWindow;
    }
    if !_sapp_glx_ctx.is_null() {
        _sapp_glx_DestroyContext.unwrap()(_sapp_x11_display, _sapp_glx_ctx);
        _sapp_glx_ctx = 0 as GLXContext;
    }
}
unsafe extern "C" fn _sapp_glx_make_current() {
    _sapp_glx_MakeCurrent.unwrap()(_sapp_x11_display, _sapp_glx_window, _sapp_glx_ctx);
}
unsafe extern "C" fn _sapp_glx_swap_buffers() {
    _sapp_glx_SwapBuffers.unwrap()(_sapp_x11_display, _sapp_glx_window);
}
unsafe extern "C" fn _sapp_glx_swapinterval(mut interval: libc::c_int) {
    _sapp_glx_make_current();
    if _sapp_glx_EXT_swap_control {
        _sapp_glx_SwapIntervalEXT
            .unwrap()(_sapp_x11_display, _sapp_glx_window, interval);
    } else if _sapp_glx_MESA_swap_control {
        _sapp_glx_SwapIntervalMESA.unwrap()(interval);
    }
}
unsafe extern "C" fn _sapp_x11_update_window_title() {
    Xutf8SetWMProperties(
        _sapp_x11_display,
        _sapp_x11_window,
        (_sapp.window_title).as_mut_ptr(),
        (_sapp.window_title).as_mut_ptr(),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
        0 as *mut XSizeHints,
        0 as *mut XWMHints,
        0 as *mut XClassHint,
    );
    XChangeProperty(
        _sapp_x11_display,
        _sapp_x11_window,
        _sapp_x11_NET_WM_NAME,
        _sapp_x11_UTF8_STRING,
        8 as libc::c_int,
        0 as libc::c_int,
        (_sapp.window_title).as_mut_ptr() as *mut libc::c_uchar,
        strlen((_sapp.window_title).as_mut_ptr()) as libc::c_int,
    );
    XChangeProperty(
        _sapp_x11_display,
        _sapp_x11_window,
        _sapp_x11_NET_WM_ICON_NAME,
        _sapp_x11_UTF8_STRING,
        8 as libc::c_int,
        0 as libc::c_int,
        (_sapp.window_title).as_mut_ptr() as *mut libc::c_uchar,
        strlen((_sapp.window_title).as_mut_ptr()) as libc::c_int,
    );
    XFlush(_sapp_x11_display);
}
unsafe extern "C" fn _sapp_x11_query_window_size() {
    let mut attribs: XWindowAttributes = XWindowAttributes {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        depth: 0,
        visual: 0 as *mut Visual,
        root: 0,
        class: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        colormap: 0,
        map_installed: 0,
        map_state: 0,
        all_event_masks: 0,
        your_event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        screen: 0 as *mut Screen,
    };
    XGetWindowAttributes(_sapp_x11_display, _sapp_x11_window, &mut attribs);
    _sapp.window_width = attribs.width;
    _sapp.window_height = attribs.height;
    _sapp.framebuffer_width = _sapp.window_width;
    _sapp.framebuffer_height = _sapp.framebuffer_height;
}
unsafe extern "C" fn _sapp_x11_create_window(
    mut visual: *mut Visual,
    mut depth: libc::c_int,
) {
    _sapp_x11_colormap = XCreateColormap(
        _sapp_x11_display,
        _sapp_x11_root,
        visual,
        0 as libc::c_int,
    );
    let mut wa: XSetWindowAttributes = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    memset(
        &mut wa as *mut XSetWindowAttributes as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<XSetWindowAttributes>() as libc::c_ulong,
    );
    let wamask: uint32_t = ((1 as libc::c_long) << 3 as libc::c_int
        | (1 as libc::c_long) << 13 as libc::c_int
        | (1 as libc::c_long) << 11 as libc::c_int) as uint32_t;
    wa.colormap = _sapp_x11_colormap;
    wa.border_pixel = 0 as libc::c_int as libc::c_ulong;
    wa
        .event_mask = (1 as libc::c_long) << 17 as libc::c_int
        | (1 as libc::c_long) << 0 as libc::c_int
        | (1 as libc::c_long) << 1 as libc::c_int
        | (1 as libc::c_long) << 6 as libc::c_int
        | (1 as libc::c_long) << 2 as libc::c_int
        | (1 as libc::c_long) << 3 as libc::c_int
        | (1 as libc::c_long) << 15 as libc::c_int
        | (1 as libc::c_long) << 21 as libc::c_int
        | (1 as libc::c_long) << 16 as libc::c_int
        | (1 as libc::c_long) << 4 as libc::c_int
        | (1 as libc::c_long) << 5 as libc::c_int
        | (1 as libc::c_long) << 22 as libc::c_int;
    _sapp_x11_grab_error_handler();
    _sapp_x11_window = XCreateWindow(
        _sapp_x11_display,
        _sapp_x11_root,
        0 as libc::c_int,
        0 as libc::c_int,
        _sapp.window_width as libc::c_uint,
        _sapp.window_height as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        depth,
        1 as libc::c_int as libc::c_uint,
        visual,
        wamask as libc::c_ulong,
        &mut wa,
    );
    _sapp_x11_release_error_handler();
    if _sapp_x11_window == 0 {
        _sapp_fail(
            b"X11: Failed to create window\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut protocols: [Atom; 1] = [_sapp_x11_WM_DELETE_WINDOW];
    XSetWMProtocols(
        _sapp_x11_display,
        _sapp_x11_window,
        protocols.as_mut_ptr(),
        1 as libc::c_int,
    );
    let mut hints: *mut XSizeHints = XAllocSizeHints();
    (*hints).flags |= (1 as libc::c_long) << 9 as libc::c_int;
    (*hints).win_gravity = 10 as libc::c_int;
    XSetWMNormalHints(_sapp_x11_display, _sapp_x11_window, hints);
    XFree(hints as *mut libc::c_void);
    _sapp_x11_update_window_title();
    _sapp_x11_query_window_size();
}
unsafe extern "C" fn _sapp_x11_destroy_window() {
    if _sapp_x11_window != 0 {
        XUnmapWindow(_sapp_x11_display, _sapp_x11_window);
        XDestroyWindow(_sapp_x11_display, _sapp_x11_window);
        _sapp_x11_window = 0 as libc::c_int as Window;
    }
    if _sapp_x11_colormap != 0 {
        XFreeColormap(_sapp_x11_display, _sapp_x11_colormap);
        _sapp_x11_colormap = 0 as libc::c_int as Colormap;
    }
    XFlush(_sapp_x11_display);
}
unsafe extern "C" fn _sapp_x11_window_visible() -> bool {
    let mut wa: XWindowAttributes = XWindowAttributes {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        depth: 0,
        visual: 0 as *mut Visual,
        root: 0,
        class: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        colormap: 0,
        map_installed: 0,
        map_state: 0,
        all_event_masks: 0,
        your_event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        screen: 0 as *mut Screen,
    };
    XGetWindowAttributes(_sapp_x11_display, _sapp_x11_window, &mut wa);
    return wa.map_state == 2 as libc::c_int;
}
unsafe extern "C" fn _sapp_x11_show_window() {
    if !_sapp_x11_window_visible() {
        XMapWindow(_sapp_x11_display, _sapp_x11_window);
        XRaiseWindow(_sapp_x11_display, _sapp_x11_window);
        XFlush(_sapp_x11_display);
    }
}
unsafe extern "C" fn _sapp_x11_get_window_property(
    mut property: Atom,
    mut type_0: Atom,
    mut value: *mut *mut libc::c_uchar,
) -> libc::c_ulong {
    let mut actualType: Atom = 0;
    let mut actualFormat: libc::c_int = 0;
    let mut itemCount: libc::c_ulong = 0;
    let mut bytesAfter: libc::c_ulong = 0;
    XGetWindowProperty(
        _sapp_x11_display,
        _sapp_x11_window,
        property,
        0 as libc::c_int as libc::c_long,
        9223372036854775807 as libc::c_long,
        0 as libc::c_int,
        type_0,
        &mut actualType,
        &mut actualFormat,
        &mut itemCount,
        &mut bytesAfter,
        value,
    );
    return itemCount;
}
unsafe extern "C" fn _sg_gl_teximage_type(mut fmt: sg_pixel_format) -> GLenum {
    match fmt as libc::c_uint {
        8 | 10 => return 0x1406 as libc::c_int as GLenum,
        9 | 11 => return 0x140b as libc::c_int as GLenum,
        2 | 3 | 12 => return 0x1401 as libc::c_int as GLenum,
        7 => return 0x8368 as libc::c_int as GLenum,
        6 => return 0x8034 as libc::c_int as GLenum,
        5 => return 0x8363 as libc::c_int as GLenum,
        4 => return 0x8033 as libc::c_int as GLenum,
        16 => return 0x1403 as libc::c_int as GLenum,
        17 => return 0x84fa as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sapp_x11_get_window_state() -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: *mut C2RustUnnamed_3 = 0 as *mut C2RustUnnamed_3;
    if _sapp_x11_get_window_property(
        _sapp_x11_WM_STATE,
        _sapp_x11_WM_STATE,
        &mut state as *mut *mut C2RustUnnamed_3 as *mut *mut libc::c_uchar,
    ) >= 2 as libc::c_int as libc::c_ulong
    {
        result = (*state).state as libc::c_int;
    }
    if !state.is_null() {
        XFree(state as *mut libc::c_void);
    }
    return result;
}
unsafe extern "C" fn _sapp_x11_mod(mut x11_mods: libc::c_int) -> uint32_t {
    let mut mods: uint32_t = 0 as libc::c_int as uint32_t;
    if x11_mods & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        mods |= SAPP_MODIFIER_SHIFT as libc::c_int as libc::c_uint;
    }
    if x11_mods & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        mods |= SAPP_MODIFIER_CTRL as libc::c_int as libc::c_uint;
    }
    if x11_mods & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        mods |= SAPP_MODIFIER_ALT as libc::c_int as libc::c_uint;
    }
    if x11_mods & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        mods |= SAPP_MODIFIER_SUPER as libc::c_int as libc::c_uint;
    }
    return mods;
}
unsafe extern "C" fn _sapp_x11_app_event(mut type_0: sapp_event_type) {
    if _sapp_events_enabled() {
        _sapp_init_event(type_0);
        _sapp_call_event(&mut _sapp.event);
    }
}
unsafe extern "C" fn _sapp_x11_translate_button(
    mut event: *const XEvent,
) -> sapp_mousebutton {
    match (*event).xbutton.button {
        1 => return SAPP_MOUSEBUTTON_LEFT,
        2 => return SAPP_MOUSEBUTTON_MIDDLE,
        3 => return SAPP_MOUSEBUTTON_RIGHT,
        _ => return SAPP_MOUSEBUTTON_INVALID,
    };
}
unsafe extern "C" fn _sapp_x11_mouse_event(
    mut type_0: sapp_event_type,
    mut btn: sapp_mousebutton,
    mut mods: uint32_t,
) {
    if _sapp_events_enabled() {
        _sapp_init_event(type_0);
        _sapp.event.mouse_button = btn;
        _sapp.event.modifiers = mods;
        _sapp.event.mouse_x = _sapp.mouse_x;
        _sapp.event.mouse_y = _sapp.mouse_y;
        _sapp_call_event(&mut _sapp.event);
    }
}
unsafe extern "C" fn _sapp_x11_scroll_event(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut mods: uint32_t,
) {
    if _sapp_events_enabled() {
        _sapp_init_event(SAPP_EVENTTYPE_MOUSE_SCROLL);
        _sapp.event.modifiers = mods;
        _sapp.event.scroll_x = x;
        _sapp.event.scroll_y = y;
        _sapp_call_event(&mut _sapp.event);
    }
}
unsafe extern "C" fn _sapp_x11_key_event(
    mut type_0: sapp_event_type,
    mut key: sapp_keycode,
    mut repeat: bool,
    mut mods: uint32_t,
) {
    if _sapp_events_enabled() {
        _sapp_init_event(type_0);
        _sapp.event.key_code = key;
        _sapp.event.key_repeat = repeat;
        _sapp.event.modifiers = mods;
        _sapp_call_event(&mut _sapp.event);
    }
}
unsafe extern "C" fn _sapp_x11_char_event(
    mut chr: uint32_t,
    mut repeat: bool,
    mut mods: uint32_t,
) {
    if _sapp_events_enabled() {
        _sapp_init_event(SAPP_EVENTTYPE_CHAR);
        _sapp.event.char_code = chr;
        _sapp.event.key_repeat = repeat;
        _sapp.event.modifiers = mods;
        _sapp_call_event(&mut _sapp.event);
    }
}
unsafe extern "C" fn _sapp_x11_translate_key(mut scancode: libc::c_int) -> sapp_keycode {
    let mut dummy: libc::c_int = 0;
    let mut keysyms: *mut KeySym = XGetKeyboardMapping(
        _sapp_x11_display,
        scancode as KeyCode,
        1 as libc::c_int,
        &mut dummy,
    );
    let mut keysym: KeySym = *keysyms.offset(0 as libc::c_int as isize);
    XFree(keysyms as *mut libc::c_void);
    match keysym {
        65307 => return SAPP_KEYCODE_ESCAPE,
        65289 => return SAPP_KEYCODE_TAB,
        65505 => return SAPP_KEYCODE_LEFT_SHIFT,
        65506 => return SAPP_KEYCODE_RIGHT_SHIFT,
        65507 => return SAPP_KEYCODE_LEFT_CONTROL,
        65508 => return SAPP_KEYCODE_RIGHT_CONTROL,
        65511 | 65513 => return SAPP_KEYCODE_LEFT_ALT,
        65406 | 65027 | 65512 | 65514 => return SAPP_KEYCODE_RIGHT_ALT,
        65515 => return SAPP_KEYCODE_LEFT_SUPER,
        65516 => return SAPP_KEYCODE_RIGHT_SUPER,
        65383 => return SAPP_KEYCODE_MENU,
        65407 => return SAPP_KEYCODE_NUM_LOCK,
        65509 => return SAPP_KEYCODE_CAPS_LOCK,
        65377 => return SAPP_KEYCODE_PRINT_SCREEN,
        65300 => return SAPP_KEYCODE_SCROLL_LOCK,
        65299 => return SAPP_KEYCODE_PAUSE,
        65535 => return SAPP_KEYCODE_DELETE,
        65288 => return SAPP_KEYCODE_BACKSPACE,
        65293 => return SAPP_KEYCODE_ENTER,
        65360 => return SAPP_KEYCODE_HOME,
        65367 => return SAPP_KEYCODE_END,
        65365 => return SAPP_KEYCODE_PAGE_UP,
        65366 => return SAPP_KEYCODE_PAGE_DOWN,
        65379 => return SAPP_KEYCODE_INSERT,
        65361 => return SAPP_KEYCODE_LEFT,
        65363 => return SAPP_KEYCODE_RIGHT,
        65364 => return SAPP_KEYCODE_DOWN,
        65362 => return SAPP_KEYCODE_UP,
        65470 => return SAPP_KEYCODE_F1,
        65471 => return SAPP_KEYCODE_F2,
        65472 => return SAPP_KEYCODE_F3,
        65473 => return SAPP_KEYCODE_F4,
        65474 => return SAPP_KEYCODE_F5,
        65475 => return SAPP_KEYCODE_F6,
        65476 => return SAPP_KEYCODE_F7,
        65477 => return SAPP_KEYCODE_F8,
        65478 => return SAPP_KEYCODE_F9,
        65479 => return SAPP_KEYCODE_F10,
        65480 => return SAPP_KEYCODE_F11,
        65481 => return SAPP_KEYCODE_F12,
        65482 => return SAPP_KEYCODE_F13,
        65483 => return SAPP_KEYCODE_F14,
        65484 => return SAPP_KEYCODE_F15,
        65485 => return SAPP_KEYCODE_F16,
        65486 => return SAPP_KEYCODE_F17,
        65487 => return SAPP_KEYCODE_F18,
        65488 => return SAPP_KEYCODE_F19,
        65489 => return SAPP_KEYCODE_F20,
        65490 => return SAPP_KEYCODE_F21,
        65491 => return SAPP_KEYCODE_F22,
        65492 => return SAPP_KEYCODE_F23,
        65493 => return SAPP_KEYCODE_F24,
        65494 => return SAPP_KEYCODE_F25,
        65455 => return SAPP_KEYCODE_KP_DIVIDE,
        65450 => return SAPP_KEYCODE_KP_MULTIPLY,
        65453 => return SAPP_KEYCODE_KP_SUBTRACT,
        65451 => return SAPP_KEYCODE_KP_ADD,
        65438 => return SAPP_KEYCODE_KP_0,
        65436 => return SAPP_KEYCODE_KP_1,
        65433 => return SAPP_KEYCODE_KP_2,
        65435 => return SAPP_KEYCODE_KP_3,
        65430 => return SAPP_KEYCODE_KP_4,
        65437 => return SAPP_KEYCODE_KP_5,
        65432 => return SAPP_KEYCODE_KP_6,
        65429 => return SAPP_KEYCODE_KP_7,
        65431 => return SAPP_KEYCODE_KP_8,
        65434 => return SAPP_KEYCODE_KP_9,
        65439 => return SAPP_KEYCODE_KP_DECIMAL,
        65469 => return SAPP_KEYCODE_KP_EQUAL,
        65421 => return SAPP_KEYCODE_KP_ENTER,
        97 => return SAPP_KEYCODE_A,
        98 => return SAPP_KEYCODE_B,
        99 => return SAPP_KEYCODE_C,
        100 => return SAPP_KEYCODE_D,
        101 => return SAPP_KEYCODE_E,
        102 => return SAPP_KEYCODE_F,
        103 => return SAPP_KEYCODE_G,
        104 => return SAPP_KEYCODE_H,
        105 => return SAPP_KEYCODE_I,
        106 => return SAPP_KEYCODE_J,
        107 => return SAPP_KEYCODE_K,
        108 => return SAPP_KEYCODE_L,
        109 => return SAPP_KEYCODE_M,
        110 => return SAPP_KEYCODE_N,
        111 => return SAPP_KEYCODE_O,
        112 => return SAPP_KEYCODE_P,
        113 => return SAPP_KEYCODE_Q,
        114 => return SAPP_KEYCODE_R,
        115 => return SAPP_KEYCODE_S,
        116 => return SAPP_KEYCODE_T,
        117 => return SAPP_KEYCODE_U,
        118 => return SAPP_KEYCODE_V,
        119 => return SAPP_KEYCODE_W,
        120 => return SAPP_KEYCODE_X,
        121 => return SAPP_KEYCODE_Y,
        122 => return SAPP_KEYCODE_Z,
        49 => return SAPP_KEYCODE_1,
        50 => return SAPP_KEYCODE_2,
        51 => return SAPP_KEYCODE_3,
        52 => return SAPP_KEYCODE_4,
        53 => return SAPP_KEYCODE_5,
        54 => return SAPP_KEYCODE_6,
        55 => return SAPP_KEYCODE_7,
        56 => return SAPP_KEYCODE_8,
        57 => return SAPP_KEYCODE_9,
        48 => return SAPP_KEYCODE_0,
        32 => return SAPP_KEYCODE_SPACE,
        45 => return SAPP_KEYCODE_MINUS,
        61 => return SAPP_KEYCODE_EQUAL,
        91 => return SAPP_KEYCODE_LEFT_BRACKET,
        93 => return SAPP_KEYCODE_RIGHT_BRACKET,
        92 => return SAPP_KEYCODE_BACKSLASH,
        59 => return SAPP_KEYCODE_SEMICOLON,
        39 => return SAPP_KEYCODE_APOSTROPHE,
        96 => return SAPP_KEYCODE_GRAVE_ACCENT,
        44 => return SAPP_KEYCODE_COMMA,
        46 => return SAPP_KEYCODE_PERIOD,
        47 => return SAPP_KEYCODE_SLASH,
        60 => return SAPP_KEYCODE_WORLD_1,
        _ => return SAPP_KEYCODE_INVALID,
    };
}
unsafe extern "C" fn _sapp_x11_keysym_to_unicode(mut keysym: KeySym) -> int32_t {
    let mut min: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_int = (::std::mem::size_of::<[_sapp_x11_codepair; 828]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<_sapp_x11_codepair>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut mid: libc::c_int = 0;
    if keysym >= 0x20 as libc::c_int as libc::c_ulong
        && keysym <= 0x7e as libc::c_int as libc::c_ulong
        || keysym >= 0xa0 as libc::c_int as libc::c_ulong
            && keysym <= 0xff as libc::c_int as libc::c_ulong
    {
        return keysym as int32_t;
    }
    if keysym & 0xff000000 as libc::c_uint as libc::c_ulong
        == 0x1000000 as libc::c_int as libc::c_ulong
    {
        return (keysym & 0xffffff as libc::c_int as libc::c_ulong) as int32_t;
    }
    while max >= min {
        mid = (min + max) / 2 as libc::c_int;
        if (_sapp_x11_keysymtab[mid as usize].keysym as libc::c_ulong) < keysym {
            min = mid + 1 as libc::c_int;
        } else if _sapp_x11_keysymtab[mid as usize].keysym as libc::c_ulong > keysym {
            max = mid - 1 as libc::c_int;
        } else {
            return _sapp_x11_keysymtab[mid as usize].ucs as int32_t
        }
    }
    return -(1 as libc::c_int);
}
static mut _sapp_x11_keycodes: [bool; 256] = [false; 256];
unsafe extern "C" fn _sapp_x11_process_event(mut event: *mut XEvent) {
    match (*event).type_0 {
        2 => {
            let mut keycode: libc::c_int = (*event).xkey.keycode as libc::c_int;
            let key: sapp_keycode = _sapp_x11_translate_key(keycode);
            let mut repeat: bool = _sapp_x11_keycodes[(keycode & 0xff as libc::c_int)
                as usize];
            _sapp_x11_keycodes[(keycode & 0xff as libc::c_int)
                as usize] = 1 as libc::c_int != 0;
            let mods: uint32_t = _sapp_x11_mod((*event).xkey.state as libc::c_int);
            if key as libc::c_uint != SAPP_KEYCODE_INVALID as libc::c_int as libc::c_uint
            {
                _sapp_x11_key_event(SAPP_EVENTTYPE_KEY_DOWN, key, repeat, mods);
            }
            let mut keysym: KeySym = 0;
            XLookupString(
                &mut (*event).xkey,
                0 as *mut libc::c_char,
                0 as libc::c_int,
                &mut keysym,
                0 as *mut XComposeStatus,
            );
            let mut chr: int32_t = _sapp_x11_keysym_to_unicode(keysym);
            if chr > 0 as libc::c_int {
                _sapp_x11_char_event(chr as uint32_t, repeat, mods);
            }
        }
        3 => {
            let mut keycode_0: libc::c_int = (*event).xkey.keycode as libc::c_int;
            let key_0: sapp_keycode = _sapp_x11_translate_key(keycode_0);
            _sapp_x11_keycodes[(keycode_0 & 0xff as libc::c_int)
                as usize] = 0 as libc::c_int != 0;
            if key_0 as libc::c_uint
                != SAPP_KEYCODE_INVALID as libc::c_int as libc::c_uint
            {
                let mods_0: uint32_t = _sapp_x11_mod((*event).xkey.state as libc::c_int);
                _sapp_x11_key_event(
                    SAPP_EVENTTYPE_KEY_UP,
                    key_0,
                    0 as libc::c_int != 0,
                    mods_0,
                );
            }
        }
        4 => {
            let btn: sapp_mousebutton = _sapp_x11_translate_button(event);
            let mods_1: uint32_t = _sapp_x11_mod((*event).xbutton.state as libc::c_int);
            if btn as libc::c_int != SAPP_MOUSEBUTTON_INVALID as libc::c_int {
                _sapp_x11_mouse_event(SAPP_EVENTTYPE_MOUSE_DOWN, btn, mods_1);
            } else {
                match (*event).xbutton.button {
                    4 => {
                        _sapp_x11_scroll_event(0.0f32, 1.0f32, mods_1);
                    }
                    5 => {
                        _sapp_x11_scroll_event(0.0f32, -1.0f32, mods_1);
                    }
                    6 => {
                        _sapp_x11_scroll_event(1.0f32, 0.0f32, mods_1);
                    }
                    7 => {
                        _sapp_x11_scroll_event(-1.0f32, 0.0f32, mods_1);
                    }
                    _ => {}
                }
            }
        }
        5 => {
            let btn_0: sapp_mousebutton = _sapp_x11_translate_button(event);
            if btn_0 as libc::c_int != SAPP_MOUSEBUTTON_INVALID as libc::c_int {
                _sapp_x11_mouse_event(
                    SAPP_EVENTTYPE_MOUSE_UP,
                    btn_0,
                    _sapp_x11_mod((*event).xbutton.state as libc::c_int),
                );
            }
        }
        7 => {
            _sapp_x11_mouse_event(
                SAPP_EVENTTYPE_MOUSE_ENTER,
                SAPP_MOUSEBUTTON_INVALID,
                _sapp_x11_mod((*event).xcrossing.state as libc::c_int),
            );
        }
        8 => {
            _sapp_x11_mouse_event(
                SAPP_EVENTTYPE_MOUSE_LEAVE,
                SAPP_MOUSEBUTTON_INVALID,
                _sapp_x11_mod((*event).xcrossing.state as libc::c_int),
            );
        }
        6 => {
            _sapp.mouse_x = (*event).xmotion.x as libc::c_float;
            _sapp.mouse_y = (*event).xmotion.y as libc::c_float;
            _sapp_x11_mouse_event(
                SAPP_EVENTTYPE_MOUSE_MOVE,
                SAPP_MOUSEBUTTON_INVALID,
                _sapp_x11_mod((*event).xmotion.state as libc::c_int),
            );
        }
        22 => {
            if (*event).xconfigure.width != _sapp.window_width
                || (*event).xconfigure.height != _sapp.window_height
            {
                _sapp.window_width = (*event).xconfigure.width;
                _sapp.window_height = (*event).xconfigure.height;
                _sapp.framebuffer_width = _sapp.window_width;
                _sapp.framebuffer_height = _sapp.window_height;
                _sapp_x11_app_event(SAPP_EVENTTYPE_RESIZED);
            }
        }
        28 => {
            if (*event).xproperty.state == 0 as libc::c_int {
                if (*event).xproperty.atom == _sapp_x11_WM_STATE {
                    let state: libc::c_int = _sapp_x11_get_window_state();
                    if state != _sapp_x11_window_state {
                        _sapp_x11_window_state = state;
                        if state == 3 as libc::c_int {
                            _sapp_x11_app_event(SAPP_EVENTTYPE_ICONIFIED);
                        } else if state == 1 as libc::c_int {
                            _sapp_x11_app_event(SAPP_EVENTTYPE_RESTORED);
                        }
                    }
                }
            }
        }
        33 => {
            if (*event).xclient.message_type == _sapp_x11_WM_PROTOCOLS {
                let protocol: Atom = (*event).xclient.data.l[0 as libc::c_int as usize]
                    as Atom;
                if protocol == _sapp_x11_WM_DELETE_WINDOW {
                    _sapp_x11_quit_requested = 1 as libc::c_int != 0;
                }
            }
        }
        17 | _ => {}
    };
}
unsafe extern "C" fn _sapp_run(mut desc: *const sapp_desc) {
    _sapp_init_state(desc);
    _sapp_x11_quit_requested = 0 as libc::c_int != 0;
    _sapp_x11_window_state = 1 as libc::c_int;
    XInitThreads();
    XrmInitialize();
    _sapp_x11_display = XOpenDisplay(0 as *const libc::c_char);
    if _sapp_x11_display.is_null() {
        _sapp_fail(b"XOpenDisplay() failed!\n\0" as *const u8 as *const libc::c_char);
    }
    _sapp_x11_screen = (*(_sapp_x11_display as _XPrivDisplay)).default_screen;
    _sapp_x11_root = (*((*(_sapp_x11_display as _XPrivDisplay)).screens)
        .offset((*(_sapp_x11_display as _XPrivDisplay)).default_screen as isize))
        .root;
    XkbSetDetectableAutoRepeat(
        _sapp_x11_display,
        1 as libc::c_int,
        0 as *mut libc::c_int,
    );
    _sapp_x11_query_system_dpi();
    _sapp.dpi_scale = _sapp_x11_dpi / 96.0f32;
    _sapp_x11_init_extensions();
    _sapp_glx_init();
    let mut visual: *mut Visual = 0 as *mut Visual;
    let mut depth: libc::c_int = 0 as libc::c_int;
    _sapp_glx_choose_visual(&mut visual, &mut depth);
    _sapp_x11_create_window(visual, depth);
    _sapp_glx_create_context();
    _sapp.valid = 1 as libc::c_int != 0;
    _sapp_x11_show_window();
    _sapp_glx_swapinterval(_sapp.swap_interval);
    XFlush(_sapp_x11_display);
    while !_sapp_x11_quit_requested {
        _sapp_glx_make_current();
        let mut count: libc::c_int = XPending(_sapp_x11_display);
        loop {
            let fresh3 = count;
            count = count - 1;
            if !(fresh3 != 0) {
                break;
            }
            let mut event: XEvent = _XEvent { type_0: 0 };
            XNextEvent(_sapp_x11_display, &mut event);
            _sapp_x11_process_event(&mut event);
        }
        _sapp_frame();
        _sapp_glx_swap_buffers();
        XFlush(_sapp_x11_display);
    }
    _sapp_call_cleanup();
    _sapp_glx_destroy_context();
    _sapp_x11_destroy_window();
    XCloseDisplay(_sapp_x11_display);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut desc: sapp_desc = sokol_main(argc, argv);
    _sapp_run(&mut desc);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn stm_setup() {
    memset(
        &mut _stm as *mut _stm_state_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_stm_state_t>() as libc::c_ulong,
    );
    _stm.initialized = 0xabcdabcd as libc::c_uint;
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1 as libc::c_int, &mut ts);
    _stm
        .start = (ts.tv_sec as uint64_t)
        .wrapping_mul(1000000000 as libc::c_int as libc::c_ulong)
        .wrapping_add(ts.tv_nsec as uint64_t);
}
static mut _stm: _stm_state_t = _stm_state_t {
    initialized: 0,
    start: 0,
};
pub unsafe extern "C" fn stm_now() -> uint64_t {
    let mut now: uint64_t = 0;
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1 as libc::c_int, &mut ts);
    now = (ts.tv_sec as uint64_t)
        .wrapping_mul(1000000000 as libc::c_int as libc::c_ulong)
        .wrapping_add(ts.tv_nsec as uint64_t)
        .wrapping_sub(_stm.start);
    return now;
}
pub unsafe extern "C" fn stm_diff(
    mut new_ticks: uint64_t,
    mut old_ticks: uint64_t,
) -> uint64_t {
    if new_ticks > old_ticks {
        return new_ticks.wrapping_sub(old_ticks)
    } else {
        return 1 as libc::c_int as uint64_t
    };
}
pub unsafe extern "C" fn stm_since(mut start_ticks: uint64_t) -> uint64_t {
    return stm_diff(stm_now(), start_ticks);
}
pub unsafe extern "C" fn stm_laptime(mut last_time: *mut uint64_t) -> uint64_t {
    let mut dt: uint64_t = 0 as libc::c_int as uint64_t;
    let mut now: uint64_t = stm_now();
    if 0 as libc::c_int as libc::c_ulong != *last_time {
        dt = stm_diff(now, *last_time);
    }
    *last_time = now;
    return dt;
}
pub unsafe extern "C" fn stm_sec(mut ticks: uint64_t) -> libc::c_double {
    return ticks as libc::c_double / 1000000000.0f64;
}
pub unsafe extern "C" fn stm_ms(mut ticks: uint64_t) -> libc::c_double {
    return ticks as libc::c_double / 1000000.0f64;
}
pub unsafe extern "C" fn stm_us(mut ticks: uint64_t) -> libc::c_double {
    return ticks as libc::c_double / 1000.0f64;
}
pub unsafe extern "C" fn stm_ns(mut ticks: uint64_t) -> libc::c_double {
    return ticks as libc::c_double;
}
pub unsafe extern "C" fn sg_setup(mut desc: *const sg_desc) {
    memset(
        &mut _sg as *mut _sg_state_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_sg_state_t>() as libc::c_ulong,
    );
    _sg.desc = *desc;
    _sg
        .desc
        .buffer_pool_size = if _sg.desc.buffer_pool_size == 0 as libc::c_int {
        _SG_DEFAULT_BUFFER_POOL_SIZE as libc::c_int
    } else {
        _sg.desc.buffer_pool_size
    };
    _sg
        .desc
        .image_pool_size = if _sg.desc.image_pool_size == 0 as libc::c_int {
        _SG_DEFAULT_IMAGE_POOL_SIZE as libc::c_int
    } else {
        _sg.desc.image_pool_size
    };
    _sg
        .desc
        .shader_pool_size = if _sg.desc.shader_pool_size == 0 as libc::c_int {
        _SG_DEFAULT_SHADER_POOL_SIZE as libc::c_int
    } else {
        _sg.desc.shader_pool_size
    };
    _sg
        .desc
        .pipeline_pool_size = if _sg.desc.pipeline_pool_size == 0 as libc::c_int {
        _SG_DEFAULT_PIPELINE_POOL_SIZE as libc::c_int
    } else {
        _sg.desc.pipeline_pool_size
    };
    _sg
        .desc
        .pass_pool_size = if _sg.desc.pass_pool_size == 0 as libc::c_int {
        _SG_DEFAULT_PASS_POOL_SIZE as libc::c_int
    } else {
        _sg.desc.pass_pool_size
    };
    _sg
        .desc
        .context_pool_size = if _sg.desc.context_pool_size == 0 as libc::c_int {
        _SG_DEFAULT_CONTEXT_POOL_SIZE as libc::c_int
    } else {
        _sg.desc.context_pool_size
    };
    _sg
        .desc
        .mtl_global_uniform_buffer_size = if _sg.desc.mtl_global_uniform_buffer_size
        == 0 as libc::c_int
    {
        _SG_MTL_DEFAULT_UB_SIZE as libc::c_int
    } else {
        _sg.desc.mtl_global_uniform_buffer_size
    };
    _sg
        .desc
        .mtl_sampler_cache_size = if _sg.desc.mtl_sampler_cache_size == 0 as libc::c_int
    {
        _SG_MTL_DEFAULT_SAMPLER_CACHE_CAPACITY as libc::c_int
    } else {
        _sg.desc.mtl_sampler_cache_size
    };
    _sg_setup_pools(&mut _sg.pools, &mut _sg.desc);
    _sg.frame_index = 1 as libc::c_int as uint32_t;
    _sg_setup_backend(&mut _sg.desc);
    sg_setup_context();
    _sg.valid = 1 as libc::c_int != 0;
}
static mut _sg: _sg_state_t = _sg_state_t {
    valid: false,
    desc: sg_desc {
        _start_canary: 0,
        buffer_pool_size: 0,
        image_pool_size: 0,
        shader_pool_size: 0,
        pipeline_pool_size: 0,
        pass_pool_size: 0,
        context_pool_size: 0,
        gl_force_gles2: false,
        mtl_device: 0 as *const libc::c_void,
        mtl_renderpass_descriptor_cb: None,
        mtl_drawable_cb: None,
        mtl_global_uniform_buffer_size: 0,
        mtl_sampler_cache_size: 0,
        d3d11_device: 0 as *const libc::c_void,
        d3d11_device_context: 0 as *const libc::c_void,
        d3d11_render_target_view_cb: None,
        d3d11_depth_stencil_view_cb: None,
        _end_canary: 0,
    },
    frame_index: 0,
    active_context: sg_context { id: 0 },
    cur_pass: sg_pass { id: 0 },
    cur_pipeline: sg_pipeline { id: 0 },
    pass_valid: false,
    bindings_valid: false,
    next_draw_valid: false,
    pools: _sg_pools_t {
        buffer_pool: _sg_pool_t {
            size: 0,
            queue_top: 0,
            gen_ctrs: 0 as *const uint32_t as *mut uint32_t,
            free_queue: 0 as *const libc::c_int as *mut libc::c_int,
        },
        image_pool: _sg_pool_t {
            size: 0,
            queue_top: 0,
            gen_ctrs: 0 as *const uint32_t as *mut uint32_t,
            free_queue: 0 as *const libc::c_int as *mut libc::c_int,
        },
        shader_pool: _sg_pool_t {
            size: 0,
            queue_top: 0,
            gen_ctrs: 0 as *const uint32_t as *mut uint32_t,
            free_queue: 0 as *const libc::c_int as *mut libc::c_int,
        },
        pipeline_pool: _sg_pool_t {
            size: 0,
            queue_top: 0,
            gen_ctrs: 0 as *const uint32_t as *mut uint32_t,
            free_queue: 0 as *const libc::c_int as *mut libc::c_int,
        },
        pass_pool: _sg_pool_t {
            size: 0,
            queue_top: 0,
            gen_ctrs: 0 as *const uint32_t as *mut uint32_t,
            free_queue: 0 as *const libc::c_int as *mut libc::c_int,
        },
        context_pool: _sg_pool_t {
            size: 0,
            queue_top: 0,
            gen_ctrs: 0 as *const uint32_t as *mut uint32_t,
            free_queue: 0 as *const libc::c_int as *mut libc::c_int,
        },
        buffers: 0 as *const _sg_buffer_t as *mut _sg_buffer_t,
        images: 0 as *const _sg_image_t as *mut _sg_image_t,
        shaders: 0 as *const _sg_shader_t as *mut _sg_shader_t,
        pipelines: 0 as *const _sg_pipeline_t as *mut _sg_pipeline_t,
        passes: 0 as *const _sg_pass_t as *mut _sg_pass_t,
        contexts: 0 as *const _sg_context_t as *mut _sg_context_t,
    },
    gl: _sg_gl_backend_t {
        valid: false,
        gles2: false,
        in_pass: false,
        cur_pass_width: 0,
        cur_pass_height: 0,
        cur_context: 0 as *const _sg_context_t as *mut _sg_context_t,
        cur_pass: 0 as *const _sg_pass_t as *mut _sg_pass_t,
        cur_pass_id: sg_pass { id: 0 },
        cache: _sg_gl_state_cache_t {
            ds: sg_depth_stencil_state {
                stencil_front: sg_stencil_state {
                    fail_op: _SG_STENCILOP_DEFAULT,
                    depth_fail_op: _SG_STENCILOP_DEFAULT,
                    pass_op: _SG_STENCILOP_DEFAULT,
                    compare_func: _SG_COMPAREFUNC_DEFAULT,
                },
                stencil_back: sg_stencil_state {
                    fail_op: _SG_STENCILOP_DEFAULT,
                    depth_fail_op: _SG_STENCILOP_DEFAULT,
                    pass_op: _SG_STENCILOP_DEFAULT,
                    compare_func: _SG_COMPAREFUNC_DEFAULT,
                },
                depth_compare_func: _SG_COMPAREFUNC_DEFAULT,
                depth_write_enabled: false,
                stencil_enabled: false,
                stencil_read_mask: 0,
                stencil_write_mask: 0,
                stencil_ref: 0,
            },
            blend: sg_blend_state {
                enabled: false,
                src_factor_rgb: _SG_BLENDFACTOR_DEFAULT,
                dst_factor_rgb: _SG_BLENDFACTOR_DEFAULT,
                op_rgb: _SG_BLENDOP_DEFAULT,
                src_factor_alpha: _SG_BLENDFACTOR_DEFAULT,
                dst_factor_alpha: _SG_BLENDFACTOR_DEFAULT,
                op_alpha: _SG_BLENDOP_DEFAULT,
                color_write_mask: 0,
                color_attachment_count: 0,
                color_format: _SG_PIXELFORMAT_DEFAULT,
                depth_format: _SG_PIXELFORMAT_DEFAULT,
                blend_color: [0.; 4],
            },
            rast: sg_rasterizer_state {
                alpha_to_coverage_enabled: false,
                cull_mode: _SG_CULLMODE_DEFAULT,
                face_winding: _SG_FACEWINDING_DEFAULT,
                sample_count: 0,
                depth_bias: 0.,
                depth_bias_slope_scale: 0.,
                depth_bias_clamp: 0.,
            },
            polygon_offset_enabled: false,
            attrs: [_sg_gl_cache_attr_t {
                gl_attr: _sg_gl_attr_t {
                    vb_index: 0,
                    divisor: 0,
                    stride: 0,
                    size: 0,
                    normalized: 0,
                    offset: 0,
                    type_0: 0,
                },
                gl_vbuf: 0,
            }; 16],
            vertex_buffer: 0,
            index_buffer: 0,
            stored_vertex_buffer: 0,
            stored_index_buffer: 0,
            textures: [_sg_gl_texture_bind_slot {
                target: 0,
                texture: 0,
            }; 12],
            stored_texture: _sg_gl_texture_bind_slot {
                target: 0,
                texture: 0,
            },
            cur_ib_offset: 0,
            cur_primitive_type: 0,
            cur_index_type: 0,
            cur_pipeline: 0 as *const _sg_pipeline_t as *mut _sg_pipeline_t,
            cur_pipeline_id: sg_pipeline { id: 0 },
        },
        features: [false; 14],
        ext_anisotropic: false,
        max_anisotropy: 0,
        max_combined_texture_image_units: 0,
    },
};
pub unsafe extern "C" fn sg_setup_context() -> sg_context {
    let mut res: sg_context = sg_context { id: 0 };
    let mut slot_index: libc::c_int = _sg_pool_alloc_index(&mut _sg.pools.context_pool);
    if 0 as libc::c_int != slot_index {
        res
            .id = _sg_slot_alloc(
            &mut _sg.pools.context_pool,
            &mut (*(_sg.pools.contexts).offset(slot_index as isize)).slot,
            slot_index,
        );
        let mut ctx: *mut _sg_context_t = _sg_context_at(&mut _sg.pools, res.id);
        (*ctx).slot.state = _sg_create_context(ctx);
        _sg_activate_context(ctx);
    } else {
        res.id = SG_INVALID_ID as libc::c_int as uint32_t;
    }
    _sg.active_context = res;
    return res;
}
unsafe extern "C" fn _sg_context_at(
    mut p: *const _sg_pools_t,
    mut context_id: uint32_t,
) -> *mut _sg_context_t {
    let mut slot_index: libc::c_int = _sg_slot_index(context_id);
    return &mut *((*p).contexts).offset(slot_index as isize) as *mut _sg_context_t;
}
unsafe extern "C" fn _sg_slot_index(mut id: uint32_t) -> libc::c_int {
    let mut slot_index: libc::c_int = (id & _SG_SLOT_MASK as libc::c_int as libc::c_uint)
        as libc::c_int;
    return slot_index;
}
unsafe extern "C" fn _sg_activate_context(mut ctx: *mut _sg_context_t) {
    _sg.gl.cur_context = ctx;
    _sg_reset_state_cache();
}
unsafe extern "C" fn _sg_reset_state_cache() {
    if !(_sg.gl.cur_context).is_null() {
        if !_sg.gl.gles2 {
            glBindVertexArray((*_sg.gl.cur_context).vao);
        }
        _sg_gl_reset_state_cache();
    }
}
unsafe extern "C" fn _sg_gl_reset_state_cache() {
    memset(
        &mut _sg.gl.cache as *mut _sg_gl_state_cache_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_sg_gl_state_cache_t>() as libc::c_ulong,
    );
    _sg_gl_clear_buffer_bindings(1 as libc::c_int != 0);
    _sg_gl_clear_texture_bindings(1 as libc::c_int != 0);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < SG_MAX_VERTEX_ATTRIBUTES as libc::c_int {
        _sg_gl_init_attr(
            &mut (*(_sg.gl.cache.attrs).as_mut_ptr().offset(i as isize)).gl_attr,
        );
        glDisableVertexAttribArray(i as GLuint);
        i += 1;
        i;
    }
    _sg.gl.cache.cur_primitive_type = 0x4 as libc::c_int as GLenum;
    _sg_gl_init_depth_stencil_state(&mut _sg.gl.cache.ds);
    glEnable(0xb71 as libc::c_int as GLenum);
    glDepthFunc(0x207 as libc::c_int as GLenum);
    glDepthMask(0 as libc::c_int as GLboolean);
    glDisable(0xb90 as libc::c_int as GLenum);
    glStencilFunc(
        0x207 as libc::c_int as GLenum,
        0 as libc::c_int,
        0 as libc::c_int as GLuint,
    );
    glStencilOp(
        0x1e00 as libc::c_int as GLenum,
        0x1e00 as libc::c_int as GLenum,
        0x1e00 as libc::c_int as GLenum,
    );
    glStencilMask(0 as libc::c_int as GLuint);
    _sg_gl_init_blend_state(&mut _sg.gl.cache.blend);
    glDisable(0xbe2 as libc::c_int as GLenum);
    glBlendFuncSeparate(
        1 as libc::c_int as GLenum,
        0 as libc::c_int as GLenum,
        1 as libc::c_int as GLenum,
        0 as libc::c_int as GLenum,
    );
    glBlendEquationSeparate(
        0x8006 as libc::c_int as GLenum,
        0x8006 as libc::c_int as GLenum,
    );
    glColorMask(
        1 as libc::c_int as GLboolean,
        1 as libc::c_int as GLboolean,
        1 as libc::c_int as GLboolean,
        1 as libc::c_int as GLboolean,
    );
    glBlendColor(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    _sg_gl_init_rasterizer_state(&mut _sg.gl.cache.rast);
    glPolygonOffset(0.0f32, 0.0f32);
    glDisable(0x8037 as libc::c_int as GLenum);
    glDisable(0xb44 as libc::c_int as GLenum);
    glFrontFace(0x900 as libc::c_int as GLenum);
    glCullFace(0x405 as libc::c_int as GLenum);
    glEnable(0xc11 as libc::c_int as GLenum);
    glDisable(0x809e as libc::c_int as GLenum);
    glEnable(0xbd0 as libc::c_int as GLenum);
    glDisable(0x8037 as libc::c_int as GLenum);
    glEnable(0x809d as libc::c_int as GLenum);
    glEnable(0x8642 as libc::c_int as GLenum);
}
unsafe extern "C" fn _sg_gl_init_rasterizer_state(mut s: *mut sg_rasterizer_state) {
    (*s).cull_mode = SG_CULLMODE_NONE;
    (*s).face_winding = SG_FACEWINDING_CW;
    (*s).sample_count = 1 as libc::c_int;
}
unsafe extern "C" fn _sg_gl_init_blend_state(mut s: *mut sg_blend_state) {
    (*s).src_factor_rgb = SG_BLENDFACTOR_ONE;
    (*s).dst_factor_rgb = SG_BLENDFACTOR_ZERO;
    (*s).op_rgb = SG_BLENDOP_ADD;
    (*s).src_factor_alpha = SG_BLENDFACTOR_ONE;
    (*s).dst_factor_alpha = SG_BLENDFACTOR_ZERO;
    (*s).op_alpha = SG_BLENDOP_ADD;
    (*s).color_write_mask = SG_COLORMASK_RGBA as libc::c_int as uint8_t;
}
unsafe extern "C" fn _sg_gl_init_depth_stencil_state(
    mut s: *mut sg_depth_stencil_state,
) {
    _sg_gl_init_stencil_state(&mut (*s).stencil_front);
    _sg_gl_init_stencil_state(&mut (*s).stencil_back);
    (*s).depth_compare_func = SG_COMPAREFUNC_ALWAYS;
}
unsafe extern "C" fn _sg_gl_init_stencil_state(mut s: *mut sg_stencil_state) {
    (*s).fail_op = SG_STENCILOP_KEEP;
    (*s).depth_fail_op = SG_STENCILOP_KEEP;
    (*s).pass_op = SG_STENCILOP_KEEP;
    (*s).compare_func = SG_COMPAREFUNC_ALWAYS;
}
unsafe extern "C" fn _sg_gl_init_attr(mut attr: *mut _sg_gl_attr_t) {
    (*attr).vb_index = -(1 as libc::c_int) as int8_t;
    (*attr).divisor = -(1 as libc::c_int) as int8_t;
}
unsafe extern "C" fn _sg_gl_clear_texture_bindings(mut force: bool) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < SG_MAX_SHADERSTAGE_IMAGES as libc::c_int
        && i < _sg.gl.max_combined_texture_image_units
    {
        if force as libc::c_int != 0
            || _sg.gl.cache.textures[i as usize].texture
                != 0 as libc::c_int as libc::c_uint
        {
            glActiveTexture((0x84c0 as libc::c_int + i) as GLenum);
            glBindTexture(0xde1 as libc::c_int as GLenum, 0 as libc::c_int as GLuint);
            glBindTexture(0x8513 as libc::c_int as GLenum, 0 as libc::c_int as GLuint);
            if !_sg.gl.gles2 {
                glBindTexture(
                    0x806f as libc::c_int as GLenum,
                    0 as libc::c_int as GLuint,
                );
                glBindTexture(
                    0x8c1a as libc::c_int as GLenum,
                    0 as libc::c_int as GLuint,
                );
            }
            _sg.gl.cache.textures[i as usize].target = 0 as libc::c_int as GLenum;
            _sg.gl.cache.textures[i as usize].texture = 0 as libc::c_int as GLuint;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn _sg_gl_clear_buffer_bindings(mut force: bool) {
    if force as libc::c_int != 0
        || _sg.gl.cache.vertex_buffer != 0 as libc::c_int as libc::c_uint
    {
        glBindBuffer(0x8892 as libc::c_int as GLenum, 0 as libc::c_int as GLuint);
        _sg.gl.cache.vertex_buffer = 0 as libc::c_int as GLuint;
    }
    if force as libc::c_int != 0
        || _sg.gl.cache.index_buffer != 0 as libc::c_int as libc::c_uint
    {
        glBindBuffer(0x8893 as libc::c_int as GLenum, 0 as libc::c_int as GLuint);
        _sg.gl.cache.index_buffer = 0 as libc::c_int as GLuint;
    }
}
unsafe extern "C" fn _sg_create_context(
    mut ctx: *mut _sg_context_t,
) -> sg_resource_state {
    glGetIntegerv(
        0x8ca6 as libc::c_int as GLenum,
        &mut (*ctx).default_framebuffer as *mut GLuint as *mut GLint,
    );
    if !_sg.gl.gles2 {
        glGenVertexArrays(1 as libc::c_int, &mut (*ctx).vao);
        glBindVertexArray((*ctx).vao);
    }
    return SG_RESOURCESTATE_VALID;
}
unsafe extern "C" fn _sg_pool_alloc_index(mut pool: *mut _sg_pool_t) -> libc::c_int {
    if (*pool).queue_top > 0 as libc::c_int {
        (*pool).queue_top -= 1;
        let mut slot_index: libc::c_int = *((*pool).free_queue)
            .offset((*pool).queue_top as isize);
        return slot_index;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn _sg_slot_alloc(
    mut pool: *mut _sg_pool_t,
    mut slot: *mut _sg_slot_t,
    mut slot_index: libc::c_int,
) -> uint32_t {
    let ref mut fresh4 = *((*pool).gen_ctrs).offset(slot_index as isize);
    *fresh4 = (*fresh4).wrapping_add(1);
    let mut ctr: uint32_t = *fresh4;
    (*slot)
        .id = ctr << _SG_SLOT_SHIFT as libc::c_int
        | (slot_index & _SG_SLOT_MASK as libc::c_int) as libc::c_uint;
    (*slot).state = SG_RESOURCESTATE_ALLOC;
    return (*slot).id;
}
unsafe extern "C" fn _sg_setup_backend(mut desc: *const sg_desc) {
    _sg.gl.valid = 1 as libc::c_int != 0;
    _sg.gl.gles2 = 0 as libc::c_int != 0;
    _sg
        .gl
        .features[SG_FEATURE_ORIGIN_BOTTOM_LEFT as libc::c_int
        as usize] = 1 as libc::c_int != 0;
    _sg
        .gl
        .features[SG_FEATURE_INSTANCING as libc::c_int as usize] = 1 as libc::c_int != 0;
    _sg
        .gl
        .features[SG_FEATURE_TEXTURE_FLOAT as libc::c_int
        as usize] = 1 as libc::c_int != 0;
    _sg
        .gl
        .features[SG_FEATURE_TEXTURE_HALF_FLOAT as libc::c_int
        as usize] = 1 as libc::c_int != 0;
    _sg
        .gl
        .features[SG_FEATURE_MSAA_RENDER_TARGETS as libc::c_int
        as usize] = 1 as libc::c_int != 0;
    _sg
        .gl
        .features[SG_FEATURE_PACKED_VERTEX_FORMAT_10_2 as libc::c_int
        as usize] = 1 as libc::c_int != 0;
    _sg
        .gl
        .features[SG_FEATURE_MULTIPLE_RENDER_TARGET as libc::c_int
        as usize] = 1 as libc::c_int != 0;
    _sg
        .gl
        .features[SG_FEATURE_IMAGETYPE_3D as libc::c_int
        as usize] = 1 as libc::c_int != 0;
    _sg
        .gl
        .features[SG_FEATURE_IMAGETYPE_ARRAY as libc::c_int
        as usize] = 1 as libc::c_int != 0;
    let mut num_ext: GLint = 0 as libc::c_int;
    glGetIntegerv(0x821d as libc::c_int as GLenum, &mut num_ext);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num_ext {
        let mut ext: *const libc::c_char = glGetStringi(
            0x1f03 as libc::c_int as GLenum,
            i as GLuint,
        ) as *const libc::c_char;
        if !ext.is_null() {
            if !(strstr(
                ext,
                b"_texture_compression_s3tc\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
            {
                _sg
                    .gl
                    .features[SG_FEATURE_TEXTURE_COMPRESSION_DXT as libc::c_int
                    as usize] = 1 as libc::c_int != 0;
            } else if !(strstr(
                ext,
                b"_texture_filter_anisotropic\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
            {
                _sg.gl.ext_anisotropic = 1 as libc::c_int != 0;
            }
        }
        i += 1;
        i;
    }
    _sg.gl.max_anisotropy = 1 as libc::c_int;
    if _sg.gl.ext_anisotropic {
        glGetIntegerv(0x84ff as libc::c_int as GLenum, &mut _sg.gl.max_anisotropy);
    }
    glGetIntegerv(
        0x8b4d as libc::c_int as GLenum,
        &mut _sg.gl.max_combined_texture_image_units,
    );
}
unsafe extern "C" fn _sg_setup_pools(mut p: *mut _sg_pools_t, mut desc: *const sg_desc) {
    _sg_init_pool(&mut (*p).buffer_pool, (*desc).buffer_pool_size);
    let mut buffer_pool_byte_size: size_t = (::std::mem::size_of::<_sg_buffer_t>()
        as libc::c_ulong)
        .wrapping_mul((*p).buffer_pool.size as libc::c_ulong);
    (*p).buffers = malloc(buffer_pool_byte_size) as *mut _sg_buffer_t;
    memset((*p).buffers as *mut libc::c_void, 0 as libc::c_int, buffer_pool_byte_size);
    _sg_init_pool(&mut (*p).image_pool, (*desc).image_pool_size);
    let mut image_pool_byte_size: size_t = (::std::mem::size_of::<_sg_image_t>()
        as libc::c_ulong)
        .wrapping_mul((*p).image_pool.size as libc::c_ulong);
    (*p).images = malloc(image_pool_byte_size) as *mut _sg_image_t;
    memset((*p).images as *mut libc::c_void, 0 as libc::c_int, image_pool_byte_size);
    _sg_init_pool(&mut (*p).shader_pool, (*desc).shader_pool_size);
    let mut shader_pool_byte_size: size_t = (::std::mem::size_of::<_sg_shader_t>()
        as libc::c_ulong)
        .wrapping_mul((*p).shader_pool.size as libc::c_ulong);
    (*p).shaders = malloc(shader_pool_byte_size) as *mut _sg_shader_t;
    memset((*p).shaders as *mut libc::c_void, 0 as libc::c_int, shader_pool_byte_size);
    _sg_init_pool(&mut (*p).pipeline_pool, (*desc).pipeline_pool_size);
    let mut pipeline_pool_byte_size: size_t = (::std::mem::size_of::<_sg_pipeline_t>()
        as libc::c_ulong)
        .wrapping_mul((*p).pipeline_pool.size as libc::c_ulong);
    (*p).pipelines = malloc(pipeline_pool_byte_size) as *mut _sg_pipeline_t;
    memset(
        (*p).pipelines as *mut libc::c_void,
        0 as libc::c_int,
        pipeline_pool_byte_size,
    );
    _sg_init_pool(&mut (*p).pass_pool, (*desc).pass_pool_size);
    let mut pass_pool_byte_size: size_t = (::std::mem::size_of::<_sg_pass_t>()
        as libc::c_ulong)
        .wrapping_mul((*p).pass_pool.size as libc::c_ulong);
    (*p).passes = malloc(pass_pool_byte_size) as *mut _sg_pass_t;
    memset((*p).passes as *mut libc::c_void, 0 as libc::c_int, pass_pool_byte_size);
    _sg_init_pool(&mut (*p).context_pool, (*desc).context_pool_size);
    let mut context_pool_byte_size: size_t = (::std::mem::size_of::<_sg_context_t>()
        as libc::c_ulong)
        .wrapping_mul((*p).context_pool.size as libc::c_ulong);
    (*p).contexts = malloc(context_pool_byte_size) as *mut _sg_context_t;
    memset((*p).contexts as *mut libc::c_void, 0 as libc::c_int, context_pool_byte_size);
}
unsafe extern "C" fn _sg_init_pool(mut pool: *mut _sg_pool_t, mut num: libc::c_int) {
    (*pool).size = num + 1 as libc::c_int;
    (*pool).queue_top = 0 as libc::c_int;
    let mut gen_ctrs_size: size_t = (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul((*pool).size as libc::c_ulong);
    (*pool).gen_ctrs = malloc(gen_ctrs_size) as *mut uint32_t;
    memset((*pool).gen_ctrs as *mut libc::c_void, 0 as libc::c_int, gen_ctrs_size);
    (*pool)
        .free_queue = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(num as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = (*pool).size - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let fresh5 = (*pool).queue_top;
        (*pool).queue_top = (*pool).queue_top + 1;
        *((*pool).free_queue).offset(fresh5 as isize) = i;
        i -= 1;
        i;
    }
}
pub unsafe extern "C" fn sg_shutdown() {
    if _sg.active_context.id != SG_INVALID_ID as libc::c_int as libc::c_uint {
        let mut ctx: *mut _sg_context_t = _sg_lookup_context(
            &mut _sg.pools,
            _sg.active_context.id,
        );
        if !ctx.is_null() {
            _sg_destroy_all_resources(&mut _sg.pools, _sg.active_context.id);
            _sg_destroy_context(ctx);
        }
    }
    _sg_discard_backend();
    _sg_discard_pools(&mut _sg.pools);
    _sg.valid = 0 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_discard_pools(mut p: *mut _sg_pools_t) {
    free((*p).contexts as *mut libc::c_void);
    (*p).contexts = 0 as *mut _sg_context_t;
    free((*p).passes as *mut libc::c_void);
    (*p).passes = 0 as *mut _sg_pass_t;
    free((*p).pipelines as *mut libc::c_void);
    (*p).pipelines = 0 as *mut _sg_pipeline_t;
    free((*p).shaders as *mut libc::c_void);
    (*p).shaders = 0 as *mut _sg_shader_t;
    free((*p).images as *mut libc::c_void);
    (*p).images = 0 as *mut _sg_image_t;
    free((*p).buffers as *mut libc::c_void);
    (*p).buffers = 0 as *mut _sg_buffer_t;
    _sg_discard_pool(&mut (*p).context_pool);
    _sg_discard_pool(&mut (*p).pass_pool);
    _sg_discard_pool(&mut (*p).pipeline_pool);
    _sg_discard_pool(&mut (*p).shader_pool);
    _sg_discard_pool(&mut (*p).image_pool);
    _sg_discard_pool(&mut (*p).buffer_pool);
}
unsafe extern "C" fn _sg_discard_pool(mut pool: *mut _sg_pool_t) {
    free((*pool).free_queue as *mut libc::c_void);
    (*pool).free_queue = 0 as *mut libc::c_int;
    free((*pool).gen_ctrs as *mut libc::c_void);
    (*pool).gen_ctrs = 0 as *mut uint32_t;
    (*pool).size = 0 as libc::c_int;
    (*pool).queue_top = 0 as libc::c_int;
}
unsafe extern "C" fn _sg_discard_backend() {
    _sg.gl.valid = 0 as libc::c_int != 0;
}
unsafe extern "C" fn _sg_lookup_context(
    mut p: *const _sg_pools_t,
    mut ctx_id: uint32_t,
) -> *mut _sg_context_t {
    if SG_INVALID_ID as libc::c_int as libc::c_uint != ctx_id {
        let mut ctx: *mut _sg_context_t = _sg_context_at(p, ctx_id);
        if (*ctx).slot.id == ctx_id {
            return ctx;
        }
    }
    return 0 as *mut _sg_context_t;
}
unsafe extern "C" fn _sg_destroy_context(mut ctx: *mut _sg_context_t) {
    if !_sg.gl.gles2 {
        if (*ctx).vao != 0 {
            glDeleteVertexArrays(1 as libc::c_int, &mut (*ctx).vao);
        }
    }
}
unsafe extern "C" fn _sg_destroy_all_resources(
    mut p: *mut _sg_pools_t,
    mut ctx_id: uint32_t,
) {
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < (*p).buffer_pool.size {
        if (*((*p).buffers).offset(i as isize)).slot.ctx_id == ctx_id {
            let mut state: sg_resource_state = (*((*p).buffers).offset(i as isize))
                .slot
                .state;
            if state as libc::c_uint
                == SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
                || state as libc::c_uint
                    == SG_RESOURCESTATE_FAILED as libc::c_int as libc::c_uint
            {
                _sg_destroy_buffer(&mut *((*p).buffers).offset(i as isize));
            }
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 1 as libc::c_int;
    while i_0 < (*p).image_pool.size {
        if (*((*p).images).offset(i_0 as isize)).slot.ctx_id == ctx_id {
            let mut state_0: sg_resource_state = (*((*p).images).offset(i_0 as isize))
                .slot
                .state;
            if state_0 as libc::c_uint
                == SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
                || state_0 as libc::c_uint
                    == SG_RESOURCESTATE_FAILED as libc::c_int as libc::c_uint
            {
                _sg_destroy_image(&mut *((*p).images).offset(i_0 as isize));
            }
        }
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 1 as libc::c_int;
    while i_1 < (*p).shader_pool.size {
        if (*((*p).shaders).offset(i_1 as isize)).slot.ctx_id == ctx_id {
            let mut state_1: sg_resource_state = (*((*p).shaders).offset(i_1 as isize))
                .slot
                .state;
            if state_1 as libc::c_uint
                == SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
                || state_1 as libc::c_uint
                    == SG_RESOURCESTATE_FAILED as libc::c_int as libc::c_uint
            {
                _sg_destroy_shader(&mut *((*p).shaders).offset(i_1 as isize));
            }
        }
        i_1 += 1;
        i_1;
    }
    let mut i_2: libc::c_int = 1 as libc::c_int;
    while i_2 < (*p).pipeline_pool.size {
        if (*((*p).pipelines).offset(i_2 as isize)).slot.ctx_id == ctx_id {
            let mut state_2: sg_resource_state = (*((*p).pipelines).offset(i_2 as isize))
                .slot
                .state;
            if state_2 as libc::c_uint
                == SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
                || state_2 as libc::c_uint
                    == SG_RESOURCESTATE_FAILED as libc::c_int as libc::c_uint
            {
                _sg_destroy_pipeline(&mut *((*p).pipelines).offset(i_2 as isize));
            }
        }
        i_2 += 1;
        i_2;
    }
    let mut i_3: libc::c_int = 1 as libc::c_int;
    while i_3 < (*p).pass_pool.size {
        if (*((*p).passes).offset(i_3 as isize)).slot.ctx_id == ctx_id {
            let mut state_3: sg_resource_state = (*((*p).passes).offset(i_3 as isize))
                .slot
                .state;
            if state_3 as libc::c_uint
                == SG_RESOURCESTATE_VALID as libc::c_int as libc::c_uint
                || state_3 as libc::c_uint
                    == SG_RESOURCESTATE_FAILED as libc::c_int as libc::c_uint
            {
                _sg_destroy_pass(&mut *((*p).passes).offset(i_3 as isize));
            }
        }
        i_3 += 1;
        i_3;
    }
}
unsafe extern "C" fn _sg_destroy_pass(mut pass: *mut _sg_pass_t) {
    if 0 as libc::c_int as libc::c_uint != (*pass).gl_fb {
        glDeleteFramebuffers(1 as libc::c_int, &mut (*pass).gl_fb);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < SG_MAX_COLOR_ATTACHMENTS as libc::c_int {
        if (*pass).color_atts[i as usize].gl_msaa_resolve_buffer != 0 {
            glDeleteFramebuffers(
                1 as libc::c_int,
                &mut (*((*pass).color_atts).as_mut_ptr().offset(i as isize))
                    .gl_msaa_resolve_buffer,
            );
        }
        i += 1;
        i;
    }
    if (*pass).ds_att.gl_msaa_resolve_buffer != 0 {
        glDeleteFramebuffers(
            1 as libc::c_int,
            &mut (*pass).ds_att.gl_msaa_resolve_buffer,
        );
    }
}
unsafe extern "C" fn _sg_destroy_pipeline(mut pip: *mut _sg_pipeline_t) {}
unsafe extern "C" fn _sg_destroy_shader(mut shd: *mut _sg_shader_t) {
    if (*shd).gl_prog != 0 {
        glDeleteProgram((*shd).gl_prog);
    }
}
unsafe extern "C" fn _sg_destroy_image(mut img: *mut _sg_image_t) {
    if !(*img).ext_textures {
        let mut slot: libc::c_int = 0 as libc::c_int;
        while slot < (*img).num_slots {
            if (*img).gl_tex[slot as usize] != 0 {
                glDeleteTextures(
                    1 as libc::c_int,
                    &mut *((*img).gl_tex).as_mut_ptr().offset(slot as isize),
                );
            }
            slot += 1;
            slot;
        }
    }
    if (*img).gl_depth_render_buffer != 0 {
        glDeleteRenderbuffers(1 as libc::c_int, &mut (*img).gl_depth_render_buffer);
    }
    if (*img).gl_msaa_render_buffer != 0 {
        glDeleteRenderbuffers(1 as libc::c_int, &mut (*img).gl_msaa_render_buffer);
    }
}
unsafe extern "C" fn _sg_destroy_buffer(mut buf: *mut _sg_buffer_t) {
    if !(*buf).ext_buffers {
        let mut slot: libc::c_int = 0 as libc::c_int;
        while slot < (*buf).num_slots {
            if (*buf).gl_buf[slot as usize] != 0 {
                glDeleteBuffers(
                    1 as libc::c_int,
                    &mut *((*buf).gl_buf).as_mut_ptr().offset(slot as isize),
                );
            }
            slot += 1;
            slot;
        }
    }
}
pub unsafe extern "C" fn sg_isvalid() -> bool {
    return _sg.valid;
}
pub unsafe extern "C" fn sg_query_desc() -> sg_desc {
    return _sg.desc;
}
pub unsafe extern "C" fn sg_query_backend() -> sg_backend {
    return SG_BACKEND_GLCORE33;
}
pub unsafe extern "C" fn sg_query_feature(mut f: sg_feature) -> bool {
    let mut res: bool = _sg_query_feature(f);
    return res;
}
unsafe extern "C" fn _sg_query_feature(mut f: sg_feature) -> bool {
    return _sg.gl.features[f as usize];
}
pub unsafe extern "C" fn sg_reset_state_cache() {
    _sg_reset_state_cache();
}
pub unsafe extern "C" fn sg_install_trace_hooks(
    mut trace_hooks: *const sg_trace_hooks,
) -> sg_trace_hooks {
    static mut old_hooks: sg_trace_hooks = sg_trace_hooks {
        user_data: 0 as *const libc::c_void as *mut libc::c_void,
        query_feature: None,
        reset_state_cache: None,
        make_buffer: None,
        make_image: None,
        make_shader: None,
        make_pipeline: None,
        make_pass: None,
        destroy_buffer: None,
        destroy_image: None,
        destroy_shader: None,
        destroy_pipeline: None,
        destroy_pass: None,
        update_buffer: None,
        update_image: None,
        append_buffer: None,
        begin_default_pass: None,
        begin_pass: None,
        apply_viewport: None,
        apply_scissor_rect: None,
        apply_pipeline: None,
        apply_bindings: None,
        apply_uniforms: None,
        draw: None,
        end_pass: None,
        commit: None,
        alloc_buffer: None,
        alloc_image: None,
        alloc_shader: None,
        alloc_pipeline: None,
        alloc_pass: None,
        init_buffer: None,
        init_image: None,
        init_shader: None,
        init_pipeline: None,
        init_pass: None,
        fail_buffer: None,
        fail_image: None,
        fail_shader: None,
        fail_pipeline: None,
        fail_pass: None,
        push_debug_group: None,
        pop_debug_group: None,
        err_buffer_pool_exhausted: None,
        err_image_pool_exhausted: None,
        err_shader_pool_exhausted: None,
        err_pipeline_pool_exhausted: None,
        err_pass_pool_exhausted: None,
        err_context_mismatch: None,
        err_pass_invalid: None,
        err_draw_invalid: None,
        err_bindings_invalid: None,
    };
    return old_hooks;
}
pub unsafe extern "C" fn sg_push_debug_group(mut name: *const libc::c_char) {}
pub unsafe extern "C" fn sg_pop_debug_group() {}
pub unsafe extern "C" fn sg_make_buffer(mut desc: *const sg_buffer_desc) -> sg_buffer {
    let mut desc_def: sg_buffer_desc = _sg_buffer_desc_defaults(desc);
    let mut buf_id: sg_buffer = _sg_alloc_buffer();
    if buf_id.id != SG_INVALID_ID as libc::c_int as libc::c_uint {
        _sg_init_buffer(buf_id, &mut desc_def);
    }
    return buf_id;
}
unsafe extern "C" fn _sg_alloc_buffer() -> sg_buffer {
    let mut res: sg_buffer = sg_buffer { id: 0 };
    let mut slot_index: libc::c_int = _sg_pool_alloc_index(&mut _sg.pools.buffer_pool);
    if 0 as libc::c_int != slot_index {
        res
            .id = _sg_slot_alloc(
            &mut _sg.pools.buffer_pool,
            &mut (*(_sg.pools.buffers).offset(slot_index as isize)).slot,
            slot_index,
        );
    } else {
        res.id = SG_INVALID_ID as libc::c_int as uint32_t;
    }
    return res;
}
unsafe extern "C" fn _sg_buffer_desc_defaults(
    mut desc: *const sg_buffer_desc,
) -> sg_buffer_desc {
    let mut def: sg_buffer_desc = *desc;
    def
        .type_0 = (if def.type_0 as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        SG_BUFFERTYPE_VERTEXBUFFER as libc::c_int as libc::c_uint
    } else {
        def.type_0 as libc::c_uint
    }) as sg_buffer_type;
    def
        .usage = (if def.usage as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        SG_USAGE_IMMUTABLE as libc::c_int as libc::c_uint
    } else {
        def.usage as libc::c_uint
    }) as sg_usage;
    return def;
}
unsafe extern "C" fn _sg_init_buffer(
    mut buf_id: sg_buffer,
    mut desc: *const sg_buffer_desc,
) {
    let mut buf: *mut _sg_buffer_t = _sg_lookup_buffer(&mut _sg.pools, buf_id.id);
    (*buf).slot.ctx_id = _sg.active_context.id;
    if _sg_validate_buffer_desc(desc) {
        (*buf).slot.state = _sg_create_buffer(buf, desc);
    } else {
        (*buf).slot.state = SG_RESOURCESTATE_FAILED;
    };
}
unsafe extern "C" fn _sg_lookup_buffer(
    mut p: *const _sg_pools_t,
    mut buf_id: uint32_t,
) -> *mut _sg_buffer_t {
    if SG_INVALID_ID as libc::c_int as libc::c_uint != buf_id {
        let mut buf: *mut _sg_buffer_t = _sg_buffer_at(p, buf_id);
        if (*buf).slot.id == buf_id {
            return buf;
        }
    }
    return 0 as *mut _sg_buffer_t;
}
unsafe extern "C" fn _sg_buffer_at(
    mut p: *const _sg_pools_t,
    mut buf_id: uint32_t,
) -> *mut _sg_buffer_t {
    let mut slot_index: libc::c_int = _sg_slot_index(buf_id);
    return &mut *((*p).buffers).offset(slot_index as isize) as *mut _sg_buffer_t;
}
unsafe extern "C" fn _sg_create_buffer(
    mut buf: *mut _sg_buffer_t,
    mut desc: *const sg_buffer_desc,
) -> sg_resource_state {
    (*buf).size = (*desc).size;
    (*buf).append_pos = 0 as libc::c_int;
    (*buf).append_overflow = 0 as libc::c_int != 0;
    (*buf).type_0 = (*desc).type_0;
    (*buf).usage = (*desc).usage;
    (*buf).update_frame_index = 0 as libc::c_int as uint32_t;
    (*buf).append_frame_index = 0 as libc::c_int as uint32_t;
    (*buf)
        .num_slots = if (*buf).usage as libc::c_uint
        == SG_USAGE_IMMUTABLE as libc::c_int as libc::c_uint
    {
        1 as libc::c_int
    } else {
        SG_NUM_INFLIGHT_FRAMES as libc::c_int
    };
    (*buf).active_slot = 0 as libc::c_int;
    (*buf)
        .ext_buffers = 0 as libc::c_int as libc::c_uint
        != (*desc).gl_buffers[0 as libc::c_int as usize];
    let mut gl_target: GLenum = _sg_gl_buffer_target((*buf).type_0);
    let mut gl_usage: GLenum = _sg_gl_usage((*buf).usage);
    let mut slot: libc::c_int = 0 as libc::c_int;
    while slot < (*buf).num_slots {
        let mut gl_buf: GLuint = 0 as libc::c_int as GLuint;
        if (*buf).ext_buffers {
            gl_buf = (*desc).gl_buffers[slot as usize];
        } else {
            glGenBuffers(1 as libc::c_int, &mut gl_buf);
            _sg_gl_store_buffer_binding(gl_target);
            _sg_gl_bind_buffer(gl_target, gl_buf);
            glBufferData(
                gl_target,
                (*buf).size as GLsizeiptr,
                0 as *const libc::c_void,
                gl_usage,
            );
            if (*buf).usage as libc::c_uint
                == SG_USAGE_IMMUTABLE as libc::c_int as libc::c_uint
            {
                glBufferSubData(
                    gl_target,
                    0 as libc::c_int as GLintptr,
                    (*buf).size as GLsizeiptr,
                    (*desc).content,
                );
            }
            _sg_gl_restore_buffer_binding(gl_target);
        }
        (*buf).gl_buf[slot as usize] = gl_buf;
        slot += 1;
        slot;
    }
    return SG_RESOURCESTATE_VALID;
}
unsafe extern "C" fn _sg_gl_buffer_target(mut t: sg_buffer_type) -> GLenum {
    match t as libc::c_uint {
        1 => return 0x8892 as libc::c_int as GLenum,
        2 => return 0x8893 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_restore_buffer_binding(mut target: GLenum) {
    if target == 0x8892 as libc::c_int as libc::c_uint {
        _sg_gl_bind_buffer(target, _sg.gl.cache.stored_vertex_buffer);
    } else {
        _sg_gl_bind_buffer(target, _sg.gl.cache.stored_index_buffer);
    };
}
unsafe extern "C" fn _sg_gl_bind_buffer(mut target: GLenum, mut buffer: GLuint) {
    if target == 0x8892 as libc::c_int as libc::c_uint {
        if _sg.gl.cache.vertex_buffer != buffer {
            _sg.gl.cache.vertex_buffer = buffer;
            glBindBuffer(target, buffer);
        }
    } else if _sg.gl.cache.index_buffer != buffer {
        _sg.gl.cache.index_buffer = buffer;
        glBindBuffer(target, buffer);
    }
}
unsafe extern "C" fn _sg_gl_usage(mut u: sg_usage) -> GLenum {
    match u as libc::c_uint {
        1 => return 0x88e4 as libc::c_int as GLenum,
        2 => return 0x88e8 as libc::c_int as GLenum,
        3 => return 0x88e0 as libc::c_int as GLenum,
        _ => return 0 as libc::c_int as GLenum,
    };
}
unsafe extern "C" fn _sg_gl_store_buffer_binding(mut target: GLenum) {
    if target == 0x8892 as libc::c_int as libc::c_uint {
        _sg.gl.cache.stored_vertex_buffer = _sg.gl.cache.vertex_buffer;
    } else {
        _sg.gl.cache.stored_index_buffer = _sg.gl.cache.index_buffer;
    };
}
unsafe extern "C" fn _sg_validate_buffer_desc(mut desc: *const sg_buffer_desc) -> bool {
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn sg_make_image(mut desc: *const sg_image_desc) -> sg_image {
    let mut desc_def: sg_image_desc = _sg_image_desc_defaults(desc);
    let mut img_id: sg_image = _sg_alloc_image();
    if img_id.id != SG_INVALID_ID as libc::c_int as libc::c_uint {
        _sg_init_image(img_id, &mut desc_def);
    }
    return img_id;
}
unsafe extern "C" fn _sg_alloc_image() -> sg_image {
    let mut res: sg_image = sg_image { id: 0 };
    let mut slot_index: libc::c_int = _sg_pool_alloc_index(&mut _sg.pools.image_pool);
    if 0 as libc::c_int != slot_index {
        res
            .id = _sg_slot_alloc(
            &mut _sg.pools.image_pool,
            &mut (*(_sg.pools.images).offset(slot_index as isize)).slot,
            slot_index,
        );
    } else {
        res.id = SG_INVALID_ID as libc::c_int as uint32_t;
    }
    return res;
}
unsafe extern "C" fn _sg_image_desc_defaults(
    mut desc: *const sg_image_desc,
) -> sg_image_desc {
    let mut def: sg_image_desc = *desc;
    def
        .type_0 = (if def.type_0 as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        SG_IMAGETYPE_2D as libc::c_int as libc::c_uint
    } else {
        def.type_0 as libc::c_uint
    }) as sg_image_type;
    def
        .c2rust_unnamed
        .depth = if def.c2rust_unnamed.depth == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        def.c2rust_unnamed.depth
    };
    def
        .num_mipmaps = if def.num_mipmaps == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        def.num_mipmaps
    };
    def
        .usage = (if def.usage as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        SG_USAGE_IMMUTABLE as libc::c_int as libc::c_uint
    } else {
        def.usage as libc::c_uint
    }) as sg_usage;
    def
        .pixel_format = (if def.pixel_format as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_PIXELFORMAT_RGBA8 as libc::c_int as libc::c_uint
    } else {
        def.pixel_format as libc::c_uint
    }) as sg_pixel_format;
    def
        .sample_count = if def.sample_count == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        def.sample_count
    };
    def
        .min_filter = (if def.min_filter as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_FILTER_NEAREST as libc::c_int as libc::c_uint
    } else {
        def.min_filter as libc::c_uint
    }) as sg_filter;
    def
        .mag_filter = (if def.mag_filter as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        SG_FILTER_NEAREST as libc::c_int as libc::c_uint
    } else {
        def.mag_filter as libc::c_uint
    }) as sg_filter;
    def
        .wrap_u = (if def.wrap_u as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        SG_WRAP_REPEAT as libc::c_int as libc::c_uint
    } else {
        def.wrap_u as libc::c_uint
    }) as sg_wrap;
    def
        .wrap_v = (if def.wrap_v as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        SG_WRAP_REPEAT as libc::c_int as libc::c_uint
    } else {
        def.wrap_v as libc::c_uint
    }) as sg_wrap;
    def
        .wrap_w = (if def.wrap_w as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        SG_WRAP_REPEAT as libc::c_int as libc::c_uint
    } else {
        def.wrap_w as libc::c_uint
    }) as sg_wrap;
    def
        .max_anisotropy = if def.max_anisotropy == 0 as libc::c_int as libc::c_uint {
        1 as libc::c_int as libc::c_uint
    } else {
        def.max_anisotropy
    };
    def.max_lod = if def.max_lod == 0.0f32 { 3.40282347e+38f32 } else { def.max_lod };
    return def;
}
unsafe extern "C" fn _sg_init_image(
    mut img_id: sg_image,
    mut desc: *const sg_image_desc,
) {
    let mut img: *mut _sg_image_t = _sg_lookup_image(&mut _sg.pools, img_id.id);
    (*img).slot.ctx_id = _sg.active_context.id;
    if _sg_validate_image_desc(desc) {
        (*img).slot.state = _sg_create_image(img, desc);
    } else {
        (*img).slot.state = SG_RESOURCESTATE_FAILED;
    };
}
unsafe extern "C" fn _sg_lookup_image(
    mut p: *const _sg_pools_t,
    mut img_id: uint32_t,
) -> *mut _sg_image_t {
    if SG_INVALID_ID as libc::c_int as libc::c_uint != img_id {
        let mut img: *mut _sg_image_t = _sg_image_at(p, img_id);
        if (*img).slot.id == img_id {
            return img;
        }
    }
    return 0 as *mut _sg_image_t;
}
unsafe extern "C" fn _sg_image_at(
    mut p: *const _sg_pools_t,
    mut img_id: uint32_t,
) -> *mut _sg_image_t {
    let mut slot_index: libc::c_int = _sg_slot_index(img_id);
    return &mut *((*p).images).offset(slot_index as isize) as *mut _sg_image_t;
}
unsafe extern "C" fn _sg_create_image(
    mut img: *mut _sg_image_t,
    mut desc: *const sg_image_desc,
) -> sg_resource_state {
    (*img).type_0 = (*desc).type_0;
    (*img).render_target = (*desc).render_target;
    (*img).width = (*desc).width;
    (*img).height = (*desc).height;
    (*img).depth = (*desc).c2rust_unnamed.depth;
    (*img).num_mipmaps = (*desc).num_mipmaps;
    (*img).usage = (*desc).usage;
    (*img).pixel_format = (*desc).pixel_format;
    (*img).sample_count = (*desc).sample_count;
    (*img).min_filter = (*desc).min_filter;
    (*img).mag_filter = (*desc).mag_filter;
    (*img).wrap_u = (*desc).wrap_u;
    (*img).wrap_v = (*desc).wrap_v;
    (*img).wrap_w = (*desc).wrap_w;
    (*img).max_anisotropy = (*desc).max_anisotropy;
    (*img).upd_frame_index = 0 as libc::c_int as uint32_t;
    if !_sg_gl_supported_texture_format((*img).pixel_format) {
        return SG_RESOURCESTATE_FAILED;
    }
    if (*img).type_0 as libc::c_uint == SG_IMAGETYPE_3D as libc::c_int as libc::c_uint
        && !_sg.gl.features[SG_FEATURE_IMAGETYPE_3D as libc::c_int as usize]
    {
        return SG_RESOURCESTATE_FAILED;
    }
    if (*img).type_0 as libc::c_uint == SG_IMAGETYPE_ARRAY as libc::c_int as libc::c_uint
        && !_sg.gl.features[SG_FEATURE_IMAGETYPE_ARRAY as libc::c_int as usize]
    {
        return SG_RESOURCESTATE_FAILED;
    }
    (*img)
        .num_slots = if (*img).usage as libc::c_uint
        == SG_USAGE_IMMUTABLE as libc::c_int as libc::c_uint
    {
        1 as libc::c_int
    } else {
        SG_NUM_INFLIGHT_FRAMES as libc::c_int
    };
    (*img).active_slot = 0 as libc::c_int;
    (*img)
        .ext_textures = 0 as libc::c_int as libc::c_uint
        != (*desc).gl_textures[0 as libc::c_int as usize];
    let mut msaa: bool = 0 as libc::c_int != 0;
    if !_sg.gl.gles2 {
        msaa = (*img).sample_count > 1 as libc::c_int
            && _sg.gl.features[SG_FEATURE_MSAA_RENDER_TARGETS as libc::c_int as usize]
                as libc::c_int != 0;
    }
    if _sg_is_valid_rendertarget_depth_format((*img).pixel_format) {
        glGenRenderbuffers(1 as libc::c_int, &mut (*img).gl_depth_render_buffer);
        glBindRenderbuffer(
            0x8d41 as libc::c_int as GLenum,
            (*img).gl_depth_render_buffer,
        );
        let mut gl_depth_format: GLenum = _sg_gl_depth_attachment_format(
            (*img).pixel_format,
        );
        if !_sg.gl.gles2 && msaa as libc::c_int != 0 {
            glRenderbufferStorageMultisample(
                0x8d41 as libc::c_int as GLenum,
                (*img).sample_count,
                gl_depth_format,
                (*img).width,
                (*img).height,
            );
        } else {
            glRenderbufferStorage(
                0x8d41 as libc::c_int as GLenum,
                gl_depth_format,
                (*img).width,
                (*img).height,
            );
        }
    } else {
        (*img).gl_target = _sg_gl_texture_target((*img).type_0);
        let gl_internal_format: GLenum = _sg_gl_teximage_internal_format(
            (*img).pixel_format,
        );
        if !_sg.gl.gles2 && (*img).render_target as libc::c_int != 0
            && msaa as libc::c_int != 0
        {
            glGenRenderbuffers(1 as libc::c_int, &mut (*img).gl_msaa_render_buffer);
            glBindRenderbuffer(
                0x8d41 as libc::c_int as GLenum,
                (*img).gl_msaa_render_buffer,
            );
            glRenderbufferStorageMultisample(
                0x8d41 as libc::c_int as GLenum,
                (*img).sample_count,
                gl_internal_format,
                (*img).width,
                (*img).height,
            );
        }
        if (*img).ext_textures {
            let mut slot: libc::c_int = 0 as libc::c_int;
            while slot < (*img).num_slots {
                (*img).gl_tex[slot as usize] = (*desc).gl_textures[slot as usize];
                slot += 1;
                slot;
            }
        } else {
            let gl_format: GLenum = _sg_gl_teximage_format((*img).pixel_format);
            let is_compressed: bool = _sg_is_compressed_pixel_format(
                (*img).pixel_format,
            );
            let mut slot_0: libc::c_int = 0 as libc::c_int;
            while slot_0 < (*img).num_slots {
                glGenTextures(
                    1 as libc::c_int,
                    &mut *((*img).gl_tex).as_mut_ptr().offset(slot_0 as isize),
                );
                _sg_gl_store_texture_binding(0 as libc::c_int);
                _sg_gl_bind_texture(
                    0 as libc::c_int,
                    (*img).gl_target,
                    (*img).gl_tex[slot_0 as usize],
                );
                let mut gl_min_filter: GLenum = _sg_gl_filter((*img).min_filter);
                let mut gl_mag_filter: GLenum = _sg_gl_filter((*img).mag_filter);
                glTexParameteri(
                    (*img).gl_target,
                    0x2801 as libc::c_int as GLenum,
                    gl_min_filter as GLint,
                );
                glTexParameteri(
                    (*img).gl_target,
                    0x2800 as libc::c_int as GLenum,
                    gl_mag_filter as GLint,
                );
                if _sg.gl.ext_anisotropic as libc::c_int != 0
                    && (*img).max_anisotropy > 1 as libc::c_int as libc::c_uint
                {
                    let mut max_aniso: GLint = (*img).max_anisotropy as GLint;
                    if max_aniso > _sg.gl.max_anisotropy {
                        max_aniso = _sg.gl.max_anisotropy;
                    }
                    glTexParameteri(
                        (*img).gl_target,
                        0x84fe as libc::c_int as GLenum,
                        max_aniso,
                    );
                }
                if (*img).type_0 as libc::c_uint
                    == SG_IMAGETYPE_CUBE as libc::c_int as libc::c_uint
                {
                    glTexParameteri(
                        (*img).gl_target,
                        0x2802 as libc::c_int as GLenum,
                        0x812f as libc::c_int,
                    );
                    glTexParameteri(
                        (*img).gl_target,
                        0x2803 as libc::c_int as GLenum,
                        0x812f as libc::c_int,
                    );
                } else {
                    glTexParameteri(
                        (*img).gl_target,
                        0x2802 as libc::c_int as GLenum,
                        _sg_gl_wrap((*img).wrap_u) as GLint,
                    );
                    glTexParameteri(
                        (*img).gl_target,
                        0x2803 as libc::c_int as GLenum,
                        _sg_gl_wrap((*img).wrap_v) as GLint,
                    );
                    if !_sg.gl.gles2
                        && (*img).type_0 as libc::c_uint
                            == SG_IMAGETYPE_3D as libc::c_int as libc::c_uint
                    {
                        glTexParameteri(
                            (*img).gl_target,
                            0x8072 as libc::c_int as GLenum,
                            _sg_gl_wrap((*img).wrap_w) as GLint,
                        );
                    }
                }
                if !_sg.gl.gles2 {
                    let min_lod: libc::c_float = if (*desc).min_lod < 0.0f32 {
                        0.0f32
                    } else if (*desc).min_lod > 1000.0f32 {
                        1000.0f32
                    } else {
                        (*desc).min_lod
                    };
                    let max_lod: libc::c_float = if (*desc).max_lod < 0.0f32 {
                        0.0f32
                    } else if (*desc).max_lod > 1000.0f32 {
                        1000.0f32
                    } else {
                        (*desc).max_lod
                    };
                    glTexParameterf(
                        (*img).gl_target,
                        0x813a as libc::c_int as GLenum,
                        min_lod,
                    );
                    glTexParameterf(
                        (*img).gl_target,
                        0x813b as libc::c_int as GLenum,
                        max_lod,
                    );
                }
                let num_faces: libc::c_int = if (*img).type_0 as libc::c_uint
                    == SG_IMAGETYPE_CUBE as libc::c_int as libc::c_uint
                {
                    6 as libc::c_int
                } else {
                    1 as libc::c_int
                };
                let mut data_index: libc::c_int = 0 as libc::c_int;
                let mut face_index: libc::c_int = 0 as libc::c_int;
                while face_index < num_faces {
                    let mut mip_index: libc::c_int = 0 as libc::c_int;
                    while mip_index < (*img).num_mipmaps {
                        let mut gl_img_target: GLenum = (*img).gl_target;
                        if SG_IMAGETYPE_CUBE as libc::c_int as libc::c_uint
                            == (*img).type_0 as libc::c_uint
                        {
                            gl_img_target = _sg_gl_cubeface_target(face_index);
                        }
                        let mut data_ptr: *const libc::c_void = (*desc)
                            .content
                            .subimage[face_index as usize][mip_index as usize]
                            .ptr;
                        let data_size: libc::c_int = (*desc)
                            .content
                            .subimage[face_index as usize][mip_index as usize]
                            .size;
                        let mut mip_width: libc::c_int = (*img).width >> mip_index;
                        if mip_width == 0 as libc::c_int {
                            mip_width = 1 as libc::c_int;
                        }
                        let mut mip_height: libc::c_int = (*img).height >> mip_index;
                        if mip_height == 0 as libc::c_int {
                            mip_height = 1 as libc::c_int;
                        }
                        if SG_IMAGETYPE_2D as libc::c_int as libc::c_uint
                            == (*img).type_0 as libc::c_uint
                            || SG_IMAGETYPE_CUBE as libc::c_int as libc::c_uint
                                == (*img).type_0 as libc::c_uint
                        {
                            if is_compressed {
                                glCompressedTexImage2D(
                                    gl_img_target,
                                    mip_index,
                                    gl_internal_format,
                                    mip_width,
                                    mip_height,
                                    0 as libc::c_int,
                                    data_size,
                                    data_ptr,
                                );
                            } else {
                                let gl_type: GLenum = _sg_gl_teximage_type(
                                    (*img).pixel_format,
                                );
                                glTexImage2D(
                                    gl_img_target,
                                    mip_index,
                                    gl_internal_format as GLint,
                                    mip_width,
                                    mip_height,
                                    0 as libc::c_int,
                                    gl_format,
                                    gl_type,
                                    data_ptr,
                                );
                            }
                        } else if !_sg.gl.gles2
                            && (SG_IMAGETYPE_3D as libc::c_int as libc::c_uint
                                == (*img).type_0 as libc::c_uint
                                || SG_IMAGETYPE_ARRAY as libc::c_int as libc::c_uint
                                    == (*img).type_0 as libc::c_uint)
                        {
                            let mut mip_depth: libc::c_int = (*img).depth;
                            if SG_IMAGETYPE_3D as libc::c_int as libc::c_uint
                                == (*img).type_0 as libc::c_uint
                            {
                                mip_depth >>= mip_index;
                            }
                            if mip_depth == 0 as libc::c_int {
                                mip_depth = 1 as libc::c_int;
                            }
                            if is_compressed {
                                glCompressedTexImage3D(
                                    gl_img_target,
                                    mip_index,
                                    gl_internal_format,
                                    mip_width,
                                    mip_height,
                                    mip_depth,
                                    0 as libc::c_int,
                                    data_size,
                                    data_ptr,
                                );
                            } else {
                                let gl_type_0: GLenum = _sg_gl_teximage_type(
                                    (*img).pixel_format,
                                );
                                glTexImage3D(
                                    gl_img_target,
                                    mip_index,
                                    gl_internal_format as GLint,
                                    mip_width,
                                    mip_height,
                                    mip_depth,
                                    0 as libc::c_int,
                                    gl_format,
                                    gl_type_0,
                                    data_ptr,
                                );
                            }
                        }
                        mip_index += 1;
                        mip_index;
                        data_index += 1;
                        data_index;
                    }
                    face_index += 1;
                    face_index;
                }
                _sg_gl_restore_texture_binding(0 as libc::c_int);
                slot_0 += 1;
                slot_0;
            }
        }
    }
    return SG_RESOURCESTATE_VALID;
}
unsafe extern "C" fn _sg_gl_restore_texture_binding(mut slot_index: libc::c_int) {
    let mut slot: *const _sg_gl_texture_bind_slot = &mut _sg.gl.cache.stored_texture;
    _sg_gl_bind_texture(slot_index, (*slot).target, (*slot).texture);
}
unsafe extern "C" fn _sg_gl_bind_texture(
    mut slot_index: libc::c_int,
    mut target: GLenum,
    mut texture: GLuint,
) {
    if slot_index >= _sg.gl.max_combined_texture_image_units {
        return;
    }
    let mut slot: *mut _sg_gl_texture_bind_slot = &mut *(_sg.gl.cache.textures)
        .as_mut_ptr()
        .offset(slot_index as isize) as *mut _sg_gl_texture_bind_slot;
    if (*slot).target != target || (*slot).texture != texture {
        glActiveTexture((0x84c0 as libc::c_int + slot_index) as GLenum);
        if target != (*slot).target && (*slot).target != 0 as libc::c_int as libc::c_uint
        {
            glBindTexture((*slot).target, 0 as libc::c_int as GLuint);
        }
        if target != 0 as libc::c_int as libc::c_uint {
            glBindTexture(target, texture);
        }
        (*slot).target = target;
        (*slot).texture = texture;
    }
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
