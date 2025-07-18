use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn InitWindow(width: libc::c_int, height: libc::c_int, title: *const libc::c_char);
    fn CloseWindow();
    fn WindowShouldClose() -> bool;
    fn GetScreenWidth() -> libc::c_int;
    fn ClearBackground(color: Color);
    fn BeginDrawing();
    fn EndDrawing();
    fn SetTargetFPS(fps: libc::c_int);
    fn TraceLog(logLevel: libc::c_int, text: *const libc::c_char, _: ...);
    fn LoadFileText(fileName: *const libc::c_char) -> *mut libc::c_char;
    fn UnloadFileText(text: *mut libc::c_char);
    fn IsFileExtension(fileName: *const libc::c_char, ext: *const libc::c_char) -> bool;
    fn GetFileName(filePath: *const libc::c_char) -> *const libc::c_char;
    fn GetDirectoryPath(filePath: *const libc::c_char) -> *const libc::c_char;
    fn IsFileDropped() -> bool;
    fn LoadDroppedFiles() -> FilePathList;
    fn UnloadDroppedFiles(files: FilePathList);
    fn DecompressData(
        compData: *const libc::c_uchar,
        compDataSize: libc::c_int,
        dataSize: *mut libc::c_int,
    ) -> *mut libc::c_uchar;
    fn IsKeyPressed(key: libc::c_int) -> bool;
    fn IsKeyDown(key: libc::c_int) -> bool;
    fn GetCharPressed() -> libc::c_int;
    fn IsMouseButtonPressed(button: libc::c_int) -> bool;
    fn IsMouseButtonDown(button: libc::c_int) -> bool;
    fn IsMouseButtonReleased(button: libc::c_int) -> bool;
    fn GetMousePosition() -> Vector2;
    fn GetMouseWheelMove() -> libc::c_float;
    fn SetShapesTexture(texture: Texture2D, source: Rectangle);
    fn DrawRectangle(
        posX: libc::c_int,
        posY: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        color: Color,
    );
    fn DrawRectangleGradientV(
        posX: libc::c_int,
        posY: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        color1: Color,
        color2: Color,
    );
    fn DrawRectangleGradientEx(
        rec: Rectangle,
        col1: Color,
        col2: Color,
        col3: Color,
        col4: Color,
    );
    fn CheckCollisionPointRec(point: Vector2, rec: Rectangle) -> bool;
    fn LoadImageRaw(
        fileName: *const libc::c_char,
        width: libc::c_int,
        height: libc::c_int,
        format: libc::c_int,
        headerSize: libc::c_int,
    ) -> Image;
    fn UnloadImage(image: Image);
    fn LoadTextureFromImage(image: Image) -> Texture2D;
    fn UnloadTexture(texture: Texture2D);
    fn DrawTextureEx(
        texture: Texture2D,
        position: Vector2,
        rotation: libc::c_float,
        scale: libc::c_float,
        tint: Color,
    );
    fn Fade(color: Color, alpha: libc::c_float) -> Color;
    fn GetColor(hexValue: libc::c_uint) -> Color;
    fn GetFontDefault() -> Font;
    fn LoadFontEx(
        fileName: *const libc::c_char,
        fontSize: libc::c_int,
        codepoints: *mut libc::c_int,
        codepointCount: libc::c_int,
    ) -> Font;
    fn DrawText(
        text: *const libc::c_char,
        posX: libc::c_int,
        posY: libc::c_int,
        fontSize: libc::c_int,
        color: Color,
    );
    fn DrawTextCodepoint(
        font: Font,
        codepoint: libc::c_int,
        position: Vector2,
        fontSize: libc::c_float,
        tint: Color,
    );
    fn MeasureTextEx(
        font: Font,
        text: *const libc::c_char,
        fontSize: libc::c_float,
        spacing: libc::c_float,
    ) -> Vector2;
    fn GetGlyphIndex(font: Font, codepoint: libc::c_int) -> libc::c_int;
    fn LoadCodepoints(
        text: *const libc::c_char,
        count: *mut libc::c_int,
    ) -> *mut libc::c_int;
    fn UnloadCodepoints(codepoints: *mut libc::c_int);
    fn GetCodepoint(
        text: *const libc::c_char,
        codepointSize: *mut libc::c_int,
    ) -> libc::c_int;
    fn GetCodepointNext(
        text: *const libc::c_char,
        codepointSize: *mut libc::c_int,
    ) -> libc::c_int;
    fn GetCodepointPrevious(
        text: *const libc::c_char,
        codepointSize: *mut libc::c_int,
    ) -> libc::c_int;
    fn CodepointToUTF8(
        codepoint: libc::c_int,
        utf8Size: *mut libc::c_int,
    ) -> *const libc::c_char;
    fn TextFormat(text: *const libc::c_char, _: ...) -> *const libc::c_char;
    fn TextJoin(
        textList: *mut *const libc::c_char,
        count: libc::c_int,
        delimiter: *const libc::c_char,
    ) -> *const libc::c_char;
    fn TextToInteger(text: *const libc::c_char) -> libc::c_int;
    fn TextToFloat(text: *const libc::c_char) -> libc::c_float;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn floorf(_: libc::c_float) -> libc::c_float;
    fn ceilf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn roundf(_: libc::c_float) -> libc::c_float;
    fn round(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector2 {
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector3 {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: libc::c_uchar,
    pub g: libc::c_uchar,
    pub b: libc::c_uchar,
    pub a: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rectangle {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub width: libc::c_float,
    pub height: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Image {
    pub data: *mut libc::c_void,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mipmaps: libc::c_int,
    pub format: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Texture {
    pub id: libc::c_uint,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mipmaps: libc::c_int,
    pub format: libc::c_int,
}
pub type Texture2D = Texture;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphInfo {
    pub value: libc::c_int,
    pub offsetX: libc::c_int,
    pub offsetY: libc::c_int,
    pub advanceX: libc::c_int,
    pub image: Image,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub baseSize: libc::c_int,
    pub glyphCount: libc::c_int,
    pub glyphPadding: libc::c_int,
    pub texture: Texture2D,
    pub recs: *mut Rectangle,
    pub glyphs: *mut GlyphInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FilePathList {
    pub capacity: libc::c_uint,
    pub count: libc::c_uint,
    pub paths: *mut *mut libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const LOG_NONE: C2RustUnnamed = 7;
pub const LOG_FATAL: C2RustUnnamed = 6;
pub const LOG_ERROR: C2RustUnnamed = 5;
pub const LOG_WARNING: C2RustUnnamed = 4;
pub const LOG_INFO: C2RustUnnamed = 3;
pub const LOG_DEBUG: C2RustUnnamed = 2;
pub const LOG_TRACE: C2RustUnnamed = 1;
pub const LOG_ALL: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const KEY_VOLUME_DOWN: C2RustUnnamed_0 = 25;
pub const KEY_VOLUME_UP: C2RustUnnamed_0 = 24;
pub const KEY_MENU: C2RustUnnamed_0 = 5;
pub const KEY_BACK: C2RustUnnamed_0 = 4;
pub const KEY_KP_EQUAL: C2RustUnnamed_0 = 336;
pub const KEY_KP_ENTER: C2RustUnnamed_0 = 335;
pub const KEY_KP_ADD: C2RustUnnamed_0 = 334;
pub const KEY_KP_SUBTRACT: C2RustUnnamed_0 = 333;
pub const KEY_KP_MULTIPLY: C2RustUnnamed_0 = 332;
pub const KEY_KP_DIVIDE: C2RustUnnamed_0 = 331;
pub const KEY_KP_DECIMAL: C2RustUnnamed_0 = 330;
pub const KEY_KP_9: C2RustUnnamed_0 = 329;
pub const KEY_KP_8: C2RustUnnamed_0 = 328;
pub const KEY_KP_7: C2RustUnnamed_0 = 327;
pub const KEY_KP_6: C2RustUnnamed_0 = 326;
pub const KEY_KP_5: C2RustUnnamed_0 = 325;
pub const KEY_KP_4: C2RustUnnamed_0 = 324;
pub const KEY_KP_3: C2RustUnnamed_0 = 323;
pub const KEY_KP_2: C2RustUnnamed_0 = 322;
pub const KEY_KP_1: C2RustUnnamed_0 = 321;
pub const KEY_KP_0: C2RustUnnamed_0 = 320;
pub const KEY_KB_MENU: C2RustUnnamed_0 = 348;
pub const KEY_RIGHT_SUPER: C2RustUnnamed_0 = 347;
pub const KEY_RIGHT_ALT: C2RustUnnamed_0 = 346;
pub const KEY_RIGHT_CONTROL: C2RustUnnamed_0 = 345;
pub const KEY_RIGHT_SHIFT: C2RustUnnamed_0 = 344;
pub const KEY_LEFT_SUPER: C2RustUnnamed_0 = 343;
pub const KEY_LEFT_ALT: C2RustUnnamed_0 = 342;
pub const KEY_LEFT_CONTROL: C2RustUnnamed_0 = 341;
pub const KEY_LEFT_SHIFT: C2RustUnnamed_0 = 340;
pub const KEY_F12: C2RustUnnamed_0 = 301;
pub const KEY_F11: C2RustUnnamed_0 = 300;
pub const KEY_F10: C2RustUnnamed_0 = 299;
pub const KEY_F9: C2RustUnnamed_0 = 298;
pub const KEY_F8: C2RustUnnamed_0 = 297;
pub const KEY_F7: C2RustUnnamed_0 = 296;
pub const KEY_F6: C2RustUnnamed_0 = 295;
pub const KEY_F5: C2RustUnnamed_0 = 294;
pub const KEY_F4: C2RustUnnamed_0 = 293;
pub const KEY_F3: C2RustUnnamed_0 = 292;
pub const KEY_F2: C2RustUnnamed_0 = 291;
pub const KEY_F1: C2RustUnnamed_0 = 290;
pub const KEY_PAUSE: C2RustUnnamed_0 = 284;
pub const KEY_PRINT_SCREEN: C2RustUnnamed_0 = 283;
pub const KEY_NUM_LOCK: C2RustUnnamed_0 = 282;
pub const KEY_SCROLL_LOCK: C2RustUnnamed_0 = 281;
pub const KEY_CAPS_LOCK: C2RustUnnamed_0 = 280;
pub const KEY_END: C2RustUnnamed_0 = 269;
pub const KEY_HOME: C2RustUnnamed_0 = 268;
pub const KEY_PAGE_DOWN: C2RustUnnamed_0 = 267;
pub const KEY_PAGE_UP: C2RustUnnamed_0 = 266;
pub const KEY_UP: C2RustUnnamed_0 = 265;
pub const KEY_DOWN: C2RustUnnamed_0 = 264;
pub const KEY_LEFT: C2RustUnnamed_0 = 263;
pub const KEY_RIGHT: C2RustUnnamed_0 = 262;
pub const KEY_DELETE: C2RustUnnamed_0 = 261;
pub const KEY_INSERT: C2RustUnnamed_0 = 260;
pub const KEY_BACKSPACE: C2RustUnnamed_0 = 259;
pub const KEY_TAB: C2RustUnnamed_0 = 258;
pub const KEY_ENTER: C2RustUnnamed_0 = 257;
pub const KEY_ESCAPE: C2RustUnnamed_0 = 256;
pub const KEY_SPACE: C2RustUnnamed_0 = 32;
pub const KEY_GRAVE: C2RustUnnamed_0 = 96;
pub const KEY_RIGHT_BRACKET: C2RustUnnamed_0 = 93;
pub const KEY_BACKSLASH: C2RustUnnamed_0 = 92;
pub const KEY_LEFT_BRACKET: C2RustUnnamed_0 = 91;
pub const KEY_Z: C2RustUnnamed_0 = 90;
pub const KEY_Y: C2RustUnnamed_0 = 89;
pub const KEY_X: C2RustUnnamed_0 = 88;
pub const KEY_W: C2RustUnnamed_0 = 87;
pub const KEY_V: C2RustUnnamed_0 = 86;
pub const KEY_U: C2RustUnnamed_0 = 85;
pub const KEY_T: C2RustUnnamed_0 = 84;
pub const KEY_S: C2RustUnnamed_0 = 83;
pub const KEY_R: C2RustUnnamed_0 = 82;
pub const KEY_Q: C2RustUnnamed_0 = 81;
pub const KEY_P: C2RustUnnamed_0 = 80;
pub const KEY_O: C2RustUnnamed_0 = 79;
pub const KEY_N: C2RustUnnamed_0 = 78;
pub const KEY_M: C2RustUnnamed_0 = 77;
pub const KEY_L: C2RustUnnamed_0 = 76;
pub const KEY_K: C2RustUnnamed_0 = 75;
pub const KEY_J: C2RustUnnamed_0 = 74;
pub const KEY_I: C2RustUnnamed_0 = 73;
pub const KEY_H: C2RustUnnamed_0 = 72;
pub const KEY_G: C2RustUnnamed_0 = 71;
pub const KEY_F: C2RustUnnamed_0 = 70;
pub const KEY_E: C2RustUnnamed_0 = 69;
pub const KEY_D: C2RustUnnamed_0 = 68;
pub const KEY_C: C2RustUnnamed_0 = 67;
pub const KEY_B: C2RustUnnamed_0 = 66;
pub const KEY_A: C2RustUnnamed_0 = 65;
pub const KEY_EQUAL: C2RustUnnamed_0 = 61;
pub const KEY_SEMICOLON: C2RustUnnamed_0 = 59;
pub const KEY_NINE: C2RustUnnamed_0 = 57;
pub const KEY_EIGHT: C2RustUnnamed_0 = 56;
pub const KEY_SEVEN: C2RustUnnamed_0 = 55;
pub const KEY_SIX: C2RustUnnamed_0 = 54;
pub const KEY_FIVE: C2RustUnnamed_0 = 53;
pub const KEY_FOUR: C2RustUnnamed_0 = 52;
pub const KEY_THREE: C2RustUnnamed_0 = 51;
pub const KEY_TWO: C2RustUnnamed_0 = 50;
pub const KEY_ONE: C2RustUnnamed_0 = 49;
pub const KEY_ZERO: C2RustUnnamed_0 = 48;
pub const KEY_SLASH: C2RustUnnamed_0 = 47;
pub const KEY_PERIOD: C2RustUnnamed_0 = 46;
pub const KEY_MINUS: C2RustUnnamed_0 = 45;
pub const KEY_COMMA: C2RustUnnamed_0 = 44;
pub const KEY_APOSTROPHE: C2RustUnnamed_0 = 39;
pub const KEY_NULL: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MOUSE_BUTTON_BACK: C2RustUnnamed_1 = 6;
pub const MOUSE_BUTTON_FORWARD: C2RustUnnamed_1 = 5;
pub const MOUSE_BUTTON_EXTRA: C2RustUnnamed_1 = 4;
pub const MOUSE_BUTTON_SIDE: C2RustUnnamed_1 = 3;
pub const MOUSE_BUTTON_MIDDLE: C2RustUnnamed_1 = 2;
pub const MOUSE_BUTTON_RIGHT: C2RustUnnamed_1 = 1;
pub const MOUSE_BUTTON_LEFT: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA: C2RustUnnamed_2 = 24;
pub const PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA: C2RustUnnamed_2 = 23;
pub const PIXELFORMAT_COMPRESSED_PVRT_RGBA: C2RustUnnamed_2 = 22;
pub const PIXELFORMAT_COMPRESSED_PVRT_RGB: C2RustUnnamed_2 = 21;
pub const PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA: C2RustUnnamed_2 = 20;
pub const PIXELFORMAT_COMPRESSED_ETC2_RGB: C2RustUnnamed_2 = 19;
pub const PIXELFORMAT_COMPRESSED_ETC1_RGB: C2RustUnnamed_2 = 18;
pub const PIXELFORMAT_COMPRESSED_DXT5_RGBA: C2RustUnnamed_2 = 17;
pub const PIXELFORMAT_COMPRESSED_DXT3_RGBA: C2RustUnnamed_2 = 16;
pub const PIXELFORMAT_COMPRESSED_DXT1_RGBA: C2RustUnnamed_2 = 15;
pub const PIXELFORMAT_COMPRESSED_DXT1_RGB: C2RustUnnamed_2 = 14;
pub const PIXELFORMAT_UNCOMPRESSED_R16G16B16A16: C2RustUnnamed_2 = 13;
pub const PIXELFORMAT_UNCOMPRESSED_R16G16B16: C2RustUnnamed_2 = 12;
pub const PIXELFORMAT_UNCOMPRESSED_R16: C2RustUnnamed_2 = 11;
pub const PIXELFORMAT_UNCOMPRESSED_R32G32B32A32: C2RustUnnamed_2 = 10;
pub const PIXELFORMAT_UNCOMPRESSED_R32G32B32: C2RustUnnamed_2 = 9;
pub const PIXELFORMAT_UNCOMPRESSED_R32: C2RustUnnamed_2 = 8;
pub const PIXELFORMAT_UNCOMPRESSED_R8G8B8A8: C2RustUnnamed_2 = 7;
pub const PIXELFORMAT_UNCOMPRESSED_R4G4B4A4: C2RustUnnamed_2 = 6;
pub const PIXELFORMAT_UNCOMPRESSED_R5G5B5A1: C2RustUnnamed_2 = 5;
pub const PIXELFORMAT_UNCOMPRESSED_R8G8B8: C2RustUnnamed_2 = 4;
pub const PIXELFORMAT_UNCOMPRESSED_R5G6B5: C2RustUnnamed_2 = 3;
pub const PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA: C2RustUnnamed_2 = 2;
pub const PIXELFORMAT_UNCOMPRESSED_GRAYSCALE: C2RustUnnamed_2 = 1;
pub type GuiState = libc::c_uint;
pub const STATE_DISABLED: GuiState = 3;
pub const STATE_PRESSED: GuiState = 2;
pub const STATE_FOCUSED: GuiState = 1;
pub const STATE_NORMAL: GuiState = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const TEXT_ALIGN_RIGHT: C2RustUnnamed_3 = 2;
pub const TEXT_ALIGN_CENTER: C2RustUnnamed_3 = 1;
pub const TEXT_ALIGN_LEFT: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const TEXT_ALIGN_BOTTOM: C2RustUnnamed_4 = 2;
pub const TEXT_ALIGN_MIDDLE: C2RustUnnamed_4 = 1;
pub const TEXT_ALIGN_TOP: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const TEXT_WRAP_WORD: C2RustUnnamed_5 = 2;
pub const TEXT_WRAP_CHAR: C2RustUnnamed_5 = 1;
pub const TEXT_WRAP_NONE: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const STATUSBAR: C2RustUnnamed_6 = 15;
pub const SCROLLBAR: C2RustUnnamed_6 = 14;
pub const COLORPICKER: C2RustUnnamed_6 = 13;
pub const LISTVIEW: C2RustUnnamed_6 = 12;
pub const SPINNER: C2RustUnnamed_6 = 11;
pub const VALUEBOX: C2RustUnnamed_6 = 10;
pub const TEXTBOX: C2RustUnnamed_6 = 9;
pub const DROPDOWNBOX: C2RustUnnamed_6 = 8;
pub const COMBOBOX: C2RustUnnamed_6 = 7;
pub const CHECKBOX: C2RustUnnamed_6 = 6;
pub const PROGRESSBAR: C2RustUnnamed_6 = 5;
pub const SLIDER: C2RustUnnamed_6 = 4;
pub const TOGGLE: C2RustUnnamed_6 = 3;
pub const BUTTON: C2RustUnnamed_6 = 2;
pub const LABEL: C2RustUnnamed_6 = 1;
pub const DEFAULT: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const TEXT_ALIGNMENT: C2RustUnnamed_7 = 14;
pub const TEXT_PADDING: C2RustUnnamed_7 = 13;
pub const BORDER_WIDTH: C2RustUnnamed_7 = 12;
pub const TEXT_COLOR_DISABLED: C2RustUnnamed_7 = 11;
pub const BASE_COLOR_DISABLED: C2RustUnnamed_7 = 10;
pub const BORDER_COLOR_DISABLED: C2RustUnnamed_7 = 9;
pub const TEXT_COLOR_PRESSED: C2RustUnnamed_7 = 8;
pub const BASE_COLOR_PRESSED: C2RustUnnamed_7 = 7;
pub const BORDER_COLOR_PRESSED: C2RustUnnamed_7 = 6;
pub const TEXT_COLOR_FOCUSED: C2RustUnnamed_7 = 5;
pub const BASE_COLOR_FOCUSED: C2RustUnnamed_7 = 4;
pub const BORDER_COLOR_FOCUSED: C2RustUnnamed_7 = 3;
pub const TEXT_COLOR_NORMAL: C2RustUnnamed_7 = 2;
pub const BASE_COLOR_NORMAL: C2RustUnnamed_7 = 1;
pub const BORDER_COLOR_NORMAL: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const TEXT_WRAP_MODE: C2RustUnnamed_8 = 22;
pub const TEXT_ALIGNMENT_VERTICAL: C2RustUnnamed_8 = 21;
pub const TEXT_LINE_SPACING: C2RustUnnamed_8 = 20;
pub const BACKGROUND_COLOR: C2RustUnnamed_8 = 19;
pub const LINE_COLOR: C2RustUnnamed_8 = 18;
pub const TEXT_SPACING: C2RustUnnamed_8 = 17;
pub const TEXT_SIZE: C2RustUnnamed_8 = 16;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const GROUP_PADDING: C2RustUnnamed_9 = 16;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const SLIDER_PADDING: C2RustUnnamed_10 = 17;
pub const SLIDER_WIDTH: C2RustUnnamed_10 = 16;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const PROGRESS_PADDING: C2RustUnnamed_11 = 16;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const SCROLL_SPEED: C2RustUnnamed_12 = 21;
pub const SCROLL_PADDING: C2RustUnnamed_12 = 20;
pub const SCROLL_SLIDER_SIZE: C2RustUnnamed_12 = 19;
pub const SCROLL_SLIDER_PADDING: C2RustUnnamed_12 = 18;
pub const ARROWS_VISIBLE: C2RustUnnamed_12 = 17;
pub const ARROWS_SIZE: C2RustUnnamed_12 = 16;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const CHECK_PADDING: C2RustUnnamed_13 = 16;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const COMBO_BUTTON_SPACING: C2RustUnnamed_14 = 17;
pub const COMBO_BUTTON_WIDTH: C2RustUnnamed_14 = 16;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const DROPDOWN_ITEMS_SPACING: C2RustUnnamed_15 = 17;
pub const ARROW_PADDING: C2RustUnnamed_15 = 16;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const TEXT_READONLY: C2RustUnnamed_16 = 16;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const SPIN_BUTTON_SPACING: C2RustUnnamed_17 = 17;
pub const SPIN_BUTTON_WIDTH: C2RustUnnamed_17 = 16;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const SCROLLBAR_SIDE: C2RustUnnamed_18 = 19;
pub const SCROLLBAR_WIDTH: C2RustUnnamed_18 = 18;
pub const LIST_ITEMS_SPACING: C2RustUnnamed_18 = 17;
pub const LIST_ITEMS_HEIGHT: C2RustUnnamed_18 = 16;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const HUEBAR_SELECTOR_OVERFLOW: C2RustUnnamed_19 = 20;
pub const HUEBAR_SELECTOR_HEIGHT: C2RustUnnamed_19 = 19;
pub const HUEBAR_PADDING: C2RustUnnamed_19 = 18;
pub const HUEBAR_WIDTH: C2RustUnnamed_19 = 17;
pub const COLOR_SELECTOR_SIZE: C2RustUnnamed_19 = 16;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub const ICON_CROSS_SMALL: C2RustUnnamed_20 = 128;
pub const TEXT: C2RustUnnamed_21 = 2;
pub const BASE: C2RustUnnamed_21 = 1;
pub const BORDER: C2RustUnnamed_21 = 0;
pub const ICON_ARROW_RIGHT_FILL: C2RustUnnamed_20 = 119;
pub const ICON_ARROW_LEFT_FILL: C2RustUnnamed_20 = 118;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const ICON_255: C2RustUnnamed_20 = 255;
pub const ICON_254: C2RustUnnamed_20 = 254;
pub const ICON_253: C2RustUnnamed_20 = 253;
pub const ICON_252: C2RustUnnamed_20 = 252;
pub const ICON_251: C2RustUnnamed_20 = 251;
pub const ICON_250: C2RustUnnamed_20 = 250;
pub const ICON_249: C2RustUnnamed_20 = 249;
pub const ICON_248: C2RustUnnamed_20 = 248;
pub const ICON_247: C2RustUnnamed_20 = 247;
pub const ICON_246: C2RustUnnamed_20 = 246;
pub const ICON_245: C2RustUnnamed_20 = 245;
pub const ICON_244: C2RustUnnamed_20 = 244;
pub const ICON_243: C2RustUnnamed_20 = 243;
pub const ICON_242: C2RustUnnamed_20 = 242;
pub const ICON_241: C2RustUnnamed_20 = 241;
pub const ICON_240: C2RustUnnamed_20 = 240;
pub const ICON_239: C2RustUnnamed_20 = 239;
pub const ICON_238: C2RustUnnamed_20 = 238;
pub const ICON_237: C2RustUnnamed_20 = 237;
pub const ICON_236: C2RustUnnamed_20 = 236;
pub const ICON_235: C2RustUnnamed_20 = 235;
pub const ICON_234: C2RustUnnamed_20 = 234;
pub const ICON_233: C2RustUnnamed_20 = 233;
pub const ICON_232: C2RustUnnamed_20 = 232;
pub const ICON_231: C2RustUnnamed_20 = 231;
pub const ICON_230: C2RustUnnamed_20 = 230;
pub const ICON_229: C2RustUnnamed_20 = 229;
pub const ICON_228: C2RustUnnamed_20 = 228;
pub const ICON_227: C2RustUnnamed_20 = 227;
pub const ICON_226: C2RustUnnamed_20 = 226;
pub const ICON_225: C2RustUnnamed_20 = 225;
pub const ICON_224: C2RustUnnamed_20 = 224;
pub const ICON_223: C2RustUnnamed_20 = 223;
pub const ICON_INFO_BOX: C2RustUnnamed_20 = 222;
pub const ICON_HELP_BOX: C2RustUnnamed_20 = 221;
pub const ICON_WARNING: C2RustUnnamed_20 = 220;
pub const ICON_SAND_TIMER: C2RustUnnamed_20 = 219;
pub const ICON_FILE: C2RustUnnamed_20 = 218;
pub const ICON_FOLDER: C2RustUnnamed_20 = 217;
pub const ICON_REG_EXP: C2RustUnnamed_20 = 216;
pub const ICON_CASE_SENSITIVE: C2RustUnnamed_20 = 215;
pub const ICON_BURGER_MENU: C2RustUnnamed_20 = 214;
pub const ICON_BREAKPOINT_OFF: C2RustUnnamed_20 = 213;
pub const ICON_BREAKPOINT_ON: C2RustUnnamed_20 = 212;
pub const ICON_RESTART: C2RustUnnamed_20 = 211;
pub const ICON_STEP_OUT: C2RustUnnamed_20 = 210;
pub const ICON_STEP_INTO: C2RustUnnamed_20 = 209;
pub const ICON_STEP_OVER: C2RustUnnamed_20 = 208;
pub const ICON_ROM: C2RustUnnamed_20 = 207;
pub const ICON_CPU: C2RustUnnamed_20 = 206;
pub const ICON_ALARM: C2RustUnnamed_20 = 205;
pub const ICON_FOLDER_ADD: C2RustUnnamed_20 = 204;
pub const ICON_FILE_NEW: C2RustUnnamed_20 = 203;
pub const ICON_SHIELD: C2RustUnnamed_20 = 202;
pub const ICON_HEX: C2RustUnnamed_20 = 201;
pub const ICON_FILETYPE_BINARY: C2RustUnnamed_20 = 200;
pub const ICON_HIDPI: C2RustUnnamed_20 = 199;
pub const ICON_WINDOW: C2RustUnnamed_20 = 198;
pub const ICON_LAYERS: C2RustUnnamed_20 = 197;
pub const ICON_LAYERS_VISIBLE: C2RustUnnamed_20 = 196;
pub const ICON_FILETYPE_HOME: C2RustUnnamed_20 = 195;
pub const ICON_FILETYPE_ALPHA: C2RustUnnamed_20 = 194;
pub const ICON_HELP: C2RustUnnamed_20 = 193;
pub const ICON_CROSSLINE: C2RustUnnamed_20 = 192;
pub const ICON_INFO: C2RustUnnamed_20 = 191;
pub const ICON_LIFE_BARS: C2RustUnnamed_20 = 190;
pub const ICON_VERTICAL_BARS_FILL: C2RustUnnamed_20 = 189;
pub const ICON_VERTICAL_BARS: C2RustUnnamed_20 = 188;
pub const ICON_CORNER: C2RustUnnamed_20 = 187;
pub const ICON_HEART: C2RustUnnamed_20 = 186;
pub const ICON_HOUSE: C2RustUnnamed_20 = 185;
pub const ICON_PHOTO_CAMERA_FLASH: C2RustUnnamed_20 = 184;
pub const ICON_PHOTO_CAMERA: C2RustUnnamed_20 = 183;
pub const ICON_PRINTER: C2RustUnnamed_20 = 182;
pub const ICON_MONITOR: C2RustUnnamed_20 = 181;
pub const ICON_MAILBOX: C2RustUnnamed_20 = 180;
pub const ICON_SUITCASE_ZIP: C2RustUnnamed_20 = 179;
pub const ICON_SUITCASE: C2RustUnnamed_20 = 178;
pub const ICON_NOTEBOOK: C2RustUnnamed_20 = 177;
pub const ICON_TEXT_NOTES: C2RustUnnamed_20 = 176;
pub const ICON_LINK_BROKE: C2RustUnnamed_20 = 175;
pub const ICON_LINK: C2RustUnnamed_20 = 174;
pub const ICON_LINK_MULTI: C2RustUnnamed_20 = 173;
pub const ICON_LINK_BOXES: C2RustUnnamed_20 = 172;
pub const ICON_LINK_NET: C2RustUnnamed_20 = 171;
pub const ICON_SPECIAL: C2RustUnnamed_20 = 170;
pub const ICON_CAMERA: C2RustUnnamed_20 = 169;
pub const ICON_CUBE_FACE_BACK: C2RustUnnamed_20 = 168;
pub const ICON_CUBE_FACE_RIGHT: C2RustUnnamed_20 = 167;
pub const ICON_CUBE_FACE_BOTTOM: C2RustUnnamed_20 = 166;
pub const ICON_CUBE_FACE_FRONT: C2RustUnnamed_20 = 165;
pub const ICON_CUBE_FACE_LEFT: C2RustUnnamed_20 = 164;
pub const ICON_CUBE_FACE_TOP: C2RustUnnamed_20 = 163;
pub const ICON_CUBE: C2RustUnnamed_20 = 162;
pub const ICON_MODE_3D: C2RustUnnamed_20 = 161;
pub const ICON_MODE_2D: C2RustUnnamed_20 = 160;
pub const ICON_EXIT: C2RustUnnamed_20 = 159;
pub const ICON_DOOR: C2RustUnnamed_20 = 158;
pub const ICON_STAR: C2RustUnnamed_20 = 157;
pub const ICON_CRACK_POINTS: C2RustUnnamed_20 = 156;
pub const ICON_CRACK: C2RustUnnamed_20 = 155;
pub const ICON_GEAR_EX: C2RustUnnamed_20 = 154;
pub const ICON_TEXT_POPUP: C2RustUnnamed_20 = 153;
pub const ICON_DEMON: C2RustUnnamed_20 = 152;
pub const ICON_KEY: C2RustUnnamed_20 = 151;
pub const ICON_PLAYER_JUMP: C2RustUnnamed_20 = 150;
pub const ICON_PLAYER: C2RustUnnamed_20 = 149;
pub const ICON_1UP: C2RustUnnamed_20 = 148;
pub const ICON_EXPLOSION: C2RustUnnamed_20 = 147;
pub const ICON_COIN: C2RustUnnamed_20 = 146;
pub const ICON_LASER: C2RustUnnamed_20 = 145;
pub const ICON_HAND_POINTER: C2RustUnnamed_20 = 144;
pub const ICON_BIN: C2RustUnnamed_20 = 143;
pub const ICON_GEAR_BIG: C2RustUnnamed_20 = 142;
pub const ICON_GEAR: C2RustUnnamed_20 = 141;
pub const ICON_TOOLS: C2RustUnnamed_20 = 140;
pub const ICON_CLOCK: C2RustUnnamed_20 = 139;
pub const ICON_LOCK_OPEN: C2RustUnnamed_20 = 138;
pub const ICON_LOCK_CLOSE: C2RustUnnamed_20 = 137;
pub const ICON_MAGNET: C2RustUnnamed_20 = 136;
pub const ICON_PLAYER_RECORD: C2RustUnnamed_20 = 135;
pub const ICON_PLAYER_NEXT: C2RustUnnamed_20 = 134;
pub const ICON_PLAYER_STOP: C2RustUnnamed_20 = 133;
pub const ICON_PLAYER_PAUSE: C2RustUnnamed_20 = 132;
pub const ICON_PLAYER_PLAY: C2RustUnnamed_20 = 131;
pub const ICON_PLAYER_PLAY_BACK: C2RustUnnamed_20 = 130;
pub const ICON_PLAYER_PREVIOUS: C2RustUnnamed_20 = 129;
pub const ICON_WAVE_TRIANGULAR: C2RustUnnamed_20 = 127;
pub const ICON_WAVE_SQUARE: C2RustUnnamed_20 = 126;
pub const ICON_WAVE_SINUS: C2RustUnnamed_20 = 125;
pub const ICON_WAVE: C2RustUnnamed_20 = 124;
pub const ICON_FX: C2RustUnnamed_20 = 123;
pub const ICON_AUDIO: C2RustUnnamed_20 = 122;
pub const ICON_ARROW_UP_FILL: C2RustUnnamed_20 = 121;
pub const ICON_ARROW_DOWN_FILL: C2RustUnnamed_20 = 120;
pub const ICON_ARROW_UP: C2RustUnnamed_20 = 117;
pub const ICON_ARROW_DOWN: C2RustUnnamed_20 = 116;
pub const ICON_ARROW_RIGHT: C2RustUnnamed_20 = 115;
pub const ICON_ARROW_LEFT: C2RustUnnamed_20 = 114;
pub const ICON_CROSS: C2RustUnnamed_20 = 113;
pub const ICON_OK_TICK: C2RustUnnamed_20 = 112;
pub const ICON_BOX_GRID_BIG: C2RustUnnamed_20 = 111;
pub const ICON_BOX_CONCENTRIC: C2RustUnnamed_20 = 110;
pub const ICON_BOX_DOTS_BIG: C2RustUnnamed_20 = 109;
pub const ICON_BOX_DOTS_SMALL: C2RustUnnamed_20 = 108;
pub const ICON_ZOOM_CENTER: C2RustUnnamed_20 = 107;
pub const ICON_ZOOM_ALL: C2RustUnnamed_20 = 106;
pub const ICON_ZOOM_BIG: C2RustUnnamed_20 = 105;
pub const ICON_ZOOM_MEDIUM: C2RustUnnamed_20 = 104;
pub const ICON_ZOOM_SMALL: C2RustUnnamed_20 = 103;
pub const ICON_BOX_MULTISIZE: C2RustUnnamed_20 = 102;
pub const ICON_GRID_FILL: C2RustUnnamed_20 = 101;
pub const ICON_FOUR_BOXES: C2RustUnnamed_20 = 100;
pub const ICON_BOX_CORNERS_BIG: C2RustUnnamed_20 = 99;
pub const ICON_BOX_CORNERS_SMALL: C2RustUnnamed_20 = 98;
pub const ICON_GRID: C2RustUnnamed_20 = 97;
pub const ICON_BOX_GRID: C2RustUnnamed_20 = 96;
pub const ICON_MIPMAPS: C2RustUnnamed_20 = 95;
pub const ICON_DITHERING: C2RustUnnamed_20 = 94;
pub const ICON_ALPHA_CLEAR: C2RustUnnamed_20 = 93;
pub const ICON_ALPHA_MULTIPLY: C2RustUnnamed_20 = 92;
pub const ICON_POT: C2RustUnnamed_20 = 91;
pub const ICON_BOX_CIRCLE_MASK: C2RustUnnamed_20 = 90;
pub const ICON_BOX_CENTER: C2RustUnnamed_20 = 89;
pub const ICON_BOX_TOP_LEFT: C2RustUnnamed_20 = 88;
pub const ICON_BOX_LEFT: C2RustUnnamed_20 = 87;
pub const ICON_BOX_BOTTOM_LEFT: C2RustUnnamed_20 = 86;
pub const ICON_BOX_BOTTOM: C2RustUnnamed_20 = 85;
pub const ICON_BOX_BOTTOM_RIGHT: C2RustUnnamed_20 = 84;
pub const ICON_BOX_RIGHT: C2RustUnnamed_20 = 83;
pub const ICON_BOX_TOP_RIGHT: C2RustUnnamed_20 = 82;
pub const ICON_BOX_TOP: C2RustUnnamed_20 = 81;
pub const ICON_BOX: C2RustUnnamed_20 = 80;
pub const ICON_EMPTYBOX_SMALL: C2RustUnnamed_20 = 79;
pub const ICON_SHUFFLE_FILL: C2RustUnnamed_20 = 78;
pub const ICON_REPEAT_FILL: C2RustUnnamed_20 = 77;
pub const ICON_ROTATE_FILL: C2RustUnnamed_20 = 76;
pub const ICON_MUTATE_FILL: C2RustUnnamed_20 = 75;
pub const ICON_REREDO_FILL: C2RustUnnamed_20 = 74;
pub const ICON_REDO_FILL: C2RustUnnamed_20 = 73;
pub const ICON_UNDO_FILL: C2RustUnnamed_20 = 72;
pub const ICON_CURSOR_SCALE_LEFT_FILL: C2RustUnnamed_20 = 71;
pub const ICON_CURSOR_SCALE_RIGHT_FILL: C2RustUnnamed_20 = 70;
pub const ICON_CURSOR_SCALE_FILL: C2RustUnnamed_20 = 69;
pub const ICON_CURSOR_MOVE_FILL: C2RustUnnamed_20 = 68;
pub const ICON_TARGET_MOVE_FILL: C2RustUnnamed_20 = 67;
pub const ICON_TARGET_BIG_FILL: C2RustUnnamed_20 = 66;
pub const ICON_TARGET_SMALL_FILL: C2RustUnnamed_20 = 65;
pub const ICON_TARGET: C2RustUnnamed_20 = 64;
pub const ICON_EMPTYBOX: C2RustUnnamed_20 = 63;
pub const ICON_SHUFFLE: C2RustUnnamed_20 = 62;
pub const ICON_REPEAT: C2RustUnnamed_20 = 61;
pub const ICON_ROTATE: C2RustUnnamed_20 = 60;
pub const ICON_MUTATE: C2RustUnnamed_20 = 59;
pub const ICON_REREDO: C2RustUnnamed_20 = 58;
pub const ICON_REDO: C2RustUnnamed_20 = 57;
pub const ICON_UNDO: C2RustUnnamed_20 = 56;
pub const ICON_CURSOR_SCALE_LEFT: C2RustUnnamed_20 = 55;
pub const ICON_CURSOR_SCALE_RIGHT: C2RustUnnamed_20 = 54;
pub const ICON_CURSOR_SCALE: C2RustUnnamed_20 = 53;
pub const ICON_CURSOR_MOVE: C2RustUnnamed_20 = 52;
pub const ICON_TARGET_MOVE: C2RustUnnamed_20 = 51;
pub const ICON_TARGET_BIG: C2RustUnnamed_20 = 50;
pub const ICON_TARGET_SMALL: C2RustUnnamed_20 = 49;
pub const ICON_TARGET_POINT: C2RustUnnamed_20 = 48;
pub const ICON_FILTER: C2RustUnnamed_20 = 47;
pub const ICON_FILTER_TOP: C2RustUnnamed_20 = 46;
pub const ICON_EYE_OFF: C2RustUnnamed_20 = 45;
pub const ICON_EYE_ON: C2RustUnnamed_20 = 44;
pub const ICON_LENS_BIG: C2RustUnnamed_20 = 43;
pub const ICON_LENS: C2RustUnnamed_20 = 42;
pub const ICON_SYMMETRY_VERTICAL: C2RustUnnamed_20 = 41;
pub const ICON_SYMMETRY_HORIZONTAL: C2RustUnnamed_20 = 40;
pub const ICON_SYMMETRY: C2RustUnnamed_20 = 39;
pub const ICON_SQUARE_TOGGLE: C2RustUnnamed_20 = 38;
pub const ICON_CROP_ALPHA: C2RustUnnamed_20 = 37;
pub const ICON_CROP: C2RustUnnamed_20 = 36;
pub const ICON_FILTER_BILINEAR: C2RustUnnamed_20 = 35;
pub const ICON_FILTER_POINT: C2RustUnnamed_20 = 34;
pub const ICON_RESIZE: C2RustUnnamed_20 = 33;
pub const ICON_SCALE: C2RustUnnamed_20 = 32;
pub const ICON_TEXT_A: C2RustUnnamed_20 = 31;
pub const ICON_TEXT_T: C2RustUnnamed_20 = 30;
pub const ICON_COLOR_BUCKET: C2RustUnnamed_20 = 29;
pub const ICON_RUBBER: C2RustUnnamed_20 = 28;
pub const ICON_COLOR_PICKER: C2RustUnnamed_20 = 27;
pub const ICON_WATER_DROP: C2RustUnnamed_20 = 26;
pub const ICON_BRUSH_PAINTER: C2RustUnnamed_20 = 25;
pub const ICON_BRUSH_CLASSIC: C2RustUnnamed_20 = 24;
pub const ICON_PENCIL_BIG: C2RustUnnamed_20 = 23;
pub const ICON_PENCIL: C2RustUnnamed_20 = 22;
pub const ICON_CURSOR_CLASSIC: C2RustUnnamed_20 = 21;
pub const ICON_CURSOR_POINTER: C2RustUnnamed_20 = 20;
pub const ICON_CURSOR_HAND: C2RustUnnamed_20 = 19;
pub const ICON_FILE_PASTE: C2RustUnnamed_20 = 18;
pub const ICON_FILE_CUT: C2RustUnnamed_20 = 17;
pub const ICON_FILE_COPY: C2RustUnnamed_20 = 16;
pub const ICON_FILETYPE_INFO: C2RustUnnamed_20 = 15;
pub const ICON_FILETYPE_VIDEO: C2RustUnnamed_20 = 14;
pub const ICON_FILETYPE_PLAY: C2RustUnnamed_20 = 13;
pub const ICON_FILETYPE_IMAGE: C2RustUnnamed_20 = 12;
pub const ICON_FILETYPE_AUDIO: C2RustUnnamed_20 = 11;
pub const ICON_FILETYPE_TEXT: C2RustUnnamed_20 = 10;
pub const ICON_FILE_DELETE: C2RustUnnamed_20 = 9;
pub const ICON_FILE_ADD: C2RustUnnamed_20 = 8;
pub const ICON_FILE_EXPORT: C2RustUnnamed_20 = 7;
pub const ICON_FILE_SAVE: C2RustUnnamed_20 = 6;
pub const ICON_FILE_OPEN: C2RustUnnamed_20 = 5;
pub const ICON_FOLDER_SAVE: C2RustUnnamed_20 = 4;
pub const ICON_FOLDER_OPEN: C2RustUnnamed_20 = 3;
pub const ICON_FILE_SAVE_CLASSIC: C2RustUnnamed_20 = 2;
pub const ICON_FOLDER_FILE_OPEN: C2RustUnnamed_20 = 1;
pub const ICON_NONE: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const OTHER: C2RustUnnamed_21 = 3;
static mut guiStyleLoaded: bool = 0 as libc::c_int != 0;
pub unsafe extern "C" fn GuiGetFont() -> Font {
    return guiFont;
}
pub unsafe extern "C" fn GuiGetStyle(
    mut control: libc::c_int,
    mut property: libc::c_int,
) -> libc::c_int {
    if !guiStyleLoaded {
        GuiLoadStyleDefault();
    }
    return guiStyle[(control * (16 as libc::c_int + 8 as libc::c_int) + property)
        as usize] as libc::c_int;
}
pub unsafe extern "C" fn GuiLoadStyle(mut fileName: *const libc::c_char) {
    let mut tryBinary: bool = 0 as libc::c_int != 0;
    if !guiStyleLoaded {
        GuiLoadStyleDefault();
    }
    let mut rgsFile: *mut FILE = fopen(
        fileName,
        b"rt\0" as *const u8 as *const libc::c_char,
    );
    if !rgsFile.is_null() {
        let mut buffer: [libc::c_char; 256] = [
            0 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        fgets(buffer.as_mut_ptr(), 256 as libc::c_int, rgsFile);
        if buffer[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            let mut controlId: libc::c_int = 0 as libc::c_int;
            let mut propertyId: libc::c_int = 0 as libc::c_int;
            let mut propertyValue: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while feof(rgsFile) == 0 {
                match buffer[0 as libc::c_int as usize] as libc::c_int {
                    112 => {
                        sscanf(
                            buffer.as_mut_ptr(),
                            b"p %d %d 0x%x\0" as *const u8 as *const libc::c_char,
                            &mut controlId as *mut libc::c_int,
                            &mut propertyId as *mut libc::c_int,
                            &mut propertyValue as *mut libc::c_uint,
                        );
                        GuiSetStyle(controlId, propertyId, propertyValue as libc::c_int);
                    }
                    102 => {
                        let mut fontSize: libc::c_int = 0 as libc::c_int;
                        let mut charmapFileName: [libc::c_char; 256] = [
                            0 as libc::c_int as libc::c_char,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ];
                        let mut fontFileName: [libc::c_char; 256] = [
                            0 as libc::c_int as libc::c_char,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ];
                        sscanf(
                            buffer.as_mut_ptr(),
                            b"f %d %s %[^\r\n]s\0" as *const u8 as *const libc::c_char,
                            &mut fontSize as *mut libc::c_int,
                            charmapFileName.as_mut_ptr(),
                            fontFileName.as_mut_ptr(),
                        );
                        let mut font: Font = {
                            let mut init = Font {
                                baseSize: 0 as libc::c_int,
                                glyphCount: 0,
                                glyphPadding: 0,
                                texture: Texture2D {
                                    id: 0,
                                    width: 0,
                                    height: 0,
                                    mipmaps: 0,
                                    format: 0,
                                },
                                recs: 0 as *mut Rectangle,
                                glyphs: 0 as *mut GlyphInfo,
                            };
                            init
                        };
                        let mut codepoints: *mut libc::c_int = 0 as *mut libc::c_int;
                        let mut codepointCount: libc::c_int = 0 as libc::c_int;
                        if charmapFileName[0 as libc::c_int as usize] as libc::c_int
                            != '0' as i32
                        {
                            let mut textData: *mut libc::c_char = LoadFileText(
                                TextFormat(
                                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                                    GetDirectoryPath(fileName),
                                    charmapFileName.as_mut_ptr(),
                                ),
                            );
                            codepoints = LoadCodepoints(textData, &mut codepointCount);
                            UnloadFileText(textData);
                        }
                        if fontFileName[0 as libc::c_int as usize] as libc::c_int
                            != '\0' as i32
                        {
                            if font.texture.id != (GetFontDefault()).texture.id {
                                UnloadTexture(font.texture);
                            }
                            if codepointCount > 0 as libc::c_int {
                                font = LoadFontEx(
                                    TextFormat(
                                        b"%s/%s\0" as *const u8 as *const libc::c_char,
                                        GetDirectoryPath(fileName),
                                        fontFileName.as_mut_ptr(),
                                    ),
                                    fontSize,
                                    codepoints,
                                    codepointCount,
                                );
                            } else {
                                font = LoadFontEx(
                                    TextFormat(
                                        b"%s/%s\0" as *const u8 as *const libc::c_char,
                                        GetDirectoryPath(fileName),
                                        fontFileName.as_mut_ptr(),
                                    ),
                                    fontSize,
                                    0 as *mut libc::c_int,
                                    0 as libc::c_int,
                                );
                            }
                        }
                        if font.texture.id == 0 as libc::c_int as libc::c_uint {
                            font = GetFontDefault();
                            GuiSetStyle(
                                DEFAULT as libc::c_int,
                                TEXT_SIZE as libc::c_int,
                                10 as libc::c_int,
                            );
                            GuiSetStyle(
                                DEFAULT as libc::c_int,
                                TEXT_SPACING as libc::c_int,
                                1 as libc::c_int,
                            );
                        }
                        UnloadCodepoints(codepoints);
                        if font.texture.id > 0 as libc::c_int as libc::c_uint
                            && font.glyphCount > 0 as libc::c_int
                        {
                            GuiSetFont(font);
                        }
                    }
                    _ => {}
                }
                fgets(buffer.as_mut_ptr(), 256 as libc::c_int, rgsFile);
            }
        } else {
            tryBinary = 1 as libc::c_int != 0;
        }
        fclose(rgsFile);
    }
    if tryBinary {
        rgsFile = fopen(fileName, b"rb\0" as *const u8 as *const libc::c_char);
        if !rgsFile.is_null() {
            fseek(rgsFile, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
            let mut fileDataSize: libc::c_int = ftell(rgsFile) as libc::c_int;
            fseek(rgsFile, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
            if fileDataSize > 0 as libc::c_int {
                let mut fileData: *mut libc::c_uchar = malloc(
                    (fileDataSize as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_uchar;
                fread(
                    fileData as *mut libc::c_void,
                    ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    fileDataSize as libc::c_ulong,
                    rgsFile,
                );
                GuiLoadStyleFromMemory(fileData, fileDataSize);
                free(fileData as *mut libc::c_void);
            }
            fclose(rgsFile);
        }
    }
}
unsafe extern "C" fn GuiLoadStyleFromMemory(
    mut fileData: *const libc::c_uchar,
    mut dataSize: libc::c_int,
) {
    let mut fileDataPtr: *mut libc::c_uchar = fileData as *mut libc::c_uchar;
    let mut signature: [libc::c_char; 5] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
    ];
    let mut version: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut reserved: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut propertyCount: libc::c_int = 0 as libc::c_int;
    memcpy(
        signature.as_mut_ptr() as *mut libc::c_void,
        fileDataPtr as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        &mut version as *mut libc::c_short as *mut libc::c_void,
        fileDataPtr.offset(4 as libc::c_int as isize) as *const libc::c_void,
        ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
    );
    memcpy(
        &mut reserved as *mut libc::c_short as *mut libc::c_void,
        fileDataPtr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *const libc::c_void,
        ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
    );
    memcpy(
        &mut propertyCount as *mut libc::c_int as *mut libc::c_void,
        fileDataPtr
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    fileDataPtr = fileDataPtr.offset(12 as libc::c_int as isize);
    if signature[0 as libc::c_int as usize] as libc::c_int == 'r' as i32
        && signature[1 as libc::c_int as usize] as libc::c_int == 'G' as i32
        && signature[2 as libc::c_int as usize] as libc::c_int == 'S' as i32
        && signature[3 as libc::c_int as usize] as libc::c_int == ' ' as i32
    {
        let mut controlId: libc::c_short = 0 as libc::c_int as libc::c_short;
        let mut propertyId: libc::c_short = 0 as libc::c_int as libc::c_short;
        let mut propertyValue: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < propertyCount {
            memcpy(
                &mut controlId as *mut libc::c_short as *mut libc::c_void,
                fileDataPtr as *const libc::c_void,
                ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
            );
            memcpy(
                &mut propertyId as *mut libc::c_short as *mut libc::c_void,
                fileDataPtr.offset(2 as libc::c_int as isize) as *const libc::c_void,
                ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
            );
            memcpy(
                &mut propertyValue as *mut libc::c_uint as *mut libc::c_void,
                fileDataPtr
                    .offset(2 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *const libc::c_void,
                ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
            );
            fileDataPtr = fileDataPtr.offset(8 as libc::c_int as isize);
            if controlId as libc::c_int == 0 as libc::c_int {
                GuiSetStyle(
                    0 as libc::c_int,
                    propertyId as libc::c_int,
                    propertyValue as libc::c_int,
                );
                if (propertyId as libc::c_int) < 16 as libc::c_int {
                    let mut i_0: libc::c_int = 1 as libc::c_int;
                    while i_0 < 16 as libc::c_int {
                        GuiSetStyle(
                            i_0,
                            propertyId as libc::c_int,
                            propertyValue as libc::c_int,
                        );
                        i_0 += 1;
                        i_0;
                    }
                }
            } else {
                GuiSetStyle(
                    controlId as libc::c_int,
                    propertyId as libc::c_int,
                    propertyValue as libc::c_int,
                );
            }
            i += 1;
            i;
        }
        let mut fontDataSize: libc::c_int = 0 as libc::c_int;
        memcpy(
            &mut fontDataSize as *mut libc::c_int as *mut libc::c_void,
            fileDataPtr as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        );
        fileDataPtr = fileDataPtr.offset(4 as libc::c_int as isize);
        if fontDataSize > 0 as libc::c_int {
            let mut font: Font = {
                let mut init = Font {
                    baseSize: 0 as libc::c_int,
                    glyphCount: 0,
                    glyphPadding: 0,
                    texture: Texture2D {
                        id: 0,
                        width: 0,
                        height: 0,
                        mipmaps: 0,
                        format: 0,
                    },
                    recs: 0 as *mut Rectangle,
                    glyphs: 0 as *mut GlyphInfo,
                };
                init
            };
            let mut fontType: libc::c_int = 0 as libc::c_int;
            memcpy(
                &mut font.baseSize as *mut libc::c_int as *mut libc::c_void,
                fileDataPtr as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            memcpy(
                &mut font.glyphCount as *mut libc::c_int as *mut libc::c_void,
                fileDataPtr.offset(4 as libc::c_int as isize) as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            memcpy(
                &mut fontType as *mut libc::c_int as *mut libc::c_void,
                fileDataPtr
                    .offset(4 as libc::c_int as isize)
                    .offset(4 as libc::c_int as isize) as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            fileDataPtr = fileDataPtr.offset(12 as libc::c_int as isize);
            let mut fontWhiteRec: Rectangle = {
                let mut init = Rectangle {
                    x: 0 as libc::c_int as libc::c_float,
                    y: 0.,
                    width: 0.,
                    height: 0.,
                };
                init
            };
            memcpy(
                &mut fontWhiteRec as *mut Rectangle as *mut libc::c_void,
                fileDataPtr as *const libc::c_void,
                ::std::mem::size_of::<Rectangle>() as libc::c_ulong,
            );
            fileDataPtr = fileDataPtr.offset(16 as libc::c_int as isize);
            let mut fontImageUncompSize: libc::c_int = 0 as libc::c_int;
            let mut fontImageCompSize: libc::c_int = 0 as libc::c_int;
            memcpy(
                &mut fontImageUncompSize as *mut libc::c_int as *mut libc::c_void,
                fileDataPtr as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            memcpy(
                &mut fontImageCompSize as *mut libc::c_int as *mut libc::c_void,
                fileDataPtr.offset(4 as libc::c_int as isize) as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            fileDataPtr = fileDataPtr.offset(8 as libc::c_int as isize);
            let mut imFont: Image = {
                let mut init = Image {
                    data: 0 as *mut libc::c_void,
                    width: 0,
                    height: 0,
                    mipmaps: 0,
                    format: 0,
                };
                init
            };
            imFont.mipmaps = 1 as libc::c_int;
            memcpy(
                &mut imFont.width as *mut libc::c_int as *mut libc::c_void,
                fileDataPtr as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            memcpy(
                &mut imFont.height as *mut libc::c_int as *mut libc::c_void,
                fileDataPtr.offset(4 as libc::c_int as isize) as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            memcpy(
                &mut imFont.format as *mut libc::c_int as *mut libc::c_void,
                fileDataPtr
                    .offset(4 as libc::c_int as isize)
                    .offset(4 as libc::c_int as isize) as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            fileDataPtr = fileDataPtr.offset(12 as libc::c_int as isize);
            if fontImageCompSize > 0 as libc::c_int
                && fontImageCompSize != fontImageUncompSize
            {
                let mut dataUncompSize: libc::c_int = 0 as libc::c_int;
                let mut compData: *mut libc::c_uchar = malloc(
                    fontImageCompSize as libc::c_ulong,
                ) as *mut libc::c_uchar;
                memcpy(
                    compData as *mut libc::c_void,
                    fileDataPtr as *const libc::c_void,
                    fontImageCompSize as libc::c_ulong,
                );
                fileDataPtr = fileDataPtr.offset(fontImageCompSize as isize);
                imFont
                    .data = DecompressData(
                    compData,
                    fontImageCompSize,
                    &mut dataUncompSize,
                ) as *mut libc::c_void;
                if dataUncompSize != fontImageUncompSize {
                    printf(
                        b"WARNING: Uncompressed font atlas image data could be corrupted\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                free(compData as *mut libc::c_void);
            } else {
                imFont
                    .data = malloc(fontImageUncompSize as libc::c_ulong)
                    as *mut libc::c_uchar as *mut libc::c_void;
                memcpy(
                    imFont.data,
                    fileDataPtr as *const libc::c_void,
                    fontImageUncompSize as libc::c_ulong,
                );
                fileDataPtr = fileDataPtr.offset(fontImageUncompSize as isize);
            }
            if font.texture.id != (GetFontDefault()).texture.id {
                UnloadTexture(font.texture);
            }
            font.texture = LoadTextureFromImage(imFont);
            free(imFont.data);
            if font.texture.id != 0 as libc::c_int as libc::c_uint {
                let mut recsDataSize: libc::c_int = (font.glyphCount as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<Rectangle>() as libc::c_ulong)
                    as libc::c_int;
                let mut recsDataCompressedSize: libc::c_int = 0 as libc::c_int;
                if version as libc::c_int >= 400 as libc::c_int {
                    memcpy(
                        &mut recsDataCompressedSize as *mut libc::c_int
                            as *mut libc::c_void,
                        fileDataPtr as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    );
                    fileDataPtr = fileDataPtr
                        .offset(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as isize,
                        );
                }
                if recsDataCompressedSize > 0 as libc::c_int
                    && recsDataCompressedSize != recsDataSize
                {
                    let mut recsDataCompressed: *mut libc::c_uchar = malloc(
                        recsDataCompressedSize as libc::c_ulong,
                    ) as *mut libc::c_uchar;
                    memcpy(
                        recsDataCompressed as *mut libc::c_void,
                        fileDataPtr as *const libc::c_void,
                        recsDataCompressedSize as libc::c_ulong,
                    );
                    fileDataPtr = fileDataPtr.offset(recsDataCompressedSize as isize);
                    let mut recsDataUncompSize: libc::c_int = 0 as libc::c_int;
                    font
                        .recs = DecompressData(
                        recsDataCompressed,
                        recsDataCompressedSize,
                        &mut recsDataUncompSize,
                    ) as *mut Rectangle;
                    if recsDataUncompSize != recsDataSize {
                        printf(
                            b"WARNING: Uncompressed font recs data could be corrupted\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    free(recsDataCompressed as *mut libc::c_void);
                } else {
                    font
                        .recs = calloc(
                        font.glyphCount as libc::c_ulong,
                        ::std::mem::size_of::<Rectangle>() as libc::c_ulong,
                    ) as *mut Rectangle;
                    let mut i_1: libc::c_int = 0 as libc::c_int;
                    while i_1 < font.glyphCount {
                        memcpy(
                            &mut *(font.recs).offset(i_1 as isize) as *mut Rectangle
                                as *mut libc::c_void,
                            fileDataPtr as *const libc::c_void,
                            ::std::mem::size_of::<Rectangle>() as libc::c_ulong,
                        );
                        fileDataPtr = fileDataPtr
                            .offset(
                                ::std::mem::size_of::<Rectangle>() as libc::c_ulong as isize,
                            );
                        i_1 += 1;
                        i_1;
                    }
                }
                let mut glyphsDataSize: libc::c_int = font.glyphCount
                    * 16 as libc::c_int;
                let mut glyphsDataCompressedSize: libc::c_int = 0 as libc::c_int;
                if version as libc::c_int >= 400 as libc::c_int {
                    memcpy(
                        &mut glyphsDataCompressedSize as *mut libc::c_int
                            as *mut libc::c_void,
                        fileDataPtr as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    );
                    fileDataPtr = fileDataPtr
                        .offset(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as isize,
                        );
                }
                font
                    .glyphs = calloc(
                    font.glyphCount as libc::c_ulong,
                    ::std::mem::size_of::<GlyphInfo>() as libc::c_ulong,
                ) as *mut GlyphInfo;
                if glyphsDataCompressedSize > 0 as libc::c_int
                    && glyphsDataCompressedSize != glyphsDataSize
                {
                    let mut glypsDataCompressed: *mut libc::c_uchar = malloc(
                        glyphsDataCompressedSize as libc::c_ulong,
                    ) as *mut libc::c_uchar;
                    memcpy(
                        glypsDataCompressed as *mut libc::c_void,
                        fileDataPtr as *const libc::c_void,
                        glyphsDataCompressedSize as libc::c_ulong,
                    );
                    fileDataPtr = fileDataPtr.offset(glyphsDataCompressedSize as isize);
                    let mut glyphsDataUncompSize: libc::c_int = 0 as libc::c_int;
                    let mut glyphsDataUncomp: *mut libc::c_uchar = DecompressData(
                        glypsDataCompressed,
                        glyphsDataCompressedSize,
                        &mut glyphsDataUncompSize,
                    );
                    if glyphsDataUncompSize != glyphsDataSize {
                        printf(
                            b"WARNING: Uncompressed font glyphs data could be corrupted\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    let mut glyphsDataUncompPtr: *mut libc::c_uchar = glyphsDataUncomp;
                    let mut i_2: libc::c_int = 0 as libc::c_int;
                    while i_2 < font.glyphCount {
                        memcpy(
                            &mut (*(font.glyphs).offset(i_2 as isize)).value
                                as *mut libc::c_int as *mut libc::c_void,
                            glyphsDataUncompPtr as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        );
                        memcpy(
                            &mut (*(font.glyphs).offset(i_2 as isize)).offsetX
                                as *mut libc::c_int as *mut libc::c_void,
                            glyphsDataUncompPtr.offset(4 as libc::c_int as isize)
                                as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        );
                        memcpy(
                            &mut (*(font.glyphs).offset(i_2 as isize)).offsetY
                                as *mut libc::c_int as *mut libc::c_void,
                            glyphsDataUncompPtr.offset(8 as libc::c_int as isize)
                                as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        );
                        memcpy(
                            &mut (*(font.glyphs).offset(i_2 as isize)).advanceX
                                as *mut libc::c_int as *mut libc::c_void,
                            glyphsDataUncompPtr.offset(12 as libc::c_int as isize)
                                as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        );
                        glyphsDataUncompPtr = glyphsDataUncompPtr
                            .offset(16 as libc::c_int as isize);
                        i_2 += 1;
                        i_2;
                    }
                    free(glypsDataCompressed as *mut libc::c_void);
                    free(glyphsDataUncomp as *mut libc::c_void);
                } else {
                    let mut i_3: libc::c_int = 0 as libc::c_int;
                    while i_3 < font.glyphCount {
                        memcpy(
                            &mut (*(font.glyphs).offset(i_3 as isize)).value
                                as *mut libc::c_int as *mut libc::c_void,
                            fileDataPtr as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        );
                        memcpy(
                            &mut (*(font.glyphs).offset(i_3 as isize)).offsetX
                                as *mut libc::c_int as *mut libc::c_void,
                            fileDataPtr.offset(4 as libc::c_int as isize)
                                as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        );
                        memcpy(
                            &mut (*(font.glyphs).offset(i_3 as isize)).offsetY
                                as *mut libc::c_int as *mut libc::c_void,
                            fileDataPtr.offset(8 as libc::c_int as isize)
                                as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        );
                        memcpy(
                            &mut (*(font.glyphs).offset(i_3 as isize)).advanceX
                                as *mut libc::c_int as *mut libc::c_void,
                            fileDataPtr.offset(12 as libc::c_int as isize)
                                as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        );
                        fileDataPtr = fileDataPtr.offset(16 as libc::c_int as isize);
                        i_3 += 1;
                        i_3;
                    }
                }
            } else {
                font = GetFontDefault();
            }
            GuiSetFont(font);
            if fontWhiteRec.x > 0 as libc::c_int as libc::c_float
                && fontWhiteRec.y > 0 as libc::c_int as libc::c_float
                && fontWhiteRec.width > 0 as libc::c_int as libc::c_float
                && fontWhiteRec.height > 0 as libc::c_int as libc::c_float
            {
                SetShapesTexture(font.texture, fontWhiteRec);
            }
        }
    }
}
pub unsafe extern "C" fn GuiEnableTooltip() {
    guiTooltip = 1 as libc::c_int != 0;
}
static mut guiTooltip: bool = 0 as libc::c_int != 0;
pub unsafe extern "C" fn GuiDisableTooltip() {
    guiTooltip = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn GuiSetTooltip(mut tooltip: *const libc::c_char) {
    guiTooltipPtr = tooltip;
}
static mut guiTooltipPtr: *const libc::c_char = 0 as *const libc::c_char;
pub unsafe extern "C" fn GuiIconText(
    mut iconId: libc::c_int,
    mut text: *const libc::c_char,
) -> *const libc::c_char {
    static mut buffer: [libc::c_char; 1024] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    static mut iconBuffer: [libc::c_char; 16] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    if !text.is_null() {
        memset(
            buffer.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            1024 as libc::c_int as libc::c_ulong,
        );
        sprintf(
            buffer.as_mut_ptr(),
            b"#%03i#\0" as *const u8 as *const libc::c_char,
            iconId,
        );
        let mut i: libc::c_int = 5 as libc::c_int;
        while i < 1024 as libc::c_int {
            buffer[i as usize] = *text.offset((i - 5 as libc::c_int) as isize);
            if *text.offset((i - 5 as libc::c_int) as isize) as libc::c_int
                == '\0' as i32
            {
                break;
            }
            i += 1;
            i;
        }
        return buffer.as_mut_ptr();
    } else {
        sprintf(
            iconBuffer.as_mut_ptr(),
            b"#%03i#\0" as *const u8 as *const libc::c_char,
            iconId,
        );
        return iconBuffer.as_mut_ptr();
    };
}
pub unsafe extern "C" fn GuiSetIconScale(mut scale: libc::c_int) {
    if scale >= 1 as libc::c_int {
        guiIconScale = scale as libc::c_uint;
    }
}
static mut guiIconScale: libc::c_uint = 1 as libc::c_int as libc::c_uint;
pub unsafe extern "C" fn GuiGetIcons() -> *mut libc::c_uint {
    return guiIconsPtr;
}
static mut guiIconsPtr: *mut libc::c_uint = unsafe { guiIcons.as_ptr() as *mut _ };
static mut guiIcons: [libc::c_uint; 2048] = [
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3ff80000 as libc::c_int as libc::c_uint,
    0x2f082008 as libc::c_int as libc::c_uint,
    0x2042207e as libc::c_int as libc::c_uint,
    0x40027fc2 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0x3ffe0000 as libc::c_int as libc::c_uint,
    0x44226422 as libc::c_int as libc::c_uint,
    0x400247e2 as libc::c_int as libc::c_uint,
    0x5ffa4002 as libc::c_int as libc::c_uint,
    0x57ea500a as libc::c_int as libc::c_uint,
    0x500a500a as libc::c_int as libc::c_uint,
    0x40025ffa as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x42007e as libc::c_int as libc::c_uint,
    0x40027fc2 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x41024002 as libc::c_int as libc::c_uint,
    0x44424282 as libc::c_int as libc::c_uint,
    0x793e4102 as libc::c_int as libc::c_uint,
    0x100 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x42007e as libc::c_int as libc::c_uint,
    0x40027fc2 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x41024102 as libc::c_int as libc::c_uint,
    0x44424102 as libc::c_int as libc::c_uint,
    0x793e4282 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x21042004 as libc::c_int as libc::c_uint,
    0x24442284 as libc::c_int as libc::c_uint,
    0x21042104 as libc::c_int as libc::c_uint,
    0x20042104 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x21042004 as libc::c_int as libc::c_uint,
    0x21042104 as libc::c_int as libc::c_uint,
    0x22842444 as libc::c_int as libc::c_uint,
    0x20042104 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x42004 as libc::c_int as libc::c_uint,
    0x20041004 as libc::c_int as libc::c_uint,
    0x20844784 as libc::c_int as libc::c_uint,
    0x841384 as libc::c_int as libc::c_uint,
    0x20042784 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x22042204 as libc::c_int as libc::c_uint,
    0x22042f84 as libc::c_int as libc::c_uint,
    0x20042204 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x25042884 as libc::c_int as libc::c_uint,
    0x25042204 as libc::c_int as libc::c_uint,
    0x20042884 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042ff4 as libc::c_int as libc::c_uint,
    0x20042ff4 as libc::c_int as libc::c_uint,
    0x20042ff4 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x27042004 as libc::c_int as libc::c_uint,
    0x244424c4 as libc::c_int as libc::c_uint,
    0x26442444 as libc::c_int as libc::c_uint,
    0x20642664 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x26042604 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x35442884 as libc::c_int as libc::c_uint,
    0x2414222c as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x20c42004 as libc::c_int as libc::c_uint,
    0x22442144 as libc::c_int as libc::c_uint,
    0x22442444 as libc::c_int as libc::c_uint,
    0x20c42144 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x3ffc2ff0 as libc::c_int as libc::c_uint,
    0x3f3c2ff4 as libc::c_int as libc::c_uint,
    0x3dbc2eb4 as libc::c_int as libc::c_uint,
    0x3dbc2bb4 as libc::c_int as libc::c_uint,
    0x3f3c2eb4 as libc::c_int as libc::c_uint,
    0x3ffc2ff4 as libc::c_int as libc::c_uint,
    0x2ff4 as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x21842184 as libc::c_int as libc::c_uint,
    0x21842004 as libc::c_int as libc::c_uint,
    0x21842184 as libc::c_int as libc::c_uint,
    0x21842184 as libc::c_int as libc::c_uint,
    0x20042184 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0xff00000 as libc::c_int as libc::c_uint,
    0x381c0810 as libc::c_int as libc::c_uint,
    0x28042804 as libc::c_int as libc::c_uint,
    0x28042804 as libc::c_int as libc::c_uint,
    0x28042804 as libc::c_int as libc::c_uint,
    0x28042804 as libc::c_int as libc::c_uint,
    0x20102ffc as libc::c_int as libc::c_uint,
    0x3ff0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x701c0000 as libc::c_int as libc::c_uint,
    0x79c1e14 as libc::c_int as libc::c_uint,
    0x55a000f0 as libc::c_int as libc::c_uint,
    0x79c00f0 as libc::c_int as libc::c_uint,
    0x701c1e14 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1c00000 as libc::c_int as libc::c_uint,
    0x13e41bec as libc::c_int as libc::c_uint,
    0x3f841004 as libc::c_int as libc::c_uint,
    0x204420c4 as libc::c_int as libc::c_uint,
    0x20442044 as libc::c_int as libc::c_uint,
    0x20442044 as libc::c_int as libc::c_uint,
    0x207c2044 as libc::c_int as libc::c_uint,
    0x3fc0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3aa00fe0 as libc::c_int as libc::c_uint,
    0x2abc2aa0 as libc::c_int as libc::c_uint,
    0x2aa42aa4 as libc::c_int as libc::c_uint,
    0x20042aa4 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc2004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3c000c as libc::c_int as libc::c_uint,
    0x30800c8 as libc::c_int as libc::c_uint,
    0x30100c10 as libc::c_int as libc::c_uint,
    0x10202020 as libc::c_int as libc::c_uint,
    0x4400840 as libc::c_int as libc::c_uint,
    0x1800280 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x180000 as libc::c_int as libc::c_uint,
    0x1f00078 as libc::c_int as libc::c_uint,
    0x3e007f0 as libc::c_int as libc::c_uint,
    0x7c003e0 as libc::c_int as libc::c_uint,
    0x4000e40 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x4000000 as libc::c_int as libc::c_uint,
    0x11000a00 as libc::c_int as libc::c_uint,
    0x4400a80 as libc::c_int as libc::c_uint,
    0x1100220 as libc::c_int as libc::c_uint,
    0x580088 as libc::c_int as libc::c_uint,
    0x38 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x4000000 as libc::c_int as libc::c_uint,
    0x15000a00 as libc::c_int as libc::c_uint,
    0x50402880 as libc::c_int as libc::c_uint,
    0x14102820 as libc::c_int as libc::c_uint,
    0x5040a08 as libc::c_int as libc::c_uint,
    0x15c028c as libc::c_int as libc::c_uint,
    0x7c00bc as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1c00000 as libc::c_int as libc::c_uint,
    0x1400140 as libc::c_int as libc::c_uint,
    0x1400140 as libc::c_int as libc::c_uint,
    0xff80140 as libc::c_int as libc::c_uint,
    0xff80808 as libc::c_int as libc::c_uint,
    0xaa80808 as libc::c_int as libc::c_uint,
    0xaa80aa8 as libc::c_int as libc::c_uint,
    0xff8 as libc::c_int as libc::c_uint,
    0x1ffc0000 as libc::c_int as libc::c_uint,
    0x5ffc7ffe as libc::c_int as libc::c_uint,
    0x40004000 as libc::c_int as libc::c_uint,
    0x807f80 as libc::c_int as libc::c_uint,
    0x1c001c0 as libc::c_int as libc::c_uint,
    0x1c001c0 as libc::c_int as libc::c_uint,
    0x1c001c0 as libc::c_int as libc::c_uint,
    0x80 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x800000 as libc::c_int as libc::c_uint,
    0x1c00080 as libc::c_int as libc::c_uint,
    0x3e001c0 as libc::c_int as libc::c_uint,
    0x7f003e0 as libc::c_int as libc::c_uint,
    0x36006f0 as libc::c_int as libc::c_uint,
    0x1c0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3e003800 as libc::c_int as libc::c_uint,
    0x1f803f80 as libc::c_int as libc::c_uint,
    0xc201e40 as libc::c_int as libc::c_uint,
    0x2080c10 as libc::c_int as libc::c_uint,
    0x840104 as libc::c_int as libc::c_uint,
    0x380044 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7800300 as libc::c_int as libc::c_uint,
    0x1fe00fc0 as libc::c_int as libc::c_uint,
    0x3f883fd0 as libc::c_int as libc::c_uint,
    0xe021f04 as libc::c_int as libc::c_uint,
    0x2040402 as libc::c_int as libc::c_uint,
    0xf00108 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xc00000 as libc::c_int as libc::c_uint,
    0x2800140 as libc::c_int as libc::c_uint,
    0x8200440 as libc::c_int as libc::c_uint,
    0x20081010 as libc::c_int as libc::c_uint,
    0x2ffe3004 as libc::c_int as libc::c_uint,
    0x3f807fc as libc::c_int as libc::c_uint,
    0xe001f0 as libc::c_int as libc::c_uint,
    0x40 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x21843ffc as libc::c_int as libc::c_uint,
    0x1800180 as libc::c_int as libc::c_uint,
    0x1800180 as libc::c_int as libc::c_uint,
    0x1800180 as libc::c_int as libc::c_uint,
    0x1800180 as libc::c_int as libc::c_uint,
    0x3c00180 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x800000 as libc::c_int as libc::c_uint,
    0x1400180 as libc::c_int as libc::c_uint,
    0x6200340 as libc::c_int as libc::c_uint,
    0xc100620 as libc::c_int as libc::c_uint,
    0x1ff80c10 as libc::c_int as libc::c_uint,
    0x380c1808 as libc::c_int as libc::c_uint,
    0x70067004 as libc::c_int as libc::c_uint,
    0xf80f as libc::c_int as libc::c_uint,
    0x78000000 as libc::c_int as libc::c_uint,
    0x50004000 as libc::c_int as libc::c_uint,
    0x4800 as libc::c_int as libc::c_uint,
    0x3c003c0 as libc::c_int as libc::c_uint,
    0x3c003c0 as libc::c_int as libc::c_uint,
    0x100000 as libc::c_int as libc::c_uint,
    0x2000a as libc::c_int as libc::c_uint,
    0xe as libc::c_int as libc::c_uint,
    0x75560000 as libc::c_int as libc::c_uint,
    0x5e004002 as libc::c_int as libc::c_uint,
    0x54001002 as libc::c_int as libc::c_uint,
    0x41001202 as libc::c_int as libc::c_uint,
    0x408200fe as libc::c_int as libc::c_uint,
    0x40820082 as libc::c_int as libc::c_uint,
    0x40820082 as libc::c_int as libc::c_uint,
    0x6afe as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3f003f00 as libc::c_int as libc::c_uint,
    0x3f003f00 as libc::c_int as libc::c_uint,
    0x3f003f00 as libc::c_int as libc::c_uint,
    0x400080 as libc::c_int as libc::c_uint,
    0x1c0020 as libc::c_int as libc::c_uint,
    0x1c001c as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x6d800000 as libc::c_int as libc::c_uint,
    0x4080 as libc::c_int as libc::c_uint,
    0x40804080 as libc::c_int as libc::c_uint,
    0x40800000 as libc::c_int as libc::c_uint,
    0x406d80 as libc::c_int as libc::c_uint,
    0x1c0020 as libc::c_int as libc::c_uint,
    0x1c001c as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x40080000 as libc::c_int as libc::c_uint,
    0x1ffe2008 as libc::c_int as libc::c_uint,
    0x14081008 as libc::c_int as libc::c_uint,
    0x11081208 as libc::c_int as libc::c_uint,
    0x10481088 as libc::c_int as libc::c_uint,
    0x10081028 as libc::c_int as libc::c_uint,
    0x10047ff8 as libc::c_int as libc::c_uint,
    0x1002 as libc::c_int as libc::c_uint,
    0x100000 as libc::c_int as libc::c_uint,
    0x3ffc0010 as libc::c_int as libc::c_uint,
    0x2ab03550 as libc::c_int as libc::c_uint,
    0x22b02550 as libc::c_int as libc::c_uint,
    0x20b02150 as libc::c_int as libc::c_uint,
    0x20302050 as libc::c_int as libc::c_uint,
    0x2000fff0 as libc::c_int as libc::c_uint,
    0x2000 as libc::c_int as libc::c_uint,
    0x40000000 as libc::c_int as libc::c_uint,
    0x1ff82000 as libc::c_int as libc::c_uint,
    0x4082808 as libc::c_int as libc::c_uint,
    0x1082208 as libc::c_int as libc::c_uint,
    0x482088 as libc::c_int as libc::c_uint,
    0x182028 as libc::c_int as libc::c_uint,
    0x35542008 as libc::c_int as libc::c_uint,
    0x2 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x2800280 as libc::c_int as libc::c_uint,
    0x6c006c0 as libc::c_int as libc::c_uint,
    0xea00ee0 as libc::c_int as libc::c_uint,
    0x1e901eb0 as libc::c_int as libc::c_uint,
    0x3e883e98 as libc::c_int as libc::c_uint,
    0x7efc7e8c as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1000000 as libc::c_int as libc::c_uint,
    0x5600100 as libc::c_int as libc::c_uint,
    0x1d480d50 as libc::c_int as libc::c_uint,
    0x7d423d44 as libc::c_int as libc::c_uint,
    0x3d447d42 as libc::c_int as libc::c_uint,
    0xd501d48 as libc::c_int as libc::c_uint,
    0x1000560 as libc::c_int as libc::c_uint,
    0x100 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x4200240 as libc::c_int as libc::c_uint,
    0x10080810 as libc::c_int as libc::c_uint,
    0x1ff8 as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0xff01ff8 as libc::c_int as libc::c_uint,
    0x3c007e0 as libc::c_int as libc::c_uint,
    0x180 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x10800f0 as libc::c_int as libc::c_uint,
    0x2040204 as libc::c_int as libc::c_uint,
    0x2040204 as libc::c_int as libc::c_uint,
    0x7f00308 as libc::c_int as libc::c_uint,
    0x1c000e00 as libc::c_int as libc::c_uint,
    0x30003800 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x61803f0 as libc::c_int as libc::c_uint,
    0x8240c0c as libc::c_int as libc::c_uint,
    0x8040814 as libc::c_int as libc::c_uint,
    0xc0c0804 as libc::c_int as libc::c_uint,
    0x23f01618 as libc::c_int as libc::c_uint,
    0x18002400 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1c7007c0 as libc::c_int as libc::c_uint,
    0x638e3398 as libc::c_int as libc::c_uint,
    0x1c703398 as libc::c_int as libc::c_uint,
    0x7c0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x10002000 as libc::c_int as libc::c_uint,
    0x4700fc0 as libc::c_int as libc::c_uint,
    0x610e3218 as libc::c_int as libc::c_uint,
    0x1c703098 as libc::c_int as libc::c_uint,
    0x1007a0 as libc::c_int as libc::c_uint,
    0x8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7ffc as libc::c_int as libc::c_uint,
    0x40047ffc as libc::c_int as libc::c_uint,
    0x10102008 as libc::c_int as libc::c_uint,
    0x4400820 as libc::c_int as libc::c_uint,
    0x2800280 as libc::c_int as libc::c_uint,
    0x2800280 as libc::c_int as libc::c_uint,
    0x100 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x40027ffe as libc::c_int as libc::c_uint,
    0x10082004 as libc::c_int as libc::c_uint,
    0x4200810 as libc::c_int as libc::c_uint,
    0x2400240 as libc::c_int as libc::c_uint,
    0x2400240 as libc::c_int as libc::c_uint,
    0x1400240 as libc::c_int as libc::c_uint,
    0xc0 as libc::c_int as libc::c_uint,
    0x800000 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x80 as libc::c_int as libc::c_uint,
    0x3c9e0000 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x800000 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x3f7e01c0 as libc::c_int as libc::c_uint,
    0x8001c0 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x800000 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x3e00080 as libc::c_int as libc::c_uint,
    0x3e3e0220 as libc::c_int as libc::c_uint,
    0x3e00220 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1000000 as libc::c_int as libc::c_uint,
    0x4400280 as libc::c_int as libc::c_uint,
    0x1000100 as libc::c_int as libc::c_uint,
    0x43842008 as libc::c_int as libc::c_uint,
    0x43849ab2 as libc::c_int as libc::c_uint,
    0x1002008 as libc::c_int as libc::c_uint,
    0x4400100 as libc::c_int as libc::c_uint,
    0x1000280 as libc::c_int as libc::c_uint,
    0x1000000 as libc::c_int as libc::c_uint,
    0x4400280 as libc::c_int as libc::c_uint,
    0x1000100 as libc::c_int as libc::c_uint,
    0x41042108 as libc::c_int as libc::c_uint,
    0x41049ff2 as libc::c_int as libc::c_uint,
    0x1002108 as libc::c_int as libc::c_uint,
    0x4400100 as libc::c_int as libc::c_uint,
    0x1000280 as libc::c_int as libc::c_uint,
    0x781e0000 as libc::c_int as libc::c_uint,
    0x500a4002 as libc::c_int as libc::c_uint,
    0x4204812 as libc::c_int as libc::c_uint,
    0x240 as libc::c_int as libc::c_uint,
    0x2400000 as libc::c_int as libc::c_uint,
    0x48120420 as libc::c_int as libc::c_uint,
    0x4002500a as libc::c_int as libc::c_uint,
    0x781e as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20003c00 as libc::c_int as libc::c_uint,
    0x24002800 as libc::c_int as libc::c_uint,
    0x1000200 as libc::c_int as libc::c_uint,
    0x400080 as libc::c_int as libc::c_uint,
    0x140024 as libc::c_int as libc::c_uint,
    0x3c0004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x4003c as libc::c_int as libc::c_uint,
    0x240014 as libc::c_int as libc::c_uint,
    0x800040 as libc::c_int as libc::c_uint,
    0x2000100 as libc::c_int as libc::c_uint,
    0x28002400 as libc::c_int as libc::c_uint,
    0x3c002000 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x100020 as libc::c_int as libc::c_uint,
    0x10101fc8 as libc::c_int as libc::c_uint,
    0x10001020 as libc::c_int as libc::c_uint,
    0x10001000 as libc::c_int as libc::c_uint,
    0x10001000 as libc::c_int as libc::c_uint,
    0x1fc0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x8000400 as libc::c_int as libc::c_uint,
    0x80813f8 as libc::c_int as libc::c_uint,
    0x80408 as libc::c_int as libc::c_uint,
    0x80008 as libc::c_int as libc::c_uint,
    0x80008 as libc::c_int as libc::c_uint,
    0x3f8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3ffc0000 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20002000 as libc::c_int as libc::c_uint,
    0x20402000 as libc::c_int as libc::c_uint,
    0x3f902020 as libc::c_int as libc::c_uint,
    0x400020 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3ffc0000 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x27fc2004 as libc::c_int as libc::c_uint,
    0x20202000 as libc::c_int as libc::c_uint,
    0x3fc82010 as libc::c_int as libc::c_uint,
    0x200010 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xff00000 as libc::c_int as libc::c_uint,
    0x10081818 as libc::c_int as libc::c_uint,
    0x11801008 as libc::c_int as libc::c_uint,
    0x10001180 as libc::c_int as libc::c_uint,
    0x18101020 as libc::c_int as libc::c_uint,
    0x100fc8 as libc::c_int as libc::c_uint,
    0x20 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x4000200 as libc::c_int as libc::c_uint,
    0x240429fc as libc::c_int as libc::c_uint,
    0x20042204 as libc::c_int as libc::c_uint,
    0x20442004 as libc::c_int as libc::c_uint,
    0x3f942024 as libc::c_int as libc::c_uint,
    0x400020 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20001000 as libc::c_int as libc::c_uint,
    0x22104c0e as libc::c_int as libc::c_uint,
    0x801120 as libc::c_int as libc::c_uint,
    0x11200040 as libc::c_int as libc::c_uint,
    0x4c0e2210 as libc::c_int as libc::c_uint,
    0x10002000 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7ffe0000 as libc::c_int as libc::c_uint,
    0x50024002 as libc::c_int as libc::c_uint,
    0x44024802 as libc::c_int as libc::c_uint,
    0x41024202 as libc::c_int as libc::c_uint,
    0x40424082 as libc::c_int as libc::c_uint,
    0x40124022 as libc::c_int as libc::c_uint,
    0x4002400a as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0x800000 as libc::c_int as libc::c_uint,
    0x3e00080 as libc::c_int as libc::c_uint,
    0x8080490 as libc::c_int as libc::c_uint,
    0x3c9e0808 as libc::c_int as libc::c_uint,
    0x8080808 as libc::c_int as libc::c_uint,
    0x3e00490 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x800000 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x3ffe01c0 as libc::c_int as libc::c_uint,
    0x8001c0 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x800000 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x3e00080 as libc::c_int as libc::c_uint,
    0x3ffe03e0 as libc::c_int as libc::c_uint,
    0x3e003e0 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1000000 as libc::c_int as libc::c_uint,
    0x7c00380 as libc::c_int as libc::c_uint,
    0x1000100 as libc::c_int as libc::c_uint,
    0x638c2008 as libc::c_int as libc::c_uint,
    0x638cfbbe as libc::c_int as libc::c_uint,
    0x1002008 as libc::c_int as libc::c_uint,
    0x7c00100 as libc::c_int as libc::c_uint,
    0x1000380 as libc::c_int as libc::c_uint,
    0x1000000 as libc::c_int as libc::c_uint,
    0x7c00380 as libc::c_int as libc::c_uint,
    0x1000100 as libc::c_int as libc::c_uint,
    0x610c2108 as libc::c_int as libc::c_uint,
    0x610cfffe as libc::c_int as libc::c_uint,
    0x1002108 as libc::c_int as libc::c_uint,
    0x7c00100 as libc::c_int as libc::c_uint,
    0x1000380 as libc::c_int as libc::c_uint,
    0x781e0000 as libc::c_int as libc::c_uint,
    0x6006700e as libc::c_int as libc::c_uint,
    0x4204812 as libc::c_int as libc::c_uint,
    0x240 as libc::c_int as libc::c_uint,
    0x2400000 as libc::c_int as libc::c_uint,
    0x48120420 as libc::c_int as libc::c_uint,
    0x700e6006 as libc::c_int as libc::c_uint,
    0x781e as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x38003c00 as libc::c_int as libc::c_uint,
    0x24003000 as libc::c_int as libc::c_uint,
    0x1000200 as libc::c_int as libc::c_uint,
    0x400080 as libc::c_int as libc::c_uint,
    0xc0024 as libc::c_int as libc::c_uint,
    0x3c001c as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1c003c as libc::c_int as libc::c_uint,
    0x24000c as libc::c_int as libc::c_uint,
    0x800040 as libc::c_int as libc::c_uint,
    0x2000100 as libc::c_int as libc::c_uint,
    0x30002400 as libc::c_int as libc::c_uint,
    0x3c003800 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x300020 as libc::c_int as libc::c_uint,
    0x10301ff8 as libc::c_int as libc::c_uint,
    0x10001020 as libc::c_int as libc::c_uint,
    0x10001000 as libc::c_int as libc::c_uint,
    0x10001000 as libc::c_int as libc::c_uint,
    0x1fc0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xc000400 as libc::c_int as libc::c_uint,
    0xc081ff8 as libc::c_int as libc::c_uint,
    0x80408 as libc::c_int as libc::c_uint,
    0x80008 as libc::c_int as libc::c_uint,
    0x80008 as libc::c_int as libc::c_uint,
    0x3f8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3ffc0000 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20002000 as libc::c_int as libc::c_uint,
    0x20402000 as libc::c_int as libc::c_uint,
    0x3ff02060 as libc::c_int as libc::c_uint,
    0x400060 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3ffc0000 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x27fc2004 as libc::c_int as libc::c_uint,
    0x20202000 as libc::c_int as libc::c_uint,
    0x3ff82030 as libc::c_int as libc::c_uint,
    0x200030 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xff00000 as libc::c_int as libc::c_uint,
    0x10081818 as libc::c_int as libc::c_uint,
    0x11801008 as libc::c_int as libc::c_uint,
    0x10001180 as libc::c_int as libc::c_uint,
    0x18301020 as libc::c_int as libc::c_uint,
    0x300ff8 as libc::c_int as libc::c_uint,
    0x20 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x6000200 as libc::c_int as libc::c_uint,
    0x26042ffc as libc::c_int as libc::c_uint,
    0x20042204 as libc::c_int as libc::c_uint,
    0x20442004 as libc::c_int as libc::c_uint,
    0x3ff42064 as libc::c_int as libc::c_uint,
    0x400060 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x30001000 as libc::c_int as libc::c_uint,
    0x32107c0e as libc::c_int as libc::c_uint,
    0x801120 as libc::c_int as libc::c_uint,
    0x11200040 as libc::c_int as libc::c_uint,
    0x7c0e3210 as libc::c_int as libc::c_uint,
    0x10003000 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x30043ffc as libc::c_int as libc::c_uint,
    0x24042804 as libc::c_int as libc::c_uint,
    0x21042204 as libc::c_int as libc::c_uint,
    0x20442084 as libc::c_int as libc::c_uint,
    0x20142024 as libc::c_int as libc::c_uint,
    0x3ffc200c as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20043ffc as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc2004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x23c43ffc as libc::c_int as libc::c_uint,
    0x23c423c4 as libc::c_int as libc::c_uint,
    0x200423c4 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc2004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3e043ffc as libc::c_int as libc::c_uint,
    0x3e043e04 as libc::c_int as libc::c_uint,
    0x20043e04 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc2004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20043ffc as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3e043e04 as libc::c_int as libc::c_uint,
    0x3e043e04 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc2004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20043ffc as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3e042004 as libc::c_int as libc::c_uint,
    0x3e043e04 as libc::c_int as libc::c_uint,
    0x3ffc3e04 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20043ffc as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x23c42004 as libc::c_int as libc::c_uint,
    0x23c423c4 as libc::c_int as libc::c_uint,
    0x3ffc23c4 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20043ffc as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x207c2004 as libc::c_int as libc::c_uint,
    0x207c207c as libc::c_int as libc::c_uint,
    0x3ffc207c as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20043ffc as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x207c207c as libc::c_int as libc::c_uint,
    0x207c207c as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc2004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x207c3ffc as libc::c_int as libc::c_uint,
    0x207c207c as libc::c_int as libc::c_uint,
    0x2004207c as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc2004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20043ffc as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x23c423c4 as libc::c_int as libc::c_uint,
    0x23c423c4 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc2004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7ffe0000 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x47e24182 as libc::c_int as libc::c_uint,
    0x4ff247e2 as libc::c_int as libc::c_uint,
    0x47e24ff2 as libc::c_int as libc::c_uint,
    0x418247e2 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0x7fff0000 as libc::c_int as libc::c_uint,
    0x40014001 as libc::c_int as libc::c_uint,
    0x40014001 as libc::c_int as libc::c_uint,
    0x49555ddd as libc::c_int as libc::c_uint,
    0x4945495d as libc::c_int as libc::c_uint,
    0x400149c5 as libc::c_int as libc::c_uint,
    0x40014001 as libc::c_int as libc::c_uint,
    0x7fff as libc::c_int as libc::c_uint,
    0x7ffe0000 as libc::c_int as libc::c_uint,
    0x53327332 as libc::c_int as libc::c_uint,
    0x44ce4cce as libc::c_int as libc::c_uint,
    0x41324332 as libc::c_int as libc::c_uint,
    0x404e40ce as libc::c_int as libc::c_uint,
    0x48125432 as libc::c_int as libc::c_uint,
    0x4006540e as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0x7ffe0000 as libc::c_int as libc::c_uint,
    0x53327332 as libc::c_int as libc::c_uint,
    0x44ce4cce as libc::c_int as libc::c_uint,
    0x41324332 as libc::c_int as libc::c_uint,
    0x5c4e40ce as libc::c_int as libc::c_uint,
    0x44124432 as libc::c_int as libc::c_uint,
    0x40065c0e as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0x7ffe0000 as libc::c_int as libc::c_uint,
    0x42fe417e as libc::c_int as libc::c_uint,
    0x42fe417e as libc::c_int as libc::c_uint,
    0x42fe417e as libc::c_int as libc::c_uint,
    0x42fe417e as libc::c_int as libc::c_uint,
    0x42fe417e as libc::c_int as libc::c_uint,
    0x42fe417e as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0x7fe0000 as libc::c_int as libc::c_uint,
    0x1ffa0002 as libc::c_int as libc::c_uint,
    0x7fea000a as libc::c_int as libc::c_uint,
    0x402a402a as libc::c_int as libc::c_uint,
    0x5b2a512a as libc::c_int as libc::c_uint,
    0x5128552a as libc::c_int as libc::c_uint,
    0x40205128 as libc::c_int as libc::c_uint,
    0x7fe0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1ff80000 as libc::c_int as libc::c_uint,
    0x12481248 as libc::c_int as libc::c_uint,
    0x12481ff8 as libc::c_int as libc::c_uint,
    0x1ff81248 as libc::c_int as libc::c_uint,
    0x12481248 as libc::c_int as libc::c_uint,
    0x1ff8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x12480000 as libc::c_int as libc::c_uint,
    0x7ffe1248 as libc::c_int as libc::c_uint,
    0x12481248 as libc::c_int as libc::c_uint,
    0x12487ffe as libc::c_int as libc::c_uint,
    0x7ffe1248 as libc::c_int as libc::c_uint,
    0x12481248 as libc::c_int as libc::c_uint,
    0x12487ffe as libc::c_int as libc::c_uint,
    0x1248 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1c380000 as libc::c_int as libc::c_uint,
    0x1c3817e8 as libc::c_int as libc::c_uint,
    0x8100810 as libc::c_int as libc::c_uint,
    0x8100810 as libc::c_int as libc::c_uint,
    0x17e81c38 as libc::c_int as libc::c_uint,
    0x1c38 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x700e0000 as libc::c_int as libc::c_uint,
    0x700e5ffa as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x5ffa700e as libc::c_int as libc::c_uint,
    0x700e as libc::c_int as libc::c_uint,
    0x3f7e0000 as libc::c_int as libc::c_uint,
    0x21422142 as libc::c_int as libc::c_uint,
    0x21422142 as libc::c_int as libc::c_uint,
    0x3f7e as libc::c_int as libc::c_uint,
    0x21423f7e as libc::c_int as libc::c_uint,
    0x21422142 as libc::c_int as libc::c_uint,
    0x3f7e2142 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3bb80000 as libc::c_int as libc::c_uint,
    0x3bb83bb8 as libc::c_int as libc::c_uint,
    0x3bb80000 as libc::c_int as libc::c_uint,
    0x3bb83bb8 as libc::c_int as libc::c_uint,
    0x3bb80000 as libc::c_int as libc::c_uint,
    0x3bb83bb8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7ffe0000 as libc::c_int as libc::c_uint,
    0x7ffe7ffe as libc::c_int as libc::c_uint,
    0x77fe7000 as libc::c_int as libc::c_uint,
    0x77fe77fe as libc::c_int as libc::c_uint,
    0x777e7700 as libc::c_int as libc::c_uint,
    0x777e777e as libc::c_int as libc::c_uint,
    0x777e777e as libc::c_int as libc::c_uint,
    0x777e as libc::c_int as libc::c_uint,
    0x781e0000 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x4002 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x180 as libc::c_int as libc::c_uint,
    0x40020000 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x781e as libc::c_int as libc::c_uint,
    0x781e0000 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x4002 as libc::c_int as libc::c_uint,
    0x3c003c0 as libc::c_int as libc::c_uint,
    0x3c003c0 as libc::c_int as libc::c_uint,
    0x40020000 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x781e as libc::c_int as libc::c_uint,
    0x781e0000 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x7e04002 as libc::c_int as libc::c_uint,
    0x7e007e0 as libc::c_int as libc::c_uint,
    0x7e007e0 as libc::c_int as libc::c_uint,
    0x400207e0 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x781e as libc::c_int as libc::c_uint,
    0x781e0000 as libc::c_int as libc::c_uint,
    0x5ffa4002 as libc::c_int as libc::c_uint,
    0x1ff85ffa as libc::c_int as libc::c_uint,
    0x1ff81ff8 as libc::c_int as libc::c_uint,
    0x1ff81ff8 as libc::c_int as libc::c_uint,
    0x5ffa1ff8 as libc::c_int as libc::c_uint,
    0x40025ffa as libc::c_int as libc::c_uint,
    0x781e as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x2004381c as libc::c_int as libc::c_uint,
    0x2004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20040000 as libc::c_int as libc::c_uint,
    0x381c2004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1db80000 as libc::c_int as libc::c_uint,
    0x10081008 as libc::c_int as libc::c_uint,
    0x10080000 as libc::c_int as libc::c_uint,
    0x1008 as libc::c_int as libc::c_uint,
    0x10081008 as libc::c_int as libc::c_uint,
    0x1db8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x35560000 as libc::c_int as libc::c_uint,
    0x2002 as libc::c_int as libc::c_uint,
    0x2002 as libc::c_int as libc::c_uint,
    0x2002 as libc::c_int as libc::c_uint,
    0x2002 as libc::c_int as libc::c_uint,
    0x2002 as libc::c_int as libc::c_uint,
    0x35562002 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7ffe0000 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x48124ff2 as libc::c_int as libc::c_uint,
    0x49924812 as libc::c_int as libc::c_uint,
    0x48124992 as libc::c_int as libc::c_uint,
    0x4ff24812 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x10841ffc as libc::c_int as libc::c_uint,
    0x10841084 as libc::c_int as libc::c_uint,
    0x1ffc1084 as libc::c_int as libc::c_uint,
    0x10841084 as libc::c_int as libc::c_uint,
    0x10841084 as libc::c_int as libc::c_uint,
    0x1ffc as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x10000000 as libc::c_int as libc::c_uint,
    0x4000800 as libc::c_int as libc::c_uint,
    0x1040200 as libc::c_int as libc::c_uint,
    0x500088 as libc::c_int as libc::c_uint,
    0x20 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x10080000 as libc::c_int as libc::c_uint,
    0x4200810 as libc::c_int as libc::c_uint,
    0x1800240 as libc::c_int as libc::c_uint,
    0x2400180 as libc::c_int as libc::c_uint,
    0x8100420 as libc::c_int as libc::c_uint,
    0x1008 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x2000000 as libc::c_int as libc::c_uint,
    0x800100 as libc::c_int as libc::c_uint,
    0x200040 as libc::c_int as libc::c_uint,
    0x200010 as libc::c_int as libc::c_uint,
    0x800040 as libc::c_int as libc::c_uint,
    0x2000100 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x400000 as libc::c_int as libc::c_uint,
    0x1000080 as libc::c_int as libc::c_uint,
    0x4000200 as libc::c_int as libc::c_uint,
    0x4000800 as libc::c_int as libc::c_uint,
    0x1000200 as libc::c_int as libc::c_uint,
    0x400080 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x8081004 as libc::c_int as libc::c_uint,
    0x2200410 as libc::c_int as libc::c_uint,
    0x800140 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1400080 as libc::c_int as libc::c_uint,
    0x4100220 as libc::c_int as libc::c_uint,
    0x10040808 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x2000000 as libc::c_int as libc::c_uint,
    0x3800300 as libc::c_int as libc::c_uint,
    0x3e003c0 as libc::c_int as libc::c_uint,
    0x3e003f0 as libc::c_int as libc::c_uint,
    0x38003c0 as libc::c_int as libc::c_uint,
    0x2000300 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x400000 as libc::c_int as libc::c_uint,
    0x1c000c0 as libc::c_int as libc::c_uint,
    0x7c003c0 as libc::c_int as libc::c_uint,
    0x7c00fc0 as libc::c_int as libc::c_uint,
    0x1c003c0 as libc::c_int as libc::c_uint,
    0x4000c0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xff81ffc as libc::c_int as libc::c_uint,
    0x3e007f0 as libc::c_int as libc::c_uint,
    0x8001c0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1c00080 as libc::c_int as libc::c_uint,
    0x7f003e0 as libc::c_int as libc::c_uint,
    0x1ffc0ff8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x18a008c0 as libc::c_int as libc::c_uint,
    0x32881290 as libc::c_int as libc::c_uint,
    0x24822686 as libc::c_int as libc::c_uint,
    0x26862482 as libc::c_int as libc::c_uint,
    0x12903288 as libc::c_int as libc::c_uint,
    0x8c018a0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x4800780 as libc::c_int as libc::c_uint,
    0x4000c0 as libc::c_int as libc::c_uint,
    0x662000f0 as libc::c_int as libc::c_uint,
    0x8103c30 as libc::c_int as libc::c_uint,
    0x130a0e18 as libc::c_int as libc::c_uint,
    0x318e as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x800000 as libc::c_int as libc::c_uint,
    0x8880888 as libc::c_int as libc::c_uint,
    0x2aaa0a8a as libc::c_int as libc::c_uint,
    0xa8a2aaa as libc::c_int as libc::c_uint,
    0x8880888 as libc::c_int as libc::c_uint,
    0x80 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x600000 as libc::c_int as libc::c_uint,
    0x1080090 as libc::c_int as libc::c_uint,
    0x2040108 as libc::c_int as libc::c_uint,
    0x42044204 as libc::c_int as libc::c_uint,
    0x24022402 as libc::c_int as libc::c_uint,
    0x1800 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7f80000 as libc::c_int as libc::c_uint,
    0x4080408 as libc::c_int as libc::c_uint,
    0x4080408 as libc::c_int as libc::c_uint,
    0x4080408 as libc::c_int as libc::c_uint,
    0x7c0e0408 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xa00040 as libc::c_int as libc::c_uint,
    0x22084110 as libc::c_int as libc::c_uint,
    0x8021404 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x4200000 as libc::c_int as libc::c_uint,
    0x1800240 as libc::c_int as libc::c_uint,
    0x2400180 as libc::c_int as libc::c_uint,
    0x420 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x18380000 as libc::c_int as libc::c_uint,
    0x12281428 as libc::c_int as libc::c_uint,
    0x10a81128 as libc::c_int as libc::c_uint,
    0x112810a8 as libc::c_int as libc::c_uint,
    0x14281228 as libc::c_int as libc::c_uint,
    0x1838 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x18000000 as libc::c_int as libc::c_uint,
    0x11801600 as libc::c_int as libc::c_uint,
    0x10181060 as libc::c_int as libc::c_uint,
    0x10601018 as libc::c_int as libc::c_uint,
    0x16001180 as libc::c_int as libc::c_uint,
    0x1800 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x180000 as libc::c_int as libc::c_uint,
    0x1880068 as libc::c_int as libc::c_uint,
    0x18080608 as libc::c_int as libc::c_uint,
    0x6081808 as libc::c_int as libc::c_uint,
    0x680188 as libc::c_int as libc::c_uint,
    0x18 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1e780000 as libc::c_int as libc::c_uint,
    0x12481248 as libc::c_int as libc::c_uint,
    0x12481248 as libc::c_int as libc::c_uint,
    0x12481248 as libc::c_int as libc::c_uint,
    0x12481248 as libc::c_int as libc::c_uint,
    0x1e78 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1ff80000 as libc::c_int as libc::c_uint,
    0x10081008 as libc::c_int as libc::c_uint,
    0x10081008 as libc::c_int as libc::c_uint,
    0x10081008 as libc::c_int as libc::c_uint,
    0x10081008 as libc::c_int as libc::c_uint,
    0x1ff8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1c180000 as libc::c_int as libc::c_uint,
    0x14481428 as libc::c_int as libc::c_uint,
    0x15081488 as libc::c_int as libc::c_uint,
    0x14881508 as libc::c_int as libc::c_uint,
    0x14281448 as libc::c_int as libc::c_uint,
    0x1c18 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3c00000 as libc::c_int as libc::c_uint,
    0x8100420 as libc::c_int as libc::c_uint,
    0x10081008 as libc::c_int as libc::c_uint,
    0x10081008 as libc::c_int as libc::c_uint,
    0x4200810 as libc::c_int as libc::c_uint,
    0x3c0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xc3007e0 as libc::c_int as libc::c_uint,
    0x13c81818 as libc::c_int as libc::c_uint,
    0x14281668 as libc::c_int as libc::c_uint,
    0x14281428 as libc::c_int as libc::c_uint,
    0x1c381c38 as libc::c_int as libc::c_uint,
    0x8102244 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7c00000 as libc::c_int as libc::c_uint,
    0x8200820 as libc::c_int as libc::c_uint,
    0x3ff80820 as libc::c_int as libc::c_uint,
    0x23882008 as libc::c_int as libc::c_uint,
    0x21082388 as libc::c_int as libc::c_uint,
    0x20082108 as libc::c_int as libc::c_uint,
    0x1ff02008 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7c00000 as libc::c_int as libc::c_uint,
    0x8000800 as libc::c_int as libc::c_uint,
    0x3ff80800 as libc::c_int as libc::c_uint,
    0x23882008 as libc::c_int as libc::c_uint,
    0x21082388 as libc::c_int as libc::c_uint,
    0x20082108 as libc::c_int as libc::c_uint,
    0x1ff02008 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1c00000 as libc::c_int as libc::c_uint,
    0xc180770 as libc::c_int as libc::c_uint,
    0x3086188c as libc::c_int as libc::c_uint,
    0x60832082 as libc::c_int as libc::c_uint,
    0x60034781 as libc::c_int as libc::c_uint,
    0x30062002 as libc::c_int as libc::c_uint,
    0xc18180c as libc::c_int as libc::c_uint,
    0x1c00770 as libc::c_int as libc::c_uint,
    0xa200000 as libc::c_int as libc::c_uint,
    0x1b201b20 as libc::c_int as libc::c_uint,
    0x4200e20 as libc::c_int as libc::c_uint,
    0x4200420 as libc::c_int as libc::c_uint,
    0x4700420 as libc::c_int as libc::c_uint,
    0xe700e70 as libc::c_int as libc::c_uint,
    0xe700e70 as libc::c_int as libc::c_uint,
    0x4200e70 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x3bdc318c as libc::c_int as libc::c_uint,
    0xff01ff8 as libc::c_int as libc::c_uint,
    0x7c3e1e78 as libc::c_int as libc::c_uint,
    0x1e787c3e as libc::c_int as libc::c_uint,
    0x1ff80ff0 as libc::c_int as libc::c_uint,
    0x318c3bdc as libc::c_int as libc::c_uint,
    0x180 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x3ffc318c as libc::c_int as libc::c_uint,
    0x1c381ff8 as libc::c_int as libc::c_uint,
    0x781e1818 as libc::c_int as libc::c_uint,
    0x1818781e as libc::c_int as libc::c_uint,
    0x1ff81c38 as libc::c_int as libc::c_uint,
    0x318c3ffc as libc::c_int as libc::c_uint,
    0x180 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x8080ff8 as libc::c_int as libc::c_uint,
    0x8081ffc as libc::c_int as libc::c_uint,
    0xaa80aa8 as libc::c_int as libc::c_uint,
    0xaa80aa8 as libc::c_int as libc::c_uint,
    0xaa80aa8 as libc::c_int as libc::c_uint,
    0x8080aa8 as libc::c_int as libc::c_uint,
    0xff8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20043ffc as libc::c_int as libc::c_uint,
    0x8043f84 as libc::c_int as libc::c_uint,
    0x4040f84 as libc::c_int as libc::c_uint,
    0x4040784 as libc::c_int as libc::c_uint,
    0x7fc as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x24400400 as libc::c_int as libc::c_uint,
    0x1480 as libc::c_int as libc::c_uint,
    0x6efe0e00 as libc::c_int as libc::c_uint,
    0xe00 as libc::c_int as libc::c_uint,
    0x24401480 as libc::c_int as libc::c_uint,
    0x400 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3c00000 as libc::c_int as libc::c_uint,
    0x8300460 as libc::c_int as libc::c_uint,
    0x11181118 as libc::c_int as libc::c_uint,
    0x11181118 as libc::c_int as libc::c_uint,
    0x4600830 as libc::c_int as libc::c_uint,
    0x3c0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x10880080 as libc::c_int as libc::c_uint,
    0x6c00810 as libc::c_int as libc::c_uint,
    0x366c07e0 as libc::c_int as libc::c_uint,
    0x7e00240 as libc::c_int as libc::c_uint,
    0x1768 as libc::c_int as libc::c_uint,
    0x4200240 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3d280000 as libc::c_int as libc::c_uint,
    0x2528252c as libc::c_int as libc::c_uint,
    0x3d282528 as libc::c_int as libc::c_uint,
    0x5280528 as libc::c_int as libc::c_uint,
    0x5e80528 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x3c003c0 as libc::c_int as libc::c_uint,
    0x18003c0 as libc::c_int as libc::c_uint,
    0xff007e0 as libc::c_int as libc::c_uint,
    0xbd00bd0 as libc::c_int as libc::c_uint,
    0xa500bd0 as libc::c_int as libc::c_uint,
    0x2400240 as libc::c_int as libc::c_uint,
    0x2400240 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x3c003c0 as libc::c_int as libc::c_uint,
    0x118013c0 as libc::c_int as libc::c_uint,
    0x3c81ff8 as libc::c_int as libc::c_uint,
    0x7c003c8 as libc::c_int as libc::c_uint,
    0x4400440 as libc::c_int as libc::c_uint,
    0xc080478 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3ff80000 as libc::c_int as libc::c_uint,
    0x30183ff8 as libc::c_int as libc::c_uint,
    0x30183018 as libc::c_int as libc::c_uint,
    0x3ff83ff8 as libc::c_int as libc::c_uint,
    0x3000300 as libc::c_int as libc::c_uint,
    0x3c003c0 as libc::c_int as libc::c_uint,
    0x3e00300 as libc::c_int as libc::c_uint,
    0x3e0 as libc::c_int as libc::c_uint,
    0x3ff80000 as libc::c_int as libc::c_uint,
    0x3ff83ff8 as libc::c_int as libc::c_uint,
    0x33983ff8 as libc::c_int as libc::c_uint,
    0x3ff83398 as libc::c_int as libc::c_uint,
    0x3ff83ff8 as libc::c_int as libc::c_uint,
    0x540 as libc::c_int as libc::c_uint,
    0xfe00aa0 as libc::c_int as libc::c_uint,
    0xfe0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xff00000 as libc::c_int as libc::c_uint,
    0x20041008 as libc::c_int as libc::c_uint,
    0x25442004 as libc::c_int as libc::c_uint,
    0x10082004 as libc::c_int as libc::c_uint,
    0x6000bf0 as libc::c_int as libc::c_uint,
    0x300 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x11440000 as libc::c_int as libc::c_uint,
    0x7f00be8 as libc::c_int as libc::c_uint,
    0x1c1c0e38 as libc::c_int as libc::c_uint,
    0x1c1c0c18 as libc::c_int as libc::c_uint,
    0x7f00e38 as libc::c_int as libc::c_uint,
    0x11440be8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20080000 as libc::c_int as libc::c_uint,
    0xc601010 as libc::c_int as libc::c_uint,
    0x7c00fe0 as libc::c_int as libc::c_uint,
    0x7c007c0 as libc::c_int as libc::c_uint,
    0xc600fe0 as libc::c_int as libc::c_uint,
    0x20081010 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20080000 as libc::c_int as libc::c_uint,
    0xc601010 as libc::c_int as libc::c_uint,
    0x4400fe0 as libc::c_int as libc::c_uint,
    0x4405554 as libc::c_int as libc::c_uint,
    0xc600fe0 as libc::c_int as libc::c_uint,
    0x20081010 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x800080 as libc::c_int as libc::c_uint,
    0x1c001c0 as libc::c_int as libc::c_uint,
    0x1ffc3ffe as libc::c_int as libc::c_uint,
    0x3e007f0 as libc::c_int as libc::c_uint,
    0x7f003e0 as libc::c_int as libc::c_uint,
    0xc180770 as libc::c_int as libc::c_uint,
    0x808 as libc::c_int as libc::c_uint,
    0xff00000 as libc::c_int as libc::c_uint,
    0x8180810 as libc::c_int as libc::c_uint,
    0x8100818 as libc::c_int as libc::c_uint,
    0xa100810 as libc::c_int as libc::c_uint,
    0x8180810 as libc::c_int as libc::c_uint,
    0x8100818 as libc::c_int as libc::c_uint,
    0x8100810 as libc::c_int as libc::c_uint,
    0x1ff8 as libc::c_int as libc::c_uint,
    0xff00000 as libc::c_int as libc::c_uint,
    0x8100810 as libc::c_int as libc::c_uint,
    0x8100810 as libc::c_int as libc::c_uint,
    0x10100010 as libc::c_int as libc::c_uint,
    0x4f902010 as libc::c_int as libc::c_uint,
    0x10102010 as libc::c_int as libc::c_uint,
    0x8100010 as libc::c_int as libc::c_uint,
    0xff0 as libc::c_int as libc::c_uint,
    0x40000 as libc::c_int as libc::c_uint,
    0x1f000e as libc::c_int as libc::c_uint,
    0xef40004 as libc::c_int as libc::c_uint,
    0x12f41284 as libc::c_int as libc::c_uint,
    0xef41214 as libc::c_int as libc::c_uint,
    0x10040004 as libc::c_int as libc::c_uint,
    0x7ffc3004 as libc::c_int as libc::c_uint,
    0x10003000 as libc::c_int as libc::c_uint,
    0x78040000 as libc::c_int as libc::c_uint,
    0x501f600e as libc::c_int as libc::c_uint,
    0xef44004 as libc::c_int as libc::c_uint,
    0x12f41284 as libc::c_int as libc::c_uint,
    0xef41284 as libc::c_int as libc::c_uint,
    0x10140004 as libc::c_int as libc::c_uint,
    0x7ffc300c as libc::c_int as libc::c_uint,
    0x10003000 as libc::c_int as libc::c_uint,
    0x7fe00000 as libc::c_int as libc::c_uint,
    0x50286030 as libc::c_int as libc::c_uint,
    0x47fe4804 as libc::c_int as libc::c_uint,
    0x44224402 as libc::c_int as libc::c_uint,
    0x44224422 as libc::c_int as libc::c_uint,
    0x241275e2 as libc::c_int as libc::c_uint,
    0xc06140a as libc::c_int as libc::c_uint,
    0x7fe as libc::c_int as libc::c_uint,
    0x7fe00000 as libc::c_int as libc::c_uint,
    0x5ff87ff0 as libc::c_int as libc::c_uint,
    0x47fe4ffc as libc::c_int as libc::c_uint,
    0x44224402 as libc::c_int as libc::c_uint,
    0x44224422 as libc::c_int as libc::c_uint,
    0x241275e2 as libc::c_int as libc::c_uint,
    0xc06140a as libc::c_int as libc::c_uint,
    0x7fe as libc::c_int as libc::c_uint,
    0x7fe00000 as libc::c_int as libc::c_uint,
    0x50386030 as libc::c_int as libc::c_uint,
    0x47c2483c as libc::c_int as libc::c_uint,
    0x443e443e as libc::c_int as libc::c_uint,
    0x443e443e as libc::c_int as libc::c_uint,
    0x241e75fe as libc::c_int as libc::c_uint,
    0xc06140e as libc::c_int as libc::c_uint,
    0x7fe as libc::c_int as libc::c_uint,
    0x7fe00000 as libc::c_int as libc::c_uint,
    0x50286030 as libc::c_int as libc::c_uint,
    0x47fe4804 as libc::c_int as libc::c_uint,
    0x47fe47fe as libc::c_int as libc::c_uint,
    0x47fe47fe as libc::c_int as libc::c_uint,
    0x27fe77fe as libc::c_int as libc::c_uint,
    0xffe17fe as libc::c_int as libc::c_uint,
    0x7fe as libc::c_int as libc::c_uint,
    0x7fe00000 as libc::c_int as libc::c_uint,
    0x50286030 as libc::c_int as libc::c_uint,
    0x47fe4804 as libc::c_int as libc::c_uint,
    0x44224402 as libc::c_int as libc::c_uint,
    0x44224422 as libc::c_int as libc::c_uint,
    0x3bf27be2 as libc::c_int as libc::c_uint,
    0xbfe1bfa as libc::c_int as libc::c_uint,
    0x7fe as libc::c_int as libc::c_uint,
    0x7fe00000 as libc::c_int as libc::c_uint,
    0x70286030 as libc::c_int as libc::c_uint,
    0x7ffe7804 as libc::c_int as libc::c_uint,
    0x7c227c02 as libc::c_int as libc::c_uint,
    0x7c227c22 as libc::c_int as libc::c_uint,
    0x3c127de2 as libc::c_int as libc::c_uint,
    0xc061c0a as libc::c_int as libc::c_uint,
    0x7fe as libc::c_int as libc::c_uint,
    0x7fe00000 as libc::c_int as libc::c_uint,
    0x6fe85ff0 as libc::c_int as libc::c_uint,
    0x781e77e4 as libc::c_int as libc::c_uint,
    0x7be27be2 as libc::c_int as libc::c_uint,
    0x7be27be2 as libc::c_int as libc::c_uint,
    0x24127be2 as libc::c_int as libc::c_uint,
    0xc06140a as libc::c_int as libc::c_uint,
    0x7fe as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x2a0233fe as libc::c_int as libc::c_uint,
    0x22022602 as libc::c_int as libc::c_uint,
    0x22022202 as libc::c_int as libc::c_uint,
    0x2a022602 as libc::c_int as libc::c_uint,
    0xa033fe as libc::c_int as libc::c_uint,
    0x2080110 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x200c3ffc as libc::c_int as libc::c_uint,
    0xc000c as libc::c_int as libc::c_uint,
    0x3ffc000c as libc::c_int as libc::c_uint,
    0x30003000 as libc::c_int as libc::c_uint,
    0x30003000 as libc::c_int as libc::c_uint,
    0x3ffc3004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x22003e as libc::c_int as libc::c_uint,
    0x12201e2 as libc::c_int as libc::c_uint,
    0x100013e as libc::c_int as libc::c_uint,
    0x1000100 as libc::c_int as libc::c_uint,
    0x79000100 as libc::c_int as libc::c_uint,
    0x4f004900 as libc::c_int as libc::c_uint,
    0x7800 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x44007c00 as libc::c_int as libc::c_uint,
    0x45004600 as libc::c_int as libc::c_uint,
    0x627cbe as libc::c_int as libc::c_uint,
    0x620022 as libc::c_int as libc::c_uint,
    0x45007cbe as libc::c_int as libc::c_uint,
    0x44004600 as libc::c_int as libc::c_uint,
    0x7c00 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x44007c as libc::c_int as libc::c_uint,
    0x10007c as libc::c_int as libc::c_uint,
    0x3f100010 as libc::c_int as libc::c_uint,
    0x3f1021f0 as libc::c_int as libc::c_uint,
    0x3f100010 as libc::c_int as libc::c_uint,
    0x3f0021f0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x44007c as libc::c_int as libc::c_uint,
    0x440044 as libc::c_int as libc::c_uint,
    0x10007c as libc::c_int as libc::c_uint,
    0x100010 as libc::c_int as libc::c_uint,
    0x44107c10 as libc::c_int as libc::c_uint,
    0x440047f0 as libc::c_int as libc::c_uint,
    0x7c00 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x44007c as libc::c_int as libc::c_uint,
    0x440044 as libc::c_int as libc::c_uint,
    0x7c as libc::c_int as libc::c_uint,
    0x10 as libc::c_int as libc::c_uint,
    0x44007c10 as libc::c_int as libc::c_uint,
    0x44004550 as libc::c_int as libc::c_uint,
    0x7c00 as libc::c_int as libc::c_uint,
    0x2a00000 as libc::c_int as libc::c_uint,
    0x22a43ffc as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042ff4 as libc::c_int as libc::c_uint,
    0x20042ff4 as libc::c_int as libc::c_uint,
    0x20042ff4 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x3ffc0000 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x245e27c4 as libc::c_int as libc::c_uint,
    0x27c42444 as libc::c_int as libc::c_uint,
    0x2004201e as libc::c_int as libc::c_uint,
    0x201e2004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7e00000 as libc::c_int as libc::c_uint,
    0x4200420 as libc::c_int as libc::c_uint,
    0x24243ffc as libc::c_int as libc::c_uint,
    0x24242424 as libc::c_int as libc::c_uint,
    0x24242424 as libc::c_int as libc::c_uint,
    0x3ffc2424 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xfe00000 as libc::c_int as libc::c_uint,
    0x8200820 as libc::c_int as libc::c_uint,
    0x40047ffc as libc::c_int as libc::c_uint,
    0x7ffc5554 as libc::c_int as libc::c_uint,
    0x40045554 as libc::c_int as libc::c_uint,
    0x7ffc4004 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x20043ffc as libc::c_int as libc::c_uint,
    0x3ffc2004 as libc::c_int as libc::c_uint,
    0x13c81008 as libc::c_int as libc::c_uint,
    0x100813c8 as libc::c_int as libc::c_uint,
    0x10081008 as libc::c_int as libc::c_uint,
    0x1ff81008 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x40027ffe as libc::c_int as libc::c_uint,
    0x5ffa5ffa as libc::c_int as libc::c_uint,
    0x5ffa5ffa as libc::c_int as libc::c_uint,
    0x40025ffa as libc::c_int as libc::c_uint,
    0x3c07ffe as libc::c_int as libc::c_uint,
    0x1ff81ff8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xff00000 as libc::c_int as libc::c_uint,
    0x6bfe7ffe as libc::c_int as libc::c_uint,
    0x7ffe7ffe as libc::c_int as libc::c_uint,
    0x68167ffe as libc::c_int as libc::c_uint,
    0x8106816 as libc::c_int as libc::c_uint,
    0x8100810 as libc::c_int as libc::c_uint,
    0xff00810 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3ff80000 as libc::c_int as libc::c_uint,
    0xfffe2008 as libc::c_uint,
    0x870a8002 as libc::c_uint,
    0x904a888a as libc::c_uint,
    0x904a904a as libc::c_uint,
    0x870a888a as libc::c_uint,
    0xfffe8002 as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xfc00000 as libc::c_int as libc::c_uint,
    0xfcfe0cd8 as libc::c_uint,
    0x8002fffe as libc::c_uint,
    0x84428382 as libc::c_uint,
    0x84428442 as libc::c_uint,
    0x80028382 as libc::c_uint,
    0xfffe8002 as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x2400180 as libc::c_int as libc::c_uint,
    0x8100420 as libc::c_int as libc::c_uint,
    0x20041008 as libc::c_int as libc::c_uint,
    0x23c42004 as libc::c_int as libc::c_uint,
    0x22442244 as libc::c_int as libc::c_uint,
    0x3ffc2244 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1c700000 as libc::c_int as libc::c_uint,
    0x3ff83ef8 as libc::c_int as libc::c_uint,
    0x3ff83ff8 as libc::c_int as libc::c_uint,
    0xfe01ff0 as libc::c_int as libc::c_uint,
    0x38007c0 as libc::c_int as libc::c_uint,
    0x100 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x80000000 as libc::c_uint,
    0xe000c000 as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x14001c00 as libc::c_int as libc::c_uint,
    0x15c01400 as libc::c_int as libc::c_uint,
    0x15401540 as libc::c_int as libc::c_uint,
    0x155c1540 as libc::c_int as libc::c_uint,
    0x15541554 as libc::c_int as libc::c_uint,
    0x1ddc1554 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3000300 as libc::c_int as libc::c_uint,
    0x1b001b00 as libc::c_int as libc::c_uint,
    0x1b601b60 as libc::c_int as libc::c_uint,
    0x1b6c1b60 as libc::c_int as libc::c_uint,
    0x1b6c1b6c as libc::c_int as libc::c_uint,
    0x1b6c1b6c as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x403e7ffe as libc::c_int as libc::c_uint,
    0x7ffe403e as libc::c_int as libc::c_uint,
    0x7ffe0000 as libc::c_int as libc::c_uint,
    0x43fe43fe as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7ffc0000 as libc::c_int as libc::c_uint,
    0x43844004 as libc::c_int as libc::c_uint,
    0x43844284 as libc::c_int as libc::c_uint,
    0x43844004 as libc::c_int as libc::c_uint,
    0x42844284 as libc::c_int as libc::c_uint,
    0x42844284 as libc::c_int as libc::c_uint,
    0x40044384 as libc::c_int as libc::c_uint,
    0x7ffc as libc::c_int as libc::c_uint,
    0x40008000 as libc::c_int as libc::c_uint,
    0x10002000 as libc::c_int as libc::c_uint,
    0x4000800 as libc::c_int as libc::c_uint,
    0x1000200 as libc::c_int as libc::c_uint,
    0x400080 as libc::c_int as libc::c_uint,
    0x100020 as libc::c_int as libc::c_uint,
    0x40008 as libc::c_int as libc::c_uint,
    0x10002 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1ff01ff0 as libc::c_int as libc::c_uint,
    0x18301830 as libc::c_int as libc::c_uint,
    0x1f001830 as libc::c_int as libc::c_uint,
    0x3001f00 as libc::c_int as libc::c_uint,
    0x300 as libc::c_int as libc::c_uint,
    0x3000300 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x2abc3550 as libc::c_int as libc::c_uint,
    0x2aac3554 as libc::c_int as libc::c_uint,
    0x2aac3554 as libc::c_int as libc::c_uint,
    0x2aac3554 as libc::c_int as libc::c_uint,
    0x2aac3554 as libc::c_int as libc::c_uint,
    0x2aac3554 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x22442184 as libc::c_int as libc::c_uint,
    0x28142424 as libc::c_int as libc::c_uint,
    0x29942814 as libc::c_int as libc::c_uint,
    0x2ff42994 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x7fe0000 as libc::c_int as libc::c_uint,
    0x4020402 as libc::c_int as libc::c_uint,
    0x7fe20402 as libc::c_int as libc::c_uint,
    0x44224422 as libc::c_int as libc::c_uint,
    0x44224422 as libc::c_int as libc::c_uint,
    0x402047fe as libc::c_int as libc::c_uint,
    0x40204020 as libc::c_int as libc::c_uint,
    0x7fe0 as libc::c_int as libc::c_uint,
    0x7fe0000 as libc::c_int as libc::c_uint,
    0x4020402 as libc::c_int as libc::c_uint,
    0x7c020402 as libc::c_int as libc::c_uint,
    0x44024402 as libc::c_int as libc::c_uint,
    0x44024402 as libc::c_int as libc::c_uint,
    0x402047fe as libc::c_int as libc::c_uint,
    0x40204020 as libc::c_int as libc::c_uint,
    0x7fe0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x40027ffe as libc::c_int as libc::c_uint,
    0x7ffe4002 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x7ffe4002 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x9100000 as libc::c_int as libc::c_uint,
    0x9f00910 as libc::c_int as libc::c_uint,
    0x9100910 as libc::c_int as libc::c_uint,
    0x910 as libc::c_int as libc::c_uint,
    0x24a2779e as libc::c_int as libc::c_uint,
    0x27a224a2 as libc::c_int as libc::c_uint,
    0x709e20a2 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x2a842e84 as libc::c_int as libc::c_uint,
    0x2e842a84 as libc::c_int as libc::c_uint,
    0x2ba42004 as libc::c_int as libc::c_uint,
    0x2aa42aa4 as libc::c_int as libc::c_uint,
    0x20042ba4 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x120012 as libc::c_int as libc::c_uint,
    0x4a5e4bd2 as libc::c_int as libc::c_uint,
    0x485233d2 as libc::c_int as libc::c_uint,
    0x4bd2 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x381c0660 as libc::c_int as libc::c_uint,
    0x23c42004 as libc::c_int as libc::c_uint,
    0x23c42044 as libc::c_int as libc::c_uint,
    0x13c82204 as libc::c_int as libc::c_uint,
    0x8101008 as libc::c_int as libc::c_uint,
    0x2400420 as libc::c_int as libc::c_uint,
    0x180 as libc::c_int as libc::c_uint,
    0x7e0000 as libc::c_int as libc::c_uint,
    0x20023fc2 as libc::c_int as libc::c_uint,
    0x40227fe2 as libc::c_int as libc::c_uint,
    0x400a403a as libc::c_int as libc::c_uint,
    0x400a400a as libc::c_int as libc::c_uint,
    0x400a400a as libc::c_int as libc::c_uint,
    0x4008400e as libc::c_int as libc::c_uint,
    0x7ff8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x42007e as libc::c_int as libc::c_uint,
    0x40027fc2 as libc::c_int as libc::c_uint,
    0x44024002 as libc::c_int as libc::c_uint,
    0x5f024402 as libc::c_int as libc::c_uint,
    0x44024402 as libc::c_int as libc::c_uint,
    0x7ffe4002 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x44220000 as libc::c_int as libc::c_uint,
    0x12482244 as libc::c_int as libc::c_uint,
    0xf3cf0000 as libc::c_uint,
    0x14280420 as libc::c_int as libc::c_uint,
    0x48122424 as libc::c_int as libc::c_uint,
    0x8100810 as libc::c_int as libc::c_uint,
    0x1ff81008 as libc::c_int as libc::c_uint,
    0x3c00420 as libc::c_int as libc::c_uint,
    0xaa00000 as libc::c_int as libc::c_uint,
    0x1ff80aa0 as libc::c_int as libc::c_uint,
    0x1068700e as libc::c_int as libc::c_uint,
    0x1008706e as libc::c_int as libc::c_uint,
    0x1008700e as libc::c_int as libc::c_uint,
    0x1008700e as libc::c_int as libc::c_uint,
    0xaa01ff8 as libc::c_int as libc::c_uint,
    0xaa0 as libc::c_int as libc::c_uint,
    0x7e00000 as libc::c_int as libc::c_uint,
    0x4201db8 as libc::c_int as libc::c_uint,
    0x4a01c38 as libc::c_int as libc::c_uint,
    0x4a01d38 as libc::c_int as libc::c_uint,
    0x4a01d38 as libc::c_int as libc::c_uint,
    0x4a01d38 as libc::c_int as libc::c_uint,
    0x4201d38 as libc::c_int as libc::c_uint,
    0x7e0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3c00000 as libc::c_int as libc::c_uint,
    0x3c382ff0 as libc::c_int as libc::c_uint,
    0x3c04380c as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x3c003c0 as libc::c_int as libc::c_uint,
    0x180 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x1800180 as libc::c_int as libc::c_uint,
    0x1800180 as libc::c_int as libc::c_uint,
    0x3c007e0 as libc::c_int as libc::c_uint,
    0x180 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x3c003c0 as libc::c_int as libc::c_uint,
    0x180 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x7e003c0 as libc::c_int as libc::c_uint,
    0x1800180 as libc::c_int as libc::c_uint,
    0x1800180 as libc::c_int as libc::c_uint,
    0x180 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x3c003c0 as libc::c_int as libc::c_uint,
    0x180 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0xff003c0 as libc::c_int as libc::c_uint,
    0x181c1c34 as libc::c_int as libc::c_uint,
    0x303c301c as libc::c_int as libc::c_uint,
    0x30003000 as libc::c_int as libc::c_uint,
    0x1c301800 as libc::c_int as libc::c_uint,
    0x3c00ff0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x7e003c0 as libc::c_int as libc::c_uint,
    0xff00ff0 as libc::c_int as libc::c_uint,
    0xff00ff0 as libc::c_int as libc::c_uint,
    0x3c007e0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x42003c0 as libc::c_int as libc::c_uint,
    0x8100810 as libc::c_int as libc::c_uint,
    0x8100810 as libc::c_int as libc::c_uint,
    0x3c00420 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x1ff81ff8 as libc::c_int as libc::c_uint,
    0x1ff80000 as libc::c_int as libc::c_uint,
    0x1ff8 as libc::c_int as libc::c_uint,
    0x1ff81ff8 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x880070 as libc::c_int as libc::c_uint,
    0xc880088 as libc::c_int as libc::c_uint,
    0x1e8810f8 as libc::c_int as libc::c_uint,
    0x3e881288 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x2000000 as libc::c_int as libc::c_uint,
    0x7000a80 as libc::c_int as libc::c_uint,
    0x7001fc0 as libc::c_int as libc::c_uint,
    0x2000a80 as libc::c_int as libc::c_uint,
    0x300030 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x42007e as libc::c_int as libc::c_uint,
    0x40027fc2 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x40024002 as libc::c_int as libc::c_uint,
    0x7ffe4002 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x3ff00000 as libc::c_int as libc::c_uint,
    0x201c2010 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x20042004 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x1ff00000 as libc::c_int as libc::c_uint,
    0x20082008 as libc::c_int as libc::c_uint,
    0x17d02fe8 as libc::c_int as libc::c_uint,
    0x5400ba0 as libc::c_int as libc::c_uint,
    0x9200540 as libc::c_int as libc::c_uint,
    0x23881010 as libc::c_int as libc::c_uint,
    0x2fe827c8 as libc::c_int as libc::c_uint,
    0x1ff0 as libc::c_int as libc::c_uint,
    0x1800000 as libc::c_int as libc::c_uint,
    0x2400240 as libc::c_int as libc::c_uint,
    0x5a00420 as libc::c_int as libc::c_uint,
    0x9900990 as libc::c_int as libc::c_uint,
    0x11881188 as libc::c_int as libc::c_uint,
    0x21842004 as libc::c_int as libc::c_uint,
    0x40024182 as libc::c_int as libc::c_uint,
    0x3ffc as libc::c_int as libc::c_uint,
    0x7ffe0000 as libc::c_int as libc::c_uint,
    0x4ff24002 as libc::c_int as libc::c_uint,
    0x4c324ff2 as libc::c_int as libc::c_uint,
    0x4f824c02 as libc::c_int as libc::c_uint,
    0x41824f82 as libc::c_int as libc::c_uint,
    0x41824002 as libc::c_int as libc::c_uint,
    0x40024182 as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0x7ffe0000 as libc::c_int as libc::c_uint,
    0x41824002 as libc::c_int as libc::c_uint,
    0x40024182 as libc::c_int as libc::c_uint,
    0x41824182 as libc::c_int as libc::c_uint,
    0x41824182 as libc::c_int as libc::c_uint,
    0x41824182 as libc::c_int as libc::c_uint,
    0x40024182 as libc::c_int as libc::c_uint,
    0x7ffe as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
];
pub unsafe extern "C" fn GuiLoadIcons(
    mut fileName: *const libc::c_char,
    mut loadIconsName: bool,
) -> *mut *mut libc::c_char {
    let mut rgiFile: *mut FILE = fopen(
        fileName,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    let mut guiIconsName: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if !rgiFile.is_null() {
        let mut signature: [libc::c_char; 5] = [
            0 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
        ];
        let mut version: libc::c_short = 0 as libc::c_int as libc::c_short;
        let mut reserved: libc::c_short = 0 as libc::c_int as libc::c_short;
        let mut iconCount: libc::c_short = 0 as libc::c_int as libc::c_short;
        let mut iconSize: libc::c_short = 0 as libc::c_int as libc::c_short;
        fread(
            signature.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            4 as libc::c_int as libc::c_ulong,
            rgiFile,
        );
        fread(
            &mut version as *mut libc::c_short as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            rgiFile,
        );
        fread(
            &mut reserved as *mut libc::c_short as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            rgiFile,
        );
        fread(
            &mut iconCount as *mut libc::c_short as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            rgiFile,
        );
        fread(
            &mut iconSize as *mut libc::c_short as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            rgiFile,
        );
        if signature[0 as libc::c_int as usize] as libc::c_int == 'r' as i32
            && signature[1 as libc::c_int as usize] as libc::c_int == 'G' as i32
            && signature[2 as libc::c_int as usize] as libc::c_int == 'I' as i32
            && signature[3 as libc::c_int as usize] as libc::c_int == ' ' as i32
        {
            if loadIconsName {
                guiIconsName = malloc(
                    (iconCount as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut *mut libc::c_char>()
                                as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < iconCount as libc::c_int {
                    let ref mut fresh0 = *guiIconsName.offset(i as isize);
                    *fresh0 = malloc(32 as libc::c_int as libc::c_ulong)
                        as *mut libc::c_char;
                    fread(
                        *guiIconsName.offset(i as isize) as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        32 as libc::c_int as libc::c_ulong,
                        rgiFile,
                    );
                    i += 1;
                    i;
                }
            } else {
                fseek(
                    rgiFile,
                    (iconCount as libc::c_int * 32 as libc::c_int) as libc::c_long,
                    1 as libc::c_int,
                );
            }
            fread(
                guiIconsPtr as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                (iconCount as libc::c_int
                    * (iconSize as libc::c_int * iconSize as libc::c_int
                        / 32 as libc::c_int)) as libc::c_ulong,
                rgiFile,
            );
        }
        fclose(rgiFile);
    }
    return guiIconsName;
}
pub unsafe extern "C" fn GuiDrawIcon(
    mut iconId: libc::c_int,
    mut posX: libc::c_int,
    mut posY: libc::c_int,
    mut pixelSize: libc::c_int,
    mut color: Color,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int * 16 as libc::c_int / 32 as libc::c_int {
        let mut k: libc::c_int = 0 as libc::c_int;
        while k < 32 as libc::c_int {
            if *guiIconsPtr
                .offset(
                    (iconId * (16 as libc::c_int * 16 as libc::c_int / 32 as libc::c_int)
                        + i) as isize,
                ) & (1 as libc::c_uint) << k != 0
            {
                GuiDrawRectangle(
                    {
                        let mut init = Rectangle {
                            x: posX as libc::c_float
                                + (k % 16 as libc::c_int * pixelSize) as libc::c_float,
                            y: posY as libc::c_float + (y * pixelSize) as libc::c_float,
                            width: pixelSize as libc::c_float,
                            height: pixelSize as libc::c_float,
                        };
                        init
                    },
                    0 as libc::c_int,
                    {
                        let mut init = Color {
                            r: 0 as libc::c_int as libc::c_uchar,
                            g: 0 as libc::c_int as libc::c_uchar,
                            b: 0 as libc::c_int as libc::c_uchar,
                            a: 0 as libc::c_int as libc::c_uchar,
                        };
                        init
                    },
                    color,
                );
            }
            if k == 15 as libc::c_int || k == 31 as libc::c_int {
                y += 1;
                y;
            }
            k += 1;
            k;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn GuiDrawRectangle(
    mut rec: Rectangle,
    mut borderWidth: libc::c_int,
    mut borderColor: Color,
    mut color: Color,
) {
    if color.a as libc::c_int > 0 as libc::c_int {
        DrawRectangle(
            rec.x as libc::c_int,
            rec.y as libc::c_int,
            rec.width as libc::c_int,
            rec.height as libc::c_int,
            GuiFade(color, guiAlpha),
        );
    }
    if borderWidth > 0 as libc::c_int {
        DrawRectangle(
            rec.x as libc::c_int,
            rec.y as libc::c_int,
            rec.width as libc::c_int,
            borderWidth,
            GuiFade(borderColor, guiAlpha),
        );
        DrawRectangle(
            rec.x as libc::c_int,
            rec.y as libc::c_int + borderWidth,
            borderWidth,
            rec.height as libc::c_int - 2 as libc::c_int * borderWidth,
            GuiFade(borderColor, guiAlpha),
        );
        DrawRectangle(
            rec.x as libc::c_int + rec.width as libc::c_int - borderWidth,
            rec.y as libc::c_int + borderWidth,
            borderWidth,
            rec.height as libc::c_int - 2 as libc::c_int * borderWidth,
            GuiFade(borderColor, guiAlpha),
        );
        DrawRectangle(
            rec.x as libc::c_int,
            rec.y as libc::c_int + rec.height as libc::c_int - borderWidth,
            rec.width as libc::c_int,
            borderWidth,
            GuiFade(borderColor, guiAlpha),
        );
    }
}
unsafe extern "C" fn GuiFade(mut color: Color, mut alpha: libc::c_float) -> Color {
    if alpha < 0.0f32 {
        alpha = 0.0f32;
    } else if alpha > 1.0f32 {
        alpha = 1.0f32;
    }
    let mut result: Color = {
        let mut init = Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: (color.a as libc::c_int as libc::c_float * alpha) as libc::c_uchar,
        };
        init
    };
    return result;
}
pub unsafe extern "C" fn GuiWindowBox(
    mut bounds: Rectangle,
    mut title: *const libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut statusBarHeight: libc::c_int = 24 as libc::c_int;
    let mut statusBar: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: statusBarHeight as libc::c_float,
        };
        init
    };
    if bounds.height < statusBarHeight as libc::c_float * 2.0f32 {
        bounds.height = statusBarHeight as libc::c_float * 2.0f32;
    }
    let mut windowPanel: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x,
            y: bounds.y + statusBarHeight as libc::c_float
                - 1 as libc::c_int as libc::c_float,
            width: bounds.width,
            height: bounds.height - statusBarHeight as libc::c_float
                + 1 as libc::c_int as libc::c_float,
        };
        init
    };
    let mut closeButtonRec: Rectangle = {
        let mut init = Rectangle {
            x: statusBar.x + statusBar.width
                - GuiGetStyle(STATUSBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float - 20 as libc::c_int as libc::c_float,
            y: statusBar.y + statusBarHeight as libc::c_float / 2.0f32
                - 18.0f32 / 2.0f32,
            width: 18 as libc::c_int as libc::c_float,
            height: 18 as libc::c_int as libc::c_float,
        };
        init
    };
    GuiStatusBar(statusBar, title);
    GuiPanel(windowPanel, 0 as *const libc::c_char);
    let mut tempBorderWidth: libc::c_int = GuiGetStyle(
        BUTTON as libc::c_int,
        BORDER_WIDTH as libc::c_int,
    );
    let mut tempTextAlignment: libc::c_int = GuiGetStyle(
        BUTTON as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
    );
    GuiSetStyle(BUTTON as libc::c_int, BORDER_WIDTH as libc::c_int, 1 as libc::c_int);
    GuiSetStyle(
        BUTTON as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_CENTER as libc::c_int,
    );
    result = GuiButton(
        closeButtonRec,
        GuiIconText(ICON_CROSS_SMALL as libc::c_int, 0 as *const libc::c_char),
    );
    GuiSetStyle(BUTTON as libc::c_int, BORDER_WIDTH as libc::c_int, tempBorderWidth);
    GuiSetStyle(BUTTON as libc::c_int, TEXT_ALIGNMENT as libc::c_int, tempTextAlignment);
    return result;
}
pub unsafe extern "C" fn GuiButton(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked && !guiControlExclusiveMode
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if CheckCollisionPointRec(mousePoint, bounds) {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
            } else {
                state = STATE_FOCUSED;
            }
            if IsMouseButtonReleased(MOUSE_BUTTON_LEFT as libc::c_int) {
                result = 1 as libc::c_int;
            }
        }
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(BUTTON as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                BUTTON as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        GetColor(
            GuiGetStyle(
                BUTTON as libc::c_int,
                (BASE as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    GuiDrawText(
        text,
        GetTextBounds(BUTTON as libc::c_int, bounds),
        GuiGetStyle(BUTTON as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
        GetColor(
            GuiGetStyle(
                BUTTON as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    if state as libc::c_uint == STATE_FOCUSED as libc::c_int as libc::c_uint {
        GuiTooltip(bounds);
    }
    return result;
}
unsafe extern "C" fn GuiTooltip(mut controlRec: Rectangle) {
    if !guiLocked && guiTooltip as libc::c_int != 0 && !guiTooltipPtr.is_null()
        && !guiControlExclusiveMode
    {
        let mut textSize: Vector2 = MeasureTextEx(
            GuiGetFont(),
            guiTooltipPtr,
            GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                as libc::c_float,
            GuiGetStyle(DEFAULT as libc::c_int, TEXT_SPACING as libc::c_int)
                as libc::c_float,
        );
        if controlRec.x + textSize.x + 16 as libc::c_int as libc::c_float
            > GetScreenWidth() as libc::c_float
        {
            controlRec.x
                -= textSize.x + 16 as libc::c_int as libc::c_float - controlRec.width;
        }
        GuiPanel(
            {
                let mut init = Rectangle {
                    x: controlRec.x,
                    y: controlRec.y + controlRec.height
                        + 4 as libc::c_int as libc::c_float,
                    width: textSize.x + 16 as libc::c_int as libc::c_float,
                    height: GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                        as libc::c_float + 8.0f32,
                };
                init
            },
            0 as *const libc::c_char,
        );
        let mut textPadding: libc::c_int = GuiGetStyle(
            LABEL as libc::c_int,
            TEXT_PADDING as libc::c_int,
        );
        let mut textAlignment: libc::c_int = GuiGetStyle(
            LABEL as libc::c_int,
            TEXT_ALIGNMENT as libc::c_int,
        );
        GuiSetStyle(LABEL as libc::c_int, TEXT_PADDING as libc::c_int, 0 as libc::c_int);
        GuiSetStyle(
            LABEL as libc::c_int,
            TEXT_ALIGNMENT as libc::c_int,
            TEXT_ALIGN_CENTER as libc::c_int,
        );
        GuiLabel(
            {
                let mut init = Rectangle {
                    x: controlRec.x,
                    y: controlRec.y + controlRec.height
                        + 4 as libc::c_int as libc::c_float,
                    width: textSize.x + 16 as libc::c_int as libc::c_float,
                    height: GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                        as libc::c_float + 8.0f32,
                };
                init
            },
            guiTooltipPtr,
        );
        GuiSetStyle(LABEL as libc::c_int, TEXT_ALIGNMENT as libc::c_int, textAlignment);
        GuiSetStyle(LABEL as libc::c_int, TEXT_PADDING as libc::c_int, textPadding);
    }
}
pub unsafe extern "C" fn GuiLabel(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    GuiDrawText(
        text,
        GetTextBounds(LABEL as libc::c_int, bounds),
        GuiGetStyle(LABEL as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
        GetColor(
            GuiGetStyle(
                LABEL as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    return result;
}
unsafe extern "C" fn GetTextBounds(
    mut control: libc::c_int,
    mut bounds: Rectangle,
) -> Rectangle {
    let mut textBounds: Rectangle = bounds;
    textBounds
        .x = bounds.x
        + GuiGetStyle(control, BORDER_WIDTH as libc::c_int) as libc::c_float;
    textBounds
        .y = bounds.y
        + GuiGetStyle(control, BORDER_WIDTH as libc::c_int) as libc::c_float
        + GuiGetStyle(control, TEXT_PADDING as libc::c_int) as libc::c_float;
    textBounds
        .width = bounds.width
        - (2 as libc::c_int * GuiGetStyle(control, BORDER_WIDTH as libc::c_int))
            as libc::c_float
        - (2 as libc::c_int * GuiGetStyle(control, TEXT_PADDING as libc::c_int))
            as libc::c_float;
    textBounds
        .height = bounds.height
        - (2 as libc::c_int * GuiGetStyle(control, BORDER_WIDTH as libc::c_int))
            as libc::c_float
        - (2 as libc::c_int * GuiGetStyle(control, TEXT_PADDING as libc::c_int))
            as libc::c_float;
    match control {
        7 | 8 | 12 | 4 | 6 | 10 | 11 | _ => {}
    }
    if GuiGetStyle(control, TEXT_ALIGNMENT as libc::c_int)
        == TEXT_ALIGN_RIGHT as libc::c_int
    {
        textBounds.x
            -= GuiGetStyle(control, TEXT_PADDING as libc::c_int) as libc::c_float;
    } else {
        textBounds.x
            += GuiGetStyle(control, TEXT_PADDING as libc::c_int) as libc::c_float;
    }
    return textBounds;
}
unsafe extern "C" fn GuiDrawText(
    mut text: *const libc::c_char,
    mut textBounds: Rectangle,
    mut alignment: libc::c_int,
    mut tint: Color,
) {
    if text.is_null()
        || *text.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return;
    }
    let mut lineCount: libc::c_int = 0 as libc::c_int;
    let mut lines: *mut *const libc::c_char = GetTextLines(text, &mut lineCount);
    let mut alignmentVertical: libc::c_int = GuiGetStyle(
        DEFAULT as libc::c_int,
        TEXT_ALIGNMENT_VERTICAL as libc::c_int,
    );
    let mut wrapMode: libc::c_int = GuiGetStyle(
        DEFAULT as libc::c_int,
        TEXT_WRAP_MODE as libc::c_int,
    );
    let mut totalHeight: libc::c_float = (lineCount
        * GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
        + (lineCount - 1 as libc::c_int)
            * GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
            / 2 as libc::c_int) as libc::c_float;
    let mut posOffsetY: libc::c_float = 0.0f32;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < lineCount {
        let mut iconId: libc::c_int = 0 as libc::c_int;
        let ref mut fresh1 = *lines.offset(i as isize);
        *fresh1 = GetTextIcon(*lines.offset(i as isize), &mut iconId);
        let mut textBoundsPosition: Vector2 = {
            let mut init = Vector2 {
                x: textBounds.x,
                y: textBounds.y,
            };
            init
        };
        let mut textBoundsWidthOffset: libc::c_float = 0.0f32;
        let mut textSizeX: libc::c_int = GetTextWidth(*lines.offset(i as isize));
        if iconId >= 0 as libc::c_int {
            textSizeX = (textSizeX as libc::c_uint)
                .wrapping_add(
                    (16 as libc::c_int as libc::c_uint).wrapping_mul(guiIconScale),
                ) as libc::c_int as libc::c_int;
            if !(*lines.offset(i as isize)).is_null()
                && *(*lines.offset(i as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int != '\0' as i32
            {
                textSizeX += 4 as libc::c_int;
            }
        }
        match alignment {
            0 => {
                textBoundsPosition.x = textBounds.x;
            }
            1 => {
                textBoundsPosition
                    .x = textBounds.x
                    + textBounds.width / 2 as libc::c_int as libc::c_float
                    - (textSizeX / 2 as libc::c_int) as libc::c_float;
            }
            2 => {
                textBoundsPosition
                    .x = textBounds.x + textBounds.width - textSizeX as libc::c_float;
            }
            _ => {}
        }
        if textSizeX as libc::c_float > textBounds.width
            && !(*lines.offset(i as isize)).is_null()
            && *(*lines.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int != '\0' as i32
        {
            textBoundsPosition.x = textBounds.x;
        }
        match alignmentVertical {
            0 => {
                textBoundsPosition.y = textBounds.y + posOffsetY;
            }
            1 => {
                textBoundsPosition
                    .y = textBounds.y + posOffsetY
                    + textBounds.height / 2 as libc::c_int as libc::c_float
                    - totalHeight / 2 as libc::c_int as libc::c_float
                    + (textBounds.height as libc::c_int % 2 as libc::c_int)
                        as libc::c_float;
            }
            2 => {
                textBoundsPosition
                    .y = textBounds.y + posOffsetY + textBounds.height - totalHeight
                    + (textBounds.height as libc::c_int % 2 as libc::c_int)
                        as libc::c_float;
            }
            _ => {}
        }
        textBoundsPosition.x = textBoundsPosition.x as libc::c_int as libc::c_float;
        textBoundsPosition.y = textBoundsPosition.y as libc::c_int as libc::c_float;
        if iconId >= 0 as libc::c_int {
            GuiDrawIcon(
                iconId,
                textBoundsPosition.x as libc::c_int,
                (textBounds.y + textBounds.height / 2 as libc::c_int as libc::c_float
                    - (16 as libc::c_int as libc::c_uint)
                        .wrapping_mul(guiIconScale)
                        .wrapping_div(2 as libc::c_int as libc::c_uint) as libc::c_float
                    + (textBounds.height as libc::c_int % 2 as libc::c_int)
                        as libc::c_float) as libc::c_int,
                guiIconScale as libc::c_int,
                tint,
            );
            textBoundsPosition.x
                += (16 as libc::c_int as libc::c_uint)
                    .wrapping_mul(guiIconScale)
                    .wrapping_add(4 as libc::c_int as libc::c_uint) as libc::c_float;
            textBoundsWidthOffset = (16 as libc::c_int as libc::c_uint)
                .wrapping_mul(guiIconScale)
                .wrapping_add(4 as libc::c_int as libc::c_uint) as libc::c_float;
        }
        let mut lineSize: libc::c_int = 0 as libc::c_int;
        let mut c: libc::c_int = 0 as libc::c_int;
        while *(*lines.offset(i as isize)).offset(c as isize) as libc::c_int
            != '\0' as i32
            && *(*lines.offset(i as isize)).offset(c as isize) as libc::c_int
                != '\n' as i32
            && *(*lines.offset(i as isize)).offset(c as isize) as libc::c_int
                != '\r' as i32
        {
            c += 1;
            c;
            lineSize += 1;
            lineSize;
        }
        let mut scaleFactor: libc::c_float = GuiGetStyle(
            DEFAULT as libc::c_int,
            TEXT_SIZE as libc::c_int,
        ) as libc::c_float / guiFont.baseSize as libc::c_float;
        let mut lastSpaceIndex: libc::c_int = 0 as libc::c_int;
        let mut tempWrapCharMode: bool = 0 as libc::c_int != 0;
        let mut textOffsetY: libc::c_int = 0 as libc::c_int;
        let mut textOffsetX: libc::c_float = 0.0f32;
        let mut glyphWidth: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut ellipsisWidth: libc::c_int = GetTextWidth(
            b"...\0" as *const u8 as *const libc::c_char,
        );
        let mut overflowReached: bool = 0 as libc::c_int != 0;
        let mut c_0: libc::c_int = 0 as libc::c_int;
        let mut codepointSize: libc::c_int = 0 as libc::c_int;
        while c_0 < lineSize {
            let mut codepoint: libc::c_int = GetCodepointNext(
                &*(*lines.offset(i as isize)).offset(c_0 as isize),
                &mut codepointSize,
            );
            let mut index: libc::c_int = GetGlyphIndex(guiFont, codepoint);
            if codepoint == 0x3f as libc::c_int {
                codepointSize = 1 as libc::c_int;
            }
            if (*(guiFont.glyphs).offset(index as isize)).advanceX == 0 as libc::c_int {
                glyphWidth = (*(guiFont.recs).offset(index as isize)).width
                    * scaleFactor;
            } else {
                glyphWidth = (*(guiFont.glyphs).offset(index as isize)).advanceX
                    as libc::c_float * scaleFactor;
            }
            if wrapMode == TEXT_WRAP_CHAR as libc::c_int {
                if textOffsetX + glyphWidth > textBounds.width - textBoundsWidthOffset {
                    textOffsetX = 0.0f32;
                    textOffsetY
                        += GuiGetStyle(
                            DEFAULT as libc::c_int,
                            TEXT_LINE_SPACING as libc::c_int,
                        );
                    if tempWrapCharMode {
                        wrapMode = TEXT_WRAP_WORD as libc::c_int;
                        tempWrapCharMode = 0 as libc::c_int != 0;
                    }
                }
            } else if wrapMode == TEXT_WRAP_WORD as libc::c_int {
                if codepoint == 32 as libc::c_int {
                    lastSpaceIndex = c_0;
                }
                let mut nextSpaceIndex: libc::c_int = 0 as libc::c_int;
                let mut nextSpaceWidth: libc::c_float = GetNextSpaceWidth(
                    (*lines.offset(i as isize)).offset(c_0 as isize),
                    &mut nextSpaceIndex,
                );
                let mut nextSpaceIndex2: libc::c_int = 0 as libc::c_int;
                let mut nextWordSize: libc::c_float = GetNextSpaceWidth(
                    (*lines.offset(i as isize))
                        .offset(lastSpaceIndex as isize)
                        .offset(1 as libc::c_int as isize),
                    &mut nextSpaceIndex2,
                );
                if nextWordSize > textBounds.width - textBoundsWidthOffset {
                    tempWrapCharMode = 1 as libc::c_int != 0;
                    wrapMode = TEXT_WRAP_CHAR as libc::c_int;
                } else if textOffsetX + nextSpaceWidth
                    > textBounds.width - textBoundsWidthOffset
                {
                    textOffsetX = 0.0f32;
                    textOffsetY
                        += GuiGetStyle(
                            DEFAULT as libc::c_int,
                            TEXT_LINE_SPACING as libc::c_int,
                        );
                }
            }
            if codepoint == '\n' as i32 {
                break;
            }
            if codepoint != ' ' as i32 && codepoint != '\t' as i32 {
                if wrapMode == TEXT_WRAP_NONE as libc::c_int {
                    if textSizeX as libc::c_float > textBounds.width {
                        if textOffsetX
                            <= textBounds.width - glyphWidth - textBoundsWidthOffset
                                - ellipsisWidth as libc::c_float
                        {
                            DrawTextCodepoint(
                                guiFont,
                                codepoint,
                                {
                                    let mut init = Vector2 {
                                        x: textBoundsPosition.x + textOffsetX,
                                        y: textBoundsPosition.y + textOffsetY as libc::c_float,
                                    };
                                    init
                                },
                                GuiGetStyle(
                                    DEFAULT as libc::c_int,
                                    TEXT_SIZE as libc::c_int,
                                ) as libc::c_float,
                                GuiFade(tint, guiAlpha),
                            );
                        } else if !overflowReached {
                            overflowReached = 1 as libc::c_int != 0;
                            let mut j: libc::c_int = 0 as libc::c_int;
                            while j < ellipsisWidth {
                                DrawTextCodepoint(
                                    guiFont,
                                    '.' as i32,
                                    {
                                        let mut init = Vector2 {
                                            x: textBoundsPosition.x + textOffsetX + j as libc::c_float,
                                            y: textBoundsPosition.y + textOffsetY as libc::c_float,
                                        };
                                        init
                                    },
                                    GuiGetStyle(
                                        DEFAULT as libc::c_int,
                                        TEXT_SIZE as libc::c_int,
                                    ) as libc::c_float,
                                    GuiFade(tint, guiAlpha),
                                );
                                j += ellipsisWidth / 3 as libc::c_int;
                            }
                        }
                    } else {
                        DrawTextCodepoint(
                            guiFont,
                            codepoint,
                            {
                                let mut init = Vector2 {
                                    x: textBoundsPosition.x + textOffsetX,
                                    y: textBoundsPosition.y + textOffsetY as libc::c_float,
                                };
                                init
                            },
                            GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                                as libc::c_float,
                            GuiFade(tint, guiAlpha),
                        );
                    }
                } else if wrapMode == TEXT_WRAP_CHAR as libc::c_int
                    || wrapMode == TEXT_WRAP_WORD as libc::c_int
                {
                    if textBoundsPosition.y + textOffsetY as libc::c_float
                        <= textBounds.y + textBounds.height
                            - GuiGetStyle(
                                DEFAULT as libc::c_int,
                                TEXT_SIZE as libc::c_int,
                            ) as libc::c_float
                    {
                        DrawTextCodepoint(
                            guiFont,
                            codepoint,
                            {
                                let mut init = Vector2 {
                                    x: textBoundsPosition.x + textOffsetX,
                                    y: textBoundsPosition.y + textOffsetY as libc::c_float,
                                };
                                init
                            },
                            GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                                as libc::c_float,
                            GuiFade(tint, guiAlpha),
                        );
                    }
                }
            }
            if (*(guiFont.glyphs).offset(index as isize)).advanceX == 0 as libc::c_int {
                textOffsetX
                    += (*(guiFont.recs).offset(index as isize)).width * scaleFactor
                        + GuiGetStyle(
                            DEFAULT as libc::c_int,
                            TEXT_SPACING as libc::c_int,
                        ) as libc::c_float;
            } else {
                textOffsetX
                    += (*(guiFont.glyphs).offset(index as isize)).advanceX
                        as libc::c_float * scaleFactor
                        + GuiGetStyle(
                            DEFAULT as libc::c_int,
                            TEXT_SPACING as libc::c_int,
                        ) as libc::c_float;
            }
            c_0 += codepointSize;
        }
        if wrapMode == TEXT_WRAP_NONE as libc::c_int {
            posOffsetY
                += GuiGetStyle(DEFAULT as libc::c_int, TEXT_LINE_SPACING as libc::c_int)
                    as libc::c_float;
        } else if wrapMode == TEXT_WRAP_CHAR as libc::c_int
            || wrapMode == TEXT_WRAP_WORD as libc::c_int
        {
            posOffsetY
                += textOffsetY as libc::c_float
                    + GuiGetStyle(
                        DEFAULT as libc::c_int,
                        TEXT_LINE_SPACING as libc::c_int,
                    ) as libc::c_float;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn GetTextLines(
    mut text: *const libc::c_char,
    mut count: *mut libc::c_int,
) -> *mut *const libc::c_char {
    static mut lines: [*const libc::c_char; 128] = [
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        lines[i as usize] = 0 as *const libc::c_char;
        i += 1;
        i;
    }
    let mut textSize: libc::c_int = strlen(text) as libc::c_int;
    lines[0 as libc::c_int as usize] = text;
    let mut len: libc::c_int = 0 as libc::c_int;
    *count = 1 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    while i_0 < textSize && *count < 128 as libc::c_int {
        if *text.offset(i_0 as isize) as libc::c_int == '\n' as i32 {
            k += 1;
            k;
            lines[k
                as usize] = &*text.offset((i_0 + 1 as libc::c_int) as isize)
                as *const libc::c_char;
            len = 0 as libc::c_int;
            *count += 1 as libc::c_int;
        } else {
            len += 1;
            len;
        }
        i_0 += 1;
        i_0;
    }
    return lines.as_mut_ptr();
}
unsafe extern "C" fn GetTextWidth(mut text: *const libc::c_char) -> libc::c_int {
    let mut textSize: Vector2 = {
        let mut init = Vector2 {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
        };
        init
    };
    let mut textIconOffset: libc::c_int = 0 as libc::c_int;
    if !text.is_null()
        && *text.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        if *text.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
            let mut i: libc::c_int = 1 as libc::c_int;
            while i < 5 as libc::c_int
                && *text.offset(i as isize) as libc::c_int != '\0' as i32
            {
                if *text.offset(i as isize) as libc::c_int == '#' as i32 {
                    textIconOffset = i;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
        text = text.offset(textIconOffset as isize);
        let mut fontSize: libc::c_float = GuiGetStyle(
            DEFAULT as libc::c_int,
            TEXT_SIZE as libc::c_int,
        ) as libc::c_float;
        if guiFont.texture.id > 0 as libc::c_int as libc::c_uint && !text.is_null() {
            let mut size: libc::c_int = 0 as libc::c_int;
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 256 as libc::c_int {
                if !(*text.offset(i_0 as isize) as libc::c_int != '\0' as i32
                    && *text.offset(i_0 as isize) as libc::c_int != '\n' as i32)
                {
                    break;
                }
                size += 1;
                size;
                i_0 += 1;
                i_0;
            }
            let mut scaleFactor: libc::c_float = fontSize
                / guiFont.baseSize as libc::c_float;
            textSize.y = guiFont.baseSize as libc::c_float * scaleFactor;
            let mut glyphWidth: libc::c_float = 0.0f32;
            let mut i_1: libc::c_int = 0 as libc::c_int;
            let mut codepointSize: libc::c_int = 0 as libc::c_int;
            while i_1 < size {
                let mut codepoint: libc::c_int = GetCodepointNext(
                    &*text.offset(i_1 as isize),
                    &mut codepointSize,
                );
                let mut codepointIndex: libc::c_int = GetGlyphIndex(guiFont, codepoint);
                if (*(guiFont.glyphs).offset(codepointIndex as isize)).advanceX
                    == 0 as libc::c_int
                {
                    glyphWidth = (*(guiFont.recs).offset(codepointIndex as isize)).width
                        * scaleFactor;
                } else {
                    glyphWidth = (*(guiFont.glyphs).offset(codepointIndex as isize))
                        .advanceX as libc::c_float * scaleFactor;
                }
                textSize.x
                    += glyphWidth
                        + GuiGetStyle(
                            DEFAULT as libc::c_int,
                            TEXT_SPACING as libc::c_int,
                        ) as libc::c_float;
                i_1 += codepointSize;
            }
        }
        if textIconOffset > 0 as libc::c_int {
            textSize.x += (16 as libc::c_int - 4 as libc::c_int) as libc::c_float;
        }
    }
    return textSize.x as libc::c_int;
}
unsafe extern "C" fn GetNextSpaceWidth(
    mut text: *const libc::c_char,
    mut nextSpaceIndex: *mut libc::c_int,
) -> libc::c_float {
    let mut width: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut codepointByteCount: libc::c_int = 0 as libc::c_int;
    let mut codepoint: libc::c_int = 0 as libc::c_int;
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut glyphWidth: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut scaleFactor: libc::c_float = GuiGetStyle(
        DEFAULT as libc::c_int,
        TEXT_SIZE as libc::c_int,
    ) as libc::c_float / guiFont.baseSize as libc::c_float;
    let mut i: libc::c_int = 0 as libc::c_int;
    while *text.offset(i as isize) as libc::c_int != '\0' as i32 {
        if *text.offset(i as isize) as libc::c_int != ' ' as i32 {
            codepoint = GetCodepoint(&*text.offset(i as isize), &mut codepointByteCount);
            index = GetGlyphIndex(guiFont, codepoint);
            glyphWidth = if (*(guiFont.glyphs).offset(index as isize)).advanceX
                == 0 as libc::c_int
            {
                (*(guiFont.recs).offset(index as isize)).width * scaleFactor
            } else {
                (*(guiFont.glyphs).offset(index as isize)).advanceX as libc::c_float
                    * scaleFactor
            };
            width
                += glyphWidth
                    + GuiGetStyle(DEFAULT as libc::c_int, TEXT_SPACING as libc::c_int)
                        as libc::c_float;
            i += 1;
            i;
        } else {
            *nextSpaceIndex = i;
            break;
        }
    }
    return width;
}
unsafe extern "C" fn GetTextIcon(
    mut text: *const libc::c_char,
    mut iconId: *mut libc::c_int,
) -> *const libc::c_char {
    *iconId = -(1 as libc::c_int);
    if *text.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
        let mut iconValue: [libc::c_char; 4] = [
            0 as libc::c_int as libc::c_char,
            0,
            0,
            0,
        ];
        let mut pos: libc::c_int = 1 as libc::c_int;
        while pos < 4 as libc::c_int
            && *text.offset(pos as isize) as libc::c_int >= '0' as i32
            && *text.offset(pos as isize) as libc::c_int <= '9' as i32
        {
            iconValue[(pos - 1 as libc::c_int) as usize] = *text.offset(pos as isize);
            pos += 1;
            pos;
        }
        if *text.offset(pos as isize) as libc::c_int == '#' as i32 {
            *iconId = TextToInteger(iconValue.as_mut_ptr());
            if *iconId >= 0 as libc::c_int {
                text = text.offset((pos + 1 as libc::c_int) as isize);
            }
        }
    }
    return text;
}
pub unsafe extern "C" fn GuiPanel(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut statusBar: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: 24 as libc::c_int as libc::c_float,
        };
        init
    };
    if !text.is_null() && bounds.height < 24 as libc::c_int as libc::c_float * 2.0f32 {
        bounds.height = 24 as libc::c_int as libc::c_float * 2.0f32;
    }
    if !text.is_null() {
        bounds.y
            += 24 as libc::c_int as libc::c_float - 1 as libc::c_int as libc::c_float;
        bounds.height
            -= 24 as libc::c_int as libc::c_float - 1 as libc::c_int as libc::c_float;
    }
    if !text.is_null() {
        GuiStatusBar(statusBar, text);
    }
    GuiDrawRectangle(
        bounds,
        1 as libc::c_int,
        GetColor(
            GuiGetStyle(
                DEFAULT as libc::c_int,
                if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint
                {
                    BORDER_COLOR_DISABLED as libc::c_int
                } else {
                    LINE_COLOR as libc::c_int
                },
            ) as libc::c_uint,
        ),
        GetColor(
            GuiGetStyle(
                DEFAULT as libc::c_int,
                if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint
                {
                    BASE_COLOR_DISABLED as libc::c_int
                } else {
                    BACKGROUND_COLOR as libc::c_int
                },
            ) as libc::c_uint,
        ),
    );
    return result;
}
pub unsafe extern "C" fn GuiStatusBar(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(STATUSBAR as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                STATUSBAR as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        GetColor(
            GuiGetStyle(
                STATUSBAR as libc::c_int,
                (BASE as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    GuiDrawText(
        text,
        GetTextBounds(STATUSBAR as libc::c_int, bounds),
        GuiGetStyle(STATUSBAR as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
        GetColor(
            GuiGetStyle(
                STATUSBAR as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    return result;
}
static mut guiControlExclusiveMode: bool = 0 as libc::c_int != 0;
pub unsafe extern "C" fn GuiGroupBox(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    GuiDrawRectangle(
        {
            let mut init = Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: 1 as libc::c_int as libc::c_float,
                height: bounds.height,
            };
            init
        },
        0 as libc::c_int,
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
        GetColor(
            GuiGetStyle(
                DEFAULT as libc::c_int,
                if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint
                {
                    BORDER_COLOR_DISABLED as libc::c_int
                } else {
                    LINE_COLOR as libc::c_int
                },
            ) as libc::c_uint,
        ),
    );
    GuiDrawRectangle(
        {
            let mut init = Rectangle {
                x: bounds.x,
                y: bounds.y + bounds.height - 1 as libc::c_int as libc::c_float,
                width: bounds.width,
                height: 1 as libc::c_int as libc::c_float,
            };
            init
        },
        0 as libc::c_int,
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
        GetColor(
            GuiGetStyle(
                DEFAULT as libc::c_int,
                if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint
                {
                    BORDER_COLOR_DISABLED as libc::c_int
                } else {
                    LINE_COLOR as libc::c_int
                },
            ) as libc::c_uint,
        ),
    );
    GuiDrawRectangle(
        {
            let mut init = Rectangle {
                x: bounds.x + bounds.width - 1 as libc::c_int as libc::c_float,
                y: bounds.y,
                width: 1 as libc::c_int as libc::c_float,
                height: bounds.height,
            };
            init
        },
        0 as libc::c_int,
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
        GetColor(
            GuiGetStyle(
                DEFAULT as libc::c_int,
                if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint
                {
                    BORDER_COLOR_DISABLED as libc::c_int
                } else {
                    LINE_COLOR as libc::c_int
                },
            ) as libc::c_uint,
        ),
    );
    GuiLine(
        {
            let mut init = Rectangle {
                x: bounds.x,
                y: bounds.y
                    - (GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                        / 2 as libc::c_int) as libc::c_float,
                width: bounds.width,
                height: GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                    as libc::c_float,
            };
            init
        },
        text,
    );
    return result;
}
pub unsafe extern "C" fn GuiLine(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut color: Color = GetColor(
        GuiGetStyle(
            DEFAULT as libc::c_int,
            if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint {
                BORDER_COLOR_DISABLED as libc::c_int
            } else {
                LINE_COLOR as libc::c_int
            },
        ) as libc::c_uint,
    );
    if text.is_null() {
        GuiDrawRectangle(
            {
                let mut init = Rectangle {
                    x: bounds.x,
                    y: bounds.y + bounds.height / 2 as libc::c_int as libc::c_float,
                    width: bounds.width,
                    height: 1 as libc::c_int as libc::c_float,
                };
                init
            },
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            color,
        );
    } else {
        let mut textBounds: Rectangle = {
            let mut init = Rectangle {
                x: 0 as libc::c_int as libc::c_float,
                y: 0.,
                width: 0.,
                height: 0.,
            };
            init
        };
        textBounds
            .width = GetTextWidth(text) as libc::c_float
            + 2 as libc::c_int as libc::c_float;
        textBounds.height = bounds.height;
        textBounds.x = bounds.x + 12 as libc::c_int as libc::c_float;
        textBounds.y = bounds.y;
        GuiDrawRectangle(
            {
                let mut init = Rectangle {
                    x: bounds.x,
                    y: bounds.y + bounds.height / 2 as libc::c_int as libc::c_float,
                    width: (12 as libc::c_int - 4 as libc::c_int) as libc::c_float,
                    height: 1 as libc::c_int as libc::c_float,
                };
                init
            },
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            color,
        );
        GuiDrawText(text, textBounds, TEXT_ALIGN_LEFT as libc::c_int, color);
        GuiDrawRectangle(
            {
                let mut init = Rectangle {
                    x: bounds.x + 12 as libc::c_int as libc::c_float + textBounds.width
                        + 4 as libc::c_int as libc::c_float,
                    y: bounds.y + bounds.height / 2 as libc::c_int as libc::c_float,
                    width: bounds.width - textBounds.width
                        - 12 as libc::c_int as libc::c_float
                        - 4 as libc::c_int as libc::c_float,
                    height: 1 as libc::c_int as libc::c_float,
                };
                init
            },
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            color,
        );
    }
    return result;
}
pub unsafe extern "C" fn GuiTabBar(
    mut bounds: Rectangle,
    mut text: *mut *const libc::c_char,
    mut count: libc::c_int,
    mut active: *mut libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = -(1 as libc::c_int);
    let mut tabBounds: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: 160 as libc::c_int as libc::c_float,
            height: bounds.height,
        };
        init
    };
    if *active < 0 as libc::c_int {
        *active = 0 as libc::c_int;
    } else if *active > count - 1 as libc::c_int {
        *active = count - 1 as libc::c_int;
    }
    let mut offsetX: libc::c_int = 0 as libc::c_int;
    offsetX = (*active + 2 as libc::c_int) * 160 as libc::c_int - GetScreenWidth();
    if offsetX < 0 as libc::c_int {
        offsetX = 0 as libc::c_int;
    }
    let mut toggle: bool = 0 as libc::c_int != 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        tabBounds
            .x = bounds.x
            + ((160 as libc::c_int + 4 as libc::c_int) * i) as libc::c_float
            - offsetX as libc::c_float;
        if tabBounds.x < GetScreenWidth() as libc::c_float {
            let mut textAlignment: libc::c_int = GuiGetStyle(
                TOGGLE as libc::c_int,
                TEXT_ALIGNMENT as libc::c_int,
            );
            let mut textPadding: libc::c_int = GuiGetStyle(
                TOGGLE as libc::c_int,
                TEXT_PADDING as libc::c_int,
            );
            GuiSetStyle(
                TOGGLE as libc::c_int,
                TEXT_ALIGNMENT as libc::c_int,
                TEXT_ALIGN_LEFT as libc::c_int,
            );
            GuiSetStyle(
                TOGGLE as libc::c_int,
                TEXT_PADDING as libc::c_int,
                8 as libc::c_int,
            );
            if i == *active {
                toggle = 1 as libc::c_int != 0;
                GuiToggle(
                    tabBounds,
                    GuiIconText(12 as libc::c_int, *text.offset(i as isize)),
                    &mut toggle,
                );
            } else {
                toggle = 0 as libc::c_int != 0;
                GuiToggle(
                    tabBounds,
                    GuiIconText(12 as libc::c_int, *text.offset(i as isize)),
                    &mut toggle,
                );
                if toggle {
                    *active = i;
                }
            }
            if CheckCollisionPointRec(GetMousePosition(), tabBounds) as libc::c_int != 0
                && IsMouseButtonPressed(MOUSE_BUTTON_MIDDLE as libc::c_int)
                    as libc::c_int != 0
            {
                result = i;
            }
            GuiSetStyle(TOGGLE as libc::c_int, TEXT_PADDING as libc::c_int, textPadding);
            GuiSetStyle(
                TOGGLE as libc::c_int,
                TEXT_ALIGNMENT as libc::c_int,
                textAlignment,
            );
            let mut tempBorderWidth: libc::c_int = GuiGetStyle(
                BUTTON as libc::c_int,
                BORDER_WIDTH as libc::c_int,
            );
            let mut tempTextAlignment: libc::c_int = GuiGetStyle(
                BUTTON as libc::c_int,
                TEXT_ALIGNMENT as libc::c_int,
            );
            GuiSetStyle(
                BUTTON as libc::c_int,
                BORDER_WIDTH as libc::c_int,
                1 as libc::c_int,
            );
            GuiSetStyle(
                BUTTON as libc::c_int,
                TEXT_ALIGNMENT as libc::c_int,
                TEXT_ALIGN_CENTER as libc::c_int,
            );
            if GuiButton(
                {
                    let mut init = Rectangle {
                        x: tabBounds.x + tabBounds.width
                            - 14 as libc::c_int as libc::c_float
                            - 5 as libc::c_int as libc::c_float,
                        y: tabBounds.y + 5 as libc::c_int as libc::c_float,
                        width: 14 as libc::c_int as libc::c_float,
                        height: 14 as libc::c_int as libc::c_float,
                    };
                    init
                },
                GuiIconText(ICON_CROSS_SMALL as libc::c_int, 0 as *const libc::c_char),
            ) != 0
            {
                result = i;
            }
            GuiSetStyle(
                BUTTON as libc::c_int,
                BORDER_WIDTH as libc::c_int,
                tempBorderWidth,
            );
            GuiSetStyle(
                BUTTON as libc::c_int,
                TEXT_ALIGNMENT as libc::c_int,
                tempTextAlignment,
            );
        }
        i += 1;
        i;
    }
    GuiDrawRectangle(
        {
            let mut init = Rectangle {
                x: bounds.x,
                y: bounds.y + bounds.height - 1 as libc::c_int as libc::c_float,
                width: bounds.width,
                height: 1 as libc::c_int as libc::c_float,
            };
            init
        },
        0 as libc::c_int,
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
        GetColor(
            GuiGetStyle(TOGGLE as libc::c_int, BORDER_COLOR_NORMAL as libc::c_int)
                as libc::c_uint,
        ),
    );
    return result;
}
static mut guiStyle: [libc::c_uint; 384] = [
    0 as libc::c_int as libc::c_uint,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub unsafe extern "C" fn GuiToggle(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut active: *mut bool,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut temp: bool = 0 as libc::c_int != 0;
    if active.is_null() {
        active = &mut temp;
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked && !guiControlExclusiveMode
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if CheckCollisionPointRec(mousePoint, bounds) {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
            } else if IsMouseButtonReleased(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_NORMAL;
                *active = !*active;
            } else {
                state = STATE_FOCUSED;
            }
        }
    }
    if state as libc::c_uint == STATE_NORMAL as libc::c_int as libc::c_uint {
        GuiDrawRectangle(
            bounds,
            GuiGetStyle(TOGGLE as libc::c_int, BORDER_WIDTH as libc::c_int),
            GetColor(
                GuiGetStyle(
                    TOGGLE as libc::c_int,
                    (if *active as libc::c_int != 0 {
                        BORDER_COLOR_PRESSED as libc::c_int as libc::c_uint
                    } else {
                        (BORDER as libc::c_int as libc::c_uint)
                            .wrapping_add(
                                (state as libc::c_uint)
                                    .wrapping_mul(3 as libc::c_int as libc::c_uint),
                            )
                    }) as libc::c_int,
                ) as libc::c_uint,
            ),
            GetColor(
                GuiGetStyle(
                    TOGGLE as libc::c_int,
                    (if *active as libc::c_int != 0 {
                        BASE_COLOR_PRESSED as libc::c_int as libc::c_uint
                    } else {
                        (BASE as libc::c_int as libc::c_uint)
                            .wrapping_add(
                                (state as libc::c_uint)
                                    .wrapping_mul(3 as libc::c_int as libc::c_uint),
                            )
                    }) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
        GuiDrawText(
            text,
            GetTextBounds(TOGGLE as libc::c_int, bounds),
            GuiGetStyle(TOGGLE as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
            GetColor(
                GuiGetStyle(
                    TOGGLE as libc::c_int,
                    (if *active as libc::c_int != 0 {
                        TEXT_COLOR_PRESSED as libc::c_int as libc::c_uint
                    } else {
                        (TEXT as libc::c_int as libc::c_uint)
                            .wrapping_add(
                                (state as libc::c_uint)
                                    .wrapping_mul(3 as libc::c_int as libc::c_uint),
                            )
                    }) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
    } else {
        GuiDrawRectangle(
            bounds,
            GuiGetStyle(TOGGLE as libc::c_int, BORDER_WIDTH as libc::c_int),
            GetColor(
                GuiGetStyle(
                    TOGGLE as libc::c_int,
                    (BORDER as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
            GetColor(
                GuiGetStyle(
                    TOGGLE as libc::c_int,
                    (BASE as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
        GuiDrawText(
            text,
            GetTextBounds(TOGGLE as libc::c_int, bounds),
            GuiGetStyle(TOGGLE as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
            GetColor(
                GuiGetStyle(
                    TOGGLE as libc::c_int,
                    (TEXT as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
    }
    if state as libc::c_uint == STATE_FOCUSED as libc::c_int as libc::c_uint {
        GuiTooltip(bounds);
    }
    return result;
}
pub unsafe extern "C" fn GuiScrollPanel(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut content: Rectangle,
    mut scroll: *mut Vector2,
    mut view: *mut Rectangle,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut temp: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    if view.is_null() {
        view = &mut temp;
    }
    let mut scrollPos: Vector2 = {
        let mut init = Vector2 { x: 0.0f32, y: 0.0f32 };
        init
    };
    if !scroll.is_null() {
        scrollPos = *scroll;
    }
    let mut statusBar: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: 24 as libc::c_int as libc::c_float,
        };
        init
    };
    if bounds.height < 24 as libc::c_int as libc::c_float * 2.0f32 {
        bounds.height = 24 as libc::c_int as libc::c_float * 2.0f32;
    }
    if !text.is_null() {
        bounds.y
            += 24 as libc::c_int as libc::c_float - 1 as libc::c_int as libc::c_float;
        bounds.height
            -= 24 as libc::c_int as libc::c_float + 1 as libc::c_int as libc::c_float;
    }
    let mut hasHorizontalScrollBar: bool = if content.width
        > bounds.width
            - (2 as libc::c_int
                * GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int))
                as libc::c_float
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
    let mut hasVerticalScrollBar: bool = if content.height
        > bounds.height
            - (2 as libc::c_int
                * GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int))
                as libc::c_float
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
    if !hasHorizontalScrollBar {
        hasHorizontalScrollBar = if hasVerticalScrollBar as libc::c_int != 0
            && content.width
                > bounds.width
                    - (2 as libc::c_int
                        * GuiGetStyle(
                            DEFAULT as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )) as libc::c_float
                    - GuiGetStyle(
                        LISTVIEW as libc::c_int,
                        SCROLLBAR_WIDTH as libc::c_int,
                    ) as libc::c_float
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0;
    }
    if !hasVerticalScrollBar {
        hasVerticalScrollBar = if hasHorizontalScrollBar as libc::c_int != 0
            && content.height
                > bounds.height
                    - (2 as libc::c_int
                        * GuiGetStyle(
                            DEFAULT as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )) as libc::c_float
                    - GuiGetStyle(
                        LISTVIEW as libc::c_int,
                        SCROLLBAR_WIDTH as libc::c_int,
                    ) as libc::c_float
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0;
    }
    let mut horizontalScrollBarWidth: libc::c_int = if hasHorizontalScrollBar
        as libc::c_int != 0
    {
        GuiGetStyle(LISTVIEW as libc::c_int, SCROLLBAR_WIDTH as libc::c_int)
    } else {
        0 as libc::c_int
    };
    let mut verticalScrollBarWidth: libc::c_int = if hasVerticalScrollBar as libc::c_int
        != 0
    {
        GuiGetStyle(LISTVIEW as libc::c_int, SCROLLBAR_WIDTH as libc::c_int)
    } else {
        0 as libc::c_int
    };
    let mut horizontalScrollBar: Rectangle = {
        let mut init = Rectangle {
            x: (if GuiGetStyle(LISTVIEW as libc::c_int, SCROLLBAR_SIDE as libc::c_int)
                == 0 as libc::c_int
            {
                bounds.x + verticalScrollBarWidth as libc::c_float
            } else {
                bounds.x
            })
                + GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float,
            y: bounds.y + bounds.height - horizontalScrollBarWidth as libc::c_float
                - GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float,
            width: bounds.width - verticalScrollBarWidth as libc::c_float
                - (2 as libc::c_int
                    * GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int))
                    as libc::c_float,
            height: horizontalScrollBarWidth as libc::c_float,
        };
        init
    };
    let mut verticalScrollBar: Rectangle = {
        let mut init = Rectangle {
            x: if GuiGetStyle(LISTVIEW as libc::c_int, SCROLLBAR_SIDE as libc::c_int)
                == 0 as libc::c_int
            {
                bounds.x
                    + GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float
            } else {
                bounds.x + bounds.width - verticalScrollBarWidth as libc::c_float
                    - GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float
            },
            y: bounds.y
                + GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float,
            width: verticalScrollBarWidth as libc::c_float,
            height: bounds.height - horizontalScrollBarWidth as libc::c_float
                - (2 as libc::c_int
                    * GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int))
                    as libc::c_float,
        };
        init
    };
    if horizontalScrollBar.width < 40 as libc::c_int as libc::c_float {
        horizontalScrollBar.width = 40 as libc::c_int as libc::c_float;
    }
    if verticalScrollBar.height < 40 as libc::c_int as libc::c_float {
        verticalScrollBar.height = 40 as libc::c_int as libc::c_float;
    }
    *view = if GuiGetStyle(LISTVIEW as libc::c_int, SCROLLBAR_SIDE as libc::c_int)
        == 0 as libc::c_int
    {
        {
            let mut init = Rectangle {
                x: bounds.x + verticalScrollBarWidth as libc::c_float
                    + GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float,
                y: bounds.y
                    + GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float,
                width: bounds.width
                    - (2 as libc::c_int
                        * GuiGetStyle(
                            DEFAULT as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )) as libc::c_float - verticalScrollBarWidth as libc::c_float,
                height: bounds.height
                    - (2 as libc::c_int
                        * GuiGetStyle(
                            DEFAULT as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )) as libc::c_float - horizontalScrollBarWidth as libc::c_float,
            };
            init
        }
    } else {
        let mut init = Rectangle {
            x: bounds.x
                + GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float,
            y: bounds.y
                + GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float,
            width: bounds.width
                - (2 as libc::c_int
                    * GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int))
                    as libc::c_float - verticalScrollBarWidth as libc::c_float,
            height: bounds.height
                - (2 as libc::c_int
                    * GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int))
                    as libc::c_float - horizontalScrollBarWidth as libc::c_float,
        };
        init
    };
    if (*view).width > content.width {
        (*view).width = content.width;
    }
    if (*view).height > content.height {
        (*view).height = content.height;
    }
    let mut horizontalMin: libc::c_float = if hasHorizontalScrollBar as libc::c_int != 0
    {
        (if GuiGetStyle(LISTVIEW as libc::c_int, SCROLLBAR_SIDE as libc::c_int)
            == 0 as libc::c_int
        {
            -verticalScrollBarWidth as libc::c_float
        } else {
            0 as libc::c_int as libc::c_float
        })
            - GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                as libc::c_float
    } else {
        (if GuiGetStyle(LISTVIEW as libc::c_int, SCROLLBAR_SIDE as libc::c_int)
            as libc::c_float == 0 as libc::c_int as libc::c_float
        {
            -verticalScrollBarWidth as libc::c_float
        } else {
            0 as libc::c_int as libc::c_float
        })
            - GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                as libc::c_float
    };
    let mut horizontalMax: libc::c_float = if hasHorizontalScrollBar as libc::c_int != 0
    {
        content.width - bounds.width + verticalScrollBarWidth as libc::c_float
            + GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                as libc::c_float
            - (if GuiGetStyle(LISTVIEW as libc::c_int, SCROLLBAR_SIDE as libc::c_int)
                as libc::c_float == 0 as libc::c_int as libc::c_float
            {
                verticalScrollBarWidth as libc::c_float
            } else {
                0 as libc::c_int as libc::c_float
            })
    } else {
        -GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
            as libc::c_float
    };
    let mut verticalMin: libc::c_float = if hasVerticalScrollBar as libc::c_int != 0 {
        0.0f32
    } else {
        -1.0f32
    };
    let mut verticalMax: libc::c_float = if hasVerticalScrollBar as libc::c_int != 0 {
        content.height - bounds.height + horizontalScrollBarWidth as libc::c_float
            + GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                as libc::c_float
    } else {
        -GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
            as libc::c_float
    };
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if CheckCollisionPointRec(mousePoint, bounds) {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
            } else {
                state = STATE_FOCUSED;
            }
            let mut wheelMove: libc::c_float = GetMouseWheelMove();
            let mut mouseWheelSpeed: Vector2 = {
                let mut init = Vector2 {
                    x: content.width / bounds.width,
                    y: content.height / bounds.height,
                };
                init
            };
            if mouseWheelSpeed.x < 20 as libc::c_int as libc::c_float {
                mouseWheelSpeed.x = 20 as libc::c_int as libc::c_float;
            }
            if mouseWheelSpeed.y < 20 as libc::c_int as libc::c_float {
                mouseWheelSpeed.y = 20 as libc::c_int as libc::c_float;
            }
            if hasHorizontalScrollBar as libc::c_int != 0
                && (IsKeyDown(KEY_LEFT_CONTROL as libc::c_int) as libc::c_int != 0
                    || IsKeyDown(KEY_LEFT_SHIFT as libc::c_int) as libc::c_int != 0)
            {
                scrollPos.x += wheelMove * mouseWheelSpeed.x;
            } else {
                scrollPos.y += wheelMove * mouseWheelSpeed.y;
            }
        }
    }
    if scrollPos.x > -horizontalMin {
        scrollPos.x = -horizontalMin;
    }
    if scrollPos.x < -horizontalMax {
        scrollPos.x = -horizontalMax;
    }
    if scrollPos.y > -verticalMin {
        scrollPos.y = -verticalMin;
    }
    if scrollPos.y < -verticalMax {
        scrollPos.y = -verticalMax;
    }
    if !text.is_null() {
        GuiStatusBar(statusBar, text);
    }
    GuiDrawRectangle(
        bounds,
        0 as libc::c_int,
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
        GetColor(
            GuiGetStyle(DEFAULT as libc::c_int, BACKGROUND_COLOR as libc::c_int)
                as libc::c_uint,
        ),
    );
    let slider: libc::c_int = GuiGetStyle(
        SCROLLBAR as libc::c_int,
        SCROLL_SLIDER_SIZE as libc::c_int,
    );
    if hasHorizontalScrollBar {
        GuiSetStyle(
            SCROLLBAR as libc::c_int,
            SCROLL_SLIDER_SIZE as libc::c_int,
            ((bounds.width
                - (2 as libc::c_int
                    * GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int))
                    as libc::c_float - verticalScrollBarWidth as libc::c_float)
                / content.width as libc::c_int as libc::c_float
                * (bounds.width as libc::c_int
                    - 2 as libc::c_int
                        * GuiGetStyle(
                            DEFAULT as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        ) - verticalScrollBarWidth) as libc::c_float) as libc::c_int,
        );
        scrollPos
            .x = -GuiScrollBar(
            horizontalScrollBar,
            -scrollPos.x as libc::c_int,
            horizontalMin as libc::c_int,
            horizontalMax as libc::c_int,
        ) as libc::c_float;
    } else {
        scrollPos.x = 0.0f32;
    }
    if hasVerticalScrollBar {
        GuiSetStyle(
            SCROLLBAR as libc::c_int,
            SCROLL_SLIDER_SIZE as libc::c_int,
            ((bounds.height
                - (2 as libc::c_int
                    * GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int))
                    as libc::c_float - horizontalScrollBarWidth as libc::c_float)
                / content.height as libc::c_int as libc::c_float
                * (bounds.height as libc::c_int
                    - 2 as libc::c_int
                        * GuiGetStyle(
                            DEFAULT as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        ) - horizontalScrollBarWidth) as libc::c_float) as libc::c_int,
        );
        scrollPos
            .y = -GuiScrollBar(
            verticalScrollBar,
            -scrollPos.y as libc::c_int,
            verticalMin as libc::c_int,
            verticalMax as libc::c_int,
        ) as libc::c_float;
    } else {
        scrollPos.y = 0.0f32;
    }
    if hasHorizontalScrollBar as libc::c_int != 0
        && hasVerticalScrollBar as libc::c_int != 0
    {
        let mut corner: Rectangle = {
            let mut init = Rectangle {
                x: if GuiGetStyle(LISTVIEW as libc::c_int, SCROLLBAR_SIDE as libc::c_int)
                    == 0 as libc::c_int
                {
                    bounds.x
                        + GuiGetStyle(
                            DEFAULT as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        ) as libc::c_float + 2 as libc::c_int as libc::c_float
                } else {
                    horizontalScrollBar.x + horizontalScrollBar.width
                        + 2 as libc::c_int as libc::c_float
                },
                y: verticalScrollBar.y + verticalScrollBar.height
                    + 2 as libc::c_int as libc::c_float,
                width: horizontalScrollBarWidth as libc::c_float
                    - 4 as libc::c_int as libc::c_float,
                height: verticalScrollBarWidth as libc::c_float
                    - 4 as libc::c_int as libc::c_float,
            };
            init
        };
        GuiDrawRectangle(
            corner,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(
                    LISTVIEW as libc::c_int,
                    (TEXT as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(LISTVIEW as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                LISTVIEW as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
    );
    GuiSetStyle(SCROLLBAR as libc::c_int, SCROLL_SLIDER_SIZE as libc::c_int, slider);
    if !scroll.is_null() {
        *scroll = scrollPos;
    }
    return result;
}
unsafe extern "C" fn GuiScrollBar(
    mut bounds: Rectangle,
    mut value: libc::c_int,
    mut minValue: libc::c_int,
    mut maxValue: libc::c_int,
) -> libc::c_int {
    let mut state: GuiState = guiState;
    let mut isVertical: bool = if bounds.width > bounds.height {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    } != 0;
    let spinnerSize: libc::c_int = if GuiGetStyle(
        SCROLLBAR as libc::c_int,
        ARROWS_VISIBLE as libc::c_int,
    ) != 0
    {
        if isVertical as libc::c_int != 0 {
            bounds.width as libc::c_int
                - 2 as libc::c_int
                    * GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
        } else {
            bounds.height as libc::c_int
                - 2 as libc::c_int
                    * GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
        }
    } else {
        0 as libc::c_int
    };
    let mut arrowUpLeft: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    let mut arrowDownRight: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    let mut scrollbar: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    let mut slider: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    if value > maxValue {
        value = maxValue;
    }
    if value < minValue {
        value = minValue;
    }
    let mut valueRange: libc::c_int = maxValue - minValue;
    if valueRange <= 0 as libc::c_int {
        valueRange = 1 as libc::c_int;
    }
    let mut sliderSize: libc::c_int = GuiGetStyle(
        SCROLLBAR as libc::c_int,
        SCROLL_SLIDER_SIZE as libc::c_int,
    );
    if sliderSize < 1 as libc::c_int {
        sliderSize = 1 as libc::c_int;
    }
    arrowUpLeft = {
        let mut init = Rectangle {
            x: bounds.x
                + GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float,
            y: bounds.y
                + GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float,
            width: spinnerSize as libc::c_float,
            height: spinnerSize as libc::c_float,
        };
        init
    };
    if isVertical {
        arrowDownRight = {
            let mut init = Rectangle {
                x: bounds.x
                    + GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float,
                y: bounds.y + bounds.height - spinnerSize as libc::c_float
                    - GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float,
                width: spinnerSize as libc::c_float,
                height: spinnerSize as libc::c_float,
            };
            init
        };
        scrollbar = {
            let mut init = Rectangle {
                x: bounds.x
                    + GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float
                    + GuiGetStyle(
                        SCROLLBAR as libc::c_int,
                        SCROLL_PADDING as libc::c_int,
                    ) as libc::c_float,
                y: arrowUpLeft.y + arrowUpLeft.height,
                width: bounds.width
                    - (2 as libc::c_int
                        * (GuiGetStyle(
                            SCROLLBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )
                            + GuiGetStyle(
                                SCROLLBAR as libc::c_int,
                                SCROLL_PADDING as libc::c_int,
                            ))) as libc::c_float,
                height: bounds.height - arrowUpLeft.height - arrowDownRight.height
                    - (2 as libc::c_int
                        * GuiGetStyle(
                            SCROLLBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )) as libc::c_float,
            };
            init
        };
        sliderSize = if sliderSize as libc::c_float >= scrollbar.height {
            scrollbar.height as libc::c_int - 2 as libc::c_int
        } else {
            sliderSize
        };
        slider = {
            let mut init = Rectangle {
                x: bounds.x
                    + GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float
                    + GuiGetStyle(
                        SCROLLBAR as libc::c_int,
                        SCROLL_SLIDER_PADDING as libc::c_int,
                    ) as libc::c_float,
                y: scrollbar.y
                    + ((value - minValue) as libc::c_float / valueRange as libc::c_float
                        * (scrollbar.height - sliderSize as libc::c_float))
                        as libc::c_int as libc::c_float,
                width: bounds.width
                    - (2 as libc::c_int
                        * (GuiGetStyle(
                            SCROLLBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )
                            + GuiGetStyle(
                                SCROLLBAR as libc::c_int,
                                SCROLL_SLIDER_PADDING as libc::c_int,
                            ))) as libc::c_float,
                height: sliderSize as libc::c_float,
            };
            init
        };
    } else {
        arrowDownRight = {
            let mut init = Rectangle {
                x: bounds.x + bounds.width - spinnerSize as libc::c_float
                    - GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float,
                y: bounds.y
                    + GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float,
                width: spinnerSize as libc::c_float,
                height: spinnerSize as libc::c_float,
            };
            init
        };
        scrollbar = {
            let mut init = Rectangle {
                x: arrowUpLeft.x + arrowUpLeft.width,
                y: bounds.y
                    + GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float
                    + GuiGetStyle(
                        SCROLLBAR as libc::c_int,
                        SCROLL_PADDING as libc::c_int,
                    ) as libc::c_float,
                width: bounds.width - arrowUpLeft.width - arrowDownRight.width
                    - (2 as libc::c_int
                        * GuiGetStyle(
                            SCROLLBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )) as libc::c_float,
                height: bounds.height
                    - (2 as libc::c_int
                        * (GuiGetStyle(
                            SCROLLBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )
                            + GuiGetStyle(
                                SCROLLBAR as libc::c_int,
                                SCROLL_PADDING as libc::c_int,
                            ))) as libc::c_float,
            };
            init
        };
        sliderSize = if sliderSize as libc::c_float >= scrollbar.width {
            scrollbar.width as libc::c_int - 2 as libc::c_int
        } else {
            sliderSize
        };
        slider = {
            let mut init = Rectangle {
                x: scrollbar.x
                    + ((value - minValue) as libc::c_float / valueRange as libc::c_float
                        * (scrollbar.width - sliderSize as libc::c_float)) as libc::c_int
                        as libc::c_float,
                y: bounds.y
                    + GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float
                    + GuiGetStyle(
                        SCROLLBAR as libc::c_int,
                        SCROLL_SLIDER_PADDING as libc::c_int,
                    ) as libc::c_float,
                width: sliderSize as libc::c_float,
                height: bounds.height
                    - (2 as libc::c_int
                        * (GuiGetStyle(
                            SCROLLBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )
                            + GuiGetStyle(
                                SCROLLBAR as libc::c_int,
                                SCROLL_SLIDER_PADDING as libc::c_int,
                            ))) as libc::c_float,
            };
            init
        };
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if guiControlExclusiveMode {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) as libc::c_int != 0
                && !CheckCollisionPointRec(mousePoint, arrowUpLeft)
                && !CheckCollisionPointRec(mousePoint, arrowDownRight)
            {
                if bounds.x == guiControlExclusiveRec.x
                    && bounds.y == guiControlExclusiveRec.y
                    && bounds.width == guiControlExclusiveRec.width
                    && bounds.height == guiControlExclusiveRec.height
                {
                    state = STATE_PRESSED;
                    if isVertical {
                        value = ((mousePoint.y - scrollbar.y
                            - slider.height / 2 as libc::c_int as libc::c_float)
                            * valueRange as libc::c_float
                            / (scrollbar.height - slider.height)
                            + minValue as libc::c_float) as libc::c_int;
                    } else {
                        value = ((mousePoint.x - scrollbar.x
                            - slider.width / 2 as libc::c_int as libc::c_float)
                            * valueRange as libc::c_float
                            / (scrollbar.width - slider.width)
                            + minValue as libc::c_float) as libc::c_int;
                    }
                }
            } else {
                guiControlExclusiveMode = 0 as libc::c_int != 0;
                guiControlExclusiveRec = {
                    let mut init = Rectangle {
                        x: 0 as libc::c_int as libc::c_float,
                        y: 0 as libc::c_int as libc::c_float,
                        width: 0 as libc::c_int as libc::c_float,
                        height: 0 as libc::c_int as libc::c_float,
                    };
                    init
                };
            }
        } else if CheckCollisionPointRec(mousePoint, bounds) {
            state = STATE_FOCUSED;
            let mut wheel: libc::c_int = GetMouseWheelMove() as libc::c_int;
            if wheel != 0 as libc::c_int {
                value += wheel;
            }
            if IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int) {
                guiControlExclusiveMode = 1 as libc::c_int != 0;
                guiControlExclusiveRec = bounds;
                if CheckCollisionPointRec(mousePoint, arrowUpLeft) {
                    value
                        -= valueRange
                            / GuiGetStyle(
                                SCROLLBAR as libc::c_int,
                                SCROLL_SPEED as libc::c_int,
                            );
                } else if CheckCollisionPointRec(mousePoint, arrowDownRight) {
                    value
                        += valueRange
                            / GuiGetStyle(
                                SCROLLBAR as libc::c_int,
                                SCROLL_SPEED as libc::c_int,
                            );
                } else if !CheckCollisionPointRec(mousePoint, slider) {
                    if isVertical {
                        value = ((mousePoint.y - scrollbar.y
                            - slider.height / 2 as libc::c_int as libc::c_float)
                            * valueRange as libc::c_float
                            / (scrollbar.height - slider.height)
                            + minValue as libc::c_float) as libc::c_int;
                    } else {
                        value = ((mousePoint.x - scrollbar.x
                            - slider.width / 2 as libc::c_int as libc::c_float)
                            * valueRange as libc::c_float
                            / (scrollbar.width - slider.width)
                            + minValue as libc::c_float) as libc::c_int;
                    }
                }
                state = STATE_PRESSED;
            }
        }
        if value > maxValue {
            value = maxValue;
        }
        if value < minValue {
            value = minValue;
        }
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                LISTVIEW as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        GetColor(
            GuiGetStyle(DEFAULT as libc::c_int, BORDER_COLOR_DISABLED as libc::c_int)
                as libc::c_uint,
        ),
    );
    GuiDrawRectangle(
        scrollbar,
        0 as libc::c_int,
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
        GetColor(
            GuiGetStyle(BUTTON as libc::c_int, BASE_COLOR_NORMAL as libc::c_int)
                as libc::c_uint,
        ),
    );
    GuiDrawRectangle(
        slider,
        0 as libc::c_int,
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
        GetColor(
            GuiGetStyle(
                SLIDER as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    if GuiGetStyle(SCROLLBAR as libc::c_int, ARROWS_VISIBLE as libc::c_int) != 0 {
        GuiDrawText(
            if isVertical as libc::c_int != 0 {
                b"#121#\0" as *const u8 as *const libc::c_char
            } else {
                b"#118#\0" as *const u8 as *const libc::c_char
            },
            {
                let mut init = Rectangle {
                    x: arrowUpLeft.x,
                    y: arrowUpLeft.y,
                    width: if isVertical as libc::c_int != 0 {
                        bounds.width
                    } else {
                        bounds.height
                    },
                    height: if isVertical as libc::c_int != 0 {
                        bounds.width
                    } else {
                        bounds.height
                    },
                };
                init
            },
            TEXT_ALIGN_CENTER as libc::c_int,
            GetColor(
                GuiGetStyle(
                    SCROLLBAR as libc::c_int,
                    (TEXT as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
        GuiDrawText(
            if isVertical as libc::c_int != 0 {
                b"#120#\0" as *const u8 as *const libc::c_char
            } else {
                b"#119#\0" as *const u8 as *const libc::c_char
            },
            {
                let mut init = Rectangle {
                    x: arrowDownRight.x,
                    y: arrowDownRight.y,
                    width: if isVertical as libc::c_int != 0 {
                        bounds.width
                    } else {
                        bounds.height
                    },
                    height: if isVertical as libc::c_int != 0 {
                        bounds.width
                    } else {
                        bounds.height
                    },
                };
                init
            },
            TEXT_ALIGN_CENTER as libc::c_int,
            GetColor(
                GuiGetStyle(
                    SCROLLBAR as libc::c_int,
                    (TEXT as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
    }
    return value;
}
static mut guiControlExclusiveRec: Rectangle = {
    let mut init = Rectangle {
        x: 0 as libc::c_int as libc::c_float,
        y: 0.,
        width: 0.,
        height: 0.,
    };
    init
};
pub unsafe extern "C" fn GuiLabelButton(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
) -> libc::c_int {
    let mut state: GuiState = guiState;
    let mut pressed: bool = 0 as libc::c_int != 0;
    let mut textWidth: libc::c_float = GetTextWidth(text) as libc::c_float;
    if (bounds.width
        - (2 as libc::c_int
            * GuiGetStyle(LABEL as libc::c_int, BORDER_WIDTH as libc::c_int))
            as libc::c_float
        - (2 as libc::c_int
            * GuiGetStyle(LABEL as libc::c_int, TEXT_PADDING as libc::c_int))
            as libc::c_float) < textWidth
    {
        bounds
            .width = textWidth
            + (2 as libc::c_int
                * GuiGetStyle(LABEL as libc::c_int, BORDER_WIDTH as libc::c_int))
                as libc::c_float
            + (2 as libc::c_int
                * GuiGetStyle(LABEL as libc::c_int, TEXT_PADDING as libc::c_int))
                as libc::c_float + 2 as libc::c_int as libc::c_float;
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked && !guiControlExclusiveMode
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if CheckCollisionPointRec(mousePoint, bounds) {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
            } else {
                state = STATE_FOCUSED;
            }
            if IsMouseButtonReleased(MOUSE_BUTTON_LEFT as libc::c_int) {
                pressed = 1 as libc::c_int != 0;
            }
        }
    }
    GuiDrawText(
        text,
        GetTextBounds(LABEL as libc::c_int, bounds),
        GuiGetStyle(LABEL as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
        GetColor(
            GuiGetStyle(
                LABEL as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    return pressed as libc::c_int;
}
pub unsafe extern "C" fn GuiToggleGroup(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut active: *mut libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut initBoundsX: libc::c_float = bounds.x;
    let mut temp: libc::c_int = 0 as libc::c_int;
    if active.is_null() {
        active = &mut temp;
    }
    let mut toggle: bool = 0 as libc::c_int != 0;
    let mut rows: [libc::c_int; 32] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut itemCount: libc::c_int = 0 as libc::c_int;
    let mut items: *mut *const libc::c_char = GuiTextSplit(
        text,
        ';' as i32 as libc::c_char,
        &mut itemCount,
        rows.as_mut_ptr(),
    );
    let mut prevRow: libc::c_int = rows[0 as libc::c_int as usize];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < itemCount {
        if prevRow != rows[i as usize] {
            bounds.x = initBoundsX;
            bounds.y
                += bounds.height
                    + GuiGetStyle(TOGGLE as libc::c_int, GROUP_PADDING as libc::c_int)
                        as libc::c_float;
            prevRow = rows[i as usize];
        }
        if i == *active {
            toggle = 1 as libc::c_int != 0;
            GuiToggle(bounds, *items.offset(i as isize), &mut toggle);
        } else {
            toggle = 0 as libc::c_int != 0;
            GuiToggle(bounds, *items.offset(i as isize), &mut toggle);
            if toggle {
                *active = i;
            }
        }
        bounds.x
            += bounds.width
                + GuiGetStyle(TOGGLE as libc::c_int, GROUP_PADDING as libc::c_int)
                    as libc::c_float;
        i += 1;
        i;
    }
    return result;
}
unsafe extern "C" fn GuiTextSplit(
    mut text: *const libc::c_char,
    mut delimiter: libc::c_char,
    mut count: *mut libc::c_int,
    mut textRow: *mut libc::c_int,
) -> *mut *const libc::c_char {
    static mut result: [*const libc::c_char; 128] = [
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut buffer: [libc::c_char; 1024] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    memset(
        buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        1024 as libc::c_int as libc::c_ulong,
    );
    result[0 as libc::c_int as usize] = buffer.as_mut_ptr();
    let mut counter: libc::c_int = 1 as libc::c_int;
    if !textRow.is_null() {
        *textRow.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        buffer[i as usize] = *text.offset(i as isize);
        if buffer[i as usize] as libc::c_int == '\0' as i32 {
            break;
        }
        if buffer[i as usize] as libc::c_int == delimiter as libc::c_int
            || buffer[i as usize] as libc::c_int == '\n' as i32
        {
            result[counter
                as usize] = buffer
                .as_mut_ptr()
                .offset(i as isize)
                .offset(1 as libc::c_int as isize);
            if !textRow.is_null() {
                if buffer[i as usize] as libc::c_int == '\n' as i32 {
                    *textRow
                        .offset(
                            counter as isize,
                        ) = *textRow.offset((counter - 1 as libc::c_int) as isize)
                        + 1 as libc::c_int;
                } else {
                    *textRow
                        .offset(
                            counter as isize,
                        ) = *textRow.offset((counter - 1 as libc::c_int) as isize);
                }
            }
            buffer[i as usize] = '\0' as i32 as libc::c_char;
            counter += 1;
            counter;
            if counter > 128 as libc::c_int {
                break;
            }
        }
        i += 1;
        i;
    }
    *count = counter;
    return result.as_mut_ptr();
}
pub unsafe extern "C" fn GuiToggleSlider(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut active: *mut libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut temp: libc::c_int = 0 as libc::c_int;
    if active.is_null() {
        active = &mut temp;
    }
    let mut itemCount: libc::c_int = 0 as libc::c_int;
    let mut items: *mut *const libc::c_char = GuiTextSplit(
        text,
        ';' as i32 as libc::c_char,
        &mut itemCount,
        0 as *mut libc::c_int,
    );
    let mut slider: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: bounds.y
                + GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float
                + GuiGetStyle(SLIDER as libc::c_int, SLIDER_PADDING as libc::c_int)
                    as libc::c_float,
            width: (bounds.width
                - (2 as libc::c_int
                    * GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int))
                    as libc::c_float
                - ((itemCount + 1 as libc::c_int)
                    * GuiGetStyle(SLIDER as libc::c_int, SLIDER_PADDING as libc::c_int))
                    as libc::c_float) / itemCount as libc::c_float,
            height: bounds.height
                - (2 as libc::c_int
                    * GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int))
                    as libc::c_float
                - (2 as libc::c_int
                    * GuiGetStyle(SLIDER as libc::c_int, SLIDER_PADDING as libc::c_int))
                    as libc::c_float,
        };
        init
    };
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if CheckCollisionPointRec(mousePoint, bounds) {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
            } else if IsMouseButtonReleased(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
                *active += 1;
                *active;
                result = 1 as libc::c_int;
            } else {
                state = STATE_FOCUSED;
            }
        }
        if *active != 0
            && state as libc::c_uint != STATE_FOCUSED as libc::c_int as libc::c_uint
        {
            state = STATE_PRESSED;
        }
    }
    if *active >= itemCount {
        *active = 0 as libc::c_int;
    }
    slider
        .x = bounds.x
        + GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int)
            as libc::c_float
        + ((*active + 1 as libc::c_int)
            * GuiGetStyle(SLIDER as libc::c_int, SLIDER_PADDING as libc::c_int))
            as libc::c_float + *active as libc::c_float * slider.width;
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                TOGGLE as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        GetColor(
            GuiGetStyle(TOGGLE as libc::c_int, BASE_COLOR_NORMAL as libc::c_int)
                as libc::c_uint,
        ),
    );
    if state as libc::c_uint == STATE_NORMAL as libc::c_int as libc::c_uint {
        GuiDrawRectangle(
            slider,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(SLIDER as libc::c_int, BASE_COLOR_PRESSED as libc::c_int)
                    as libc::c_uint,
            ),
        );
    } else if state as libc::c_uint == STATE_FOCUSED as libc::c_int as libc::c_uint {
        GuiDrawRectangle(
            slider,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(SLIDER as libc::c_int, BASE_COLOR_FOCUSED as libc::c_int)
                    as libc::c_uint,
            ),
        );
    } else if state as libc::c_uint == STATE_PRESSED as libc::c_int as libc::c_uint {
        GuiDrawRectangle(
            slider,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(SLIDER as libc::c_int, BASE_COLOR_PRESSED as libc::c_int)
                    as libc::c_uint,
            ),
        );
    }
    if !text.is_null() {
        let mut textBounds: Rectangle = {
            let mut init = Rectangle {
                x: 0 as libc::c_int as libc::c_float,
                y: 0.,
                width: 0.,
                height: 0.,
            };
            init
        };
        textBounds.width = GetTextWidth(text) as libc::c_float;
        textBounds
            .height = GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
            as libc::c_float;
        textBounds
            .x = slider.x + slider.width / 2 as libc::c_int as libc::c_float
            - textBounds.width / 2 as libc::c_int as libc::c_float;
        textBounds
            .y = bounds.y + bounds.height / 2 as libc::c_int as libc::c_float
            - (GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                / 2 as libc::c_int) as libc::c_float;
        GuiDrawText(
            *items.offset(*active as isize),
            textBounds,
            GuiGetStyle(TOGGLE as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
            Fade(
                GetColor(
                    GuiGetStyle(
                        TOGGLE as libc::c_int,
                        (TEXT as libc::c_int as libc::c_uint)
                            .wrapping_add(
                                (state as libc::c_uint)
                                    .wrapping_mul(3 as libc::c_int as libc::c_uint),
                            ) as libc::c_int,
                    ) as libc::c_uint,
                ),
                guiAlpha,
            ),
        );
    }
    return result;
}
pub unsafe extern "C" fn GuiCheckBox(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut checked: *mut bool,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut temp: bool = 0 as libc::c_int != 0;
    if checked.is_null() {
        checked = &mut temp;
    }
    let mut textBounds: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    if !text.is_null() {
        textBounds
            .width = GetTextWidth(text) as libc::c_float
            + 2 as libc::c_int as libc::c_float;
        textBounds
            .height = GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
            as libc::c_float;
        textBounds
            .x = bounds.x + bounds.width
            + GuiGetStyle(CHECKBOX as libc::c_int, TEXT_PADDING as libc::c_int)
                as libc::c_float;
        textBounds
            .y = bounds.y + bounds.height / 2 as libc::c_int as libc::c_float
            - (GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                / 2 as libc::c_int) as libc::c_float;
        if GuiGetStyle(CHECKBOX as libc::c_int, TEXT_ALIGNMENT as libc::c_int)
            == TEXT_ALIGN_LEFT as libc::c_int
        {
            textBounds
                .x = bounds.x - textBounds.width
                - GuiGetStyle(CHECKBOX as libc::c_int, TEXT_PADDING as libc::c_int)
                    as libc::c_float;
        }
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked && !guiControlExclusiveMode
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        let mut totalBounds: Rectangle = {
            let mut init = Rectangle {
                x: if GuiGetStyle(CHECKBOX as libc::c_int, TEXT_ALIGNMENT as libc::c_int)
                    == TEXT_ALIGN_LEFT as libc::c_int
                {
                    textBounds.x
                } else {
                    bounds.x
                },
                y: bounds.y,
                width: bounds.width + textBounds.width
                    + GuiGetStyle(CHECKBOX as libc::c_int, TEXT_PADDING as libc::c_int)
                        as libc::c_float,
                height: bounds.height,
            };
            init
        };
        if CheckCollisionPointRec(mousePoint, totalBounds) {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
            } else {
                state = STATE_FOCUSED;
            }
            if IsMouseButtonReleased(MOUSE_BUTTON_LEFT as libc::c_int) {
                *checked = !*checked;
                result = 1 as libc::c_int;
            }
        }
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(CHECKBOX as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                CHECKBOX as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
    );
    if *checked {
        let mut check: Rectangle = {
            let mut init = Rectangle {
                x: bounds.x
                    + GuiGetStyle(CHECKBOX as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float
                    + GuiGetStyle(CHECKBOX as libc::c_int, CHECK_PADDING as libc::c_int)
                        as libc::c_float,
                y: bounds.y
                    + GuiGetStyle(CHECKBOX as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float
                    + GuiGetStyle(CHECKBOX as libc::c_int, CHECK_PADDING as libc::c_int)
                        as libc::c_float,
                width: bounds.width
                    - (2 as libc::c_int
                        * (GuiGetStyle(
                            CHECKBOX as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )
                            + GuiGetStyle(
                                CHECKBOX as libc::c_int,
                                CHECK_PADDING as libc::c_int,
                            ))) as libc::c_float,
                height: bounds.height
                    - (2 as libc::c_int
                        * (GuiGetStyle(
                            CHECKBOX as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )
                            + GuiGetStyle(
                                CHECKBOX as libc::c_int,
                                CHECK_PADDING as libc::c_int,
                            ))) as libc::c_float,
            };
            init
        };
        GuiDrawRectangle(
            check,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(
                    CHECKBOX as libc::c_int,
                    (TEXT as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
    }
    GuiDrawText(
        text,
        textBounds,
        if GuiGetStyle(CHECKBOX as libc::c_int, TEXT_ALIGNMENT as libc::c_int)
            == TEXT_ALIGN_RIGHT as libc::c_int
        {
            TEXT_ALIGN_LEFT as libc::c_int
        } else {
            TEXT_ALIGN_RIGHT as libc::c_int
        },
        GetColor(
            GuiGetStyle(
                LABEL as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    return result;
}
pub unsafe extern "C" fn GuiComboBox(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut active: *mut libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut temp: libc::c_int = 0 as libc::c_int;
    if active.is_null() {
        active = &mut temp;
    }
    bounds.width
        -= (GuiGetStyle(COMBOBOX as libc::c_int, COMBO_BUTTON_WIDTH as libc::c_int)
            + GuiGetStyle(COMBOBOX as libc::c_int, COMBO_BUTTON_SPACING as libc::c_int))
            as libc::c_float;
    let mut selector: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x + bounds.width
                + GuiGetStyle(
                    COMBOBOX as libc::c_int,
                    COMBO_BUTTON_SPACING as libc::c_int,
                ) as libc::c_float,
            y: bounds.y,
            width: GuiGetStyle(
                COMBOBOX as libc::c_int,
                COMBO_BUTTON_WIDTH as libc::c_int,
            ) as libc::c_float,
            height: bounds.height,
        };
        init
    };
    let mut itemCount: libc::c_int = 0 as libc::c_int;
    let mut items: *mut *const libc::c_char = GuiTextSplit(
        text,
        ';' as i32 as libc::c_char,
        &mut itemCount,
        0 as *mut libc::c_int,
    );
    if *active < 0 as libc::c_int {
        *active = 0 as libc::c_int;
    } else if *active > itemCount - 1 as libc::c_int {
        *active = itemCount - 1 as libc::c_int;
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked && itemCount > 1 as libc::c_int && !guiControlExclusiveMode
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if CheckCollisionPointRec(mousePoint, bounds) as libc::c_int != 0
            || CheckCollisionPointRec(mousePoint, selector) as libc::c_int != 0
        {
            if IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int) {
                *active += 1 as libc::c_int;
                if *active >= itemCount {
                    *active = 0 as libc::c_int;
                }
            }
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
            } else {
                state = STATE_FOCUSED;
            }
        }
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(COMBOBOX as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                COMBOBOX as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        GetColor(
            GuiGetStyle(
                COMBOBOX as libc::c_int,
                (BASE as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    GuiDrawText(
        *items.offset(*active as isize),
        GetTextBounds(COMBOBOX as libc::c_int, bounds),
        GuiGetStyle(COMBOBOX as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
        GetColor(
            GuiGetStyle(
                COMBOBOX as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    let mut tempBorderWidth: libc::c_int = GuiGetStyle(
        BUTTON as libc::c_int,
        BORDER_WIDTH as libc::c_int,
    );
    let mut tempTextAlign: libc::c_int = GuiGetStyle(
        BUTTON as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
    );
    GuiSetStyle(BUTTON as libc::c_int, BORDER_WIDTH as libc::c_int, 1 as libc::c_int);
    GuiSetStyle(
        BUTTON as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_CENTER as libc::c_int,
    );
    GuiButton(
        selector,
        TextFormat(
            b"%i/%i\0" as *const u8 as *const libc::c_char,
            *active + 1 as libc::c_int,
            itemCount,
        ),
    );
    GuiSetStyle(BUTTON as libc::c_int, TEXT_ALIGNMENT as libc::c_int, tempTextAlign);
    GuiSetStyle(BUTTON as libc::c_int, BORDER_WIDTH as libc::c_int, tempBorderWidth);
    return result;
}
pub unsafe extern "C" fn GuiDropdownBox(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut active: *mut libc::c_int,
    mut editMode: bool,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut itemSelected: libc::c_int = *active;
    let mut itemFocused: libc::c_int = -(1 as libc::c_int);
    let mut itemCount: libc::c_int = 0 as libc::c_int;
    let mut items: *mut *const libc::c_char = GuiTextSplit(
        text,
        ';' as i32 as libc::c_char,
        &mut itemCount,
        0 as *mut libc::c_int,
    );
    let mut boundsOpen: Rectangle = bounds;
    boundsOpen
        .height = (itemCount + 1 as libc::c_int) as libc::c_float
        * (bounds.height
            + GuiGetStyle(
                DROPDOWNBOX as libc::c_int,
                DROPDOWN_ITEMS_SPACING as libc::c_int,
            ) as libc::c_float);
    let mut itemBounds: Rectangle = bounds;
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && (editMode as libc::c_int != 0 || !guiLocked) && itemCount > 1 as libc::c_int
        && !guiControlExclusiveMode
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if editMode {
            state = STATE_PRESSED;
            if !CheckCollisionPointRec(mousePoint, boundsOpen) {
                if IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int) as libc::c_int
                    != 0
                    || IsMouseButtonReleased(MOUSE_BUTTON_LEFT as libc::c_int)
                        as libc::c_int != 0
                {
                    result = 1 as libc::c_int;
                }
            }
            if CheckCollisionPointRec(mousePoint, bounds) as libc::c_int != 0
                && IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int) as libc::c_int
                    != 0
            {
                result = 1 as libc::c_int;
            }
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < itemCount {
                itemBounds.y
                    += bounds.height
                        + GuiGetStyle(
                            DROPDOWNBOX as libc::c_int,
                            DROPDOWN_ITEMS_SPACING as libc::c_int,
                        ) as libc::c_float;
                if CheckCollisionPointRec(mousePoint, itemBounds) {
                    itemFocused = i;
                    if IsMouseButtonReleased(MOUSE_BUTTON_LEFT as libc::c_int) {
                        itemSelected = i;
                        result = 1 as libc::c_int;
                    }
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            itemBounds = bounds;
        } else if CheckCollisionPointRec(mousePoint, bounds) {
            if IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int) {
                result = 1 as libc::c_int;
                state = STATE_PRESSED;
            } else {
                state = STATE_FOCUSED;
            }
        }
    }
    if editMode {
        GuiPanel(boundsOpen, 0 as *const libc::c_char);
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(DROPDOWNBOX as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                DROPDOWNBOX as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        GetColor(
            GuiGetStyle(
                DROPDOWNBOX as libc::c_int,
                (BASE as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    GuiDrawText(
        *items.offset(itemSelected as isize),
        GetTextBounds(DROPDOWNBOX as libc::c_int, bounds),
        GuiGetStyle(DROPDOWNBOX as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
        GetColor(
            GuiGetStyle(
                DROPDOWNBOX as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    if editMode {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < itemCount {
            itemBounds.y
                += bounds.height
                    + GuiGetStyle(
                        DROPDOWNBOX as libc::c_int,
                        DROPDOWN_ITEMS_SPACING as libc::c_int,
                    ) as libc::c_float;
            if i_0 == itemSelected {
                GuiDrawRectangle(
                    itemBounds,
                    GuiGetStyle(DROPDOWNBOX as libc::c_int, BORDER_WIDTH as libc::c_int),
                    GetColor(
                        GuiGetStyle(
                            DROPDOWNBOX as libc::c_int,
                            BORDER_COLOR_PRESSED as libc::c_int,
                        ) as libc::c_uint,
                    ),
                    GetColor(
                        GuiGetStyle(
                            DROPDOWNBOX as libc::c_int,
                            BASE_COLOR_PRESSED as libc::c_int,
                        ) as libc::c_uint,
                    ),
                );
                GuiDrawText(
                    *items.offset(i_0 as isize),
                    GetTextBounds(DROPDOWNBOX as libc::c_int, itemBounds),
                    GuiGetStyle(
                        DROPDOWNBOX as libc::c_int,
                        TEXT_ALIGNMENT as libc::c_int,
                    ),
                    GetColor(
                        GuiGetStyle(
                            DROPDOWNBOX as libc::c_int,
                            TEXT_COLOR_PRESSED as libc::c_int,
                        ) as libc::c_uint,
                    ),
                );
            } else if i_0 == itemFocused {
                GuiDrawRectangle(
                    itemBounds,
                    GuiGetStyle(DROPDOWNBOX as libc::c_int, BORDER_WIDTH as libc::c_int),
                    GetColor(
                        GuiGetStyle(
                            DROPDOWNBOX as libc::c_int,
                            BORDER_COLOR_FOCUSED as libc::c_int,
                        ) as libc::c_uint,
                    ),
                    GetColor(
                        GuiGetStyle(
                            DROPDOWNBOX as libc::c_int,
                            BASE_COLOR_FOCUSED as libc::c_int,
                        ) as libc::c_uint,
                    ),
                );
                GuiDrawText(
                    *items.offset(i_0 as isize),
                    GetTextBounds(DROPDOWNBOX as libc::c_int, itemBounds),
                    GuiGetStyle(
                        DROPDOWNBOX as libc::c_int,
                        TEXT_ALIGNMENT as libc::c_int,
                    ),
                    GetColor(
                        GuiGetStyle(
                            DROPDOWNBOX as libc::c_int,
                            TEXT_COLOR_FOCUSED as libc::c_int,
                        ) as libc::c_uint,
                    ),
                );
            } else {
                GuiDrawText(
                    *items.offset(i_0 as isize),
                    GetTextBounds(DROPDOWNBOX as libc::c_int, itemBounds),
                    GuiGetStyle(
                        DROPDOWNBOX as libc::c_int,
                        TEXT_ALIGNMENT as libc::c_int,
                    ),
                    GetColor(
                        GuiGetStyle(
                            DROPDOWNBOX as libc::c_int,
                            TEXT_COLOR_NORMAL as libc::c_int,
                        ) as libc::c_uint,
                    ),
                );
            }
            i_0 += 1;
            i_0;
        }
    }
    GuiDrawText(
        b"#120#\0" as *const u8 as *const libc::c_char,
        {
            let mut init = Rectangle {
                x: bounds.x + bounds.width
                    - GuiGetStyle(
                        DROPDOWNBOX as libc::c_int,
                        ARROW_PADDING as libc::c_int,
                    ) as libc::c_float,
                y: bounds.y + bounds.height / 2 as libc::c_int as libc::c_float
                    - 6 as libc::c_int as libc::c_float,
                width: 10 as libc::c_int as libc::c_float,
                height: 10 as libc::c_int as libc::c_float,
            };
            init
        },
        TEXT_ALIGN_CENTER as libc::c_int,
        GetColor(
            GuiGetStyle(
                DROPDOWNBOX as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    *active = itemSelected;
    return result;
}
pub unsafe extern "C" fn GuiSpinner(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut value: *mut libc::c_int,
    mut minValue: libc::c_int,
    mut maxValue: libc::c_int,
    mut editMode: bool,
) -> libc::c_int {
    let mut result: libc::c_int = 1 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut tempValue: libc::c_int = *value;
    let mut spinner: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x
                + GuiGetStyle(SPINNER as libc::c_int, SPIN_BUTTON_WIDTH as libc::c_int)
                    as libc::c_float
                + GuiGetStyle(SPINNER as libc::c_int, SPIN_BUTTON_SPACING as libc::c_int)
                    as libc::c_float,
            y: bounds.y,
            width: bounds.width
                - (2 as libc::c_int
                    * (GuiGetStyle(
                        SPINNER as libc::c_int,
                        SPIN_BUTTON_WIDTH as libc::c_int,
                    )
                        + GuiGetStyle(
                            SPINNER as libc::c_int,
                            SPIN_BUTTON_SPACING as libc::c_int,
                        ))) as libc::c_float,
            height: bounds.height,
        };
        init
    };
    let mut leftButtonBound: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: GuiGetStyle(SPINNER as libc::c_int, SPIN_BUTTON_WIDTH as libc::c_int)
                as libc::c_float,
            height: bounds.height,
        };
        init
    };
    let mut rightButtonBound: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x + bounds.width
                - GuiGetStyle(SPINNER as libc::c_int, SPIN_BUTTON_WIDTH as libc::c_int)
                    as libc::c_float,
            y: bounds.y,
            width: GuiGetStyle(SPINNER as libc::c_int, SPIN_BUTTON_WIDTH as libc::c_int)
                as libc::c_float,
            height: bounds.height,
        };
        init
    };
    let mut textBounds: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    if !text.is_null() {
        textBounds
            .width = GetTextWidth(text) as libc::c_float
            + 2 as libc::c_int as libc::c_float;
        textBounds
            .height = GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
            as libc::c_float;
        textBounds
            .x = bounds.x + bounds.width
            + GuiGetStyle(SPINNER as libc::c_int, TEXT_PADDING as libc::c_int)
                as libc::c_float;
        textBounds
            .y = bounds.y + bounds.height / 2 as libc::c_int as libc::c_float
            - (GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                / 2 as libc::c_int) as libc::c_float;
        if GuiGetStyle(SPINNER as libc::c_int, TEXT_ALIGNMENT as libc::c_int)
            == TEXT_ALIGN_LEFT as libc::c_int
        {
            textBounds
                .x = bounds.x - textBounds.width
                - GuiGetStyle(SPINNER as libc::c_int, TEXT_PADDING as libc::c_int)
                    as libc::c_float;
        }
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked && !guiControlExclusiveMode
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if CheckCollisionPointRec(mousePoint, bounds) {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
            } else {
                state = STATE_FOCUSED;
            }
        }
    }
    if GuiButton(
        leftButtonBound,
        GuiIconText(ICON_ARROW_LEFT_FILL as libc::c_int, 0 as *const libc::c_char),
    ) != 0
    {
        tempValue -= 1;
        tempValue;
    }
    if GuiButton(
        rightButtonBound,
        GuiIconText(ICON_ARROW_RIGHT_FILL as libc::c_int, 0 as *const libc::c_char),
    ) != 0
    {
        tempValue += 1;
        tempValue;
    }
    if !editMode {
        if tempValue < minValue {
            tempValue = minValue;
        }
        if tempValue > maxValue {
            tempValue = maxValue;
        }
    }
    result = GuiValueBox(
        spinner,
        0 as *const libc::c_char,
        &mut tempValue,
        minValue,
        maxValue,
        editMode,
    );
    let mut tempBorderWidth: libc::c_int = GuiGetStyle(
        BUTTON as libc::c_int,
        BORDER_WIDTH as libc::c_int,
    );
    let mut tempTextAlign: libc::c_int = GuiGetStyle(
        BUTTON as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
    );
    GuiSetStyle(
        BUTTON as libc::c_int,
        BORDER_WIDTH as libc::c_int,
        GuiGetStyle(SPINNER as libc::c_int, BORDER_WIDTH as libc::c_int),
    );
    GuiSetStyle(
        BUTTON as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_CENTER as libc::c_int,
    );
    GuiSetStyle(BUTTON as libc::c_int, TEXT_ALIGNMENT as libc::c_int, tempTextAlign);
    GuiSetStyle(BUTTON as libc::c_int, BORDER_WIDTH as libc::c_int, tempBorderWidth);
    GuiDrawText(
        text,
        textBounds,
        if GuiGetStyle(SPINNER as libc::c_int, TEXT_ALIGNMENT as libc::c_int)
            == TEXT_ALIGN_RIGHT as libc::c_int
        {
            TEXT_ALIGN_LEFT as libc::c_int
        } else {
            TEXT_ALIGN_RIGHT as libc::c_int
        },
        GetColor(
            GuiGetStyle(
                LABEL as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    *value = tempValue;
    return result;
}
pub unsafe extern "C" fn GuiValueBox(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut value: *mut libc::c_int,
    mut minValue: libc::c_int,
    mut maxValue: libc::c_int,
    mut editMode: bool,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut textValue: [libc::c_char; 33] = *::std::mem::transmute::<
        &[u8; 33],
        &mut [libc::c_char; 33],
    >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    sprintf(textValue.as_mut_ptr(), b"%i\0" as *const u8 as *const libc::c_char, *value);
    let mut textBounds: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    if !text.is_null() {
        textBounds
            .width = GetTextWidth(text) as libc::c_float
            + 2 as libc::c_int as libc::c_float;
        textBounds
            .height = GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
            as libc::c_float;
        textBounds
            .x = bounds.x + bounds.width
            + GuiGetStyle(VALUEBOX as libc::c_int, TEXT_PADDING as libc::c_int)
                as libc::c_float;
        textBounds
            .y = bounds.y + bounds.height / 2 as libc::c_int as libc::c_float
            - (GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                / 2 as libc::c_int) as libc::c_float;
        if GuiGetStyle(VALUEBOX as libc::c_int, TEXT_ALIGNMENT as libc::c_int)
            == TEXT_ALIGN_LEFT as libc::c_int
        {
            textBounds
                .x = bounds.x - textBounds.width
                - GuiGetStyle(VALUEBOX as libc::c_int, TEXT_PADDING as libc::c_int)
                    as libc::c_float;
        }
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked && !guiControlExclusiveMode
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        let mut valueHasChanged: bool = 0 as libc::c_int != 0;
        if editMode {
            state = STATE_PRESSED;
            let mut keyCount: libc::c_int = strlen(textValue.as_mut_ptr())
                as libc::c_int;
            if keyCount < 32 as libc::c_int {
                if (GetTextWidth(textValue.as_mut_ptr()) as libc::c_float) < bounds.width
                {
                    let mut key: libc::c_int = GetCharPressed();
                    if key >= 48 as libc::c_int && key <= 57 as libc::c_int {
                        textValue[keyCount as usize] = key as libc::c_char;
                        keyCount += 1;
                        keyCount;
                        valueHasChanged = 1 as libc::c_int != 0;
                    }
                }
            }
            if keyCount > 0 as libc::c_int {
                if IsKeyPressed(KEY_BACKSPACE as libc::c_int) {
                    keyCount -= 1;
                    keyCount;
                    textValue[keyCount as usize] = '\0' as i32 as libc::c_char;
                    valueHasChanged = 1 as libc::c_int != 0;
                }
            }
            if valueHasChanged {
                *value = TextToInteger(textValue.as_mut_ptr());
            }
            if IsKeyPressed(KEY_ENTER as libc::c_int) as libc::c_int != 0
                || !CheckCollisionPointRec(mousePoint, bounds)
                    && IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int)
                        as libc::c_int != 0
            {
                if *value > maxValue {
                    *value = maxValue;
                } else if *value < minValue {
                    *value = minValue;
                }
                result = 1 as libc::c_int;
            }
        } else {
            if *value > maxValue {
                *value = maxValue;
            } else if *value < minValue {
                *value = minValue;
            }
            if CheckCollisionPointRec(mousePoint, bounds) {
                state = STATE_FOCUSED;
                if IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int) {
                    result = 1 as libc::c_int;
                }
            }
        }
    }
    let mut baseColor: Color = {
        let mut init = Color {
            r: 0 as libc::c_int as libc::c_uchar,
            g: 0 as libc::c_int as libc::c_uchar,
            b: 0 as libc::c_int as libc::c_uchar,
            a: 0 as libc::c_int as libc::c_uchar,
        };
        init
    };
    if state as libc::c_uint == STATE_PRESSED as libc::c_int as libc::c_uint {
        baseColor = GetColor(
            GuiGetStyle(VALUEBOX as libc::c_int, BASE_COLOR_PRESSED as libc::c_int)
                as libc::c_uint,
        );
    } else if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint {
        baseColor = GetColor(
            GuiGetStyle(VALUEBOX as libc::c_int, BASE_COLOR_DISABLED as libc::c_int)
                as libc::c_uint,
        );
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(VALUEBOX as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                VALUEBOX as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        baseColor,
    );
    GuiDrawText(
        textValue.as_mut_ptr(),
        GetTextBounds(VALUEBOX as libc::c_int, bounds),
        TEXT_ALIGN_CENTER as libc::c_int,
        GetColor(
            GuiGetStyle(
                VALUEBOX as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    if editMode {
        let mut cursor: Rectangle = {
            let mut init = Rectangle {
                x: bounds.x
                    + (GetTextWidth(textValue.as_mut_ptr()) / 2 as libc::c_int)
                        as libc::c_float
                    + bounds.width / 2 as libc::c_int as libc::c_float
                    + 1 as libc::c_int as libc::c_float,
                y: bounds.y
                    + (2 as libc::c_int
                        * GuiGetStyle(
                            VALUEBOX as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )) as libc::c_float,
                width: 4 as libc::c_int as libc::c_float,
                height: bounds.height
                    - (4 as libc::c_int
                        * GuiGetStyle(
                            VALUEBOX as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )) as libc::c_float,
            };
            init
        };
        GuiDrawRectangle(
            cursor,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(VALUEBOX as libc::c_int, BORDER_COLOR_PRESSED as libc::c_int)
                    as libc::c_uint,
            ),
        );
    }
    GuiDrawText(
        text,
        textBounds,
        if GuiGetStyle(VALUEBOX as libc::c_int, TEXT_ALIGNMENT as libc::c_int)
            == TEXT_ALIGN_RIGHT as libc::c_int
        {
            TEXT_ALIGN_LEFT as libc::c_int
        } else {
            TEXT_ALIGN_RIGHT as libc::c_int
        },
        GetColor(
            GuiGetStyle(
                LABEL as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    return result;
}
pub unsafe extern "C" fn GuiValueBoxFloat(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut textValue: *mut libc::c_char,
    mut value: *mut libc::c_float,
    mut editMode: bool,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut textBounds: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    if !text.is_null() {
        textBounds
            .width = GetTextWidth(text) as libc::c_float
            + 2 as libc::c_int as libc::c_float;
        textBounds
            .height = GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
            as libc::c_float;
        textBounds
            .x = bounds.x + bounds.width
            + GuiGetStyle(VALUEBOX as libc::c_int, TEXT_PADDING as libc::c_int)
                as libc::c_float;
        textBounds
            .y = bounds.y + bounds.height / 2 as libc::c_int as libc::c_float
            - (GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                / 2 as libc::c_int) as libc::c_float;
        if GuiGetStyle(VALUEBOX as libc::c_int, TEXT_ALIGNMENT as libc::c_int)
            == TEXT_ALIGN_LEFT as libc::c_int
        {
            textBounds
                .x = bounds.x - textBounds.width
                - GuiGetStyle(VALUEBOX as libc::c_int, TEXT_PADDING as libc::c_int)
                    as libc::c_float;
        }
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked && !guiControlExclusiveMode
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        let mut valueHasChanged: bool = 0 as libc::c_int != 0;
        if editMode {
            state = STATE_PRESSED;
            let mut keyCount: libc::c_int = strlen(textValue) as libc::c_int;
            if keyCount < 32 as libc::c_int {
                if (GetTextWidth(textValue) as libc::c_float) < bounds.width {
                    let mut key: libc::c_int = GetCharPressed();
                    if key >= 48 as libc::c_int && key <= 57 as libc::c_int
                        || key == '.' as i32
                        || keyCount == 0 as libc::c_int && key == '+' as i32
                        || keyCount == 0 as libc::c_int && key == '-' as i32
                    {
                        *textValue.offset(keyCount as isize) = key as libc::c_char;
                        keyCount += 1;
                        keyCount;
                        valueHasChanged = 1 as libc::c_int != 0;
                    }
                }
            }
            if IsKeyPressed(KEY_BACKSPACE as libc::c_int) {
                if keyCount > 0 as libc::c_int {
                    keyCount -= 1;
                    keyCount;
                    *textValue.offset(keyCount as isize) = '\0' as i32 as libc::c_char;
                    valueHasChanged = 1 as libc::c_int != 0;
                }
            }
            if valueHasChanged {
                *value = TextToFloat(textValue);
            }
            if IsKeyPressed(KEY_ENTER as libc::c_int) as libc::c_int != 0
                || !CheckCollisionPointRec(mousePoint, bounds)
                    && IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int)
                        as libc::c_int != 0
            {
                result = 1 as libc::c_int;
            }
        } else if CheckCollisionPointRec(mousePoint, bounds) {
            state = STATE_FOCUSED;
            if IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int) {
                result = 1 as libc::c_int;
            }
        }
    }
    let mut baseColor: Color = {
        let mut init = Color {
            r: 0 as libc::c_int as libc::c_uchar,
            g: 0 as libc::c_int as libc::c_uchar,
            b: 0 as libc::c_int as libc::c_uchar,
            a: 0 as libc::c_int as libc::c_uchar,
        };
        init
    };
    if state as libc::c_uint == STATE_PRESSED as libc::c_int as libc::c_uint {
        baseColor = GetColor(
            GuiGetStyle(VALUEBOX as libc::c_int, BASE_COLOR_PRESSED as libc::c_int)
                as libc::c_uint,
        );
    } else if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint {
        baseColor = GetColor(
            GuiGetStyle(VALUEBOX as libc::c_int, BASE_COLOR_DISABLED as libc::c_int)
                as libc::c_uint,
        );
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(VALUEBOX as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                VALUEBOX as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        baseColor,
    );
    GuiDrawText(
        textValue,
        GetTextBounds(VALUEBOX as libc::c_int, bounds),
        TEXT_ALIGN_CENTER as libc::c_int,
        GetColor(
            GuiGetStyle(
                VALUEBOX as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    if editMode {
        let mut cursor: Rectangle = {
            let mut init = Rectangle {
                x: bounds.x
                    + (GetTextWidth(textValue) / 2 as libc::c_int) as libc::c_float
                    + bounds.width / 2 as libc::c_int as libc::c_float
                    + 1 as libc::c_int as libc::c_float,
                y: bounds.y
                    + (2 as libc::c_int
                        * GuiGetStyle(
                            VALUEBOX as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )) as libc::c_float,
                width: 4 as libc::c_int as libc::c_float,
                height: bounds.height
                    - (4 as libc::c_int
                        * GuiGetStyle(
                            VALUEBOX as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )) as libc::c_float,
            };
            init
        };
        GuiDrawRectangle(
            cursor,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(VALUEBOX as libc::c_int, BORDER_COLOR_PRESSED as libc::c_int)
                    as libc::c_uint,
            ),
        );
    }
    GuiDrawText(
        text,
        textBounds,
        if GuiGetStyle(VALUEBOX as libc::c_int, TEXT_ALIGNMENT as libc::c_int)
            == TEXT_ALIGN_RIGHT as libc::c_int
        {
            TEXT_ALIGN_LEFT as libc::c_int
        } else {
            TEXT_ALIGN_RIGHT as libc::c_int
        },
        GetColor(
            GuiGetStyle(
                LABEL as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    return result;
}
pub unsafe extern "C" fn GuiTextBox(
    mut bounds: Rectangle,
    mut text: *mut libc::c_char,
    mut textSize: libc::c_int,
    mut editMode: bool,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut multiline: bool = 0 as libc::c_int != 0;
    let mut wrapMode: libc::c_int = GuiGetStyle(
        DEFAULT as libc::c_int,
        TEXT_WRAP_MODE as libc::c_int,
    );
    let mut textBounds: Rectangle = GetTextBounds(TEXTBOX as libc::c_int, bounds);
    let mut textWidth: libc::c_int = GetTextWidth(text)
        - GetTextWidth(text.offset(textBoxCursorIndex as isize));
    let mut textIndexOffset: libc::c_int = 0 as libc::c_int;
    let mut cursor: Rectangle = {
        let mut init = Rectangle {
            x: textBounds.x + textWidth as libc::c_float
                + GuiGetStyle(DEFAULT as libc::c_int, TEXT_SPACING as libc::c_int)
                    as libc::c_float,
            y: textBounds.y + textBounds.height / 2 as libc::c_int as libc::c_float
                - GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                    as libc::c_float,
            width: 2 as libc::c_int as libc::c_float,
            height: GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                as libc::c_float * 2 as libc::c_int as libc::c_float,
        };
        init
    };
    if cursor.height >= bounds.height {
        cursor
            .height = bounds.height
            - (GuiGetStyle(TEXTBOX as libc::c_int, BORDER_WIDTH as libc::c_int)
                * 2 as libc::c_int) as libc::c_float;
    }
    if cursor.y
        < bounds.y
            + GuiGetStyle(TEXTBOX as libc::c_int, BORDER_WIDTH as libc::c_int)
                as libc::c_float
    {
        cursor
            .y = bounds.y
            + GuiGetStyle(TEXTBOX as libc::c_int, BORDER_WIDTH as libc::c_int)
                as libc::c_float;
    }
    let mut mouseCursor: Rectangle = cursor;
    mouseCursor.x = -(1 as libc::c_int) as libc::c_float;
    mouseCursor.width = 1 as libc::c_int as libc::c_float;
    if IsKeyDown(KEY_LEFT as libc::c_int) as libc::c_int != 0
        || IsKeyDown(KEY_RIGHT as libc::c_int) as libc::c_int != 0
        || IsKeyDown(KEY_UP as libc::c_int) as libc::c_int != 0
        || IsKeyDown(KEY_DOWN as libc::c_int) as libc::c_int != 0
        || IsKeyDown(KEY_BACKSPACE as libc::c_int) as libc::c_int != 0
        || IsKeyDown(KEY_DELETE as libc::c_int) as libc::c_int != 0
    {
        autoCursorCooldownCounter += 1;
        autoCursorCooldownCounter;
    } else {
        autoCursorCooldownCounter = 0 as libc::c_int;
        autoCursorDelayCounter = 0 as libc::c_int;
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && GuiGetStyle(TEXTBOX as libc::c_int, TEXT_READONLY as libc::c_int) == 0
        && !guiLocked && !guiControlExclusiveMode
        && wrapMode == TEXT_WRAP_NONE as libc::c_int
    {
        let mut mousePosition: Vector2 = GetMousePosition();
        if editMode {
            state = STATE_PRESSED;
            while textWidth as libc::c_float >= textBounds.width {
                let mut nextCodepointSize: libc::c_int = 0 as libc::c_int;
                GetCodepointNext(
                    text.offset(textIndexOffset as isize),
                    &mut nextCodepointSize,
                );
                textIndexOffset += nextCodepointSize;
                textWidth = GetTextWidth(text.offset(textIndexOffset as isize))
                    - GetTextWidth(text.offset(textBoxCursorIndex as isize));
            }
            let mut textLength: libc::c_int = strlen(text) as libc::c_int;
            let mut codepoint: libc::c_int = GetCharPressed();
            if multiline as libc::c_int != 0
                && IsKeyPressed(KEY_ENTER as libc::c_int) as libc::c_int != 0
            {
                codepoint = '\n' as i32;
            }
            if textBoxCursorIndex > textLength {
                textBoxCursorIndex = textLength;
            }
            let mut codepointSize: libc::c_int = 0 as libc::c_int;
            let mut charEncoded: *const libc::c_char = CodepointToUTF8(
                codepoint,
                &mut codepointSize,
            );
            if (multiline as libc::c_int != 0 && codepoint == '\n' as i32
                || codepoint >= 32 as libc::c_int)
                && textLength + codepointSize < textSize
            {
                let mut i: libc::c_int = textLength + codepointSize;
                while i > textBoxCursorIndex {
                    *text
                        .offset(i as isize) = *text.offset((i - codepointSize) as isize);
                    i -= 1;
                    i;
                }
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < codepointSize {
                    *text
                        .offset(
                            (textBoxCursorIndex + i_0) as isize,
                        ) = *charEncoded.offset(i_0 as isize);
                    i_0 += 1;
                    i_0;
                }
                textBoxCursorIndex += codepointSize;
                textLength += codepointSize;
                *text.offset(textLength as isize) = '\0' as i32 as libc::c_char;
            }
            if textLength > 0 as libc::c_int
                && IsKeyPressed(KEY_HOME as libc::c_int) as libc::c_int != 0
            {
                textBoxCursorIndex = 0 as libc::c_int;
            }
            if textLength > textBoxCursorIndex
                && IsKeyPressed(KEY_END as libc::c_int) as libc::c_int != 0
            {
                textBoxCursorIndex = textLength;
            }
            if textLength > textBoxCursorIndex
                && (IsKeyPressed(KEY_DELETE as libc::c_int) as libc::c_int != 0
                    || IsKeyDown(KEY_DELETE as libc::c_int) as libc::c_int != 0
                        && autoCursorCooldownCounter >= 40 as libc::c_int)
            {
                autoCursorDelayCounter += 1;
                autoCursorDelayCounter;
                if IsKeyPressed(KEY_DELETE as libc::c_int) as libc::c_int != 0
                    || autoCursorDelayCounter % 1 as libc::c_int == 0 as libc::c_int
                {
                    let mut nextCodepointSize_0: libc::c_int = 0 as libc::c_int;
                    GetCodepointNext(
                        text.offset(textBoxCursorIndex as isize),
                        &mut nextCodepointSize_0,
                    );
                    let mut i_1: libc::c_int = textBoxCursorIndex;
                    while i_1 < textLength {
                        *text
                            .offset(
                                i_1 as isize,
                            ) = *text.offset((i_1 + nextCodepointSize_0) as isize);
                        i_1 += 1;
                        i_1;
                    }
                    textLength -= codepointSize;
                    *text.offset(textLength as isize) = '\0' as i32 as libc::c_char;
                }
            }
            if textLength > 0 as libc::c_int
                && (IsKeyPressed(KEY_BACKSPACE as libc::c_int) as libc::c_int != 0
                    || IsKeyDown(KEY_BACKSPACE as libc::c_int) as libc::c_int != 0
                        && autoCursorCooldownCounter >= 40 as libc::c_int)
            {
                autoCursorDelayCounter += 1;
                autoCursorDelayCounter;
                if IsKeyPressed(KEY_BACKSPACE as libc::c_int) as libc::c_int != 0
                    || autoCursorDelayCounter % 1 as libc::c_int == 0 as libc::c_int
                {
                    let mut prevCodepointSize: libc::c_int = 0 as libc::c_int;
                    GetCodepointPrevious(
                        text.offset(textBoxCursorIndex as isize),
                        &mut prevCodepointSize,
                    );
                    let mut i_2: libc::c_int = textBoxCursorIndex - prevCodepointSize;
                    while i_2 < textLength {
                        *text
                            .offset(
                                i_2 as isize,
                            ) = *text.offset((i_2 + prevCodepointSize) as isize);
                        i_2 += 1;
                        i_2;
                    }
                    if textBoxCursorIndex > 0 as libc::c_int {
                        textBoxCursorIndex -= codepointSize;
                        textLength -= codepointSize;
                    }
                    *text.offset(textLength as isize) = '\0' as i32 as libc::c_char;
                }
            }
            if IsKeyPressed(KEY_LEFT as libc::c_int) as libc::c_int != 0
                || IsKeyDown(KEY_LEFT as libc::c_int) as libc::c_int != 0
                    && autoCursorCooldownCounter > 40 as libc::c_int
            {
                autoCursorDelayCounter += 1;
                autoCursorDelayCounter;
                if IsKeyPressed(KEY_LEFT as libc::c_int) as libc::c_int != 0
                    || autoCursorDelayCounter % 1 as libc::c_int == 0 as libc::c_int
                {
                    let mut prevCodepointSize_0: libc::c_int = 0 as libc::c_int;
                    GetCodepointPrevious(
                        text.offset(textBoxCursorIndex as isize),
                        &mut prevCodepointSize_0,
                    );
                    if textBoxCursorIndex >= prevCodepointSize_0 {
                        textBoxCursorIndex -= prevCodepointSize_0;
                    }
                }
            } else if IsKeyPressed(KEY_RIGHT as libc::c_int) as libc::c_int != 0
                || IsKeyDown(KEY_RIGHT as libc::c_int) as libc::c_int != 0
                    && autoCursorCooldownCounter > 40 as libc::c_int
            {
                autoCursorDelayCounter += 1;
                autoCursorDelayCounter;
                if IsKeyPressed(KEY_RIGHT as libc::c_int) as libc::c_int != 0
                    || autoCursorDelayCounter % 1 as libc::c_int == 0 as libc::c_int
                {
                    let mut nextCodepointSize_1: libc::c_int = 0 as libc::c_int;
                    GetCodepointNext(
                        text.offset(textBoxCursorIndex as isize),
                        &mut nextCodepointSize_1,
                    );
                    if textBoxCursorIndex + nextCodepointSize_1 <= textLength {
                        textBoxCursorIndex += nextCodepointSize_1;
                    }
                }
            }
            if CheckCollisionPointRec(mousePosition, textBounds) {
                let mut scaleFactor: libc::c_float = GuiGetStyle(
                    DEFAULT as libc::c_int,
                    TEXT_SIZE as libc::c_int,
                ) as libc::c_float / guiFont.baseSize as libc::c_float;
                let mut codepointIndex: libc::c_int = 0 as libc::c_int;
                let mut glyphWidth: libc::c_float = 0.0f32;
                let mut widthToMouseX: libc::c_float = 0 as libc::c_int as libc::c_float;
                let mut mouseCursorIndex: libc::c_int = 0 as libc::c_int;
                let mut i_3: libc::c_int = textIndexOffset;
                while i_3 < textLength {
                    codepoint = GetCodepointNext(
                        &mut *text.offset(i_3 as isize),
                        &mut codepointSize,
                    );
                    codepointIndex = GetGlyphIndex(guiFont, codepoint);
                    if (*(guiFont.glyphs).offset(codepointIndex as isize)).advanceX
                        == 0 as libc::c_int
                    {
                        glyphWidth = (*(guiFont.recs).offset(codepointIndex as isize))
                            .width * scaleFactor;
                    } else {
                        glyphWidth = (*(guiFont.glyphs).offset(codepointIndex as isize))
                            .advanceX as libc::c_float * scaleFactor;
                    }
                    if mousePosition.x
                        <= textBounds.x
                            + (widthToMouseX
                                + glyphWidth / 2 as libc::c_int as libc::c_float)
                    {
                        mouseCursor.x = textBounds.x + widthToMouseX;
                        mouseCursorIndex = i_3;
                        break;
                    } else {
                        widthToMouseX
                            += glyphWidth
                                + GuiGetStyle(
                                    DEFAULT as libc::c_int,
                                    TEXT_SPACING as libc::c_int,
                                ) as libc::c_float;
                        i_3 += 1;
                        i_3;
                    }
                }
                let mut textEndWidth: libc::c_int = GetTextWidth(
                    text.offset(textIndexOffset as isize),
                );
                if (GetMousePosition()).x
                    >= textBounds.x + textEndWidth as libc::c_float
                        - glyphWidth / 2 as libc::c_int as libc::c_float
                {
                    mouseCursor.x = textBounds.x + textEndWidth as libc::c_float;
                    mouseCursorIndex = strlen(text) as libc::c_int;
                }
                if mouseCursor.x >= 0 as libc::c_int as libc::c_float
                    && IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int)
                        as libc::c_int != 0
                {
                    cursor.x = mouseCursor.x;
                    textBoxCursorIndex = mouseCursorIndex;
                }
            } else {
                mouseCursor.x = -(1 as libc::c_int) as libc::c_float;
            }
            cursor
                .x = bounds.x
                + GuiGetStyle(TEXTBOX as libc::c_int, TEXT_PADDING as libc::c_int)
                    as libc::c_float
                + GetTextWidth(text.offset(textIndexOffset as isize)) as libc::c_float
                - GetTextWidth(text.offset(textBoxCursorIndex as isize)) as libc::c_float
                + GuiGetStyle(DEFAULT as libc::c_int, TEXT_SPACING as libc::c_int)
                    as libc::c_float;
            if !multiline && IsKeyPressed(KEY_ENTER as libc::c_int) as libc::c_int != 0
                || !CheckCollisionPointRec(mousePosition, bounds)
                    && IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int)
                        as libc::c_int != 0
            {
                textBoxCursorIndex = 0 as libc::c_int;
                result = 1 as libc::c_int;
            }
        } else if CheckCollisionPointRec(mousePosition, bounds) {
            state = STATE_FOCUSED;
            if IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int) {
                textBoxCursorIndex = strlen(text) as libc::c_int;
                result = 1 as libc::c_int;
            }
        }
    }
    if state as libc::c_uint == STATE_PRESSED as libc::c_int as libc::c_uint {
        GuiDrawRectangle(
            bounds,
            GuiGetStyle(TEXTBOX as libc::c_int, BORDER_WIDTH as libc::c_int),
            GetColor(
                GuiGetStyle(
                    TEXTBOX as libc::c_int,
                    (BORDER as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
            GetColor(
                GuiGetStyle(TEXTBOX as libc::c_int, BASE_COLOR_PRESSED as libc::c_int)
                    as libc::c_uint,
            ),
        );
    } else if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint {
        GuiDrawRectangle(
            bounds,
            GuiGetStyle(TEXTBOX as libc::c_int, BORDER_WIDTH as libc::c_int),
            GetColor(
                GuiGetStyle(
                    TEXTBOX as libc::c_int,
                    (BORDER as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
            GetColor(
                GuiGetStyle(TEXTBOX as libc::c_int, BASE_COLOR_DISABLED as libc::c_int)
                    as libc::c_uint,
            ),
        );
    } else {
        GuiDrawRectangle(
            bounds,
            GuiGetStyle(TEXTBOX as libc::c_int, BORDER_WIDTH as libc::c_int),
            GetColor(
                GuiGetStyle(
                    TEXTBOX as libc::c_int,
                    (BORDER as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
        );
    }
    GuiDrawText(
        text.offset(textIndexOffset as isize),
        textBounds,
        GuiGetStyle(TEXTBOX as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
        GetColor(
            GuiGetStyle(
                TEXTBOX as libc::c_int,
                (TEXT as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    if editMode as libc::c_int != 0
        && GuiGetStyle(TEXTBOX as libc::c_int, TEXT_READONLY as libc::c_int) == 0
    {
        GuiDrawRectangle(
            cursor,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(TEXTBOX as libc::c_int, BORDER_COLOR_PRESSED as libc::c_int)
                    as libc::c_uint,
            ),
        );
        if mouseCursor.x >= 0 as libc::c_int as libc::c_float {
            GuiDrawRectangle(
                mouseCursor,
                0 as libc::c_int,
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 0 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                GetColor(
                    GuiGetStyle(
                        TEXTBOX as libc::c_int,
                        BORDER_COLOR_PRESSED as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
        }
    } else if state as libc::c_uint == STATE_FOCUSED as libc::c_int as libc::c_uint {
        GuiTooltip(bounds);
    }
    return result;
}
static mut textBoxCursorIndex: libc::c_int = 0 as libc::c_int;
static mut autoCursorDelayCounter: libc::c_int = 0 as libc::c_int;
static mut autoCursorCooldownCounter: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn GuiSlider(
    mut bounds: Rectangle,
    mut textLeft: *const libc::c_char,
    mut textRight: *const libc::c_char,
    mut value: *mut libc::c_float,
    mut minValue: libc::c_float,
    mut maxValue: libc::c_float,
) -> libc::c_int {
    return GuiSliderPro(
        bounds,
        textLeft,
        textRight,
        value,
        minValue,
        maxValue,
        GuiGetStyle(SLIDER as libc::c_int, SLIDER_WIDTH as libc::c_int),
    );
}
pub unsafe extern "C" fn GuiSliderPro(
    mut bounds: Rectangle,
    mut textLeft: *const libc::c_char,
    mut textRight: *const libc::c_char,
    mut value: *mut libc::c_float,
    mut minValue: libc::c_float,
    mut maxValue: libc::c_float,
    mut sliderWidth: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut temp: libc::c_float = (maxValue - minValue) / 2.0f32;
    if value.is_null() {
        value = &mut temp;
    }
    let mut oldValue: libc::c_float = *value;
    let mut slider: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x,
            y: bounds.y
                + GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float
                + GuiGetStyle(SLIDER as libc::c_int, SLIDER_PADDING as libc::c_int)
                    as libc::c_float,
            width: 0 as libc::c_int as libc::c_float,
            height: bounds.height
                - (2 as libc::c_int
                    * GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int))
                    as libc::c_float
                - (2 as libc::c_int
                    * GuiGetStyle(SLIDER as libc::c_int, SLIDER_PADDING as libc::c_int))
                    as libc::c_float,
        };
        init
    };
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if guiControlExclusiveMode {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                if bounds.x == guiControlExclusiveRec.x
                    && bounds.y == guiControlExclusiveRec.y
                    && bounds.width == guiControlExclusiveRec.width
                    && bounds.height == guiControlExclusiveRec.height
                {
                    state = STATE_PRESSED;
                    *value = (maxValue - minValue)
                        * ((mousePoint.x - bounds.x
                            - (sliderWidth / 2 as libc::c_int) as libc::c_float)
                            / (bounds.width - sliderWidth as libc::c_float)) + minValue;
                }
            } else {
                guiControlExclusiveMode = 0 as libc::c_int != 0;
                guiControlExclusiveRec = {
                    let mut init = Rectangle {
                        x: 0 as libc::c_int as libc::c_float,
                        y: 0 as libc::c_int as libc::c_float,
                        width: 0 as libc::c_int as libc::c_float,
                        height: 0 as libc::c_int as libc::c_float,
                    };
                    init
                };
            }
        } else if CheckCollisionPointRec(mousePoint, bounds) {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
                guiControlExclusiveMode = 1 as libc::c_int != 0;
                guiControlExclusiveRec = bounds;
                if !CheckCollisionPointRec(mousePoint, slider) {
                    *value = (maxValue - minValue)
                        * ((mousePoint.x - bounds.x
                            - (sliderWidth / 2 as libc::c_int) as libc::c_float)
                            / (bounds.width - sliderWidth as libc::c_float)) + minValue;
                }
            } else {
                state = STATE_FOCUSED;
            }
        }
        if *value > maxValue {
            *value = maxValue;
        } else if *value < minValue {
            *value = minValue;
        }
    }
    if oldValue == *value {
        result = 0 as libc::c_int;
    } else {
        result = 1 as libc::c_int;
    }
    let mut sliderValue: libc::c_float = (*value - minValue) / (maxValue - minValue)
        * (bounds.width - sliderWidth as libc::c_float
            - (2 as libc::c_int
                * GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int))
                as libc::c_float);
    if sliderWidth > 0 as libc::c_int {
        slider.x += sliderValue;
        slider.width = sliderWidth as libc::c_float;
        if slider.x
            <= bounds.x
                + GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float
        {
            slider
                .x = bounds.x
                + GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float;
        } else if slider.x + slider.width >= bounds.x + bounds.width {
            slider
                .x = bounds.x + bounds.width - slider.width
                - GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float;
        }
    } else if sliderWidth == 0 as libc::c_int {
        slider.x
            += GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int)
                as libc::c_float;
        slider.width = sliderValue;
        if slider.width > bounds.width {
            slider
                .width = bounds.width
                - (2 as libc::c_int
                    * GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int))
                    as libc::c_float;
        }
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(SLIDER as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                SLIDER as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        GetColor(
            GuiGetStyle(
                SLIDER as libc::c_int,
                if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
                {
                    BASE_COLOR_NORMAL as libc::c_int
                } else {
                    BASE_COLOR_DISABLED as libc::c_int
                },
            ) as libc::c_uint,
        ),
    );
    if state as libc::c_uint == STATE_NORMAL as libc::c_int as libc::c_uint {
        GuiDrawRectangle(
            slider,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(SLIDER as libc::c_int, BASE_COLOR_PRESSED as libc::c_int)
                    as libc::c_uint,
            ),
        );
    } else if state as libc::c_uint == STATE_FOCUSED as libc::c_int as libc::c_uint {
        GuiDrawRectangle(
            slider,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(SLIDER as libc::c_int, TEXT_COLOR_FOCUSED as libc::c_int)
                    as libc::c_uint,
            ),
        );
    } else if state as libc::c_uint == STATE_PRESSED as libc::c_int as libc::c_uint {
        GuiDrawRectangle(
            slider,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(SLIDER as libc::c_int, TEXT_COLOR_PRESSED as libc::c_int)
                    as libc::c_uint,
            ),
        );
    }
    if !textLeft.is_null() {
        let mut textBounds: Rectangle = {
            let mut init = Rectangle {
                x: 0 as libc::c_int as libc::c_float,
                y: 0.,
                width: 0.,
                height: 0.,
            };
            init
        };
        textBounds.width = GetTextWidth(textLeft) as libc::c_float;
        textBounds
            .height = GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
            as libc::c_float;
        textBounds
            .x = bounds.x - textBounds.width
            - GuiGetStyle(SLIDER as libc::c_int, TEXT_PADDING as libc::c_int)
                as libc::c_float;
        textBounds
            .y = bounds.y + bounds.height / 2 as libc::c_int as libc::c_float
            - (GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                / 2 as libc::c_int) as libc::c_float;
        GuiDrawText(
            textLeft,
            textBounds,
            TEXT_ALIGN_RIGHT as libc::c_int,
            GetColor(
                GuiGetStyle(
                    SLIDER as libc::c_int,
                    (TEXT as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
    }
    if !textRight.is_null() {
        let mut textBounds_0: Rectangle = {
            let mut init = Rectangle {
                x: 0 as libc::c_int as libc::c_float,
                y: 0.,
                width: 0.,
                height: 0.,
            };
            init
        };
        textBounds_0.width = GetTextWidth(textRight) as libc::c_float;
        textBounds_0
            .height = GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
            as libc::c_float;
        textBounds_0
            .x = bounds.x + bounds.width
            + GuiGetStyle(SLIDER as libc::c_int, TEXT_PADDING as libc::c_int)
                as libc::c_float;
        textBounds_0
            .y = bounds.y + bounds.height / 2 as libc::c_int as libc::c_float
            - (GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                / 2 as libc::c_int) as libc::c_float;
        GuiDrawText(
            textRight,
            textBounds_0,
            TEXT_ALIGN_LEFT as libc::c_int,
            GetColor(
                GuiGetStyle(
                    SLIDER as libc::c_int,
                    (TEXT as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
    }
    return result;
}
pub unsafe extern "C" fn GuiSliderBar(
    mut bounds: Rectangle,
    mut textLeft: *const libc::c_char,
    mut textRight: *const libc::c_char,
    mut value: *mut libc::c_float,
    mut minValue: libc::c_float,
    mut maxValue: libc::c_float,
) -> libc::c_int {
    return GuiSliderPro(
        bounds,
        textLeft,
        textRight,
        value,
        minValue,
        maxValue,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn GuiProgressBar(
    mut bounds: Rectangle,
    mut textLeft: *const libc::c_char,
    mut textRight: *const libc::c_char,
    mut value: *mut libc::c_float,
    mut minValue: libc::c_float,
    mut maxValue: libc::c_float,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut temp: libc::c_float = (maxValue - minValue) / 2.0f32;
    if value.is_null() {
        value = &mut temp;
    }
    let mut progress: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x
                + GuiGetStyle(PROGRESSBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float,
            y: bounds.y
                + GuiGetStyle(PROGRESSBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
                    as libc::c_float
                + GuiGetStyle(
                    PROGRESSBAR as libc::c_int,
                    PROGRESS_PADDING as libc::c_int,
                ) as libc::c_float,
            width: 0 as libc::c_int as libc::c_float,
            height: bounds.height
                - (2 as libc::c_int
                    * GuiGetStyle(
                        PROGRESSBAR as libc::c_int,
                        BORDER_WIDTH as libc::c_int,
                    )) as libc::c_float
                - (2 as libc::c_int
                    * GuiGetStyle(
                        PROGRESSBAR as libc::c_int,
                        PROGRESS_PADDING as libc::c_int,
                    )) as libc::c_float,
        };
        init
    };
    if *value > maxValue {
        *value = maxValue;
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint {
        progress
            .width = *value / (maxValue - minValue) * bounds.width
            - (if *value >= maxValue {
                (2 as libc::c_int
                    * GuiGetStyle(
                        PROGRESSBAR as libc::c_int,
                        BORDER_WIDTH as libc::c_int,
                    )) as libc::c_float
            } else {
                0.0f32
            });
    }
    if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint {
        GuiDrawRectangle(
            bounds,
            GuiGetStyle(PROGRESSBAR as libc::c_int, BORDER_WIDTH as libc::c_int),
            GetColor(
                GuiGetStyle(
                    PROGRESSBAR as libc::c_int,
                    (BORDER as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
        );
    } else {
        if *value > minValue {
            GuiDrawRectangle(
                {
                    let mut init = Rectangle {
                        x: bounds.x,
                        y: bounds.y,
                        width: progress.width as libc::c_int as libc::c_float
                            + GuiGetStyle(
                                PROGRESSBAR as libc::c_int,
                                BORDER_WIDTH as libc::c_int,
                            ) as libc::c_float,
                        height: GuiGetStyle(
                            PROGRESSBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        ) as libc::c_float,
                    };
                    init
                },
                0 as libc::c_int,
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 0 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                GetColor(
                    GuiGetStyle(
                        PROGRESSBAR as libc::c_int,
                        BORDER_COLOR_FOCUSED as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
            GuiDrawRectangle(
                {
                    let mut init = Rectangle {
                        x: bounds.x,
                        y: bounds.y + 1 as libc::c_int as libc::c_float,
                        width: GuiGetStyle(
                            PROGRESSBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        ) as libc::c_float,
                        height: bounds.height - 2 as libc::c_int as libc::c_float,
                    };
                    init
                },
                0 as libc::c_int,
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 0 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                GetColor(
                    GuiGetStyle(
                        PROGRESSBAR as libc::c_int,
                        BORDER_COLOR_FOCUSED as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
            GuiDrawRectangle(
                {
                    let mut init = Rectangle {
                        x: bounds.x,
                        y: bounds.y + bounds.height - 1 as libc::c_int as libc::c_float,
                        width: progress.width as libc::c_int as libc::c_float
                            + GuiGetStyle(
                                PROGRESSBAR as libc::c_int,
                                BORDER_WIDTH as libc::c_int,
                            ) as libc::c_float,
                        height: GuiGetStyle(
                            PROGRESSBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        ) as libc::c_float,
                    };
                    init
                },
                0 as libc::c_int,
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 0 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                GetColor(
                    GuiGetStyle(
                        PROGRESSBAR as libc::c_int,
                        BORDER_COLOR_FOCUSED as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
        } else {
            GuiDrawRectangle(
                {
                    let mut init = Rectangle {
                        x: bounds.x,
                        y: bounds.y,
                        width: GuiGetStyle(
                            PROGRESSBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        ) as libc::c_float,
                        height: bounds.height,
                    };
                    init
                },
                0 as libc::c_int,
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 0 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                GetColor(
                    GuiGetStyle(
                        PROGRESSBAR as libc::c_int,
                        BORDER_COLOR_NORMAL as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
        }
        if *value >= maxValue {
            GuiDrawRectangle(
                {
                    let mut init = Rectangle {
                        x: bounds.x + progress.width + 1 as libc::c_int as libc::c_float,
                        y: bounds.y,
                        width: GuiGetStyle(
                            PROGRESSBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        ) as libc::c_float,
                        height: bounds.height,
                    };
                    init
                },
                0 as libc::c_int,
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 0 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                GetColor(
                    GuiGetStyle(
                        PROGRESSBAR as libc::c_int,
                        BORDER_COLOR_FOCUSED as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
        } else {
            GuiDrawRectangle(
                {
                    let mut init = Rectangle {
                        x: bounds.x + progress.width as libc::c_int as libc::c_float
                            + 1 as libc::c_int as libc::c_float,
                        y: bounds.y,
                        width: bounds.width
                            - progress.width as libc::c_int as libc::c_float
                            - 1 as libc::c_int as libc::c_float,
                        height: GuiGetStyle(
                            PROGRESSBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        ) as libc::c_float,
                    };
                    init
                },
                0 as libc::c_int,
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 0 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                GetColor(
                    GuiGetStyle(
                        PROGRESSBAR as libc::c_int,
                        BORDER_COLOR_NORMAL as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
            GuiDrawRectangle(
                {
                    let mut init = Rectangle {
                        x: bounds.x + progress.width as libc::c_int as libc::c_float
                            + 1 as libc::c_int as libc::c_float,
                        y: bounds.y + bounds.height - 1 as libc::c_int as libc::c_float,
                        width: bounds.width
                            - progress.width as libc::c_int as libc::c_float
                            - 1 as libc::c_int as libc::c_float,
                        height: GuiGetStyle(
                            PROGRESSBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        ) as libc::c_float,
                    };
                    init
                },
                0 as libc::c_int,
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 0 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                GetColor(
                    GuiGetStyle(
                        PROGRESSBAR as libc::c_int,
                        BORDER_COLOR_NORMAL as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
            GuiDrawRectangle(
                {
                    let mut init = Rectangle {
                        x: bounds.x + bounds.width - 1 as libc::c_int as libc::c_float,
                        y: bounds.y + 1 as libc::c_int as libc::c_float,
                        width: GuiGetStyle(
                            PROGRESSBAR as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        ) as libc::c_float,
                        height: bounds.height - 2 as libc::c_int as libc::c_float,
                    };
                    init
                },
                0 as libc::c_int,
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 0 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                GetColor(
                    GuiGetStyle(
                        PROGRESSBAR as libc::c_int,
                        BORDER_COLOR_NORMAL as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
        }
        GuiDrawRectangle(
            progress,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            GetColor(
                GuiGetStyle(
                    PROGRESSBAR as libc::c_int,
                    BASE_COLOR_PRESSED as libc::c_int,
                ) as libc::c_uint,
            ),
        );
    }
    if !textLeft.is_null() {
        let mut textBounds: Rectangle = {
            let mut init = Rectangle {
                x: 0 as libc::c_int as libc::c_float,
                y: 0.,
                width: 0.,
                height: 0.,
            };
            init
        };
        textBounds.width = GetTextWidth(textLeft) as libc::c_float;
        textBounds
            .height = GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
            as libc::c_float;
        textBounds
            .x = bounds.x - textBounds.width
            - GuiGetStyle(PROGRESSBAR as libc::c_int, TEXT_PADDING as libc::c_int)
                as libc::c_float;
        textBounds
            .y = bounds.y + bounds.height / 2 as libc::c_int as libc::c_float
            - (GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                / 2 as libc::c_int) as libc::c_float;
        GuiDrawText(
            textLeft,
            textBounds,
            TEXT_ALIGN_RIGHT as libc::c_int,
            GetColor(
                GuiGetStyle(
                    PROGRESSBAR as libc::c_int,
                    (TEXT as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
    }
    if !textRight.is_null() {
        let mut textBounds_0: Rectangle = {
            let mut init = Rectangle {
                x: 0 as libc::c_int as libc::c_float,
                y: 0.,
                width: 0.,
                height: 0.,
            };
            init
        };
        textBounds_0.width = GetTextWidth(textRight) as libc::c_float;
        textBounds_0
            .height = GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
            as libc::c_float;
        textBounds_0
            .x = bounds.x + bounds.width
            + GuiGetStyle(PROGRESSBAR as libc::c_int, TEXT_PADDING as libc::c_int)
                as libc::c_float;
        textBounds_0
            .y = bounds.y + bounds.height / 2 as libc::c_int as libc::c_float
            - (GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                / 2 as libc::c_int) as libc::c_float;
        GuiDrawText(
            textRight,
            textBounds_0,
            TEXT_ALIGN_LEFT as libc::c_int,
            GetColor(
                GuiGetStyle(
                    PROGRESSBAR as libc::c_int,
                    (TEXT as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            (state as libc::c_uint)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                        ) as libc::c_int,
                ) as libc::c_uint,
            ),
        );
    }
    return result;
}
pub unsafe extern "C" fn GuiDummyRec(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked && !guiControlExclusiveMode
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if CheckCollisionPointRec(mousePoint, bounds) {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
            } else {
                state = STATE_FOCUSED;
            }
        }
    }
    GuiDrawRectangle(
        bounds,
        0 as libc::c_int,
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
        GetColor(
            GuiGetStyle(
                DEFAULT as libc::c_int,
                if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
                {
                    BASE_COLOR_NORMAL as libc::c_int
                } else {
                    BASE_COLOR_DISABLED as libc::c_int
                },
            ) as libc::c_uint,
        ),
    );
    GuiDrawText(
        text,
        GetTextBounds(DEFAULT as libc::c_int, bounds),
        TEXT_ALIGN_CENTER as libc::c_int,
        GetColor(
            GuiGetStyle(
                BUTTON as libc::c_int,
                if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
                {
                    TEXT_COLOR_NORMAL as libc::c_int
                } else {
                    TEXT_COLOR_DISABLED as libc::c_int
                },
            ) as libc::c_uint,
        ),
    );
    return result;
}
pub unsafe extern "C" fn GuiGrid(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut spacing: libc::c_float,
    mut subdivs: libc::c_int,
    mut mouseCell: *mut Vector2,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut mousePoint: Vector2 = GetMousePosition();
    let mut currentMouseCell: Vector2 = {
        let mut init = Vector2 {
            x: -(1 as libc::c_int) as libc::c_float,
            y: -(1 as libc::c_int) as libc::c_float,
        };
        init
    };
    let mut spaceWidth: libc::c_float = spacing / subdivs as libc::c_float;
    let mut linesV: libc::c_int = (bounds.width / spaceWidth) as libc::c_int
        + 1 as libc::c_int;
    let mut linesH: libc::c_int = (bounds.height / spaceWidth) as libc::c_int
        + 1 as libc::c_int;
    let mut color: libc::c_int = GuiGetStyle(
        DEFAULT as libc::c_int,
        LINE_COLOR as libc::c_int,
    );
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked && !guiControlExclusiveMode
    {
        if CheckCollisionPointRec(mousePoint, bounds) {
            currentMouseCell.x = floorf((mousePoint.x - bounds.x) / spacing);
            currentMouseCell.y = floorf((mousePoint.y - bounds.y) / spacing);
        }
    }
    if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint {
        color = GuiGetStyle(
            DEFAULT as libc::c_int,
            BORDER_COLOR_DISABLED as libc::c_int,
        );
    }
    if subdivs > 0 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < linesV {
            let mut lineV: Rectangle = {
                let mut init = Rectangle {
                    x: bounds.x
                        + spacing * i as libc::c_float / subdivs as libc::c_float,
                    y: bounds.y,
                    width: 1 as libc::c_int as libc::c_float,
                    height: bounds.height + 1 as libc::c_int as libc::c_float,
                };
                init
            };
            GuiDrawRectangle(
                lineV,
                0 as libc::c_int,
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 0 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                if i % subdivs == 0 as libc::c_int {
                    GuiFade(
                        GetColor(color as libc::c_uint),
                        0.15f32 * 4 as libc::c_int as libc::c_float,
                    )
                } else {
                    GuiFade(GetColor(color as libc::c_uint), 0.15f32)
                },
            );
            i += 1;
            i;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < linesH {
            let mut lineH: Rectangle = {
                let mut init = Rectangle {
                    x: bounds.x,
                    y: bounds.y
                        + spacing * i_0 as libc::c_float / subdivs as libc::c_float,
                    width: bounds.width + 1 as libc::c_int as libc::c_float,
                    height: 1 as libc::c_int as libc::c_float,
                };
                init
            };
            GuiDrawRectangle(
                lineH,
                0 as libc::c_int,
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 0 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                if i_0 % subdivs == 0 as libc::c_int {
                    GuiFade(
                        GetColor(color as libc::c_uint),
                        0.15f32 * 4 as libc::c_int as libc::c_float,
                    )
                } else {
                    GuiFade(GetColor(color as libc::c_uint), 0.15f32)
                },
            );
            i_0 += 1;
            i_0;
        }
    }
    if !mouseCell.is_null() {
        *mouseCell = currentMouseCell;
    }
    return result;
}
pub unsafe extern "C" fn GuiListView(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut scrollIndex: *mut libc::c_int,
    mut active: *mut libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut itemCount: libc::c_int = 0 as libc::c_int;
    let mut items: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if !text.is_null() {
        items = GuiTextSplit(
            text,
            ';' as i32 as libc::c_char,
            &mut itemCount,
            0 as *mut libc::c_int,
        );
    }
    result = GuiListViewEx(
        bounds,
        items,
        itemCount,
        scrollIndex,
        active,
        0 as *mut libc::c_int,
    );
    return result;
}
pub unsafe extern "C" fn GuiListViewEx(
    mut bounds: Rectangle,
    mut text: *mut *const libc::c_char,
    mut count: libc::c_int,
    mut scrollIndex: *mut libc::c_int,
    mut active: *mut libc::c_int,
    mut focus: *mut libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut itemFocused: libc::c_int = if focus.is_null() {
        -(1 as libc::c_int)
    } else {
        *focus
    };
    let mut itemSelected: libc::c_int = if active.is_null() {
        -(1 as libc::c_int)
    } else {
        *active
    };
    let mut useScrollBar: bool = 0 as libc::c_int != 0;
    if ((GuiGetStyle(LISTVIEW as libc::c_int, LIST_ITEMS_HEIGHT as libc::c_int)
        + GuiGetStyle(LISTVIEW as libc::c_int, LIST_ITEMS_SPACING as libc::c_int))
        * count) as libc::c_float > bounds.height
    {
        useScrollBar = 1 as libc::c_int != 0;
    }
    let mut itemBounds: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    itemBounds
        .x = bounds.x
        + GuiGetStyle(LISTVIEW as libc::c_int, LIST_ITEMS_SPACING as libc::c_int)
            as libc::c_float;
    itemBounds
        .y = bounds.y
        + GuiGetStyle(LISTVIEW as libc::c_int, LIST_ITEMS_SPACING as libc::c_int)
            as libc::c_float
        + GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
            as libc::c_float;
    itemBounds
        .width = bounds.width
        - (2 as libc::c_int
            * GuiGetStyle(LISTVIEW as libc::c_int, LIST_ITEMS_SPACING as libc::c_int))
            as libc::c_float
        - GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
            as libc::c_float;
    itemBounds
        .height = GuiGetStyle(LISTVIEW as libc::c_int, LIST_ITEMS_HEIGHT as libc::c_int)
        as libc::c_float;
    if useScrollBar {
        itemBounds.width
            -= GuiGetStyle(LISTVIEW as libc::c_int, SCROLLBAR_WIDTH as libc::c_int)
                as libc::c_float;
    }
    let mut visibleItems: libc::c_int = bounds.height as libc::c_int
        / (GuiGetStyle(LISTVIEW as libc::c_int, LIST_ITEMS_HEIGHT as libc::c_int)
            + GuiGetStyle(LISTVIEW as libc::c_int, LIST_ITEMS_SPACING as libc::c_int));
    if visibleItems > count {
        visibleItems = count;
    }
    let mut startIndex: libc::c_int = if scrollIndex.is_null() {
        0 as libc::c_int
    } else {
        *scrollIndex
    };
    if startIndex < 0 as libc::c_int || startIndex > count - visibleItems {
        startIndex = 0 as libc::c_int;
    }
    let mut endIndex: libc::c_int = startIndex + visibleItems;
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked && !guiControlExclusiveMode
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if CheckCollisionPointRec(mousePoint, bounds) {
            state = STATE_FOCUSED;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < visibleItems {
                if CheckCollisionPointRec(mousePoint, itemBounds) {
                    itemFocused = startIndex + i;
                    if IsMouseButtonPressed(MOUSE_BUTTON_LEFT as libc::c_int) {
                        if itemSelected == startIndex + i {
                            itemSelected = -(1 as libc::c_int);
                        } else {
                            itemSelected = startIndex + i;
                        }
                    }
                    break;
                } else {
                    itemBounds.y
                        += (GuiGetStyle(
                            LISTVIEW as libc::c_int,
                            LIST_ITEMS_HEIGHT as libc::c_int,
                        )
                            + GuiGetStyle(
                                LISTVIEW as libc::c_int,
                                LIST_ITEMS_SPACING as libc::c_int,
                            )) as libc::c_float;
                    i += 1;
                    i;
                }
            }
            if useScrollBar {
                let mut wheelMove: libc::c_int = GetMouseWheelMove() as libc::c_int;
                startIndex -= wheelMove;
                if startIndex < 0 as libc::c_int {
                    startIndex = 0 as libc::c_int;
                } else if startIndex > count - visibleItems {
                    startIndex = count - visibleItems;
                }
                endIndex = startIndex + visibleItems;
                if endIndex > count {
                    endIndex = count;
                }
            }
        } else {
            itemFocused = -(1 as libc::c_int);
        }
        itemBounds
            .y = bounds.y
            + GuiGetStyle(LISTVIEW as libc::c_int, LIST_ITEMS_SPACING as libc::c_int)
                as libc::c_float
            + GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int)
                as libc::c_float;
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                LISTVIEW as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        GetColor(
            GuiGetStyle(DEFAULT as libc::c_int, BACKGROUND_COLOR as libc::c_int)
                as libc::c_uint,
        ),
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < visibleItems && !text.is_null() {
        if state as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint {
            if startIndex + i_0 == itemSelected {
                GuiDrawRectangle(
                    itemBounds,
                    GuiGetStyle(LISTVIEW as libc::c_int, BORDER_WIDTH as libc::c_int),
                    GetColor(
                        GuiGetStyle(
                            LISTVIEW as libc::c_int,
                            BORDER_COLOR_DISABLED as libc::c_int,
                        ) as libc::c_uint,
                    ),
                    GetColor(
                        GuiGetStyle(
                            LISTVIEW as libc::c_int,
                            BASE_COLOR_DISABLED as libc::c_int,
                        ) as libc::c_uint,
                    ),
                );
            }
            GuiDrawText(
                *text.offset((startIndex + i_0) as isize),
                GetTextBounds(DEFAULT as libc::c_int, itemBounds),
                GuiGetStyle(LISTVIEW as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
                GetColor(
                    GuiGetStyle(
                        LISTVIEW as libc::c_int,
                        TEXT_COLOR_DISABLED as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
        } else if startIndex + i_0 == itemSelected && !active.is_null() {
            GuiDrawRectangle(
                itemBounds,
                GuiGetStyle(LISTVIEW as libc::c_int, BORDER_WIDTH as libc::c_int),
                GetColor(
                    GuiGetStyle(
                        LISTVIEW as libc::c_int,
                        BORDER_COLOR_PRESSED as libc::c_int,
                    ) as libc::c_uint,
                ),
                GetColor(
                    GuiGetStyle(
                        LISTVIEW as libc::c_int,
                        BASE_COLOR_PRESSED as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
            GuiDrawText(
                *text.offset((startIndex + i_0) as isize),
                GetTextBounds(DEFAULT as libc::c_int, itemBounds),
                GuiGetStyle(LISTVIEW as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
                GetColor(
                    GuiGetStyle(
                        LISTVIEW as libc::c_int,
                        TEXT_COLOR_PRESSED as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
        } else if startIndex + i_0 == itemFocused {
            GuiDrawRectangle(
                itemBounds,
                GuiGetStyle(LISTVIEW as libc::c_int, BORDER_WIDTH as libc::c_int),
                GetColor(
                    GuiGetStyle(
                        LISTVIEW as libc::c_int,
                        BORDER_COLOR_FOCUSED as libc::c_int,
                    ) as libc::c_uint,
                ),
                GetColor(
                    GuiGetStyle(
                        LISTVIEW as libc::c_int,
                        BASE_COLOR_FOCUSED as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
            GuiDrawText(
                *text.offset((startIndex + i_0) as isize),
                GetTextBounds(DEFAULT as libc::c_int, itemBounds),
                GuiGetStyle(LISTVIEW as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
                GetColor(
                    GuiGetStyle(
                        LISTVIEW as libc::c_int,
                        TEXT_COLOR_FOCUSED as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
        } else {
            GuiDrawText(
                *text.offset((startIndex + i_0) as isize),
                GetTextBounds(DEFAULT as libc::c_int, itemBounds),
                GuiGetStyle(LISTVIEW as libc::c_int, TEXT_ALIGNMENT as libc::c_int),
                GetColor(
                    GuiGetStyle(
                        LISTVIEW as libc::c_int,
                        TEXT_COLOR_NORMAL as libc::c_int,
                    ) as libc::c_uint,
                ),
            );
        }
        itemBounds.y
            += (GuiGetStyle(LISTVIEW as libc::c_int, LIST_ITEMS_HEIGHT as libc::c_int)
                + GuiGetStyle(
                    LISTVIEW as libc::c_int,
                    LIST_ITEMS_SPACING as libc::c_int,
                )) as libc::c_float;
        i_0 += 1;
        i_0;
    }
    if useScrollBar {
        let mut scrollBarBounds: Rectangle = {
            let mut init = Rectangle {
                x: bounds.x + bounds.width
                    - GuiGetStyle(LISTVIEW as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float
                    - GuiGetStyle(
                        LISTVIEW as libc::c_int,
                        SCROLLBAR_WIDTH as libc::c_int,
                    ) as libc::c_float,
                y: bounds.y
                    + GuiGetStyle(LISTVIEW as libc::c_int, BORDER_WIDTH as libc::c_int)
                        as libc::c_float,
                width: GuiGetStyle(
                    LISTVIEW as libc::c_int,
                    SCROLLBAR_WIDTH as libc::c_int,
                ) as libc::c_float,
                height: bounds.height
                    - (2 as libc::c_int
                        * GuiGetStyle(
                            DEFAULT as libc::c_int,
                            BORDER_WIDTH as libc::c_int,
                        )) as libc::c_float,
            };
            init
        };
        let mut percentVisible: libc::c_float = (endIndex - startIndex) as libc::c_float
            / count as libc::c_float;
        let mut sliderSize: libc::c_float = bounds.height * percentVisible;
        let mut prevSliderSize: libc::c_int = GuiGetStyle(
            SCROLLBAR as libc::c_int,
            SCROLL_SLIDER_SIZE as libc::c_int,
        );
        let mut prevScrollSpeed: libc::c_int = GuiGetStyle(
            SCROLLBAR as libc::c_int,
            SCROLL_SPEED as libc::c_int,
        );
        GuiSetStyle(
            SCROLLBAR as libc::c_int,
            SCROLL_SLIDER_SIZE as libc::c_int,
            sliderSize as libc::c_int,
        );
        GuiSetStyle(
            SCROLLBAR as libc::c_int,
            SCROLL_SPEED as libc::c_int,
            count - visibleItems,
        );
        startIndex = GuiScrollBar(
            scrollBarBounds,
            startIndex,
            0 as libc::c_int,
            count - visibleItems,
        );
        GuiSetStyle(
            SCROLLBAR as libc::c_int,
            SCROLL_SPEED as libc::c_int,
            prevScrollSpeed,
        );
        GuiSetStyle(
            SCROLLBAR as libc::c_int,
            SCROLL_SLIDER_SIZE as libc::c_int,
            prevSliderSize,
        );
    }
    if !active.is_null() {
        *active = itemSelected;
    }
    if !focus.is_null() {
        *focus = itemFocused;
    }
    if !scrollIndex.is_null() {
        *scrollIndex = startIndex;
    }
    return result;
}
pub unsafe extern "C" fn GuiMessageBox(
    mut bounds: Rectangle,
    mut title: *const libc::c_char,
    mut message: *const libc::c_char,
    mut buttons: *const libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = -(1 as libc::c_int);
    let mut buttonCount: libc::c_int = 0 as libc::c_int;
    let mut buttonsText: *mut *const libc::c_char = GuiTextSplit(
        buttons,
        ';' as i32 as libc::c_char,
        &mut buttonCount,
        0 as *mut libc::c_int,
    );
    let mut buttonBounds: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    buttonBounds.x = bounds.x + 12 as libc::c_int as libc::c_float;
    buttonBounds
        .y = bounds.y + bounds.height - 24 as libc::c_int as libc::c_float
        - 12 as libc::c_int as libc::c_float;
    buttonBounds
        .width = (bounds.width
        - (12 as libc::c_int * (buttonCount + 1 as libc::c_int)) as libc::c_float)
        / buttonCount as libc::c_float;
    buttonBounds.height = 24 as libc::c_int as libc::c_float;
    let mut textWidth: libc::c_int = GetTextWidth(message) + 2 as libc::c_int;
    let mut textBounds: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    textBounds.x = bounds.x + 12 as libc::c_int as libc::c_float;
    textBounds
        .y = bounds.y + 24 as libc::c_int as libc::c_float
        + 12 as libc::c_int as libc::c_float;
    textBounds
        .width = bounds.width - (12 as libc::c_int * 2 as libc::c_int) as libc::c_float;
    textBounds
        .height = bounds.height - 24 as libc::c_int as libc::c_float
        - (3 as libc::c_int * 12 as libc::c_int) as libc::c_float
        - 24 as libc::c_int as libc::c_float;
    if GuiWindowBox(bounds, title) != 0 {
        result = 0 as libc::c_int;
    }
    let mut prevTextAlignment: libc::c_int = GuiGetStyle(
        LABEL as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
    );
    GuiSetStyle(
        LABEL as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_CENTER as libc::c_int,
    );
    GuiLabel(textBounds, message);
    GuiSetStyle(LABEL as libc::c_int, TEXT_ALIGNMENT as libc::c_int, prevTextAlignment);
    prevTextAlignment = GuiGetStyle(
        BUTTON as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
    );
    GuiSetStyle(
        BUTTON as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_CENTER as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < buttonCount {
        if GuiButton(buttonBounds, *buttonsText.offset(i as isize)) != 0 {
            result = i + 1 as libc::c_int;
        }
        buttonBounds.x += buttonBounds.width + 12 as libc::c_int as libc::c_float;
        i += 1;
        i;
    }
    GuiSetStyle(BUTTON as libc::c_int, TEXT_ALIGNMENT as libc::c_int, prevTextAlignment);
    return result;
}
pub unsafe extern "C" fn GuiTextInputBox(
    mut bounds: Rectangle,
    mut title: *const libc::c_char,
    mut message: *const libc::c_char,
    mut buttons: *const libc::c_char,
    mut text: *mut libc::c_char,
    mut textMaxSize: libc::c_int,
    mut secretViewActive: *mut bool,
) -> libc::c_int {
    static mut textEditMode: bool = 0 as libc::c_int != 0;
    let mut result: libc::c_int = -(1 as libc::c_int);
    let mut buttonCount: libc::c_int = 0 as libc::c_int;
    let mut buttonsText: *mut *const libc::c_char = GuiTextSplit(
        buttons,
        ';' as i32 as libc::c_char,
        &mut buttonCount,
        0 as *mut libc::c_int,
    );
    let mut buttonBounds: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    buttonBounds.x = bounds.x + 12 as libc::c_int as libc::c_float;
    buttonBounds
        .y = bounds.y + bounds.height - 24 as libc::c_int as libc::c_float
        - 12 as libc::c_int as libc::c_float;
    buttonBounds
        .width = (bounds.width
        - (12 as libc::c_int * (buttonCount + 1 as libc::c_int)) as libc::c_float)
        / buttonCount as libc::c_float;
    buttonBounds.height = 24 as libc::c_int as libc::c_float;
    let mut messageInputHeight: libc::c_int = bounds.height as libc::c_int
        - 24 as libc::c_int
        - GuiGetStyle(STATUSBAR as libc::c_int, BORDER_WIDTH as libc::c_int)
        - 24 as libc::c_int - 2 as libc::c_int * 12 as libc::c_int;
    let mut textBounds: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    if !message.is_null() {
        let mut textSize: libc::c_int = GetTextWidth(message) + 2 as libc::c_int;
        textBounds
            .x = bounds.x + bounds.width / 2 as libc::c_int as libc::c_float
            - (textSize / 2 as libc::c_int) as libc::c_float;
        textBounds
            .y = bounds.y + 24 as libc::c_int as libc::c_float
            + (messageInputHeight / 4 as libc::c_int) as libc::c_float
            - GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
                as libc::c_float / 2 as libc::c_int as libc::c_float;
        textBounds.width = textSize as libc::c_float;
        textBounds
            .height = GuiGetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int)
            as libc::c_float;
    }
    let mut textBoxBounds: Rectangle = {
        let mut init = Rectangle {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        init
    };
    textBoxBounds.x = bounds.x + 12 as libc::c_int as libc::c_float;
    textBoxBounds
        .y = bounds.y + 24 as libc::c_int as libc::c_float
        - (26 as libc::c_int / 2 as libc::c_int) as libc::c_float;
    if message.is_null() {
        textBoxBounds
            .y = bounds.y + 24 as libc::c_int as libc::c_float
            + 12 as libc::c_int as libc::c_float;
    } else {
        textBoxBounds.y
            += (messageInputHeight / 2 as libc::c_int
                + messageInputHeight / 4 as libc::c_int) as libc::c_float;
    }
    textBoxBounds
        .width = bounds.width - (12 as libc::c_int * 2 as libc::c_int) as libc::c_float;
    textBoxBounds.height = 26 as libc::c_int as libc::c_float;
    if GuiWindowBox(bounds, title) != 0 {
        result = 0 as libc::c_int;
    }
    if !message.is_null() {
        let mut prevTextAlignment: libc::c_int = GuiGetStyle(
            LABEL as libc::c_int,
            TEXT_ALIGNMENT as libc::c_int,
        );
        GuiSetStyle(
            LABEL as libc::c_int,
            TEXT_ALIGNMENT as libc::c_int,
            TEXT_ALIGN_CENTER as libc::c_int,
        );
        GuiLabel(textBounds, message);
        GuiSetStyle(
            LABEL as libc::c_int,
            TEXT_ALIGNMENT as libc::c_int,
            prevTextAlignment,
        );
    }
    if !secretViewActive.is_null() {
        static mut stars: [libc::c_char; 17] = unsafe {
            *::std::mem::transmute::<
                &[u8; 17],
                &mut [libc::c_char; 17],
            >(b"****************\0")
        };
        if GuiTextBox(
            {
                let mut init = Rectangle {
                    x: textBoxBounds.x,
                    y: textBoxBounds.y,
                    width: textBoxBounds.width - 4 as libc::c_int as libc::c_float
                        - 26 as libc::c_int as libc::c_float,
                    height: textBoxBounds.height,
                };
                init
            },
            if *secretViewActive as libc::c_int == 1 as libc::c_int
                || textEditMode as libc::c_int != 0
            {
                text
            } else {
                stars.as_mut_ptr()
            },
            textMaxSize,
            textEditMode,
        ) != 0
        {
            textEditMode = !textEditMode;
        }
        GuiToggle(
            {
                let mut init = Rectangle {
                    x: textBoxBounds.x + textBoxBounds.width
                        - 26 as libc::c_int as libc::c_float,
                    y: textBoxBounds.y,
                    width: 26 as libc::c_int as libc::c_float,
                    height: 26 as libc::c_int as libc::c_float,
                };
                init
            },
            if *secretViewActive as libc::c_int == 1 as libc::c_int {
                b"#44#\0" as *const u8 as *const libc::c_char
            } else {
                b"#45#\0" as *const u8 as *const libc::c_char
            },
            secretViewActive,
        );
    } else if GuiTextBox(textBoxBounds, text, textMaxSize, textEditMode) != 0 {
        textEditMode = !textEditMode;
    }
    let mut prevBtnTextAlignment: libc::c_int = GuiGetStyle(
        BUTTON as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
    );
    GuiSetStyle(
        BUTTON as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_CENTER as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < buttonCount {
        if GuiButton(buttonBounds, *buttonsText.offset(i as isize)) != 0 {
            result = i + 1 as libc::c_int;
        }
        buttonBounds.x += buttonBounds.width + 12 as libc::c_int as libc::c_float;
        i += 1;
        i;
    }
    if result >= 0 as libc::c_int {
        textEditMode = 0 as libc::c_int != 0;
    }
    GuiSetStyle(
        BUTTON as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        prevBtnTextAlignment,
    );
    return result;
}
pub unsafe extern "C" fn GuiColorPicker(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut color: *mut Color,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut temp: Color = {
        let mut init = Color {
            r: 200 as libc::c_int as libc::c_uchar,
            g: 0 as libc::c_int as libc::c_uchar,
            b: 0 as libc::c_int as libc::c_uchar,
            a: 255 as libc::c_int as libc::c_uchar,
        };
        init
    };
    if color.is_null() {
        color = &mut temp;
    }
    GuiColorPanel(bounds, 0 as *const libc::c_char, color);
    let mut boundsHue: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x + bounds.width
                + GuiGetStyle(COLORPICKER as libc::c_int, HUEBAR_PADDING as libc::c_int)
                    as libc::c_float,
            y: bounds.y,
            width: GuiGetStyle(COLORPICKER as libc::c_int, HUEBAR_WIDTH as libc::c_int)
                as libc::c_float,
            height: bounds.height,
        };
        init
    };
    let mut hsv: Vector3 = ConvertRGBtoHSV({
        let mut init = Vector3 {
            x: (*color).r as libc::c_int as libc::c_float / 255.0f32,
            y: (*color).g as libc::c_int as libc::c_float / 255.0f32,
            z: (*color).b as libc::c_int as libc::c_float / 255.0f32,
        };
        init
    });
    GuiColorBarHue(boundsHue, 0 as *const libc::c_char, &mut hsv.x);
    let mut rgb: Vector3 = ConvertHSVtoRGB(hsv);
    *color = {
        let mut init = Color {
            r: roundf(rgb.x * 255.0f32) as libc::c_uchar,
            g: roundf(rgb.y * 255.0f32) as libc::c_uchar,
            b: roundf(rgb.z * 255.0f32) as libc::c_uchar,
            a: (*color).a,
        };
        init
    };
    return result;
}
unsafe extern "C" fn ConvertRGBtoHSV(mut rgb: Vector3) -> Vector3 {
    let mut hsv: Vector3 = {
        let mut init = Vector3 {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            z: 0.,
        };
        init
    };
    let mut min: libc::c_float = 0.0f32;
    let mut max: libc::c_float = 0.0f32;
    let mut delta: libc::c_float = 0.0f32;
    min = if rgb.x < rgb.y { rgb.x } else { rgb.y };
    min = if min < rgb.z { min } else { rgb.z };
    max = if rgb.x > rgb.y { rgb.x } else { rgb.y };
    max = if max > rgb.z { max } else { rgb.z };
    hsv.z = max;
    delta = max - min;
    if delta < 0.00001f32 {
        hsv.y = 0.0f32;
        hsv.x = 0.0f32;
        return hsv;
    }
    if max > 0.0f32 {
        hsv.y = delta / max;
    } else {
        hsv.y = 0.0f32;
        hsv.x = 0.0f32;
        return hsv;
    }
    if rgb.x >= max {
        hsv.x = (rgb.y - rgb.z) / delta;
    } else if rgb.y >= max {
        hsv.x = 2.0f32 + (rgb.z - rgb.x) / delta;
    } else {
        hsv.x = 4.0f32 + (rgb.x - rgb.y) / delta;
    }
    hsv.x *= 60.0f32;
    if hsv.x < 0.0f32 {
        hsv.x += 360.0f32;
    }
    return hsv;
}
unsafe extern "C" fn ConvertHSVtoRGB(mut hsv: Vector3) -> Vector3 {
    let mut rgb: Vector3 = {
        let mut init = Vector3 {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            z: 0.,
        };
        init
    };
    let mut hh: libc::c_float = 0.0f32;
    let mut p: libc::c_float = 0.0f32;
    let mut q: libc::c_float = 0.0f32;
    let mut t: libc::c_float = 0.0f32;
    let mut ff: libc::c_float = 0.0f32;
    let mut i: libc::c_long = 0 as libc::c_int as libc::c_long;
    if hsv.y <= 0.0f32 {
        rgb.x = hsv.z;
        rgb.y = hsv.z;
        rgb.z = hsv.z;
        return rgb;
    }
    hh = hsv.x;
    if hh >= 360.0f32 {
        hh = 0.0f32;
    }
    hh /= 60.0f32;
    i = hh as libc::c_long;
    ff = hh - i as libc::c_float;
    p = hsv.z * (1.0f32 - hsv.y);
    q = hsv.z * (1.0f32 - hsv.y * ff);
    t = hsv.z * (1.0f32 - hsv.y * (1.0f32 - ff));
    match i {
        0 => {
            rgb.x = hsv.z;
            rgb.y = t;
            rgb.z = p;
        }
        1 => {
            rgb.x = q;
            rgb.y = hsv.z;
            rgb.z = p;
        }
        2 => {
            rgb.x = p;
            rgb.y = hsv.z;
            rgb.z = t;
        }
        3 => {
            rgb.x = p;
            rgb.y = q;
            rgb.z = hsv.z;
        }
        4 => {
            rgb.x = t;
            rgb.y = p;
            rgb.z = hsv.z;
        }
        5 | _ => {
            rgb.x = hsv.z;
            rgb.y = p;
            rgb.z = q;
        }
    }
    return rgb;
}
pub unsafe extern "C" fn GuiColorBarHue(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut hue: *mut libc::c_float,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut selector: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x
                - GuiGetStyle(
                    COLORPICKER as libc::c_int,
                    HUEBAR_SELECTOR_OVERFLOW as libc::c_int,
                ) as libc::c_float,
            y: bounds.y + *hue / 360.0f32 * bounds.height
                - (GuiGetStyle(
                    COLORPICKER as libc::c_int,
                    HUEBAR_SELECTOR_HEIGHT as libc::c_int,
                ) / 2 as libc::c_int) as libc::c_float,
            width: bounds.width
                + (GuiGetStyle(
                    COLORPICKER as libc::c_int,
                    HUEBAR_SELECTOR_OVERFLOW as libc::c_int,
                ) * 2 as libc::c_int) as libc::c_float,
            height: GuiGetStyle(
                COLORPICKER as libc::c_int,
                HUEBAR_SELECTOR_HEIGHT as libc::c_int,
            ) as libc::c_float,
        };
        init
    };
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if guiControlExclusiveMode {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                if bounds.x == guiControlExclusiveRec.x
                    && bounds.y == guiControlExclusiveRec.y
                    && bounds.width == guiControlExclusiveRec.width
                    && bounds.height == guiControlExclusiveRec.height
                {
                    state = STATE_PRESSED;
                    *hue = (mousePoint.y - bounds.y)
                        * 360 as libc::c_int as libc::c_float / bounds.height;
                    if *hue <= 0.0f32 {
                        *hue = 0.0f32;
                    }
                    if *hue >= 359.0f32 {
                        *hue = 359.0f32;
                    }
                }
            } else {
                guiControlExclusiveMode = 0 as libc::c_int != 0;
                guiControlExclusiveRec = {
                    let mut init = Rectangle {
                        x: 0 as libc::c_int as libc::c_float,
                        y: 0 as libc::c_int as libc::c_float,
                        width: 0 as libc::c_int as libc::c_float,
                        height: 0 as libc::c_int as libc::c_float,
                    };
                    init
                };
            }
        } else if CheckCollisionPointRec(mousePoint, bounds) as libc::c_int != 0
            || CheckCollisionPointRec(mousePoint, selector) as libc::c_int != 0
        {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
                guiControlExclusiveMode = 1 as libc::c_int != 0;
                guiControlExclusiveRec = bounds;
                *hue = (mousePoint.y - bounds.y) * 360 as libc::c_int as libc::c_float
                    / bounds.height;
                if *hue <= 0.0f32 {
                    *hue = 0.0f32;
                }
                if *hue >= 359.0f32 {
                    *hue = 359.0f32;
                }
            } else {
                state = STATE_FOCUSED;
            }
        }
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint {
        DrawRectangleGradientV(
            bounds.x as libc::c_int,
            bounds.y as libc::c_int,
            bounds.width as libc::c_int,
            ceilf(bounds.height / 6 as libc::c_int as libc::c_float) as libc::c_int,
            Fade(
                {
                    let mut init = Color {
                        r: 255 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
            Fade(
                {
                    let mut init = Color {
                        r: 255 as libc::c_int as libc::c_uchar,
                        g: 255 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
        );
        DrawRectangleGradientV(
            bounds.x as libc::c_int,
            (bounds.y + bounds.height / 6 as libc::c_int as libc::c_float)
                as libc::c_int,
            bounds.width as libc::c_int,
            ceilf(bounds.height / 6 as libc::c_int as libc::c_float) as libc::c_int,
            Fade(
                {
                    let mut init = Color {
                        r: 255 as libc::c_int as libc::c_uchar,
                        g: 255 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
            Fade(
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 255 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
        );
        DrawRectangleGradientV(
            bounds.x as libc::c_int,
            (bounds.y
                + 2 as libc::c_int as libc::c_float
                    * (bounds.height / 6 as libc::c_int as libc::c_float))
                as libc::c_int,
            bounds.width as libc::c_int,
            ceilf(bounds.height / 6 as libc::c_int as libc::c_float) as libc::c_int,
            Fade(
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 255 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
            Fade(
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 255 as libc::c_int as libc::c_uchar,
                        b: 255 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
        );
        DrawRectangleGradientV(
            bounds.x as libc::c_int,
            (bounds.y
                + 3 as libc::c_int as libc::c_float
                    * (bounds.height / 6 as libc::c_int as libc::c_float))
                as libc::c_int,
            bounds.width as libc::c_int,
            ceilf(bounds.height / 6 as libc::c_int as libc::c_float) as libc::c_int,
            Fade(
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 255 as libc::c_int as libc::c_uchar,
                        b: 255 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
            Fade(
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 255 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
        );
        DrawRectangleGradientV(
            bounds.x as libc::c_int,
            (bounds.y
                + 4 as libc::c_int as libc::c_float
                    * (bounds.height / 6 as libc::c_int as libc::c_float))
                as libc::c_int,
            bounds.width as libc::c_int,
            ceilf(bounds.height / 6 as libc::c_int as libc::c_float) as libc::c_int,
            Fade(
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 255 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
            Fade(
                {
                    let mut init = Color {
                        r: 255 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 255 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
        );
        DrawRectangleGradientV(
            bounds.x as libc::c_int,
            (bounds.y
                + 5 as libc::c_int as libc::c_float
                    * (bounds.height / 6 as libc::c_int as libc::c_float))
                as libc::c_int,
            bounds.width as libc::c_int,
            (bounds.height / 6 as libc::c_int as libc::c_float) as libc::c_int,
            Fade(
                {
                    let mut init = Color {
                        r: 255 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 255 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
            Fade(
                {
                    let mut init = Color {
                        r: 255 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
        );
    } else {
        DrawRectangleGradientV(
            bounds.x as libc::c_int,
            bounds.y as libc::c_int,
            bounds.width as libc::c_int,
            bounds.height as libc::c_int,
            Fade(
                Fade(
                    GetColor(
                        GuiGetStyle(
                            COLORPICKER as libc::c_int,
                            BASE_COLOR_DISABLED as libc::c_int,
                        ) as libc::c_uint,
                    ),
                    0.1f32,
                ),
                guiAlpha,
            ),
            Fade(
                GetColor(
                    GuiGetStyle(
                        COLORPICKER as libc::c_int,
                        BORDER_COLOR_DISABLED as libc::c_int,
                    ) as libc::c_uint,
                ),
                guiAlpha,
            ),
        );
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(COLORPICKER as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                COLORPICKER as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
    );
    GuiDrawRectangle(
        selector,
        0 as libc::c_int,
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
        GetColor(
            GuiGetStyle(
                COLORPICKER as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    return result;
}
pub unsafe extern "C" fn GuiColorPanel(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut color: *mut Color,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut vcolor: Vector3 = {
        let mut init = Vector3 {
            x: (*color).r as libc::c_float / 255.0f32,
            y: (*color).g as libc::c_float / 255.0f32,
            z: (*color).b as libc::c_float / 255.0f32,
        };
        init
    };
    let mut hsv: Vector3 = ConvertRGBtoHSV(vcolor);
    let mut prevHsv: Vector3 = hsv;
    GuiColorPanelHSV(bounds, text, &mut hsv);
    if hsv.x != prevHsv.x || hsv.y != prevHsv.y || hsv.z != prevHsv.z {
        let mut rgb: Vector3 = ConvertHSVtoRGB(hsv);
        *color = {
            let mut init = Color {
                r: (255.0f32 * rgb.x) as libc::c_uchar,
                g: (255.0f32 * rgb.y) as libc::c_uchar,
                b: (255.0f32 * rgb.z) as libc::c_uchar,
                a: (*color).a,
            };
            init
        };
    }
    return result;
}
pub unsafe extern "C" fn GuiColorPanelHSV(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut colorHsv: *mut Vector3,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut pickerSelector: Vector2 = {
        let mut init = Vector2 {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
        };
        init
    };
    let colWhite: Color = {
        let mut init = Color {
            r: 255 as libc::c_int as libc::c_uchar,
            g: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
            a: 255 as libc::c_int as libc::c_uchar,
        };
        init
    };
    let colBlack: Color = {
        let mut init = Color {
            r: 0 as libc::c_int as libc::c_uchar,
            g: 0 as libc::c_int as libc::c_uchar,
            b: 0 as libc::c_int as libc::c_uchar,
            a: 255 as libc::c_int as libc::c_uchar,
        };
        init
    };
    pickerSelector.x = bounds.x + (*colorHsv).y * bounds.width;
    pickerSelector.y = bounds.y + (1.0f32 - (*colorHsv).z) * bounds.height;
    let mut maxHue: Vector3 = {
        let mut init = Vector3 {
            x: (*colorHsv).x,
            y: 1.0f32,
            z: 1.0f32,
        };
        init
    };
    let mut rgbHue: Vector3 = ConvertHSVtoRGB(maxHue);
    let mut maxHueCol: Color = {
        let mut init = Color {
            r: (255.0f32 * rgbHue.x) as libc::c_uchar,
            g: (255.0f32 * rgbHue.y) as libc::c_uchar,
            b: (255.0f32 * rgbHue.z) as libc::c_uchar,
            a: 255 as libc::c_int as libc::c_uchar,
        };
        init
    };
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if guiControlExclusiveMode {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                if bounds.x == guiControlExclusiveRec.x
                    && bounds.y == guiControlExclusiveRec.y
                    && bounds.width == guiControlExclusiveRec.width
                    && bounds.height == guiControlExclusiveRec.height
                {
                    pickerSelector = mousePoint;
                    if pickerSelector.x < bounds.x {
                        pickerSelector.x = bounds.x;
                    }
                    if pickerSelector.x > bounds.x + bounds.width {
                        pickerSelector.x = bounds.x + bounds.width;
                    }
                    if pickerSelector.y < bounds.y {
                        pickerSelector.y = bounds.y;
                    }
                    if pickerSelector.y > bounds.y + bounds.height {
                        pickerSelector.y = bounds.y + bounds.height;
                    }
                    let mut colorPick: Vector2 = {
                        let mut init = Vector2 {
                            x: pickerSelector.x - bounds.x,
                            y: pickerSelector.y - bounds.y,
                        };
                        init
                    };
                    colorPick.x /= bounds.width;
                    colorPick.y /= bounds.height;
                    (*colorHsv).y = colorPick.x;
                    (*colorHsv).z = 1.0f32 - colorPick.y;
                }
            } else {
                guiControlExclusiveMode = 0 as libc::c_int != 0;
                guiControlExclusiveRec = {
                    let mut init = Rectangle {
                        x: 0 as libc::c_int as libc::c_float,
                        y: 0 as libc::c_int as libc::c_float,
                        width: 0 as libc::c_int as libc::c_float,
                        height: 0 as libc::c_int as libc::c_float,
                    };
                    init
                };
            }
        } else if CheckCollisionPointRec(mousePoint, bounds) {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
                guiControlExclusiveMode = 1 as libc::c_int != 0;
                guiControlExclusiveRec = bounds;
                pickerSelector = mousePoint;
                let mut colorPick_0: Vector2 = {
                    let mut init = Vector2 {
                        x: pickerSelector.x - bounds.x,
                        y: pickerSelector.y - bounds.y,
                    };
                    init
                };
                colorPick_0.x /= bounds.width;
                colorPick_0.y /= bounds.height;
                (*colorHsv).y = colorPick_0.x;
                (*colorHsv).z = 1.0f32 - colorPick_0.y;
            } else {
                state = STATE_FOCUSED;
            }
        }
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint {
        DrawRectangleGradientEx(
            bounds,
            Fade(colWhite, guiAlpha),
            Fade(colWhite, guiAlpha),
            Fade(maxHueCol, guiAlpha),
            Fade(maxHueCol, guiAlpha),
        );
        DrawRectangleGradientEx(
            bounds,
            Fade(colBlack, 0 as libc::c_int as libc::c_float),
            Fade(colBlack, guiAlpha),
            Fade(colBlack, guiAlpha),
            Fade(colBlack, 0 as libc::c_int as libc::c_float),
        );
        let mut selector: Rectangle = {
            let mut init = Rectangle {
                x: pickerSelector.x
                    - (GuiGetStyle(
                        COLORPICKER as libc::c_int,
                        COLOR_SELECTOR_SIZE as libc::c_int,
                    ) / 2 as libc::c_int) as libc::c_float,
                y: pickerSelector.y
                    - (GuiGetStyle(
                        COLORPICKER as libc::c_int,
                        COLOR_SELECTOR_SIZE as libc::c_int,
                    ) / 2 as libc::c_int) as libc::c_float,
                width: GuiGetStyle(
                    COLORPICKER as libc::c_int,
                    COLOR_SELECTOR_SIZE as libc::c_int,
                ) as libc::c_float,
                height: GuiGetStyle(
                    COLORPICKER as libc::c_int,
                    COLOR_SELECTOR_SIZE as libc::c_int,
                ) as libc::c_float,
            };
            init
        };
        GuiDrawRectangle(
            selector,
            0 as libc::c_int,
            {
                let mut init = Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            colWhite,
        );
    } else {
        DrawRectangleGradientEx(
            bounds,
            Fade(
                Fade(
                    GetColor(
                        GuiGetStyle(
                            COLORPICKER as libc::c_int,
                            BASE_COLOR_DISABLED as libc::c_int,
                        ) as libc::c_uint,
                    ),
                    0.1f32,
                ),
                guiAlpha,
            ),
            Fade(Fade(colBlack, 0.6f32), guiAlpha),
            Fade(Fade(colBlack, 0.6f32), guiAlpha),
            Fade(
                Fade(
                    GetColor(
                        GuiGetStyle(
                            COLORPICKER as libc::c_int,
                            BORDER_COLOR_DISABLED as libc::c_int,
                        ) as libc::c_uint,
                    ),
                    0.6f32,
                ),
                guiAlpha,
            ),
        );
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(COLORPICKER as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                COLORPICKER as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
    );
    return result;
}
pub unsafe extern "C" fn GuiColorBarAlpha(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut alpha: *mut libc::c_float,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut state: GuiState = guiState;
    let mut selector: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x + *alpha * bounds.width
                - (GuiGetStyle(
                    COLORPICKER as libc::c_int,
                    HUEBAR_SELECTOR_HEIGHT as libc::c_int,
                ) / 2 as libc::c_int) as libc::c_float,
            y: bounds.y
                - GuiGetStyle(
                    COLORPICKER as libc::c_int,
                    HUEBAR_SELECTOR_OVERFLOW as libc::c_int,
                ) as libc::c_float,
            width: GuiGetStyle(
                COLORPICKER as libc::c_int,
                HUEBAR_SELECTOR_HEIGHT as libc::c_int,
            ) as libc::c_float,
            height: bounds.height
                + (GuiGetStyle(
                    COLORPICKER as libc::c_int,
                    HUEBAR_SELECTOR_OVERFLOW as libc::c_int,
                ) * 2 as libc::c_int) as libc::c_float,
        };
        init
    };
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint
        && !guiLocked
    {
        let mut mousePoint: Vector2 = GetMousePosition();
        if guiControlExclusiveMode {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                if bounds.x == guiControlExclusiveRec.x
                    && bounds.y == guiControlExclusiveRec.y
                    && bounds.width == guiControlExclusiveRec.width
                    && bounds.height == guiControlExclusiveRec.height
                {
                    state = STATE_PRESSED;
                    *alpha = (mousePoint.x - bounds.x) / bounds.width;
                    if *alpha <= 0.0f32 {
                        *alpha = 0.0f32;
                    }
                    if *alpha >= 1.0f32 {
                        *alpha = 1.0f32;
                    }
                }
            } else {
                guiControlExclusiveMode = 0 as libc::c_int != 0;
                guiControlExclusiveRec = {
                    let mut init = Rectangle {
                        x: 0 as libc::c_int as libc::c_float,
                        y: 0 as libc::c_int as libc::c_float,
                        width: 0 as libc::c_int as libc::c_float,
                        height: 0 as libc::c_int as libc::c_float,
                    };
                    init
                };
            }
        } else if CheckCollisionPointRec(mousePoint, bounds) as libc::c_int != 0
            || CheckCollisionPointRec(mousePoint, selector) as libc::c_int != 0
        {
            if IsMouseButtonDown(MOUSE_BUTTON_LEFT as libc::c_int) {
                state = STATE_PRESSED;
                guiControlExclusiveMode = 1 as libc::c_int != 0;
                guiControlExclusiveRec = bounds;
                *alpha = (mousePoint.x - bounds.x) / bounds.width;
                if *alpha <= 0.0f32 {
                    *alpha = 0.0f32;
                }
                if *alpha >= 1.0f32 {
                    *alpha = 1.0f32;
                }
            } else {
                state = STATE_FOCUSED;
            }
        }
    }
    if state as libc::c_uint != STATE_DISABLED as libc::c_int as libc::c_uint {
        let mut checksX: libc::c_int = bounds.width as libc::c_int / 10 as libc::c_int;
        let mut checksY: libc::c_int = bounds.height as libc::c_int / 10 as libc::c_int;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < checksX {
            let mut y: libc::c_int = 0 as libc::c_int;
            while y < checksY {
                let mut check: Rectangle = {
                    let mut init = Rectangle {
                        x: bounds.x + (x * 10 as libc::c_int) as libc::c_float,
                        y: bounds.y + (y * 10 as libc::c_int) as libc::c_float,
                        width: 10 as libc::c_int as libc::c_float,
                        height: 10 as libc::c_int as libc::c_float,
                    };
                    init
                };
                GuiDrawRectangle(
                    check,
                    0 as libc::c_int,
                    {
                        let mut init = Color {
                            r: 0 as libc::c_int as libc::c_uchar,
                            g: 0 as libc::c_int as libc::c_uchar,
                            b: 0 as libc::c_int as libc::c_uchar,
                            a: 0 as libc::c_int as libc::c_uchar,
                        };
                        init
                    },
                    if (x + y) % 2 as libc::c_int != 0 {
                        Fade(
                            GetColor(
                                GuiGetStyle(
                                    COLORPICKER as libc::c_int,
                                    BORDER_COLOR_DISABLED as libc::c_int,
                                ) as libc::c_uint,
                            ),
                            0.4f32,
                        )
                    } else {
                        Fade(
                            GetColor(
                                GuiGetStyle(
                                    COLORPICKER as libc::c_int,
                                    BASE_COLOR_DISABLED as libc::c_int,
                                ) as libc::c_uint,
                            ),
                            0.4f32,
                        )
                    },
                );
                y += 1;
                y;
            }
            x += 1;
            x;
        }
        DrawRectangleGradientEx(
            bounds,
            {
                let mut init = Color {
                    r: 255 as libc::c_int as libc::c_uchar,
                    g: 255 as libc::c_int as libc::c_uchar,
                    b: 255 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = Color {
                    r: 255 as libc::c_int as libc::c_uchar,
                    g: 255 as libc::c_int as libc::c_uchar,
                    b: 255 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            Fade(
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
            Fade(
                {
                    let mut init = Color {
                        r: 0 as libc::c_int as libc::c_uchar,
                        g: 0 as libc::c_int as libc::c_uchar,
                        b: 0 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
                guiAlpha,
            ),
        );
    } else {
        DrawRectangleGradientEx(
            bounds,
            Fade(
                GetColor(
                    GuiGetStyle(
                        COLORPICKER as libc::c_int,
                        BASE_COLOR_DISABLED as libc::c_int,
                    ) as libc::c_uint,
                ),
                0.1f32,
            ),
            Fade(
                GetColor(
                    GuiGetStyle(
                        COLORPICKER as libc::c_int,
                        BASE_COLOR_DISABLED as libc::c_int,
                    ) as libc::c_uint,
                ),
                0.1f32,
            ),
            Fade(
                GetColor(
                    GuiGetStyle(
                        COLORPICKER as libc::c_int,
                        BORDER_COLOR_DISABLED as libc::c_int,
                    ) as libc::c_uint,
                ),
                guiAlpha,
            ),
            Fade(
                GetColor(
                    GuiGetStyle(
                        COLORPICKER as libc::c_int,
                        BORDER_COLOR_DISABLED as libc::c_int,
                    ) as libc::c_uint,
                ),
                guiAlpha,
            ),
        );
    }
    GuiDrawRectangle(
        bounds,
        GuiGetStyle(COLORPICKER as libc::c_int, BORDER_WIDTH as libc::c_int),
        GetColor(
            GuiGetStyle(
                COLORPICKER as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
    );
    GuiDrawRectangle(
        selector,
        0 as libc::c_int,
        {
            let mut init = Color {
                r: 0 as libc::c_int as libc::c_uchar,
                g: 0 as libc::c_int as libc::c_uchar,
                b: 0 as libc::c_int as libc::c_uchar,
                a: 0 as libc::c_int as libc::c_uchar,
            };
            init
        },
        GetColor(
            GuiGetStyle(
                COLORPICKER as libc::c_int,
                (BORDER as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (state as libc::c_uint)
                            .wrapping_mul(3 as libc::c_int as libc::c_uint),
                    ) as libc::c_int,
            ) as libc::c_uint,
        ),
    );
    return result;
}
pub unsafe extern "C" fn GuiColorPickerHSV(
    mut bounds: Rectangle,
    mut text: *const libc::c_char,
    mut colorHsv: *mut Vector3,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut tempHsv: Vector3 = {
        let mut init = Vector3 {
            x: 0 as libc::c_int as libc::c_float,
            y: 0.,
            z: 0.,
        };
        init
    };
    if colorHsv.is_null() {
        let tempColor: Vector3 = {
            let mut init = Vector3 {
                x: 200.0f32 / 255.0f32,
                y: 0.0f32,
                z: 0.0f32,
            };
            init
        };
        tempHsv = ConvertRGBtoHSV(tempColor);
        colorHsv = &mut tempHsv;
    }
    GuiColorPanelHSV(bounds, 0 as *const libc::c_char, colorHsv);
    let boundsHue: Rectangle = {
        let mut init = Rectangle {
            x: bounds.x + bounds.width
                + GuiGetStyle(COLORPICKER as libc::c_int, HUEBAR_PADDING as libc::c_int)
                    as libc::c_float,
            y: bounds.y,
            width: GuiGetStyle(COLORPICKER as libc::c_int, HUEBAR_WIDTH as libc::c_int)
                as libc::c_float,
            height: bounds.height,
        };
        init
    };
    GuiColorBarHue(boundsHue, 0 as *const libc::c_char, &mut (*colorHsv).x);
    return result;
}
pub unsafe extern "C" fn GuiSetStyle(
    mut control: libc::c_int,
    mut property: libc::c_int,
    mut value: libc::c_int,
) {
    if !guiStyleLoaded {
        GuiLoadStyleDefault();
    }
    guiStyle[(control * (16 as libc::c_int + 8 as libc::c_int) + property)
        as usize] = value as libc::c_uint;
    if control == 0 as libc::c_int && property < 16 as libc::c_int {
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < 16 as libc::c_int {
            guiStyle[(i * (16 as libc::c_int + 8 as libc::c_int) + property)
                as usize] = value as libc::c_uint;
            i += 1;
            i;
        }
    }
}
pub unsafe extern "C" fn GuiLoadStyleDefault() {
    guiStyleLoaded = 1 as libc::c_int != 0;
    GuiSetStyle(
        DEFAULT as libc::c_int,
        BORDER_COLOR_NORMAL as libc::c_int,
        0x838383ff as libc::c_uint as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        BASE_COLOR_NORMAL as libc::c_int,
        0xc9c9c9ff as libc::c_uint as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        TEXT_COLOR_NORMAL as libc::c_int,
        0x686868ff as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        BORDER_COLOR_FOCUSED as libc::c_int,
        0x5bb2d9ff as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        BASE_COLOR_FOCUSED as libc::c_int,
        0xc9effeff as libc::c_uint as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        TEXT_COLOR_FOCUSED as libc::c_int,
        0x6c9bbcff as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        BORDER_COLOR_PRESSED as libc::c_int,
        0x492c7ff as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        BASE_COLOR_PRESSED as libc::c_int,
        0x97e8ffff as libc::c_uint as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        TEXT_COLOR_PRESSED as libc::c_int,
        0x368bafff as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        BORDER_COLOR_DISABLED as libc::c_int,
        0xb5c1c2ff as libc::c_uint as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        BASE_COLOR_DISABLED as libc::c_int,
        0xe6e9e9ff as libc::c_uint as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        TEXT_COLOR_DISABLED as libc::c_int,
        0xaeb7b8ff as libc::c_uint as libc::c_int,
    );
    GuiSetStyle(DEFAULT as libc::c_int, BORDER_WIDTH as libc::c_int, 1 as libc::c_int);
    GuiSetStyle(DEFAULT as libc::c_int, TEXT_PADDING as libc::c_int, 0 as libc::c_int);
    GuiSetStyle(
        DEFAULT as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_CENTER as libc::c_int,
    );
    GuiSetStyle(DEFAULT as libc::c_int, TEXT_SIZE as libc::c_int, 10 as libc::c_int);
    GuiSetStyle(DEFAULT as libc::c_int, TEXT_SPACING as libc::c_int, 1 as libc::c_int);
    GuiSetStyle(
        DEFAULT as libc::c_int,
        LINE_COLOR as libc::c_int,
        0x90abb5ff as libc::c_uint as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        BACKGROUND_COLOR as libc::c_int,
        0xf5f5f5ff as libc::c_uint as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        TEXT_LINE_SPACING as libc::c_int,
        15 as libc::c_int,
    );
    GuiSetStyle(
        DEFAULT as libc::c_int,
        TEXT_ALIGNMENT_VERTICAL as libc::c_int,
        TEXT_ALIGN_MIDDLE as libc::c_int,
    );
    GuiSetStyle(
        LABEL as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_LEFT as libc::c_int,
    );
    GuiSetStyle(BUTTON as libc::c_int, BORDER_WIDTH as libc::c_int, 2 as libc::c_int);
    GuiSetStyle(SLIDER as libc::c_int, TEXT_PADDING as libc::c_int, 4 as libc::c_int);
    GuiSetStyle(
        PROGRESSBAR as libc::c_int,
        TEXT_PADDING as libc::c_int,
        4 as libc::c_int,
    );
    GuiSetStyle(CHECKBOX as libc::c_int, TEXT_PADDING as libc::c_int, 4 as libc::c_int);
    GuiSetStyle(
        CHECKBOX as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_RIGHT as libc::c_int,
    );
    GuiSetStyle(
        DROPDOWNBOX as libc::c_int,
        TEXT_PADDING as libc::c_int,
        0 as libc::c_int,
    );
    GuiSetStyle(
        DROPDOWNBOX as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_CENTER as libc::c_int,
    );
    GuiSetStyle(TEXTBOX as libc::c_int, TEXT_PADDING as libc::c_int, 4 as libc::c_int);
    GuiSetStyle(
        TEXTBOX as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_LEFT as libc::c_int,
    );
    GuiSetStyle(VALUEBOX as libc::c_int, TEXT_PADDING as libc::c_int, 0 as libc::c_int);
    GuiSetStyle(
        VALUEBOX as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_LEFT as libc::c_int,
    );
    GuiSetStyle(SPINNER as libc::c_int, TEXT_PADDING as libc::c_int, 0 as libc::c_int);
    GuiSetStyle(
        SPINNER as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_LEFT as libc::c_int,
    );
    GuiSetStyle(STATUSBAR as libc::c_int, TEXT_PADDING as libc::c_int, 8 as libc::c_int);
    GuiSetStyle(
        STATUSBAR as libc::c_int,
        TEXT_ALIGNMENT as libc::c_int,
        TEXT_ALIGN_LEFT as libc::c_int,
    );
    GuiSetStyle(TOGGLE as libc::c_int, GROUP_PADDING as libc::c_int, 2 as libc::c_int);
    GuiSetStyle(SLIDER as libc::c_int, SLIDER_WIDTH as libc::c_int, 16 as libc::c_int);
    GuiSetStyle(SLIDER as libc::c_int, SLIDER_PADDING as libc::c_int, 1 as libc::c_int);
    GuiSetStyle(
        PROGRESSBAR as libc::c_int,
        PROGRESS_PADDING as libc::c_int,
        1 as libc::c_int,
    );
    GuiSetStyle(CHECKBOX as libc::c_int, CHECK_PADDING as libc::c_int, 1 as libc::c_int);
    GuiSetStyle(
        COMBOBOX as libc::c_int,
        COMBO_BUTTON_WIDTH as libc::c_int,
        32 as libc::c_int,
    );
    GuiSetStyle(
        COMBOBOX as libc::c_int,
        COMBO_BUTTON_SPACING as libc::c_int,
        2 as libc::c_int,
    );
    GuiSetStyle(
        DROPDOWNBOX as libc::c_int,
        ARROW_PADDING as libc::c_int,
        16 as libc::c_int,
    );
    GuiSetStyle(
        DROPDOWNBOX as libc::c_int,
        DROPDOWN_ITEMS_SPACING as libc::c_int,
        2 as libc::c_int,
    );
    GuiSetStyle(
        SPINNER as libc::c_int,
        SPIN_BUTTON_WIDTH as libc::c_int,
        24 as libc::c_int,
    );
    GuiSetStyle(
        SPINNER as libc::c_int,
        SPIN_BUTTON_SPACING as libc::c_int,
        2 as libc::c_int,
    );
    GuiSetStyle(SCROLLBAR as libc::c_int, BORDER_WIDTH as libc::c_int, 0 as libc::c_int);
    GuiSetStyle(
        SCROLLBAR as libc::c_int,
        ARROWS_VISIBLE as libc::c_int,
        0 as libc::c_int,
    );
    GuiSetStyle(SCROLLBAR as libc::c_int, ARROWS_SIZE as libc::c_int, 6 as libc::c_int);
    GuiSetStyle(
        SCROLLBAR as libc::c_int,
        SCROLL_SLIDER_PADDING as libc::c_int,
        0 as libc::c_int,
    );
    GuiSetStyle(
        SCROLLBAR as libc::c_int,
        SCROLL_SLIDER_SIZE as libc::c_int,
        16 as libc::c_int,
    );
    GuiSetStyle(
        SCROLLBAR as libc::c_int,
        SCROLL_PADDING as libc::c_int,
        0 as libc::c_int,
    );
    GuiSetStyle(
        SCROLLBAR as libc::c_int,
        SCROLL_SPEED as libc::c_int,
        12 as libc::c_int,
    );
    GuiSetStyle(
        LISTVIEW as libc::c_int,
        LIST_ITEMS_HEIGHT as libc::c_int,
        28 as libc::c_int,
    );
    GuiSetStyle(
        LISTVIEW as libc::c_int,
        LIST_ITEMS_SPACING as libc::c_int,
        2 as libc::c_int,
    );
    GuiSetStyle(
        LISTVIEW as libc::c_int,
        SCROLLBAR_WIDTH as libc::c_int,
        12 as libc::c_int,
    );
    GuiSetStyle(
        LISTVIEW as libc::c_int,
        SCROLLBAR_SIDE as libc::c_int,
        1 as libc::c_int,
    );
    GuiSetStyle(
        COLORPICKER as libc::c_int,
        COLOR_SELECTOR_SIZE as libc::c_int,
        8 as libc::c_int,
    );
    GuiSetStyle(
        COLORPICKER as libc::c_int,
        HUEBAR_WIDTH as libc::c_int,
        16 as libc::c_int,
    );
    GuiSetStyle(
        COLORPICKER as libc::c_int,
        HUEBAR_PADDING as libc::c_int,
        8 as libc::c_int,
    );
    GuiSetStyle(
        COLORPICKER as libc::c_int,
        HUEBAR_SELECTOR_HEIGHT as libc::c_int,
        8 as libc::c_int,
    );
    GuiSetStyle(
        COLORPICKER as libc::c_int,
        HUEBAR_SELECTOR_OVERFLOW as libc::c_int,
        2 as libc::c_int,
    );
    if guiFont.texture.id != (GetFontDefault()).texture.id {
        UnloadTexture(guiFont.texture);
        free(guiFont.recs as *mut libc::c_void);
        free(guiFont.glyphs as *mut libc::c_void);
        guiFont.recs = 0 as *mut Rectangle;
        guiFont.glyphs = 0 as *mut GlyphInfo;
        guiFont = GetFontDefault();
        let mut whiteChar: Rectangle = *(guiFont.recs)
            .offset(95 as libc::c_int as isize);
        SetShapesTexture(
            guiFont.texture,
            {
                let mut init = Rectangle {
                    x: whiteChar.x + 1 as libc::c_int as libc::c_float,
                    y: whiteChar.y + 1 as libc::c_int as libc::c_float,
                    width: whiteChar.width - 2 as libc::c_int as libc::c_float,
                    height: whiteChar.height - 2 as libc::c_int as libc::c_float,
                };
                init
            },
        );
    }
}
static mut guiFont: Font = {
    let mut init = Font {
        baseSize: 0 as libc::c_int,
        glyphCount: 0,
        glyphPadding: 0,
        texture: Texture2D {
            id: 0,
            width: 0,
            height: 0,
            mipmaps: 0,
            format: 0,
        },
        recs: 0 as *const Rectangle as *mut Rectangle,
        glyphs: 0 as *const GlyphInfo as *mut GlyphInfo,
    };
    init
};
pub unsafe extern "C" fn GuiSetFont(mut font: Font) {
    if font.texture.id > 0 as libc::c_int as libc::c_uint {
        if !guiStyleLoaded {
            GuiLoadStyleDefault();
        }
        guiFont = font;
    }
}
pub unsafe extern "C" fn GuiGetState() -> libc::c_int {
    return guiState as libc::c_int;
}
pub unsafe extern "C" fn GuiSetState(mut state: libc::c_int) {
    guiState = state as GuiState;
}
static mut guiAlpha: libc::c_float = 1.0f32;
pub unsafe extern "C" fn GuiSetAlpha(mut alpha: libc::c_float) {
    if alpha < 0.0f32 {
        alpha = 0.0f32;
    } else if alpha > 1.0f32 {
        alpha = 1.0f32;
    }
    guiAlpha = alpha;
}
pub unsafe extern "C" fn GuiIsLocked() -> bool {
    return guiLocked;
}
pub unsafe extern "C" fn GuiUnlock() {
    guiLocked = 0 as libc::c_int != 0;
}
static mut guiLocked: bool = 0 as libc::c_int != 0;
pub unsafe extern "C" fn GuiLock() {
    guiLocked = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn GuiDisable() {
    if guiState as libc::c_uint == STATE_NORMAL as libc::c_int as libc::c_uint {
        guiState = STATE_DISABLED;
    }
}
static mut guiState: GuiState = STATE_NORMAL;
pub unsafe extern "C" fn GuiEnable() {
    if guiState as libc::c_uint == STATE_DISABLED as libc::c_int as libc::c_uint {
        guiState = STATE_NORMAL;
    }
}
unsafe fn main_0() -> libc::c_int {
    let screenWidth: libc::c_int = 800 as libc::c_int;
    let screenHeight: libc::c_int = 600 as libc::c_int;
    InitWindow(
        screenWidth,
        screenHeight,
        b"raygui - image raw importer\0" as *const u8 as *const libc::c_char,
    );
    let mut texture: Texture2D = {
        let mut init = Texture {
            id: 0 as libc::c_int as libc::c_uint,
            width: 0,
            height: 0,
            mipmaps: 0,
            format: 0,
        };
        init
    };
    let mut windowOffset: Vector2 = {
        let mut init = Vector2 {
            x: (screenWidth / 2 as libc::c_int - 200 as libc::c_int / 2 as libc::c_int)
                as libc::c_float,
            y: (screenHeight / 2 as libc::c_int - 465 as libc::c_int / 2 as libc::c_int)
                as libc::c_float,
        };
        init
    };
    let mut importWindowActive: bool = 0 as libc::c_int != 0;
    let mut widthValue: libc::c_int = 0 as libc::c_int;
    let mut widthEditMode: bool = 0 as libc::c_int != 0;
    let mut heightValue: libc::c_int = 0 as libc::c_int;
    let mut heightEditMode: bool = 0 as libc::c_int != 0;
    let mut pixelFormatActive: libc::c_int = 0 as libc::c_int;
    let mut pixelFormatTextList: [*const libc::c_char; 8] = [
        b"CUSTOM\0" as *const u8 as *const libc::c_char,
        b"GRAYSCALE\0" as *const u8 as *const libc::c_char,
        b"GRAY ALPHA\0" as *const u8 as *const libc::c_char,
        b"R5G6B5\0" as *const u8 as *const libc::c_char,
        b"R8G8B8\0" as *const u8 as *const libc::c_char,
        b"R5G5B5A1\0" as *const u8 as *const libc::c_char,
        b"R4G4B4A4\0" as *const u8 as *const libc::c_char,
        b"R8G8B8A8\0" as *const u8 as *const libc::c_char,
    ];
    let mut channelsActive: libc::c_int = 3 as libc::c_int;
    let mut channelsTextList: [*const libc::c_char; 4] = [
        b"1\0" as *const u8 as *const libc::c_char,
        b"2\0" as *const u8 as *const libc::c_char,
        b"3\0" as *const u8 as *const libc::c_char,
        b"4\0" as *const u8 as *const libc::c_char,
    ];
    let mut bitDepthActive: libc::c_int = 0 as libc::c_int;
    let mut bitDepthTextList: [*const libc::c_char; 3] = [
        b"8\0" as *const u8 as *const libc::c_char,
        b"16\0" as *const u8 as *const libc::c_char,
        b"32\0" as *const u8 as *const libc::c_char,
    ];
    let mut headerSizeValue: libc::c_int = 0 as libc::c_int;
    let mut headerSizeEditMode: bool = 0 as libc::c_int != 0;
    let mut dataSize: libc::c_int = 0 as libc::c_int;
    let mut fileNamePath: [libc::c_char; 256] = *::std::mem::transmute::<
        &[u8; 256],
        &mut [libc::c_char; 256],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut fileName: [libc::c_char; 64] = *::std::mem::transmute::<
        &[u8; 64],
        &mut [libc::c_char; 64],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut btnLoadPressed: bool = 0 as libc::c_int != 0;
    let mut imageLoaded: bool = 0 as libc::c_int != 0;
    let mut imageScale: libc::c_float = 1.0f32;
    SetTargetFPS(60 as libc::c_int);
    while !WindowShouldClose() {
        if IsFileDropped() {
            let mut droppedFiles: FilePathList = LoadDroppedFiles();
            if droppedFiles.count == 1 as libc::c_int as libc::c_uint
                && IsFileExtension(
                    *(droppedFiles.paths).offset(0 as libc::c_int as isize),
                    b".raw\0" as *const u8 as *const libc::c_char,
                ) as libc::c_int != 0
            {
                let mut imageFile: *mut FILE = fopen(
                    *(droppedFiles.paths).offset(0 as libc::c_int as isize),
                    b"rb\0" as *const u8 as *const libc::c_char,
                );
                fseek(imageFile, 0 as libc::c_long, 2 as libc::c_int);
                dataSize = ftell(imageFile) as libc::c_int;
                fclose(imageFile);
                strcpy(
                    fileNamePath.as_mut_ptr(),
                    *(droppedFiles.paths).offset(0 as libc::c_int as isize),
                );
                strcpy(
                    fileName.as_mut_ptr(),
                    GetFileName(*(droppedFiles.paths).offset(0 as libc::c_int as isize)),
                );
                widthValue = round(sqrt((dataSize / 4 as libc::c_int) as libc::c_double))
                    as libc::c_int;
                heightValue = widthValue;
                headerSizeValue = dataSize - widthValue * heightValue * 4 as libc::c_int;
                if headerSizeValue < 0 as libc::c_int {
                    headerSizeValue = 0 as libc::c_int;
                }
                importWindowActive = 1 as libc::c_int != 0;
            }
            UnloadDroppedFiles(droppedFiles);
        }
        if btnLoadPressed {
            if widthValue != 0 as libc::c_int && heightValue != 0 as libc::c_int {
                let mut format: libc::c_int = -(1 as libc::c_int);
                if pixelFormatActive == 0 as libc::c_int {
                    let mut channels: libc::c_int = atoi(
                        channelsTextList[channelsActive as usize],
                    );
                    let mut bpp: libc::c_int = atoi(
                        bitDepthTextList[bitDepthActive as usize],
                    );
                    if bpp == 8 as libc::c_int {
                        if channels == 1 as libc::c_int {
                            format = PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as libc::c_int;
                        } else if channels == 2 as libc::c_int {
                            format = PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA as libc::c_int;
                        } else if channels == 3 as libc::c_int {
                            format = PIXELFORMAT_UNCOMPRESSED_R8G8B8 as libc::c_int;
                        } else if channels == 4 as libc::c_int {
                            format = PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as libc::c_int;
                        }
                    } else if bpp == 32 as libc::c_int {
                        if channels == 1 as libc::c_int {
                            format = PIXELFORMAT_UNCOMPRESSED_R32 as libc::c_int;
                        } else if channels == 2 as libc::c_int {
                            TraceLog(
                                LOG_WARNING as libc::c_int,
                                b"Channel bit-depth not supported!\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else if channels == 3 as libc::c_int {
                            format = PIXELFORMAT_UNCOMPRESSED_R32G32B32 as libc::c_int;
                        } else if channels == 4 as libc::c_int {
                            format = PIXELFORMAT_UNCOMPRESSED_R32G32B32A32
                                as libc::c_int;
                        }
                    } else if bpp == 16 as libc::c_int {
                        TraceLog(
                            LOG_WARNING as libc::c_int,
                            b"Channel bit-depth not supported!\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                } else {
                    format = pixelFormatActive;
                }
                if format != -(1 as libc::c_int) {
                    let mut image: Image = LoadImageRaw(
                        fileNamePath.as_mut_ptr(),
                        widthValue,
                        heightValue,
                        format,
                        headerSizeValue,
                    );
                    texture = LoadTextureFromImage(image);
                    UnloadImage(image);
                    importWindowActive = 0 as libc::c_int != 0;
                    btnLoadPressed = 0 as libc::c_int != 0;
                    if texture.id > 0 as libc::c_int as libc::c_uint {
                        imageLoaded = 1 as libc::c_int != 0;
                        imageScale = (screenHeight - 100 as libc::c_int) as libc::c_float
                            / texture.height as libc::c_float;
                    }
                }
            }
        }
        if imageLoaded {
            imageScale += GetMouseWheelMove();
        }
        BeginDrawing();
        ClearBackground(
            GetColor(
                GuiGetStyle(DEFAULT as libc::c_int, BACKGROUND_COLOR as libc::c_int)
                    as libc::c_uint,
            ),
        );
        if texture.id != 0 as libc::c_int as libc::c_uint {
            DrawTextureEx(
                texture,
                {
                    let mut init = Vector2 {
                        x: (screenWidth / 2 as libc::c_int) as libc::c_float
                            - texture.width as libc::c_float * imageScale
                                / 2 as libc::c_int as libc::c_float,
                        y: (screenHeight / 2 as libc::c_int) as libc::c_float
                            - texture.height as libc::c_float * imageScale
                                / 2 as libc::c_int as libc::c_float,
                    };
                    init
                },
                0 as libc::c_int as libc::c_float,
                imageScale,
                {
                    let mut init = Color {
                        r: 255 as libc::c_int as libc::c_uchar,
                        g: 255 as libc::c_int as libc::c_uchar,
                        b: 255 as libc::c_int as libc::c_uchar,
                        a: 255 as libc::c_int as libc::c_uchar,
                    };
                    init
                },
            );
            DrawText(
                TextFormat(
                    b"SCALE x%.0f\0" as *const u8 as *const libc::c_char,
                    imageScale as libc::c_double,
                ),
                20 as libc::c_int,
                screenHeight - 40 as libc::c_int,
                20 as libc::c_int,
                GetColor(
                    GuiGetStyle(DEFAULT as libc::c_int, LINE_COLOR as libc::c_int)
                        as libc::c_uint,
                ),
            );
        } else {
            DrawText(
                b"drag & drop RAW image file\0" as *const u8 as *const libc::c_char,
                320 as libc::c_int,
                180 as libc::c_int,
                10 as libc::c_int,
                GetColor(
                    GuiGetStyle(DEFAULT as libc::c_int, LINE_COLOR as libc::c_int)
                        as libc::c_uint,
                ),
            );
        }
        if importWindowActive {
            importWindowActive = GuiWindowBox(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 0 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 0 as libc::c_int as libc::c_float,
                        width: 200 as libc::c_int as libc::c_float,
                        height: 465 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"Image RAW Import Options\0" as *const u8 as *const libc::c_char,
            ) == 0;
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 10 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 30 as libc::c_int as libc::c_float,
                        width: 65 as libc::c_int as libc::c_float,
                        height: 20 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"Import file:\0" as *const u8 as *const libc::c_char,
            );
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 85 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 30 as libc::c_int as libc::c_float,
                        width: 75 as libc::c_int as libc::c_float,
                        height: 20 as libc::c_int as libc::c_float,
                    };
                    init
                },
                fileName.as_mut_ptr(),
            );
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 10 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 50 as libc::c_int as libc::c_float,
                        width: 65 as libc::c_int as libc::c_float,
                        height: 20 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"File size:\0" as *const u8 as *const libc::c_char,
            );
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 85 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 50 as libc::c_int as libc::c_float,
                        width: 75 as libc::c_int as libc::c_float,
                        height: 20 as libc::c_int as libc::c_float,
                    };
                    init
                },
                TextFormat(b"%i bytes\0" as *const u8 as *const libc::c_char, dataSize),
            );
            GuiGroupBox(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 10 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 85 as libc::c_int as libc::c_float,
                        width: 180 as libc::c_int as libc::c_float,
                        height: 80 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"Resolution\0" as *const u8 as *const libc::c_char,
            );
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 20 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 100 as libc::c_int as libc::c_float,
                        width: 33 as libc::c_int as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"Width:\0" as *const u8 as *const libc::c_char,
            );
            if GuiValueBox(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 60 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 100 as libc::c_int as libc::c_float,
                        width: 80 as libc::c_int as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                0 as *const libc::c_char,
                &mut widthValue,
                0 as libc::c_int,
                8192 as libc::c_int,
                widthEditMode,
            ) != 0
            {
                widthEditMode = !widthEditMode;
            }
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 145 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 100 as libc::c_int as libc::c_float,
                        width: 30 as libc::c_int as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"pixels\0" as *const u8 as *const libc::c_char,
            );
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 20 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 130 as libc::c_int as libc::c_float,
                        width: 33 as libc::c_int as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"Height:\0" as *const u8 as *const libc::c_char,
            );
            if GuiValueBox(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 60 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 130 as libc::c_int as libc::c_float,
                        width: 80 as libc::c_int as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                0 as *const libc::c_char,
                &mut heightValue,
                0 as libc::c_int,
                8192 as libc::c_int,
                heightEditMode,
            ) != 0
            {
                heightEditMode = !heightEditMode;
            }
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 145 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 130 as libc::c_int as libc::c_float,
                        width: 30 as libc::c_int as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"pixels\0" as *const u8 as *const libc::c_char,
            );
            GuiGroupBox(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 10 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 180 as libc::c_int as libc::c_float,
                        width: 180 as libc::c_int as libc::c_float,
                        height: 160 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"Pixel Format\0" as *const u8 as *const libc::c_char,
            );
            GuiComboBox(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 20 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 195 as libc::c_int as libc::c_float,
                        width: 160 as libc::c_int as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                TextJoin(
                    pixelFormatTextList.as_mut_ptr(),
                    8 as libc::c_int,
                    b";\0" as *const u8 as *const libc::c_char,
                ),
                &mut pixelFormatActive,
            );
            GuiLine(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 20 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 220 as libc::c_int as libc::c_float,
                        width: 160 as libc::c_int as libc::c_float,
                        height: 20 as libc::c_int as libc::c_float,
                    };
                    init
                },
                0 as *const libc::c_char,
            );
            if pixelFormatActive != 0 as libc::c_int {
                GuiDisable();
            }
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 20 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 235 as libc::c_int as libc::c_float,
                        width: 50 as libc::c_int as libc::c_float,
                        height: 20 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"Channels:\0" as *const u8 as *const libc::c_char,
            );
            GuiToggleGroup(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 20 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 255 as libc::c_int as libc::c_float,
                        width: (156 as libc::c_int / 4 as libc::c_int) as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                TextJoin(
                    channelsTextList.as_mut_ptr(),
                    4 as libc::c_int,
                    b";\0" as *const u8 as *const libc::c_char,
                ),
                &mut channelsActive,
            );
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 20 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 285 as libc::c_int as libc::c_float,
                        width: 50 as libc::c_int as libc::c_float,
                        height: 20 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"Bit Depth:\0" as *const u8 as *const libc::c_char,
            );
            GuiToggleGroup(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 20 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 305 as libc::c_int as libc::c_float,
                        width: (160 as libc::c_int / 3 as libc::c_int) as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                TextJoin(
                    bitDepthTextList.as_mut_ptr(),
                    3 as libc::c_int,
                    b";\0" as *const u8 as *const libc::c_char,
                ),
                &mut bitDepthActive,
            );
            GuiEnable();
            GuiGroupBox(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 10 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 355 as libc::c_int as libc::c_float,
                        width: 180 as libc::c_int as libc::c_float,
                        height: 50 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"Header\0" as *const u8 as *const libc::c_char,
            );
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 25 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 370 as libc::c_int as libc::c_float,
                        width: 27 as libc::c_int as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"Size:\0" as *const u8 as *const libc::c_char,
            );
            if GuiValueBox(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 55 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 370 as libc::c_int as libc::c_float,
                        width: 85 as libc::c_int as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                0 as *const libc::c_char,
                &mut headerSizeValue,
                0 as libc::c_int,
                10000 as libc::c_int,
                headerSizeEditMode,
            ) != 0
            {
                headerSizeEditMode = !headerSizeEditMode;
            }
            GuiLabel(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 145 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 370 as libc::c_int as libc::c_float,
                        width: 30 as libc::c_int as libc::c_float,
                        height: 25 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"bytes\0" as *const u8 as *const libc::c_char,
            );
            btnLoadPressed = GuiButton(
                {
                    let mut init = Rectangle {
                        x: windowOffset.x + 10 as libc::c_int as libc::c_float,
                        y: windowOffset.y + 420 as libc::c_int as libc::c_float,
                        width: 180 as libc::c_int as libc::c_float,
                        height: 30 as libc::c_int as libc::c_float,
                    };
                    init
                },
                b"Import RAW\0" as *const u8 as *const libc::c_char,
            ) != 0;
        }
        EndDrawing();
    }
    if texture.id != 0 as libc::c_int as libc::c_uint {
        UnloadTexture(texture);
    }
    CloseWindow();
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
