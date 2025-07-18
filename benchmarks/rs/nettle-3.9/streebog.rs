use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _nettle_write_le64(length: size_t, dst: *mut uint8_t, src: *const uint64_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streebog512_ctx {
    pub state: [uint64_t; 8],
    pub count: [uint64_t; 8],
    pub sigma: [uint64_t; 8],
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
static mut streebog_table: [[uint64_t; 256]; 8] = [
    [
        0xd01f715b5c7ef8e6 as libc::c_ulonglong as uint64_t,
        0x16fa240980778325 as libc::c_ulonglong as uint64_t,
        0xa8a42e857ee049c8 as libc::c_ulonglong as uint64_t,
        0x6ac1068fa186465b as libc::c_ulonglong as uint64_t,
        0x6e417bd7a2e9320b as libc::c_ulonglong as uint64_t,
        0x665c8167a437daab as libc::c_ulonglong as uint64_t,
        0x7666681aa89617f6 as libc::c_ulonglong as uint64_t,
        0x4b959163700bdcf5 as libc::c_ulonglong as uint64_t,
        0xf14be6b78df36248 as libc::c_ulonglong as uint64_t,
        0xc585bd689a625cff as libc::c_ulonglong as uint64_t,
        0x9557d7fca67d82cb as libc::c_ulonglong as uint64_t,
        0x89f0b969af6dd366 as libc::c_ulonglong as uint64_t,
        0xb0833d48749f6c35 as libc::c_ulonglong as uint64_t,
        0xa1998c23b1ecbc7c as libc::c_ulonglong as uint64_t,
        0x8d70c431ac02a736 as libc::c_ulonglong as uint64_t,
        0xd6dfbc2fd0a8b69e as libc::c_ulonglong as uint64_t,
        0x37aeb3e551fa198b as libc::c_ulonglong as uint64_t,
        0xb7d128a40b5cf9c as libc::c_ulonglong as uint64_t,
        0x5a8f2008b5780cbc as libc::c_ulonglong as uint64_t,
        0xedec882284e333e5 as libc::c_ulonglong as uint64_t,
        0xd25fc177d3c7c2ce as libc::c_ulonglong as uint64_t,
        0x5e0f5d50b61778ec as libc::c_ulonglong as uint64_t,
        0x1d873683c0c24cb9 as libc::c_ulonglong as uint64_t,
        0xad040bcbb45d208c as libc::c_ulonglong as uint64_t,
        0x2f89a0285b853c76 as libc::c_ulonglong as uint64_t,
        0x5732fff6791b8d58 as libc::c_ulonglong as uint64_t,
        0x3e9311439ef6ec3f as libc::c_ulonglong as uint64_t,
        0xc9183a809fd3c00f as libc::c_ulonglong as uint64_t,
        0x83adf3f5260a01ee as libc::c_ulonglong as uint64_t,
        0xa6791941f4e8ef10 as libc::c_ulonglong as uint64_t,
        0x103ae97d0ca1cd5d as libc::c_ulonglong as uint64_t,
        0x2ce948121dee1b4a as libc::c_ulonglong as uint64_t,
        0x39738421dbf2bf53 as libc::c_ulonglong as uint64_t,
        0x93da2a6cf0cf5b4 as libc::c_ulonglong as uint64_t,
        0xcd9847d89cbcb45f as libc::c_ulonglong as uint64_t,
        0xf9561c078b2d8ae8 as libc::c_ulonglong as uint64_t,
        0x9c6a755a6971777f as libc::c_ulonglong as uint64_t,
        0xbc1ebaa0712ef0c5 as libc::c_ulonglong as uint64_t,
        0x72e61542abf963a6 as libc::c_ulonglong as uint64_t,
        0x78bb5fde229eb12e as libc::c_ulonglong as uint64_t,
        0x14ba94250fceb90d as libc::c_ulonglong as uint64_t,
        0x844d6697630e5282 as libc::c_ulonglong as uint64_t,
        0x98ea08026a1e032f as libc::c_ulonglong as uint64_t,
        0xf06bbea144217f5c as libc::c_ulonglong as uint64_t,
        0xdb6263d11ccb377a as libc::c_ulonglong as uint64_t,
        0x641c314b2b8ee083 as libc::c_ulonglong as uint64_t,
        0x320e96ab9b4770cf as libc::c_ulonglong as uint64_t,
        0x1ee7deb986a96b85 as libc::c_ulonglong as uint64_t,
        0xe96cf57a878c47b5 as libc::c_ulonglong as uint64_t,
        0xfdd6615f8842feb8 as libc::c_ulonglong as uint64_t,
        0xc83862965601dd1b as libc::c_ulonglong as uint64_t,
        0x2ea9f83e92572162 as libc::c_ulonglong as uint64_t,
        0xf876441142ff97fc as libc::c_ulonglong as uint64_t,
        0xeb2c455608357d9d as libc::c_ulonglong as uint64_t,
        0x5612a7e0b0c9904c as libc::c_ulonglong as uint64_t,
        0x6c01cbfb2d500823 as libc::c_ulonglong as uint64_t,
        0x4548a6a7fa037a2d as libc::c_ulonglong as uint64_t,
        0xabc4c6bf388b6ef4 as libc::c_ulonglong as uint64_t,
        0xbade77d4fdf8bebd as libc::c_ulonglong as uint64_t,
        0x799b07c8eb4cac3a as libc::c_ulonglong as uint64_t,
        0xc9d87e805b19cf0 as libc::c_ulonglong as uint64_t,
        0xcb588aac106afa27 as libc::c_ulonglong as uint64_t,
        0xea0c1d40c1e76089 as libc::c_ulonglong as uint64_t,
        0x2869354a1e816f1a as libc::c_ulonglong as uint64_t,
        0xff96d17307fbc490 as libc::c_ulonglong as uint64_t,
        0x9f0a9d602f1a5043 as libc::c_ulonglong as uint64_t,
        0x96373fc6e016a5f7 as libc::c_ulonglong as uint64_t,
        0x5292dab8b3a6e41c as libc::c_ulonglong as uint64_t,
        0x9b8ae0382c752413 as libc::c_ulonglong as uint64_t,
        0x4f15ec3b7364a8a5 as libc::c_ulonglong as uint64_t,
        0x3fb349555724f12b as libc::c_ulonglong as uint64_t,
        0xc7c50d4415db66d7 as libc::c_ulonglong as uint64_t,
        0x92b7429ee379d1a7 as libc::c_ulonglong as uint64_t,
        0xd37f99611a15dfda as libc::c_ulonglong as uint64_t,
        0x231427c05e34a086 as libc::c_ulonglong as uint64_t,
        0xa439a96d7b51d538 as libc::c_ulonglong as uint64_t,
        0xb403401077f01865 as libc::c_ulonglong as uint64_t,
        0xdda2aea5901d7902 as libc::c_ulonglong as uint64_t,
        0xa5d4a9c8967d288 as libc::c_ulonglong as uint64_t,
        0xc265280adf660f93 as libc::c_ulonglong as uint64_t,
        0x8bb0094520d4e94e as libc::c_ulonglong as uint64_t,
        0x2a29856691385532 as libc::c_ulonglong as uint64_t,
        0x42a833c5bf072941 as libc::c_ulonglong as uint64_t,
        0x73c64d54622b7eb2 as libc::c_ulonglong as uint64_t,
        0x7e095624504536c as libc::c_ulonglong as uint64_t,
        0x8a905153e906f45a as libc::c_ulonglong as uint64_t,
        0x6f6123c16b3b2f1f as libc::c_ulonglong as uint64_t,
        0xc6e55552dc097bc3 as libc::c_ulonglong as uint64_t,
        0x4468feb133d16739 as libc::c_ulonglong as uint64_t,
        0xe211e7f0c7398829 as libc::c_ulonglong as uint64_t,
        0xa2f96419f7879b40 as libc::c_ulonglong as uint64_t,
        0x19074bdbc3ad38e9 as libc::c_ulonglong as uint64_t,
        0xf4ebc3f9474e0b0c as libc::c_ulonglong as uint64_t,
        0x43886bd376d53455 as libc::c_ulonglong as uint64_t,
        0xd8028beb5aa01046 as libc::c_ulonglong as uint64_t,
        0x51f23282f5cdc320 as libc::c_ulonglong as uint64_t,
        0xe7b1c2be0d84e16d as libc::c_ulonglong as uint64_t,
        0x81dfab006dee8a0 as libc::c_ulonglong as uint64_t,
        0x3b33340d544b857b as libc::c_ulonglong as uint64_t,
        0x7f5bcabc679ae242 as libc::c_ulonglong as uint64_t,
        0xedd37c48a08a6d8 as libc::c_ulonglong as uint64_t,
        0x81ed43d9a9b33bc6 as libc::c_ulonglong as uint64_t,
        0xb1a3655ebd4d7121 as libc::c_ulonglong as uint64_t,
        0x69a1eeb5e7ed6167 as libc::c_ulonglong as uint64_t,
        0xf6ab73d5c8f73124 as libc::c_ulonglong as uint64_t,
        0x1a67a3e185c61fd5 as libc::c_ulonglong as uint64_t,
        0x2dc91004d43c065e as libc::c_ulonglong as uint64_t,
        0x240b02c8fb93a28 as libc::c_ulonglong as uint64_t,
        0x90f7f2b26cc0eb8f as libc::c_ulonglong as uint64_t,
        0x3cd3a16f114fd617 as libc::c_ulonglong as uint64_t,
        0xaae49ea9f15973e0 as libc::c_ulonglong as uint64_t,
        0x6c0cd748cd64e78 as libc::c_ulonglong as uint64_t,
        0xda423bc7d5192a6e as libc::c_ulonglong as uint64_t,
        0xc345701c16b41287 as libc::c_ulonglong as uint64_t,
        0x6d2193ede4821537 as libc::c_ulonglong as uint64_t,
        0xfcf639494190e3ac as libc::c_ulonglong as uint64_t,
        0x7c3b228621f1c57e as libc::c_ulonglong as uint64_t,
        0xfb16ac2b0494b0c0 as libc::c_ulonglong as uint64_t,
        0xbf7e529a3745d7f9 as libc::c_ulonglong as uint64_t,
        0x6881b6a32e3f7c73 as libc::c_ulonglong as uint64_t,
        0xca78d2bad9b8e733 as libc::c_ulonglong as uint64_t,
        0xbbfe2fc2342aa3a9 as libc::c_ulonglong as uint64_t,
        0xdbddffecc6381e4 as libc::c_ulonglong as uint64_t,
        0x70a6a56e2440598e as libc::c_ulonglong as uint64_t,
        0xe4d12a844befc651 as libc::c_ulonglong as uint64_t,
        0x8c509c2765d0ba22 as libc::c_ulonglong as uint64_t,
        0xee8c6018c28814d9 as libc::c_ulonglong as uint64_t,
        0x17da7c1f49a59e31 as libc::c_ulonglong as uint64_t,
        0x609c4c1328e194d3 as libc::c_ulonglong as uint64_t,
        0xb3e3d57232f44b09 as libc::c_ulonglong as uint64_t,
        0x91d7aaa4a512f69b as libc::c_ulonglong as uint64_t,
        0xffd6fd243dabbcc as libc::c_ulonglong as uint64_t,
        0x50d26a943c1fde34 as libc::c_ulonglong as uint64_t,
        0x6be15e9968545b4f as libc::c_ulonglong as uint64_t,
        0x94778fea6faf9fdf as libc::c_ulonglong as uint64_t,
        0x2b09dd7058ea4826 as libc::c_ulonglong as uint64_t,
        0x677cd9716de5c7bf as libc::c_ulonglong as uint64_t,
        0x49d5214fffb2e6dd as libc::c_ulonglong as uint64_t,
        0x360e83a466b273c as libc::c_ulonglong as uint64_t,
        0x1fc786af4f7b7691 as libc::c_ulonglong as uint64_t,
        0xa0b9d435783ea168 as libc::c_ulonglong as uint64_t,
        0xd49f0c035f118cb6 as libc::c_ulonglong as uint64_t,
        0x1205816c9d21d14 as libc::c_ulonglong as uint64_t,
        0xac2453dd7d8f3d98 as libc::c_ulonglong as uint64_t,
        0x545217cc3f70aa64 as libc::c_ulonglong as uint64_t,
        0x26b4028e9489c9c2 as libc::c_ulonglong as uint64_t,
        0xdec2469fd6765e3e as libc::c_ulonglong as uint64_t,
        0x4807d58036f7450 as libc::c_ulonglong as uint64_t,
        0xe5f17292823ddb45 as libc::c_ulonglong as uint64_t,
        0xf30b569b024a5860 as libc::c_ulonglong as uint64_t,
        0x62dcfc3fa758aefb as libc::c_ulonglong as uint64_t,
        0xe84cad6c4e5e5aa1 as libc::c_ulonglong as uint64_t,
        0xccb81fce556ea94b as libc::c_ulonglong as uint64_t,
        0x53b282ae7a74f908 as libc::c_ulonglong as uint64_t,
        0x1b47fbf74c1402c1 as libc::c_ulonglong as uint64_t,
        0x368eebf39828049f as libc::c_ulonglong as uint64_t,
        0x7afbeff2ad278b06 as libc::c_ulonglong as uint64_t,
        0xbe5e0a8cfe97caed as libc::c_ulonglong as uint64_t,
        0xcfd8f7f413058e77 as libc::c_ulonglong as uint64_t,
        0xf78b2bc301252c30 as libc::c_ulonglong as uint64_t,
        0x4d555c17fcdd928d as libc::c_ulonglong as uint64_t,
        0x5f2f05467fc565f8 as libc::c_ulonglong as uint64_t,
        0x24f4b2a21b30f3ea as libc::c_ulonglong as uint64_t,
        0x860dd6bbecb768aa as libc::c_ulonglong as uint64_t,
        0x4c750401350f8f99 as libc::c_ulonglong as uint64_t,
        0 as libc::c_ulonglong as uint64_t,
        0xecccd0344d312ef1 as libc::c_ulonglong as uint64_t,
        0xb5231806be220571 as libc::c_ulonglong as uint64_t,
        0xc105c030990d28af as libc::c_ulonglong as uint64_t,
        0x653c695de25cfd97 as libc::c_ulonglong as uint64_t,
        0x159acc33c61ca419 as libc::c_ulonglong as uint64_t,
        0xb89ec7f872418495 as libc::c_ulonglong as uint64_t,
        0xa9847693b73254dc as libc::c_ulonglong as uint64_t,
        0x58cf90243ac13694 as libc::c_ulonglong as uint64_t,
        0x59efc832f3132b80 as libc::c_ulonglong as uint64_t,
        0x5c4fed7c39ae42c4 as libc::c_ulonglong as uint64_t,
        0x828dabe3efd81cfa as libc::c_ulonglong as uint64_t,
        0xd13f294d95ace5f2 as libc::c_ulonglong as uint64_t,
        0x7d1b7a90e823d86a as libc::c_ulonglong as uint64_t,
        0xb643f03cf849224d as libc::c_ulonglong as uint64_t,
        0x3df3f979d89dcb03 as libc::c_ulonglong as uint64_t,
        0x7426d836272f2dde as libc::c_ulonglong as uint64_t,
        0xdfe21e891fa4432a as libc::c_ulonglong as uint64_t,
        0x3a136c1b9d99986f as libc::c_ulonglong as uint64_t,
        0xfa36f43dcd46add4 as libc::c_ulonglong as uint64_t,
        0xc025982650df35bb as libc::c_ulonglong as uint64_t,
        0x856d3e81aadc4f96 as libc::c_ulonglong as uint64_t,
        0xc4a5e57e53b041eb as libc::c_ulonglong as uint64_t,
        0x4708168b75ba4005 as libc::c_ulonglong as uint64_t,
        0xaf44bbe73be41aa4 as libc::c_ulonglong as uint64_t,
        0x971767d029c4b8e3 as libc::c_ulonglong as uint64_t,
        0xb9be9feebb939981 as libc::c_ulonglong as uint64_t,
        0x215497ecd18d9aae as libc::c_ulonglong as uint64_t,
        0x316e7e91dd2c57f3 as libc::c_ulonglong as uint64_t,
        0xcef8afe2dad79363 as libc::c_ulonglong as uint64_t,
        0x3853dc371220a247 as libc::c_ulonglong as uint64_t,
        0x35ee03c9de4323a3 as libc::c_ulonglong as uint64_t,
        0xe6919aa8c456fc79 as libc::c_ulonglong as uint64_t,
        0xe05157dc4880b201 as libc::c_ulonglong as uint64_t,
        0x7bdbb7e464f59612 as libc::c_ulonglong as uint64_t,
        0x127a59518318f775 as libc::c_ulonglong as uint64_t,
        0x332ecebd52956ddb as libc::c_ulonglong as uint64_t,
        0x8f30741d23bb9d1e as libc::c_ulonglong as uint64_t,
        0xd922d3fd93720d52 as libc::c_ulonglong as uint64_t,
        0x7746300c61440ae2 as libc::c_ulonglong as uint64_t,
        0x25d4eab4d2e2eefe as libc::c_ulonglong as uint64_t,
        0x75068020eefd30ca as libc::c_ulonglong as uint64_t,
        0x135a01474acaea61 as libc::c_ulonglong as uint64_t,
        0x304e268714fe4ae7 as libc::c_ulonglong as uint64_t,
        0xa519f17bb283c82c as libc::c_ulonglong as uint64_t,
        0xdc82f6b359cf6416 as libc::c_ulonglong as uint64_t,
        0x5baf781e7caa11a8 as libc::c_ulonglong as uint64_t,
        0xb2c38d64fb26561d as libc::c_ulonglong as uint64_t,
        0x34ce5bdf17913eb7 as libc::c_ulonglong as uint64_t,
        0x5d6fb56af07c5fd0 as libc::c_ulonglong as uint64_t,
        0x182713cd0a7f25fd as libc::c_ulonglong as uint64_t,
        0x9e2ac576e6c84d57 as libc::c_ulonglong as uint64_t,
        0x9aaab82ee5a73907 as libc::c_ulonglong as uint64_t,
        0xa3d93c0f3e558654 as libc::c_ulonglong as uint64_t,
        0x7e7b92aaae48ff56 as libc::c_ulonglong as uint64_t,
        0x872d8ead256575be as libc::c_ulonglong as uint64_t,
        0x41c8dbfff96c0e7d as libc::c_ulonglong as uint64_t,
        0x99ca5014a3cc1e3b as libc::c_ulonglong as uint64_t,
        0x40e883e930be1369 as libc::c_ulonglong as uint64_t,
        0x1ca76e95091051ad as libc::c_ulonglong as uint64_t,
        0x4e35b42dbab6b5b1 as libc::c_ulonglong as uint64_t,
        0x5a0254ecabd6944 as libc::c_ulonglong as uint64_t,
        0xe1710fca8152af15 as libc::c_ulonglong as uint64_t,
        0xf22b0e8dcb984574 as libc::c_ulonglong as uint64_t,
        0xb763a82a319b3f59 as libc::c_ulonglong as uint64_t,
        0x63fca4296e8ab3ef as libc::c_ulonglong as uint64_t,
        0x9d4a2d4ca0a36a6b as libc::c_ulonglong as uint64_t,
        0xe331bfe60eeb953d as libc::c_ulonglong as uint64_t,
        0xd5bf541596c391a2 as libc::c_ulonglong as uint64_t,
        0xf5cb9bef8e9c1618 as libc::c_ulonglong as uint64_t,
        0x46284e9dbc685d11 as libc::c_ulonglong as uint64_t,
        0x2074cffa185f87ba as libc::c_ulonglong as uint64_t,
        0xbd3ee2b6b8fcedd1 as libc::c_ulonglong as uint64_t,
        0xae64e3f1f23607b0 as libc::c_ulonglong as uint64_t,
        0xfeb68965ce29d984 as libc::c_ulonglong as uint64_t,
        0x55724fdaf6a2b770 as libc::c_ulonglong as uint64_t,
        0x29496d5cd753720e as libc::c_ulonglong as uint64_t,
        0xa75941573d3af204 as libc::c_ulonglong as uint64_t,
        0x8e102c0bea69800a as libc::c_ulonglong as uint64_t,
        0x111ab16bc573d049 as libc::c_ulonglong as uint64_t,
        0xd7ffe439197aab8a as libc::c_ulonglong as uint64_t,
        0xefac380e0b5a09cd as libc::c_ulonglong as uint64_t,
        0x48f579593660fbc9 as libc::c_ulonglong as uint64_t,
        0x22347fd697e6bd92 as libc::c_ulonglong as uint64_t,
        0x61bc1405e13389c7 as libc::c_ulonglong as uint64_t,
        0x4ab5c975b9d9c1e1 as libc::c_ulonglong as uint64_t,
        0x80cd1bcf606126d2 as libc::c_ulonglong as uint64_t,
        0x7186fd78ed92449a as libc::c_ulonglong as uint64_t,
        0x93971a882aabccb3 as libc::c_ulonglong as uint64_t,
        0x88d0e17f66bfce72 as libc::c_ulonglong as uint64_t,
        0x27945a985d5bd4d6 as libc::c_ulonglong as uint64_t,
    ],
    [
        0xde553f8c05a811c8 as libc::c_ulonglong as uint64_t,
        0x1906b59631b4f565 as libc::c_ulonglong as uint64_t,
        0x436e70d6b1964ff7 as libc::c_ulonglong as uint64_t,
        0x36d343cb8b1e9d85 as libc::c_ulonglong as uint64_t,
        0x843dfacc858aab5a as libc::c_ulonglong as uint64_t,
        0xfdfc95c299bfc7f9 as libc::c_ulonglong as uint64_t,
        0xf634bdea1d51fa2 as libc::c_ulonglong as uint64_t,
        0x6d458b3b76efb3cd as libc::c_ulonglong as uint64_t,
        0x85c3f77cf8593f80 as libc::c_ulonglong as uint64_t,
        0x3c91315fbe737cb2 as libc::c_ulonglong as uint64_t,
        0x2148b03366ace398 as libc::c_ulonglong as uint64_t,
        0x18f8b8264c6761bf as libc::c_ulonglong as uint64_t,
        0xc830c1c495c9fb0f as libc::c_ulonglong as uint64_t,
        0x981a76102086a0aa as libc::c_ulonglong as uint64_t,
        0xaa16012142f35760 as libc::c_ulonglong as uint64_t,
        0x35cc54060c763cf6 as libc::c_ulonglong as uint64_t,
        0x42907d66cc45db2d as libc::c_ulonglong as uint64_t,
        0x8203d44b965af4bc as libc::c_ulonglong as uint64_t,
        0x3d6f3cefc3a0e868 as libc::c_ulonglong as uint64_t,
        0xbc73ff69d292bda7 as libc::c_ulonglong as uint64_t,
        0x8722ed0102e20a29 as libc::c_ulonglong as uint64_t,
        0x8f8185e8cd34deb7 as libc::c_ulonglong as uint64_t,
        0x9b0561dda7ee01d9 as libc::c_ulonglong as uint64_t,
        0x5335a0193227fad6 as libc::c_ulonglong as uint64_t,
        0xc9cecc74e81a6fd5 as libc::c_ulonglong as uint64_t,
        0x54f5832e5c2431ea as libc::c_ulonglong as uint64_t,
        0x99e47ba05d553470 as libc::c_ulonglong as uint64_t,
        0xf7bee756acd226ce as libc::c_ulonglong as uint64_t,
        0x384e05a5571816fd as libc::c_ulonglong as uint64_t,
        0xd1367452a47d0e6a as libc::c_ulonglong as uint64_t,
        0xf29fde1c386ad85b as libc::c_ulonglong as uint64_t,
        0x320c77316275f7ca as libc::c_ulonglong as uint64_t,
        0xd0c879e2d9ae9ab0 as libc::c_ulonglong as uint64_t,
        0xdb7406c69110ef5d as libc::c_ulonglong as uint64_t,
        0x45505e51a2461011 as libc::c_ulonglong as uint64_t,
        0xfc029872e46c5323 as libc::c_ulonglong as uint64_t,
        0xfa3cb6f5f7bc0cc5 as libc::c_ulonglong as uint64_t,
        0x31f17cd8768a173 as libc::c_ulonglong as uint64_t,
        0xbd8df2d9af41297d as libc::c_ulonglong as uint64_t,
        0x9d3b4f5ab43e5e3f as libc::c_ulonglong as uint64_t,
        0x4071671b36feee84 as libc::c_ulonglong as uint64_t,
        0x716207e7d3e3b83d as libc::c_ulonglong as uint64_t,
        0x48d20ff2f9283a1a as libc::c_ulonglong as uint64_t,
        0x27769eb4757cbc7e as libc::c_ulonglong as uint64_t,
        0x5c56ebc793f2e574 as libc::c_ulonglong as uint64_t,
        0xa48b474f9ef5dc18 as libc::c_ulonglong as uint64_t,
        0x52cbada94ff46e0c as libc::c_ulonglong as uint64_t,
        0x60c7da982d8199c6 as libc::c_ulonglong as uint64_t,
        0xe9d466edc068b78 as libc::c_ulonglong as uint64_t,
        0x4eec2175eaf865fc as libc::c_ulonglong as uint64_t,
        0x550b8e9e21f7a530 as libc::c_ulonglong as uint64_t,
        0x6b7ba5bc653fec2b as libc::c_ulonglong as uint64_t,
        0x5eb7f1ba6949d0dd as libc::c_ulonglong as uint64_t,
        0x57ea94e3db4c9099 as libc::c_ulonglong as uint64_t,
        0xf640eae6d101b214 as libc::c_ulonglong as uint64_t,
        0xdd4a284182c0b0bb as libc::c_ulonglong as uint64_t,
        0xff1d8fbf6304f250 as libc::c_ulonglong as uint64_t,
        0xb8accb933bf9d7e8 as libc::c_ulonglong as uint64_t,
        0xe8867c478eb68c4d as libc::c_ulonglong as uint64_t,
        0x3f8e2692391bddc1 as libc::c_ulonglong as uint64_t,
        0xcb2fd60912a15a7c as libc::c_ulonglong as uint64_t,
        0xaec935dbab983d2f as libc::c_ulonglong as uint64_t,
        0xf55ffd2b56691367 as libc::c_ulonglong as uint64_t,
        0x80e2ce366ce1c115 as libc::c_ulonglong as uint64_t,
        0x179bf3f8edb27e1d as libc::c_ulonglong as uint64_t,
        0x1fe0db07dd394da as libc::c_ulonglong as uint64_t,
        0xda8a0b76ecc37b87 as libc::c_ulonglong as uint64_t,
        0x44ae53e1df9584cb as libc::c_ulonglong as uint64_t,
        0xb310b4b77347a205 as libc::c_ulonglong as uint64_t,
        0xdfab323c787b8512 as libc::c_ulonglong as uint64_t,
        0x3b511268d070b78e as libc::c_ulonglong as uint64_t,
        0x65e6e3d2b9396753 as libc::c_ulonglong as uint64_t,
        0x6864b271e2574d58 as libc::c_ulonglong as uint64_t,
        0x259784c98fc789d7 as libc::c_ulonglong as uint64_t,
        0x2e11a7dfabb35a9 as libc::c_ulonglong as uint64_t,
        0x8841a6dfa337158b as libc::c_ulonglong as uint64_t,
        0x7ade78c39b5dcdd0 as libc::c_ulonglong as uint64_t,
        0xb7cf804d9a2cc84a as libc::c_ulonglong as uint64_t,
        0x20b6bd831b7f7742 as libc::c_ulonglong as uint64_t,
        0x75bd331d3a88d272 as libc::c_ulonglong as uint64_t,
        0x418f6aab4b2d7a5e as libc::c_ulonglong as uint64_t,
        0xd9951cbb6babdaf4 as libc::c_ulonglong as uint64_t,
        0xb6318dfde7ff5c90 as libc::c_ulonglong as uint64_t,
        0x1f389b112264aa83 as libc::c_ulonglong as uint64_t,
        0x492c024284fbaec0 as libc::c_ulonglong as uint64_t,
        0xe33a0363c608f9a0 as libc::c_ulonglong as uint64_t,
        0x2688930408af28a4 as libc::c_ulonglong as uint64_t,
        0xc7538a1a341ce4ad as libc::c_ulonglong as uint64_t,
        0x5da8e677ee2171ae as libc::c_ulonglong as uint64_t,
        0x8c9e92254a5c7fc4 as libc::c_ulonglong as uint64_t,
        0x63d8cd55aae938b5 as libc::c_ulonglong as uint64_t,
        0x29ebd8daa97a3706 as libc::c_ulonglong as uint64_t,
        0x959827b37be88aa1 as libc::c_ulonglong as uint64_t,
        0x1484e4356adadf6e as libc::c_ulonglong as uint64_t,
        0xa7945082199d7d6b as libc::c_ulonglong as uint64_t,
        0xbf6ce8a455fa1cd4 as libc::c_ulonglong as uint64_t,
        0x9cc542eac9edcae5 as libc::c_ulonglong as uint64_t,
        0x79c16f0e1c356ca3 as libc::c_ulonglong as uint64_t,
        0x89bfab6fdee48151 as libc::c_ulonglong as uint64_t,
        0xd4174d1830c5f0ff as libc::c_ulonglong as uint64_t,
        0x9258048415eb419d as libc::c_ulonglong as uint64_t,
        0x6139d72850520d1c as libc::c_ulonglong as uint64_t,
        0x6a85a80c18ec78f1 as libc::c_ulonglong as uint64_t,
        0xcd11f88e0171059a as libc::c_ulonglong as uint64_t,
        0xcceff53e7ca29140 as libc::c_ulonglong as uint64_t,
        0xd229639f2315af19 as libc::c_ulonglong as uint64_t,
        0x90b91ef9ef507434 as libc::c_ulonglong as uint64_t,
        0x5977d28d074a1be1 as libc::c_ulonglong as uint64_t,
        0x311360fce51d56b9 as libc::c_ulonglong as uint64_t,
        0xc093a92d5a1f2f91 as libc::c_ulonglong as uint64_t,
        0x1a19a25bb6dc5416 as libc::c_ulonglong as uint64_t,
        0xeb996b8a09de2d3e as libc::c_ulonglong as uint64_t,
        0xfee3820f1ed7668a as libc::c_ulonglong as uint64_t,
        0xd7085ad5b7ad518c as libc::c_ulonglong as uint64_t,
        0x7fff41890fe53345 as libc::c_ulonglong as uint64_t,
        0xec5948bd67dde602 as libc::c_ulonglong as uint64_t,
        0x2fd5f65dbaaa68e0 as libc::c_ulonglong as uint64_t,
        0xa5754affe32648c2 as libc::c_ulonglong as uint64_t,
        0xf8ddac880d07396c as libc::c_ulonglong as uint64_t,
        0x6fa491468c548664 as libc::c_ulonglong as uint64_t,
        0xc7c5c1326bdbed1 as libc::c_ulonglong as uint64_t,
        0x4a33158f03930fb3 as libc::c_ulonglong as uint64_t,
        0x699abfc19f84d982 as libc::c_ulonglong as uint64_t,
        0xe4fa2054a80b329c as libc::c_ulonglong as uint64_t,
        0x6707f9af438252fa as libc::c_ulonglong as uint64_t,
        0x8a368e9cfd6d49e as libc::c_ulonglong as uint64_t,
        0x47b1442c58fd25b8 as libc::c_ulonglong as uint64_t,
        0xbbb3dc5ebc91769b as libc::c_ulonglong as uint64_t,
        0x1665fe489061eac7 as libc::c_ulonglong as uint64_t,
        0x33f27a811fa66310 as libc::c_ulonglong as uint64_t,
        0x93a609346838d547 as libc::c_ulonglong as uint64_t,
        0x30ed6d4c98cec263 as libc::c_ulonglong as uint64_t,
        0x1dd9816cd8df9f2a as libc::c_ulonglong as uint64_t,
        0x94662a03063b1e7b as libc::c_ulonglong as uint64_t,
        0x83fdd9fbeb896066 as libc::c_ulonglong as uint64_t,
        0x7b207573e68e590a as libc::c_ulonglong as uint64_t,
        0x5f49fc0a149a4407 as libc::c_ulonglong as uint64_t,
        0x343259b671a5a82c as libc::c_ulonglong as uint64_t,
        0xfbc2bb458a6f981f as libc::c_ulonglong as uint64_t,
        0xc272b350a0a41a38 as libc::c_ulonglong as uint64_t,
        0x3aaf1fd8ada32354 as libc::c_ulonglong as uint64_t,
        0x6cbb868b0b3c2717 as libc::c_ulonglong as uint64_t,
        0xa2b569c88d2583fe as libc::c_ulonglong as uint64_t,
        0xf180c9d1bf027928 as libc::c_ulonglong as uint64_t,
        0xaf37386bd64ba9f5 as libc::c_ulonglong as uint64_t,
        0x12bacab2790a8088 as libc::c_ulonglong as uint64_t,
        0x4c0d3b0810435055 as libc::c_ulonglong as uint64_t,
        0xb2eeb9070e9436df as libc::c_ulonglong as uint64_t,
        0xc5b29067cea7d104 as libc::c_ulonglong as uint64_t,
        0xdcb425f1ff132461 as libc::c_ulonglong as uint64_t,
        0x4f122cc5972bf126 as libc::c_ulonglong as uint64_t,
        0xac282fa651230886 as libc::c_ulonglong as uint64_t,
        0xe7e537992f6393ef as libc::c_ulonglong as uint64_t,
        0xe61b3a2952b00735 as libc::c_ulonglong as uint64_t,
        0x709c0a57ae302ce7 as libc::c_ulonglong as uint64_t,
        0xe02514ae416058d3 as libc::c_ulonglong as uint64_t,
        0xc44c9dd7b37445de as libc::c_ulonglong as uint64_t,
        0x5a68c5408022ba92 as libc::c_ulonglong as uint64_t,
        0x1c278cdca50c0bf0 as libc::c_ulonglong as uint64_t,
        0x6e5a9cf6f18712be as libc::c_ulonglong as uint64_t,
        0x86dce0b17f319ef3 as libc::c_ulonglong as uint64_t,
        0x2d34ec2040115d49 as libc::c_ulonglong as uint64_t,
        0x4bcd183f7e409b69 as libc::c_ulonglong as uint64_t,
        0x2815d56ad4a9a3dc as libc::c_ulonglong as uint64_t,
        0x24698979f2141d0d as libc::c_ulonglong as uint64_t,
        0 as libc::c_ulonglong as uint64_t,
        0x1ec696a15fb73e59 as libc::c_ulonglong as uint64_t,
        0xd86b110b16784e2e as libc::c_ulonglong as uint64_t,
        0x8e7f8858b0e74a6d as libc::c_ulonglong as uint64_t,
        0x63e2e8713d05fe6 as libc::c_ulonglong as uint64_t,
        0xe2c40ed3bbdb6d7a as libc::c_ulonglong as uint64_t,
        0xb1f1aeca89fc97ac as libc::c_ulonglong as uint64_t,
        0xe1db191e3cb3cc09 as libc::c_ulonglong as uint64_t,
        0x6418ee62c4eaf389 as libc::c_ulonglong as uint64_t,
        0xc6ad87aa49cf7077 as libc::c_ulonglong as uint64_t,
        0xd6f65765ca7ec556 as libc::c_ulonglong as uint64_t,
        0x9afb6c6dda3d9503 as libc::c_ulonglong as uint64_t,
        0x7ce05644888d9236 as libc::c_ulonglong as uint64_t,
        0x8d609f95378feb1e as libc::c_ulonglong as uint64_t,
        0x23a9aa4e9c17d631 as libc::c_ulonglong as uint64_t,
        0x6226c0e5d73aac6f as libc::c_ulonglong as uint64_t,
        0x56149953a69f0443 as libc::c_ulonglong as uint64_t,
        0xeeb852c09d66d3ab as libc::c_ulonglong as uint64_t,
        0x2b0ac2a753c102af as libc::c_ulonglong as uint64_t,
        0x7c023376e03cb3c as libc::c_ulonglong as uint64_t,
        0x2ccae1903dc2c993 as libc::c_ulonglong as uint64_t,
        0xd3d76e2f5ec63bc3 as libc::c_ulonglong as uint64_t,
        0x9e2458973356ff4c as libc::c_ulonglong as uint64_t,
        0xa66a5d32644ee9b1 as libc::c_ulonglong as uint64_t,
        0xa427294356de137 as libc::c_ulonglong as uint64_t,
        0x783f62be61e6f879 as libc::c_ulonglong as uint64_t,
        0x1344c70204d91452 as libc::c_ulonglong as uint64_t,
        0x5b96c8f0fdf12e48 as libc::c_ulonglong as uint64_t,
        0xa90916ecc59bf613 as libc::c_ulonglong as uint64_t,
        0xbe92e5142829880e as libc::c_ulonglong as uint64_t,
        0x727d102a548b194e as libc::c_ulonglong as uint64_t,
        0x1be7afebcb0fc0cc as libc::c_ulonglong as uint64_t,
        0x3e702b2244c8491b as libc::c_ulonglong as uint64_t,
        0xd5e940a84d166425 as libc::c_ulonglong as uint64_t,
        0x66f9f41f3e51c620 as libc::c_ulonglong as uint64_t,
        0xabe80c913f20c3ba as libc::c_ulonglong as uint64_t,
        0xf07ec461c2d1edf2 as libc::c_ulonglong as uint64_t,
        0xf361d3ac45b94c81 as libc::c_ulonglong as uint64_t,
        0x521394a94b8fe95 as libc::c_ulonglong as uint64_t,
        0xadd622162cf09c5c as libc::c_ulonglong as uint64_t,
        0xe97871f7f3651897 as libc::c_ulonglong as uint64_t,
        0xf4a1f09b2bba87bd as libc::c_ulonglong as uint64_t,
        0x95d6559b2054044 as libc::c_ulonglong as uint64_t,
        0xbbc7f2448be75ed as libc::c_ulonglong as uint64_t,
        0x2af4cf172e129675 as libc::c_ulonglong as uint64_t,
        0x157ae98517094bb4 as libc::c_ulonglong as uint64_t,
        0x9fda55274e856b96 as libc::c_ulonglong as uint64_t,
        0x914713499283e0ee as libc::c_ulonglong as uint64_t,
        0xb952c623462a4332 as libc::c_ulonglong as uint64_t,
        0x74433ead475b46a8 as libc::c_ulonglong as uint64_t,
        0x8b5eb112245fb4f8 as libc::c_ulonglong as uint64_t,
        0xa34b6478f0f61724 as libc::c_ulonglong as uint64_t,
        0x11a5dd7ffe6221fb as libc::c_ulonglong as uint64_t,
        0xc16da49d27ccbb4b as libc::c_ulonglong as uint64_t,
        0x76a224d0bde07301 as libc::c_ulonglong as uint64_t,
        0x8aa0bca2598c2022 as libc::c_ulonglong as uint64_t,
        0x4df336b86d90c48f as libc::c_ulonglong as uint64_t,
        0xea67663a740db9e4 as libc::c_ulonglong as uint64_t,
        0xef465f70e0b54771 as libc::c_ulonglong as uint64_t,
        0x39b008152acb8227 as libc::c_ulonglong as uint64_t,
        0x7d1e5bf4f55e06ec as libc::c_ulonglong as uint64_t,
        0x105bd0cf83b1b521 as libc::c_ulonglong as uint64_t,
        0x775c2960c033e7db as libc::c_ulonglong as uint64_t,
        0x7e014c397236a79f as libc::c_ulonglong as uint64_t,
        0x811cc386113255cf as libc::c_ulonglong as uint64_t,
        0xeda7450d1a0e72d8 as libc::c_ulonglong as uint64_t,
        0x5889df3d7a998f3b as libc::c_ulonglong as uint64_t,
        0x2e2bfbedc779fc3a as libc::c_ulonglong as uint64_t,
        0xce0eef438619a4e9 as libc::c_ulonglong as uint64_t,
        0x372d4e7bf6cd095f as libc::c_ulonglong as uint64_t,
        0x4df34fae96b6a4f as libc::c_ulonglong as uint64_t,
        0xf923a13870d4adb6 as libc::c_ulonglong as uint64_t,
        0xa1aa7e050a4d228d as libc::c_ulonglong as uint64_t,
        0xa8f71b5cb84862c9 as libc::c_ulonglong as uint64_t,
        0xb52e9a306097fde3 as libc::c_ulonglong as uint64_t,
        0xd8251a35b6e2a0b as libc::c_ulonglong as uint64_t,
        0x2257a7fee1c442eb as libc::c_ulonglong as uint64_t,
        0x73831d9a29588d94 as libc::c_ulonglong as uint64_t,
        0x51d4ba64c89ccf7f as libc::c_ulonglong as uint64_t,
        0x502ab7d4b54f5ba5 as libc::c_ulonglong as uint64_t,
        0x97793dce8153bf08 as libc::c_ulonglong as uint64_t,
        0xe5042de4d5d8a646 as libc::c_ulonglong as uint64_t,
        0x9687307efc802bd2 as libc::c_ulonglong as uint64_t,
        0xa05473b5779eb657 as libc::c_ulonglong as uint64_t,
        0xb4d097801d446939 as libc::c_ulonglong as uint64_t,
        0xcff0e2f3fbca3033 as libc::c_ulonglong as uint64_t,
        0xc38cbee0dd778ee2 as libc::c_ulonglong as uint64_t,
        0x464f499c252eb162 as libc::c_ulonglong as uint64_t,
        0xcad1dbb96f72cea6 as libc::c_ulonglong as uint64_t,
        0xba4dd1eec142e241 as libc::c_ulonglong as uint64_t,
        0xb00fa37af42f0376 as libc::c_ulonglong as uint64_t,
    ],
    [
        0xcce4cd3aa968b245 as libc::c_ulonglong as uint64_t,
        0x89d5484e80b7faf as libc::c_ulonglong as uint64_t,
        0x638246c1b3548304 as libc::c_ulonglong as uint64_t,
        0xd2fe0ec8c2355492 as libc::c_ulonglong as uint64_t,
        0xa7fbdf7ff2374eee as libc::c_ulonglong as uint64_t,
        0x4df1600c92337a16 as libc::c_ulonglong as uint64_t,
        0x84e503ea523b12fb as libc::c_ulonglong as uint64_t,
        0x790bbfd53ab0c4a as libc::c_ulonglong as uint64_t,
        0x198a780f38f6ea9d as libc::c_ulonglong as uint64_t,
        0x2ab30c8f55ec48cb as libc::c_ulonglong as uint64_t,
        0xe0f7fed6b2c49db5 as libc::c_ulonglong as uint64_t,
        0xb6ecf3f422cadbdc as libc::c_ulonglong as uint64_t,
        0x409c9a541358df11 as libc::c_ulonglong as uint64_t,
        0xd3ce8a56dfde3fe3 as libc::c_ulonglong as uint64_t,
        0xc3e9224312c8c1a0 as libc::c_ulonglong as uint64_t,
        0xd6dfa58816ba507 as libc::c_ulonglong as uint64_t,
        0xddf3e1b179952777 as libc::c_ulonglong as uint64_t,
        0x4c02a42748bb1d9 as libc::c_ulonglong as uint64_t,
        0x94c2abff9f2decb8 as libc::c_ulonglong as uint64_t,
        0x4f91752da8f8acf4 as libc::c_ulonglong as uint64_t,
        0x78682befb169bf7b as libc::c_ulonglong as uint64_t,
        0xe1c77a48af2ff6c4 as libc::c_ulonglong as uint64_t,
        0xc5d7ec69c80ce76 as libc::c_ulonglong as uint64_t,
        0x4cc1e4928fd81167 as libc::c_ulonglong as uint64_t,
        0xfeed3d24d9997b62 as libc::c_ulonglong as uint64_t,
        0x518bb6dfc3a54a23 as libc::c_ulonglong as uint64_t,
        0x6dbf2d26151f9b90 as libc::c_ulonglong as uint64_t,
        0xb5bc624b05ea664f as libc::c_ulonglong as uint64_t,
        0xe86aaa525acfe21a as libc::c_ulonglong as uint64_t,
        0x4801ced0fb53a0be as libc::c_ulonglong as uint64_t,
        0xc91463e6c00868ed as libc::c_ulonglong as uint64_t,
        0x1027a815cd16fe43 as libc::c_ulonglong as uint64_t,
        0xf67069a0319204cd as libc::c_ulonglong as uint64_t,
        0xb04ccc976c8abce7 as libc::c_ulonglong as uint64_t,
        0xc0b9b3fc35e87c33 as libc::c_ulonglong as uint64_t,
        0xf380c77c58f2de65 as libc::c_ulonglong as uint64_t,
        0x50bb3241de4e2152 as libc::c_ulonglong as uint64_t,
        0xdf93f490435ef195 as libc::c_ulonglong as uint64_t,
        0xf1e0d25d62390887 as libc::c_ulonglong as uint64_t,
        0xaf668bfb1a3c3141 as libc::c_ulonglong as uint64_t,
        0xbc11b251f00a7291 as libc::c_ulonglong as uint64_t,
        0x73a5eed47e427d47 as libc::c_ulonglong as uint64_t,
        0x25bee3f6ee4c3b2e as libc::c_ulonglong as uint64_t,
        0x43cc0beb34786282 as libc::c_ulonglong as uint64_t,
        0xc824e778dde3039c as libc::c_ulonglong as uint64_t,
        0xf97d86d98a327728 as libc::c_ulonglong as uint64_t,
        0xf2b043e24519b514 as libc::c_ulonglong as uint64_t,
        0xe297ebf7880f4b57 as libc::c_ulonglong as uint64_t,
        0x3a94a49a98fab688 as libc::c_ulonglong as uint64_t,
        0x868516cb68f0c419 as libc::c_ulonglong as uint64_t,
        0xeffa11af0964ee50 as libc::c_ulonglong as uint64_t,
        0xa4ab4ec0d517f37d as libc::c_ulonglong as uint64_t,
        0xa9c6b498547c567a as libc::c_ulonglong as uint64_t,
        0x8e18424f80fbbbb6 as libc::c_ulonglong as uint64_t,
        0xbcdc53bcf2bc23c as libc::c_ulonglong as uint64_t,
        0x137739aaea3643d0 as libc::c_ulonglong as uint64_t,
        0x2c1333ec1bac2ff0 as libc::c_ulonglong as uint64_t,
        0x8d48d3f0a7db0625 as libc::c_ulonglong as uint64_t,
        0x1e1ac3f26b5de6d7 as libc::c_ulonglong as uint64_t,
        0xf520f81f16b2b95e as libc::c_ulonglong as uint64_t,
        0x9f0f6ec450062e84 as libc::c_ulonglong as uint64_t,
        0x130849e1deb6b71 as libc::c_ulonglong as uint64_t,
        0xd45e31ab8c7533a9 as libc::c_ulonglong as uint64_t,
        0x652279a2fd14e43f as libc::c_ulonglong as uint64_t,
        0x3209f01e70f1c927 as libc::c_ulonglong as uint64_t,
        0xbe71a770cac1a473 as libc::c_ulonglong as uint64_t,
        0xe3d6be7a64b1894 as libc::c_ulonglong as uint64_t,
        0x7ec8148cff29d840 as libc::c_ulonglong as uint64_t,
        0xcb7476c7fac3be0f as libc::c_ulonglong as uint64_t,
        0x72956a4a63a91636 as libc::c_ulonglong as uint64_t,
        0x37f95ec21991138f as libc::c_ulonglong as uint64_t,
        0x9e3fea5a4ded45f5 as libc::c_ulonglong as uint64_t,
        0x7b38ba50964902e8 as libc::c_ulonglong as uint64_t,
        0x222e580bbde73764 as libc::c_ulonglong as uint64_t,
        0x61e253e0899f55e6 as libc::c_ulonglong as uint64_t,
        0xfc8d2805e352ad80 as libc::c_ulonglong as uint64_t,
        0x35994be3235ac56d as libc::c_ulonglong as uint64_t,
        0x9add01af5e014de as libc::c_ulonglong as uint64_t,
        0x5e8659a6780539c6 as libc::c_ulonglong as uint64_t,
        0xb17c48097161d796 as libc::c_ulonglong as uint64_t,
        0x26015213acbd6e2 as libc::c_ulonglong as uint64_t,
        0xd1ae9f77e515e901 as libc::c_ulonglong as uint64_t,
        0xb7dc776a3f21b0ad as libc::c_ulonglong as uint64_t,
        0xaba6a1b96eb78098 as libc::c_ulonglong as uint64_t,
        0x9bcf4486248d9f5d as libc::c_ulonglong as uint64_t,
        0x582666c536455efd as libc::c_ulonglong as uint64_t,
        0xfdbdac9bfeb9c6f1 as libc::c_ulonglong as uint64_t,
        0xc47999be4163cdea as libc::c_ulonglong as uint64_t,
        0x765540081722a7ef as libc::c_ulonglong as uint64_t,
        0x3e548ed8ec710751 as libc::c_ulonglong as uint64_t,
        0x3d041f67cb51bac2 as libc::c_ulonglong as uint64_t,
        0x7958af71ac82d40a as libc::c_ulonglong as uint64_t,
        0x36c9da5c047a78fe as libc::c_ulonglong as uint64_t,
        0xed9a048e33af38b2 as libc::c_ulonglong as uint64_t,
        0x26ee7249c96c86bd as libc::c_ulonglong as uint64_t,
        0x900281bdeba65d61 as libc::c_ulonglong as uint64_t,
        0x11172c8bd0fd9532 as libc::c_ulonglong as uint64_t,
        0xea0abf73600434f8 as libc::c_ulonglong as uint64_t,
        0x42fc8f75299309f3 as libc::c_ulonglong as uint64_t,
        0x34a9cf7d3eb1ae1c as libc::c_ulonglong as uint64_t,
        0x2b838811480723ba as libc::c_ulonglong as uint64_t,
        0x5ce64c8742ceef24 as libc::c_ulonglong as uint64_t,
        0x1adae9b01fd6570e as libc::c_ulonglong as uint64_t,
        0x3c349bf9d6bad1b3 as libc::c_ulonglong as uint64_t,
        0x82453c891c7b75c0 as libc::c_ulonglong as uint64_t,
        0x97923a40b80d512b as libc::c_ulonglong as uint64_t,
        0x4a61dbf1c198765c as libc::c_ulonglong as uint64_t,
        0xb48ce6d518010d3e as libc::c_ulonglong as uint64_t,
        0xcfb45c858e480fd6 as libc::c_ulonglong as uint64_t,
        0xd933cbf30d1e96ae as libc::c_ulonglong as uint64_t,
        0xd70ea014ab558e3a as libc::c_ulonglong as uint64_t,
        0xc189376228031742 as libc::c_ulonglong as uint64_t,
        0x9262949cd16d8b83 as libc::c_ulonglong as uint64_t,
        0xeb3a3bed7def5f89 as libc::c_ulonglong as uint64_t,
        0x49314a4ee6b8cbcf as libc::c_ulonglong as uint64_t,
        0xdcc3652f647e4c06 as libc::c_ulonglong as uint64_t,
        0xda635a4c2a3e2b3d as libc::c_ulonglong as uint64_t,
        0x470c21a940f3d35b as libc::c_ulonglong as uint64_t,
        0x315961a157d174b4 as libc::c_ulonglong as uint64_t,
        0x6672e81dda3459ac as libc::c_ulonglong as uint64_t,
        0x5b76f77a1165e36e as libc::c_ulonglong as uint64_t,
        0x445cb01667d36ec8 as libc::c_ulonglong as uint64_t,
        0xc5491d205c88a69b as libc::c_ulonglong as uint64_t,
        0x456c34887a3805b9 as libc::c_ulonglong as uint64_t,
        0xffddb9bac4721013 as libc::c_ulonglong as uint64_t,
        0x99af51a71e4649bf as libc::c_ulonglong as uint64_t,
        0xa15be01cbc7729d5 as libc::c_ulonglong as uint64_t,
        0x52db2760e485f7b0 as libc::c_ulonglong as uint64_t,
        0x8c78576eba306d54 as libc::c_ulonglong as uint64_t,
        0xae560f6507d75a30 as libc::c_ulonglong as uint64_t,
        0x95f22f6182c687c9 as libc::c_ulonglong as uint64_t,
        0x71c5fbf54489aba5 as libc::c_ulonglong as uint64_t,
        0xca44f259e728d57e as libc::c_ulonglong as uint64_t,
        0x88b87d2ccebbdc8d as libc::c_ulonglong as uint64_t,
        0xbab18d32be4a15aa as libc::c_ulonglong as uint64_t,
        0x8be8ec93e99b611e as libc::c_ulonglong as uint64_t,
        0x17b713e89ebdf209 as libc::c_ulonglong as uint64_t,
        0xb31c5d284baa0174 as libc::c_ulonglong as uint64_t,
        0xeeca9531148f8521 as libc::c_ulonglong as uint64_t,
        0xb8d198138481c348 as libc::c_ulonglong as uint64_t,
        0x8988f9b2d350b7fc as libc::c_ulonglong as uint64_t,
        0xb9e11c8d996aa839 as libc::c_ulonglong as uint64_t,
        0x5a4673e40c8e881f as libc::c_ulonglong as uint64_t,
        0x1687977683569978 as libc::c_ulonglong as uint64_t,
        0xbf4123eed72acf02 as libc::c_ulonglong as uint64_t,
        0x4ea1f1b3b513c785 as libc::c_ulonglong as uint64_t,
        0xe767452be16f91ff as libc::c_ulonglong as uint64_t,
        0x7505d1b730021a7c as libc::c_ulonglong as uint64_t,
        0xa59bca5ec8fc980c as libc::c_ulonglong as uint64_t,
        0xad069eda20f7e7a3 as libc::c_ulonglong as uint64_t,
        0x38f4b1bba231606a as libc::c_ulonglong as uint64_t,
        0x60d2d77e94743e97 as libc::c_ulonglong as uint64_t,
        0x9affc0183966f42c as libc::c_ulonglong as uint64_t,
        0x248e6768f3a7505f as libc::c_ulonglong as uint64_t,
        0xcdd449a4b483d934 as libc::c_ulonglong as uint64_t,
        0x87b59255751baf68 as libc::c_ulonglong as uint64_t,
        0x1bea6d2e023d3c7f as libc::c_ulonglong as uint64_t,
        0x6b1f12455b5ffcab as libc::c_ulonglong as uint64_t,
        0x743555292de9710d as libc::c_ulonglong as uint64_t,
        0xd8034f6d10f5fddf as libc::c_ulonglong as uint64_t,
        0xc6198c9f7ba81b08 as libc::c_ulonglong as uint64_t,
        0xbb8109aca3a17edb as libc::c_ulonglong as uint64_t,
        0xfa2d1766ad12cabb as libc::c_ulonglong as uint64_t,
        0xc729080166437079 as libc::c_ulonglong as uint64_t,
        0x9c5fff7b77269317 as libc::c_ulonglong as uint64_t,
        0 as libc::c_ulonglong as uint64_t,
        0x15d706c9a47624eb as libc::c_ulonglong as uint64_t,
        0x6fdf38072fd44d72 as libc::c_ulonglong as uint64_t,
        0x5fb6dd3865ee52b7 as libc::c_ulonglong as uint64_t,
        0xa33bf53d86bcff37 as libc::c_ulonglong as uint64_t,
        0xe657c1b5fc84fa8e as libc::c_ulonglong as uint64_t,
        0xaa962527735cebe9 as libc::c_ulonglong as uint64_t,
        0x39c43525bfda0b1b as libc::c_ulonglong as uint64_t,
        0x204e4d2a872ce186 as libc::c_ulonglong as uint64_t,
        0x7a083ece8ba26999 as libc::c_ulonglong as uint64_t,
        0x554b9c9db72efbfa as libc::c_ulonglong as uint64_t,
        0xb22cd9b656416a05 as libc::c_ulonglong as uint64_t,
        0x96a2bedea5e63a5a as libc::c_ulonglong as uint64_t,
        0x802529a826b0a322 as libc::c_ulonglong as uint64_t,
        0x8115ad363b5bc853 as libc::c_ulonglong as uint64_t,
        0x8375b81701901eb1 as libc::c_ulonglong as uint64_t,
        0x3069e53f4a3a1fc5 as libc::c_ulonglong as uint64_t,
        0xbd2136cfede119e0 as libc::c_ulonglong as uint64_t,
        0x18bafc91251d81ec as libc::c_ulonglong as uint64_t,
        0x1d4a524d4c7d5b44 as libc::c_ulonglong as uint64_t,
        0x5f0aedc6960daa8 as libc::c_ulonglong as uint64_t,
        0x29e39d3072ccf558 as libc::c_ulonglong as uint64_t,
        0x70f57f6b5962c0d4 as libc::c_ulonglong as uint64_t,
        0x989fd53903ad22ce as libc::c_ulonglong as uint64_t,
        0xf84d024797d91c59 as libc::c_ulonglong as uint64_t,
        0x547b1803aac5908b as libc::c_ulonglong as uint64_t,
        0xf0d056c37fd263f6 as libc::c_ulonglong as uint64_t,
        0xd56eb535919e58d8 as libc::c_ulonglong as uint64_t,
        0x1c7ad6d351963035 as libc::c_ulonglong as uint64_t,
        0x2e7326cd2167f912 as libc::c_ulonglong as uint64_t,
        0xac361a443d1c8cd2 as libc::c_ulonglong as uint64_t,
        0x697f076461942a49 as libc::c_ulonglong as uint64_t,
        0x4b515f6fdc731d2d as libc::c_ulonglong as uint64_t,
        0x8ad8680df4700a6f as libc::c_ulonglong as uint64_t,
        0x41ac1eca0eb3b460 as libc::c_ulonglong as uint64_t,
        0x7d988533d80965d3 as libc::c_ulonglong as uint64_t,
        0xa8f6300649973d0b as libc::c_ulonglong as uint64_t,
        0x7765c4960ac9cc9e as libc::c_ulonglong as uint64_t,
        0x7ca801adc5e20ea2 as libc::c_ulonglong as uint64_t,
        0xdea3700e5eb59ae4 as libc::c_ulonglong as uint64_t,
        0xa06b6482a19c42a4 as libc::c_ulonglong as uint64_t,
        0x6a2f96db46b497da as libc::c_ulonglong as uint64_t,
        0x27def6d7d487edcc as libc::c_ulonglong as uint64_t,
        0x463ca5375d18b82a as libc::c_ulonglong as uint64_t,
        0xa6cb5be1efdc259f as libc::c_ulonglong as uint64_t,
        0x53eba3fef96e9cc1 as libc::c_ulonglong as uint64_t,
        0xce84d81b93a364a7 as libc::c_ulonglong as uint64_t,
        0xf4107c810b59d22f as libc::c_ulonglong as uint64_t,
        0x333974806d1aa256 as libc::c_ulonglong as uint64_t,
        0xf0def79bba073e5 as libc::c_ulonglong as uint64_t,
        0x231edc95a00c5c15 as libc::c_ulonglong as uint64_t,
        0xe437d494c64f2c6c as libc::c_ulonglong as uint64_t,
        0x91320523f64d3610 as libc::c_ulonglong as uint64_t,
        0x67426c83c7df32dd as libc::c_ulonglong as uint64_t,
        0x6eefbc99323f2603 as libc::c_ulonglong as uint64_t,
        0x9d6f7be56acdf866 as libc::c_ulonglong as uint64_t,
        0x5916e25b2bae358c as libc::c_ulonglong as uint64_t,
        0x7ff89012e2c2b331 as libc::c_ulonglong as uint64_t,
        0x35091bf2720bd93 as libc::c_ulonglong as uint64_t,
        0x561b0d22900e4669 as libc::c_ulonglong as uint64_t,
        0x28d319ae6f279e29 as libc::c_ulonglong as uint64_t,
        0x2f43a2533c8c9263 as libc::c_ulonglong as uint64_t,
        0xd09e1be9f8fe8270 as libc::c_ulonglong as uint64_t,
        0xf740ed3e2c796fbc as libc::c_ulonglong as uint64_t,
        0xdb53ded237d5404c as libc::c_ulonglong as uint64_t,
        0x62b2c25faebfe875 as libc::c_ulonglong as uint64_t,
        0xafd41a5d2c0a94d as libc::c_ulonglong as uint64_t,
        0x6412fd3ce0ff8f4e as libc::c_ulonglong as uint64_t,
        0xe3a76f6995e42026 as libc::c_ulonglong as uint64_t,
        0x6c8fa9b808f4f0e1 as libc::c_ulonglong as uint64_t,
        0xc2d9a6dd0f23aad1 as libc::c_ulonglong as uint64_t,
        0x8f28c6d19d10d0c7 as libc::c_ulonglong as uint64_t,
        0x85d587744fd0798a as libc::c_ulonglong as uint64_t,
        0xa20b71a39b579446 as libc::c_ulonglong as uint64_t,
        0x684f83fa7c7f4138 as libc::c_ulonglong as uint64_t,
        0xe507500adba4471d as libc::c_ulonglong as uint64_t,
        0x3f640a46f19a6c20 as libc::c_ulonglong as uint64_t,
        0x1247bd34f7dd28a1 as libc::c_ulonglong as uint64_t,
        0x2d23b77206474481 as libc::c_ulonglong as uint64_t,
        0x93521002cc86e0f2 as libc::c_ulonglong as uint64_t,
        0x572b89bc8de52d18 as libc::c_ulonglong as uint64_t,
        0xfb1d93f8b0f9a1ca as libc::c_ulonglong as uint64_t,
        0xe95a2ecc4724896b as libc::c_ulonglong as uint64_t,
        0x3ba420048511ddf9 as libc::c_ulonglong as uint64_t,
        0xd63e248ab6bee54b as libc::c_ulonglong as uint64_t,
        0x5dd6c8195f258455 as libc::c_ulonglong as uint64_t,
        0x6a03f634e40673b as libc::c_ulonglong as uint64_t,
        0x1f2a476c76b68da6 as libc::c_ulonglong as uint64_t,
        0x217ec9b49ac78af7 as libc::c_ulonglong as uint64_t,
        0xecaa80102e4453c3 as libc::c_ulonglong as uint64_t,
        0x14e78257b99d4f9a as libc::c_ulonglong as uint64_t,
    ],
    [
        0x20329b2cc87bba05 as libc::c_ulonglong as uint64_t,
        0x4f5eb6f86546a531 as libc::c_ulonglong as uint64_t,
        0xd4f44775f751b6b1 as libc::c_ulonglong as uint64_t,
        0x8266a47b850dfa8b as libc::c_ulonglong as uint64_t,
        0xbb986aa15a6ca985 as libc::c_ulonglong as uint64_t,
        0xc979eb08f9ae0f99 as libc::c_ulonglong as uint64_t,
        0x2da6f447a2375ea1 as libc::c_ulonglong as uint64_t,
        0x1e74275dcd7d8576 as libc::c_ulonglong as uint64_t,
        0xbc20180a800bc5f8 as libc::c_ulonglong as uint64_t,
        0xb4a2f701b2dc65be as libc::c_ulonglong as uint64_t,
        0xe726946f981b6d66 as libc::c_ulonglong as uint64_t,
        0x48e6c453bf21c94c as libc::c_ulonglong as uint64_t,
        0x42cad9930f0a4195 as libc::c_ulonglong as uint64_t,
        0xefa47b64aacccd20 as libc::c_ulonglong as uint64_t,
        0x71180a8960409a42 as libc::c_ulonglong as uint64_t,
        0x8bb3329bf6a44e0c as libc::c_ulonglong as uint64_t,
        0xd34c35de2d36dacc as libc::c_ulonglong as uint64_t,
        0xa92f5b7cbc23dc96 as libc::c_ulonglong as uint64_t,
        0xb31a85aa68bb09c3 as libc::c_ulonglong as uint64_t,
        0x13e04836a73161d2 as libc::c_ulonglong as uint64_t,
        0xb24dfc4129c51d02 as libc::c_ulonglong as uint64_t,
        0x8ae44b70b7da5acd as libc::c_ulonglong as uint64_t,
        0xe671ed84d96579a7 as libc::c_ulonglong as uint64_t,
        0xa4bb3417d66f3832 as libc::c_ulonglong as uint64_t,
        0x4572ab38d56d2de8 as libc::c_ulonglong as uint64_t,
        0xb1b47761ea47215c as libc::c_ulonglong as uint64_t,
        0xe81c09cf70aba15d as libc::c_ulonglong as uint64_t,
        0xffbdb872ce7f90ac as libc::c_ulonglong as uint64_t,
        0xa8782297fd5dc857 as libc::c_ulonglong as uint64_t,
        0xd946f6b6a4ce4a4 as libc::c_ulonglong as uint64_t,
        0xe4df1f4f5b995138 as libc::c_ulonglong as uint64_t,
        0x9ebc71edca8c5762 as libc::c_ulonglong as uint64_t,
        0xa2c1dc0b02b88d9 as libc::c_ulonglong as uint64_t,
        0x3b503c115d9d7b91 as libc::c_ulonglong as uint64_t,
        0xc64376a8111ec3a2 as libc::c_ulonglong as uint64_t,
        0xcec199a323c963e4 as libc::c_ulonglong as uint64_t,
        0xdc76a87ec58616f7 as libc::c_ulonglong as uint64_t,
        0x9d596e073a9b487 as libc::c_ulonglong as uint64_t,
        0x14583a9d7d560daf as libc::c_ulonglong as uint64_t,
        0xf4c6dc593f2a0cb4 as libc::c_ulonglong as uint64_t,
        0xdd21d19584f80236 as libc::c_ulonglong as uint64_t,
        0x4a4836983ddde1d3 as libc::c_ulonglong as uint64_t,
        0xe58866a41ae745f9 as libc::c_ulonglong as uint64_t,
        0xf591a5b27e541875 as libc::c_ulonglong as uint64_t,
        0x891dc05074586693 as libc::c_ulonglong as uint64_t,
        0x5b068c651810a89e as libc::c_ulonglong as uint64_t,
        0xa30346bc0c08544f as libc::c_ulonglong as uint64_t,
        0x3dbf3751c684032d as libc::c_ulonglong as uint64_t,
        0x2a1e86ec785032dc as libc::c_ulonglong as uint64_t,
        0xf73f5779fca830ea as libc::c_ulonglong as uint64_t,
        0xb60c05ca30204d21 as libc::c_ulonglong as uint64_t,
        0xcc316802b32f065 as libc::c_ulonglong as uint64_t,
        0x8770241bdd96be69 as libc::c_ulonglong as uint64_t,
        0xb861e18199ee95db as libc::c_ulonglong as uint64_t,
        0xf805cad91418fcd1 as libc::c_ulonglong as uint64_t,
        0x29e70dccbbd20e82 as libc::c_ulonglong as uint64_t,
        0xc7140f435060d763 as libc::c_ulonglong as uint64_t,
        0xf3a9da0e8b0cc3b as libc::c_ulonglong as uint64_t,
        0xa2543f574d76408e as libc::c_ulonglong as uint64_t,
        0xbd7761e1c175d139 as libc::c_ulonglong as uint64_t,
        0x4b1f4f737ca3f512 as libc::c_ulonglong as uint64_t,
        0x6dc2df1f2fc137ab as libc::c_ulonglong as uint64_t,
        0xf1d05c3967b14856 as libc::c_ulonglong as uint64_t,
        0xa742bf3715ed046c as libc::c_ulonglong as uint64_t,
        0x654030141d1697ed as libc::c_ulonglong as uint64_t,
        0x7b872abda676c7d as libc::c_ulonglong as uint64_t,
        0x3ce84eba87fa17ec as libc::c_ulonglong as uint64_t,
        0xc1fb0403cb79afdf as libc::c_ulonglong as uint64_t,
        0x3e46bc7105063f73 as libc::c_ulonglong as uint64_t,
        0x278ae987121cd678 as libc::c_ulonglong as uint64_t,
        0xa1adb4778ef47cd0 as libc::c_ulonglong as uint64_t,
        0x26dd906c5362c2b9 as libc::c_ulonglong as uint64_t,
        0x5168060589b44e2 as libc::c_ulonglong as uint64_t,
        0xfbfc41f9d79ac08f as libc::c_ulonglong as uint64_t,
        0xe6de44ba9ced8fa as libc::c_ulonglong as uint64_t,
        0x9feb08068bf243a3 as libc::c_ulonglong as uint64_t,
        0x7b341749d06b129b as libc::c_ulonglong as uint64_t,
        0x229c69e74a87929a as libc::c_ulonglong as uint64_t,
        0xe09ee6c4427c011b as libc::c_ulonglong as uint64_t,
        0x5692e30e725c4c3a as libc::c_ulonglong as uint64_t,
        0xda99a33e5e9f6e4b as libc::c_ulonglong as uint64_t,
        0x353dd85af453a36b as libc::c_ulonglong as uint64_t,
        0x25241b4c90e0fee7 as libc::c_ulonglong as uint64_t,
        0x5de987258309d022 as libc::c_ulonglong as uint64_t,
        0xe230140fc0802984 as libc::c_ulonglong as uint64_t,
        0x93281e86a0c0b3c6 as libc::c_ulonglong as uint64_t,
        0xf229d719a4337408 as libc::c_ulonglong as uint64_t,
        0x6f6c2dd4ad3d1f34 as libc::c_ulonglong as uint64_t,
        0x8ea5b2fbae3f0aee as libc::c_ulonglong as uint64_t,
        0x8331dd90c473ee4a as libc::c_ulonglong as uint64_t,
        0x346aa1b1b52db7aa as libc::c_ulonglong as uint64_t,
        0xdf8f235e06042aa9 as libc::c_ulonglong as uint64_t,
        0xcc6f6b68a1354b7b as libc::c_ulonglong as uint64_t,
        0x6c95a6f46ebf236a as libc::c_ulonglong as uint64_t,
        0x52d31a856bb91c19 as libc::c_ulonglong as uint64_t,
        0x1a35ded6d498d555 as libc::c_ulonglong as uint64_t,
        0xf37eaef2e54d60c9 as libc::c_ulonglong as uint64_t,
        0x72e181a9a3c2a61c as libc::c_ulonglong as uint64_t,
        0x98537aad51952fde as libc::c_ulonglong as uint64_t,
        0x16f6c856ffaa2530 as libc::c_ulonglong as uint64_t,
        0xd960281e9d1d5215 as libc::c_ulonglong as uint64_t,
        0x3a0745fa1ce36f50 as libc::c_ulonglong as uint64_t,
        0xb7b642bf1559c18 as libc::c_ulonglong as uint64_t,
        0x59a87eae9aec8001 as libc::c_ulonglong as uint64_t,
        0x5e100c05408bec7c as libc::c_ulonglong as uint64_t,
        0x441f98b19e55023 as libc::c_ulonglong as uint64_t,
        0xd70dcc5534d38aef as libc::c_ulonglong as uint64_t,
        0x927f676de1bea707 as libc::c_ulonglong as uint64_t,
        0x9769e70db925e3e5 as libc::c_ulonglong as uint64_t,
        0x7a636ea29115065a as libc::c_ulonglong as uint64_t,
        0x468b201816ef11b6 as libc::c_ulonglong as uint64_t,
        0xab81a9b73edff409 as libc::c_ulonglong as uint64_t,
        0xc0ac7de88a07bb1e as libc::c_ulonglong as uint64_t,
        0x1f235eb68c0391b7 as libc::c_ulonglong as uint64_t,
        0x6056b074458dd30f as libc::c_ulonglong as uint64_t,
        0xbe8eeac102f7ed67 as libc::c_ulonglong as uint64_t,
        0xcd381283e04b5fba as libc::c_ulonglong as uint64_t,
        0x5cbefecec277c4e3 as libc::c_ulonglong as uint64_t,
        0xd21b4c356c48ce0d as libc::c_ulonglong as uint64_t,
        0x1019c31664b35d8c as libc::c_ulonglong as uint64_t,
        0x247362a7d19eea26 as libc::c_ulonglong as uint64_t,
        0xebe582efb3299d03 as libc::c_ulonglong as uint64_t,
        0x2aef2cb82fc289f as libc::c_ulonglong as uint64_t,
        0x86275df09ce8aaa8 as libc::c_ulonglong as uint64_t,
        0x28b07427faac1a43 as libc::c_ulonglong as uint64_t,
        0x38a9b7319e1f47cf as libc::c_ulonglong as uint64_t,
        0xc82e92e3b8d01b58 as libc::c_ulonglong as uint64_t,
        0x6ef0b409b1978bc as libc::c_ulonglong as uint64_t,
        0x62f842bfc771fb90 as libc::c_ulonglong as uint64_t,
        0x9904034610eb3b1f as libc::c_ulonglong as uint64_t,
        0xded85ab5477a3e68 as libc::c_ulonglong as uint64_t,
        0x90d195a663428f98 as libc::c_ulonglong as uint64_t,
        0x5384636e2ac708d8 as libc::c_ulonglong as uint64_t,
        0xcbd719c37b522706 as libc::c_ulonglong as uint64_t,
        0xae9729d76644b0eb as libc::c_ulonglong as uint64_t,
        0x7c8c65e20a0c7ee6 as libc::c_ulonglong as uint64_t,
        0x80c856b007f1d214 as libc::c_ulonglong as uint64_t,
        0x8c0b40302cc32271 as libc::c_ulonglong as uint64_t,
        0xdbcedad51fe17a8a as libc::c_ulonglong as uint64_t,
        0x740e8ae938dbdea0 as libc::c_ulonglong as uint64_t,
        0xa615c6dc549310ad as libc::c_ulonglong as uint64_t,
        0x19cc55f6171ae90b as libc::c_ulonglong as uint64_t,
        0x49b1bdb8fe5fdd8d as libc::c_ulonglong as uint64_t,
        0xed0a89af2830e5bf as libc::c_ulonglong as uint64_t,
        0x6a7aadb4f5a65bd6 as libc::c_ulonglong as uint64_t,
        0x7e22972988f05679 as libc::c_ulonglong as uint64_t,
        0xf952b3325566e810 as libc::c_ulonglong as uint64_t,
        0x39fecedadf61530e as libc::c_ulonglong as uint64_t,
        0x6101c99f04f3c7ce as libc::c_ulonglong as uint64_t,
        0x2e5f7f6761b562ff as libc::c_ulonglong as uint64_t,
        0xf08725d226cf5c97 as libc::c_ulonglong as uint64_t,
        0x63af3b54860fef51 as libc::c_ulonglong as uint64_t,
        0x8ff2cb10ef411e2f as libc::c_ulonglong as uint64_t,
        0x884ab9bb35267252 as libc::c_ulonglong as uint64_t,
        0x4df04433e7ba8dae as libc::c_ulonglong as uint64_t,
        0x9afd8866d3690741 as libc::c_ulonglong as uint64_t,
        0x66b9bb34de94abb3 as libc::c_ulonglong as uint64_t,
        0x9baaf18d92171380 as libc::c_ulonglong as uint64_t,
        0x543c11c5f0a064a5 as libc::c_ulonglong as uint64_t,
        0x17a1b1bdbed431f1 as libc::c_ulonglong as uint64_t,
        0xb5f58eeaf3a2717f as libc::c_ulonglong as uint64_t,
        0xc355f6c849858740 as libc::c_ulonglong as uint64_t,
        0xec5df044694ef17e as libc::c_ulonglong as uint64_t,
        0xd83751f5dc6346d4 as libc::c_ulonglong as uint64_t,
        0xfc4433520dfdacf2 as libc::c_ulonglong as uint64_t,
        0 as libc::c_ulonglong as uint64_t,
        0x5a51f58e596ebc5f as libc::c_ulonglong as uint64_t,
        0x3285aaf12e34cf16 as libc::c_ulonglong as uint64_t,
        0x8d5c39db6dbd36b0 as libc::c_ulonglong as uint64_t,
        0x12b731dde64f7513 as libc::c_ulonglong as uint64_t,
        0x94906c2d7aa7dfbb as libc::c_ulonglong as uint64_t,
        0x302b583aacc8e789 as libc::c_ulonglong as uint64_t,
        0x9d45facd090e6b3c as libc::c_ulonglong as uint64_t,
        0x2165e2c78905aec4 as libc::c_ulonglong as uint64_t,
        0x68d45f7f775a7349 as libc::c_ulonglong as uint64_t,
        0x189b2c1d5664fdca as libc::c_ulonglong as uint64_t,
        0xe1c99f2f030215da as libc::c_ulonglong as uint64_t,
        0x6983269436246788 as libc::c_ulonglong as uint64_t,
        0x8489af3b1e148237 as libc::c_ulonglong as uint64_t,
        0xe94b702431d5b59c as libc::c_ulonglong as uint64_t,
        0x33d2d31a6f4adbd7 as libc::c_ulonglong as uint64_t,
        0xbfd9932a4389f9a6 as libc::c_ulonglong as uint64_t,
        0xb0e30e8aab39359d as libc::c_ulonglong as uint64_t,
        0xd1e2c715afcaf253 as libc::c_ulonglong as uint64_t,
        0x150f43763c28196e as libc::c_ulonglong as uint64_t,
        0xc4ed846393e2eb3d as libc::c_ulonglong as uint64_t,
        0x3f98b20c3823c5e as libc::c_ulonglong as uint64_t,
        0xfd134ab94c83b833 as libc::c_ulonglong as uint64_t,
        0x556b682eb1de7064 as libc::c_ulonglong as uint64_t,
        0x36c4537a37d19f35 as libc::c_ulonglong as uint64_t,
        0x7559f30279a5ca61 as libc::c_ulonglong as uint64_t,
        0x799ae58252973a04 as libc::c_ulonglong as uint64_t,
        0x9c12832648707ffd as libc::c_ulonglong as uint64_t,
        0x78cd9c6913e92ec5 as libc::c_ulonglong as uint64_t,
        0x1d8dac7d0effb928 as libc::c_ulonglong as uint64_t,
        0x439da0784e745554 as libc::c_ulonglong as uint64_t,
        0x413352b3cc887dcb as libc::c_ulonglong as uint64_t,
        0xbacf134a1b12bd44 as libc::c_ulonglong as uint64_t,
        0x114ebafd25cd494d as libc::c_ulonglong as uint64_t,
        0x2f08068c20cb763e as libc::c_ulonglong as uint64_t,
        0x76a07822ba27f63f as libc::c_ulonglong as uint64_t,
        0xeab2fb04f25789c2 as libc::c_ulonglong as uint64_t,
        0xe3676de481fe3d45 as libc::c_ulonglong as uint64_t,
        0x1b62a73d95e6c194 as libc::c_ulonglong as uint64_t,
        0x641749ff5c68832c as libc::c_ulonglong as uint64_t,
        0xa5ec4dfc97112cf3 as libc::c_ulonglong as uint64_t,
        0xf6682e92bdd6242b as libc::c_ulonglong as uint64_t,
        0x3f11c59a44782bb2 as libc::c_ulonglong as uint64_t,
        0x317c21d1edb6f348 as libc::c_ulonglong as uint64_t,
        0xd65ab5be75ad9e2e as libc::c_ulonglong as uint64_t,
        0x6b2dd45fb4d84f17 as libc::c_ulonglong as uint64_t,
        0xfaab381296e4d44e as libc::c_ulonglong as uint64_t,
        0xd0b5befeeeb4e692 as libc::c_ulonglong as uint64_t,
        0x882ef0b32d7a046 as libc::c_ulonglong as uint64_t,
        0x512a91a5a83b2047 as libc::c_ulonglong as uint64_t,
        0x963e9ee6f85bf724 as libc::c_ulonglong as uint64_t,
        0x4e09cf132438b1f0 as libc::c_ulonglong as uint64_t,
        0x77f701c9fb59e2fe as libc::c_ulonglong as uint64_t,
        0x7ddb1c094b726a27 as libc::c_ulonglong as uint64_t,
        0x5f4775ee01f5f8bd as libc::c_ulonglong as uint64_t,
        0x9186ec4d223c9b59 as libc::c_ulonglong as uint64_t,
        0xfeeac1998f01846d as libc::c_ulonglong as uint64_t,
        0xac39db1ce4b89874 as libc::c_ulonglong as uint64_t,
        0xb75b7c21715e59e0 as libc::c_ulonglong as uint64_t,
        0xafc0503c273aa42a as libc::c_ulonglong as uint64_t,
        0x6e3b543fec430bf5 as libc::c_ulonglong as uint64_t,
        0x704f7362213e8e83 as libc::c_ulonglong as uint64_t,
        0x58ff0745db9294c0 as libc::c_ulonglong as uint64_t,
        0x67eec2df9feabf72 as libc::c_ulonglong as uint64_t,
        0xa0facd9ccf8a6811 as libc::c_ulonglong as uint64_t,
        0xb936986ad890811a as libc::c_ulonglong as uint64_t,
        0x95c715c63bd9cb7a as libc::c_ulonglong as uint64_t,
        0xca8060283a2c33c7 as libc::c_ulonglong as uint64_t,
        0x507de84ee9453486 as libc::c_ulonglong as uint64_t,
        0x85ded6d05f6a96f6 as libc::c_ulonglong as uint64_t,
        0x1cdad5964f81ade9 as libc::c_ulonglong as uint64_t,
        0xd5a33e9eb62fa270 as libc::c_ulonglong as uint64_t,
        0x40642b588df6690a as libc::c_ulonglong as uint64_t,
        0x7f75eec2c98e42b8 as libc::c_ulonglong as uint64_t,
        0x2cf18dace3494a60 as libc::c_ulonglong as uint64_t,
        0x23cb100c0bf9865b as libc::c_ulonglong as uint64_t,
        0xeef3028febb2d9e1 as libc::c_ulonglong as uint64_t,
        0x4425d2d394133929 as libc::c_ulonglong as uint64_t,
        0xaad6d05c7fa1e0c8 as libc::c_ulonglong as uint64_t,
        0xad6ea2f7a5c68cb5 as libc::c_ulonglong as uint64_t,
        0xc2028f2308fb9381 as libc::c_ulonglong as uint64_t,
        0x819f2f5b468fc6d5 as libc::c_ulonglong as uint64_t,
        0xc5bafd88d29cfffc as libc::c_ulonglong as uint64_t,
        0x47dc59f357910577 as libc::c_ulonglong as uint64_t,
        0x2b49ff07392e261d as libc::c_ulonglong as uint64_t,
        0x57c59ae5332258fb as libc::c_ulonglong as uint64_t,
        0x73b6f842e2bcb2dd as libc::c_ulonglong as uint64_t,
        0xcf96e04862b77725 as libc::c_ulonglong as uint64_t,
        0x4ca73dd8a6c4996f as libc::c_ulonglong as uint64_t,
        0x15779eb417e14c1 as libc::c_ulonglong as uint64_t,
        0x37932a9176af8bf4 as libc::c_ulonglong as uint64_t,
    ],
    [
        0x190a2c9b249df23e as libc::c_ulonglong as uint64_t,
        0x2f62f8b62263e1e9 as libc::c_ulonglong as uint64_t,
        0x7a7f754740993655 as libc::c_ulonglong as uint64_t,
        0x330b7ba4d5564d9f as libc::c_ulonglong as uint64_t,
        0x4c17a16a46672582 as libc::c_ulonglong as uint64_t,
        0xb22f08eb7d05f5b8 as libc::c_ulonglong as uint64_t,
        0x535f47f40bc148cc as libc::c_ulonglong as uint64_t,
        0x3aec5d27d4883037 as libc::c_ulonglong as uint64_t,
        0x10ed0a1825438f96 as libc::c_ulonglong as uint64_t,
        0x516101f72c233d17 as libc::c_ulonglong as uint64_t,
        0x13cc6f949fd04eae as libc::c_ulonglong as uint64_t,
        0x739853c441474bfd as libc::c_ulonglong as uint64_t,
        0x653793d90d3f5b1b as libc::c_ulonglong as uint64_t,
        0x5240647b96b0fc2f as libc::c_ulonglong as uint64_t,
        0xc84890ad27623e0 as libc::c_ulonglong as uint64_t,
        0xd7189b32703aaea3 as libc::c_ulonglong as uint64_t,
        0x2685de3523bd9c41 as libc::c_ulonglong as uint64_t,
        0x99317c5b11bffefa as libc::c_ulonglong as uint64_t,
        0xd9baa854f079703 as libc::c_ulonglong as uint64_t,
        0x70b93648fbd48ac5 as libc::c_ulonglong as uint64_t,
        0xa80441fce30bc6be as libc::c_ulonglong as uint64_t,
        0x7287704bdc36ff1e as libc::c_ulonglong as uint64_t,
        0xb65384ed33dc1f13 as libc::c_ulonglong as uint64_t,
        0xd36417343ee34408 as libc::c_ulonglong as uint64_t,
        0x39cd38ab6e1bf10f as libc::c_ulonglong as uint64_t,
        0x5ab861770a1f3564 as libc::c_ulonglong as uint64_t,
        0xebacf09f594563b as libc::c_ulonglong as uint64_t,
        0xd04572b884708530 as libc::c_ulonglong as uint64_t,
        0x3cae9722bdb3af47 as libc::c_ulonglong as uint64_t,
        0x4a556b6f2f5cbaf2 as libc::c_ulonglong as uint64_t,
        0xe1704f1f76c4bd74 as libc::c_ulonglong as uint64_t,
        0x5ec4ed7144c6dfcf as libc::c_ulonglong as uint64_t,
        0x16afc01d4c7810e6 as libc::c_ulonglong as uint64_t,
        0x283f113cd629ca7a as libc::c_ulonglong as uint64_t,
        0xaf59a8761741ed2d as libc::c_ulonglong as uint64_t,
        0xeed5a3991e215fac as libc::c_ulonglong as uint64_t,
        0x3bf37ea849f984d4 as libc::c_ulonglong as uint64_t,
        0xe413e096a56ce33c as libc::c_ulonglong as uint64_t,
        0x2c439d3a98f020d1 as libc::c_ulonglong as uint64_t,
        0x637559dc6404c46b as libc::c_ulonglong as uint64_t,
        0x9e6c95d1e5f5d569 as libc::c_ulonglong as uint64_t,
        0x24bb9836045fe99a as libc::c_ulonglong as uint64_t,
        0x44efa466dac8ecc9 as libc::c_ulonglong as uint64_t,
        0xc6eab2a5c80895d6 as libc::c_ulonglong as uint64_t,
        0x803b50c035220cc4 as libc::c_ulonglong as uint64_t,
        0x321658cba93c138 as libc::c_ulonglong as uint64_t,
        0x8f9ebc465dc7ee1c as libc::c_ulonglong as uint64_t,
        0xd15a5137190131d3 as libc::c_ulonglong as uint64_t,
        0xfa5ec8668e5e2d8 as libc::c_ulonglong as uint64_t,
        0x91c979578d1037b1 as libc::c_ulonglong as uint64_t,
        0x642ca05693b9f70 as libc::c_ulonglong as uint64_t,
        0xefca80168350eb4f as libc::c_ulonglong as uint64_t,
        0x38d21b24f36a45ec as libc::c_ulonglong as uint64_t,
        0xbeab81e1af73d658 as libc::c_ulonglong as uint64_t,
        0x8cbfd9cae7542f24 as libc::c_ulonglong as uint64_t,
        0xfd19cc0d81f11102 as libc::c_ulonglong as uint64_t,
        0xac6430fbb4dbc90 as libc::c_ulonglong as uint64_t,
        0x1d76a09d6a441895 as libc::c_ulonglong as uint64_t,
        0x2a01573ff1cbbfa1 as libc::c_ulonglong as uint64_t,
        0xb572e161894fde2b as libc::c_ulonglong as uint64_t,
        0x8124734fa853b827 as libc::c_ulonglong as uint64_t,
        0x614b1fdf43e6b1b0 as libc::c_ulonglong as uint64_t,
        0x68ac395c4238cc18 as libc::c_ulonglong as uint64_t,
        0x21d837bfd7f7b7d2 as libc::c_ulonglong as uint64_t,
        0x20c714304a860331 as libc::c_ulonglong as uint64_t,
        0x5cfaab726324aa14 as libc::c_ulonglong as uint64_t,
        0x74c5ba4eb50d606e as libc::c_ulonglong as uint64_t,
        0xf3a3030474654739 as libc::c_ulonglong as uint64_t,
        0x23e671bcf015c209 as libc::c_ulonglong as uint64_t,
        0x45f087e947b9582a as libc::c_ulonglong as uint64_t,
        0xd8bd77b418df4c7b as libc::c_ulonglong as uint64_t,
        0xe06f6c90ebb50997 as libc::c_ulonglong as uint64_t,
        0xbd96080263c0873 as libc::c_ulonglong as uint64_t,
        0x7e03f9410e40dcfe as libc::c_ulonglong as uint64_t,
        0xb8e94be4c6484928 as libc::c_ulonglong as uint64_t,
        0xfb5b0608e8ca8e72 as libc::c_ulonglong as uint64_t,
        0x1a2b49179e0e3306 as libc::c_ulonglong as uint64_t,
        0x4e29e76961855059 as libc::c_ulonglong as uint64_t,
        0x4f36c4e6fcf4e4ba as libc::c_ulonglong as uint64_t,
        0x49740ee395cf7bca as libc::c_ulonglong as uint64_t,
        0xc2963ea386d17f7d as libc::c_ulonglong as uint64_t,
        0x90d65ad810618352 as libc::c_ulonglong as uint64_t,
        0x12d34c1b02a1fa4d as libc::c_ulonglong as uint64_t,
        0xfa44258775bb3a91 as libc::c_ulonglong as uint64_t,
        0x18150f14b9ec46dd as libc::c_ulonglong as uint64_t,
        0x1491861e6b9a653d as libc::c_ulonglong as uint64_t,
        0x9a1019d7ab2c3fc2 as libc::c_ulonglong as uint64_t,
        0x3668d42d06fe13d7 as libc::c_ulonglong as uint64_t,
        0xdcc1fbb25606a6d0 as libc::c_ulonglong as uint64_t,
        0x969490dd795a1c22 as libc::c_ulonglong as uint64_t,
        0x3549b1a1bc6dd2ef as libc::c_ulonglong as uint64_t,
        0xc94f5e23a0ed770e as libc::c_ulonglong as uint64_t,
        0xb9f6686b5b39fdcb as libc::c_ulonglong as uint64_t,
        0xc4d4f4a6efeae00d as libc::c_ulonglong as uint64_t,
        0xe732851a1fff2204 as libc::c_ulonglong as uint64_t,
        0x94aad6de5eb869f9 as libc::c_ulonglong as uint64_t,
        0x3f8ff2ae07206e7f as libc::c_ulonglong as uint64_t,
        0xfe38a9813b62d03a as libc::c_ulonglong as uint64_t,
        0xa7a1ad7a8bee2466 as libc::c_ulonglong as uint64_t,
        0x7b6056c8dde882b6 as libc::c_ulonglong as uint64_t,
        0x302a1e286fc58ca7 as libc::c_ulonglong as uint64_t,
        0x8da0fa457a259bc7 as libc::c_ulonglong as uint64_t,
        0xb3302b64e074415b as libc::c_ulonglong as uint64_t,
        0x5402ae7eff8b635f as libc::c_ulonglong as uint64_t,
        0x8f8050c9cafc94b as libc::c_ulonglong as uint64_t,
        0xae468bf98a3059ce as libc::c_ulonglong as uint64_t,
        0x88c355cca98dc58f as libc::c_ulonglong as uint64_t,
        0xb10e6d67c7963480 as libc::c_ulonglong as uint64_t,
        0xbad70de7e1aa3cf3 as libc::c_ulonglong as uint64_t,
        0xbfb4a26e320262bb as libc::c_ulonglong as uint64_t,
        0xcb711820870f02d5 as libc::c_ulonglong as uint64_t,
        0xce12b7a954a75c9d as libc::c_ulonglong as uint64_t,
        0x563ce87dd8691684 as libc::c_ulonglong as uint64_t,
        0x9f73b65e7884618a as libc::c_ulonglong as uint64_t,
        0x2b1e74b06cba0b42 as libc::c_ulonglong as uint64_t,
        0x47cec1ea605b2df1 as libc::c_ulonglong as uint64_t,
        0x1c698312f735ac76 as libc::c_ulonglong as uint64_t,
        0x5fdbcefed9b76b2c as libc::c_ulonglong as uint64_t,
        0x831a354c8fb1cdfc as libc::c_ulonglong as uint64_t,
        0x820516c312c0791f as libc::c_ulonglong as uint64_t,
        0xb74ca762aeadabf0 as libc::c_ulonglong as uint64_t,
        0xfc06ef821c80a5e1 as libc::c_ulonglong as uint64_t,
        0x5723cbf24518a267 as libc::c_ulonglong as uint64_t,
        0x9d4df05d5f661451 as libc::c_ulonglong as uint64_t,
        0x588627742dfd40bf as libc::c_ulonglong as uint64_t,
        0xda8331b73f3d39a0 as libc::c_ulonglong as uint64_t,
        0x17b0e392d109a405 as libc::c_ulonglong as uint64_t,
        0xf965400bcf28fba9 as libc::c_ulonglong as uint64_t,
        0x7c3dbf4229a2a925 as libc::c_ulonglong as uint64_t,
        0x23e460327e275db as libc::c_ulonglong as uint64_t,
        0x6cd0b55a0ce126b3 as libc::c_ulonglong as uint64_t,
        0xe62da695828e96e7 as libc::c_ulonglong as uint64_t,
        0x42ad6e63b3f373b9 as libc::c_ulonglong as uint64_t,
        0xe50cc319381d57df as libc::c_ulonglong as uint64_t,
        0xc5cbd729729b54ee as libc::c_ulonglong as uint64_t,
        0x46d1e265fd2a9912 as libc::c_ulonglong as uint64_t,
        0x6428b056904eeff8 as libc::c_ulonglong as uint64_t,
        0x8be23040131e04b7 as libc::c_ulonglong as uint64_t,
        0x6709d5da2add2ec0 as libc::c_ulonglong as uint64_t,
        0x75de98af44a2b93 as libc::c_ulonglong as uint64_t,
        0x8447dcc67bfbe66f as libc::c_ulonglong as uint64_t,
        0x6616f655b7ac9a23 as libc::c_ulonglong as uint64_t,
        0xd607b8bded4b1a40 as libc::c_ulonglong as uint64_t,
        0x563af89d3a85e48 as libc::c_ulonglong as uint64_t,
        0x3db1b4ad20c21ba4 as libc::c_ulonglong as uint64_t,
        0x11f22997b8323b75 as libc::c_ulonglong as uint64_t,
        0x292032b34b587e99 as libc::c_ulonglong as uint64_t,
        0x7f1cdace9331681d as libc::c_ulonglong as uint64_t,
        0x8e819fc9c0b65aff as libc::c_ulonglong as uint64_t,
        0xa1e3677fe2d5bb16 as libc::c_ulonglong as uint64_t,
        0xcd33d225ee349da5 as libc::c_ulonglong as uint64_t,
        0xd9a2543b85aef898 as libc::c_ulonglong as uint64_t,
        0x795e10cbfa0af76d as libc::c_ulonglong as uint64_t,
        0x25a4bbb9992e5d79 as libc::c_ulonglong as uint64_t,
        0x78413344677b438e as libc::c_ulonglong as uint64_t,
        0xf0826688cef68601 as libc::c_ulonglong as uint64_t,
        0xd27b34bba392f0eb as libc::c_ulonglong as uint64_t,
        0x551d8df162fad7bc as libc::c_ulonglong as uint64_t,
        0x1e57c511d0d7d9ad as libc::c_ulonglong as uint64_t,
        0xdeffbdb171e4d30b as libc::c_ulonglong as uint64_t,
        0xf4feea8e802f6caa as libc::c_ulonglong as uint64_t,
        0xa480c8f6317de55e as libc::c_ulonglong as uint64_t,
        0xa0fc44f07fa40ff5 as libc::c_ulonglong as uint64_t,
        0x95b5f551c3c9dd1a as libc::c_ulonglong as uint64_t,
        0x22f952336d6476ea as libc::c_ulonglong as uint64_t,
        0 as libc::c_ulonglong as uint64_t,
        0xa6be8ef5169f9085 as libc::c_ulonglong as uint64_t,
        0xcc2cf1aa73452946 as libc::c_ulonglong as uint64_t,
        0x2e7ddb39bf12550a as libc::c_ulonglong as uint64_t,
        0xd526dd3157d8db78 as libc::c_ulonglong as uint64_t,
        0x486b2d6c08becf29 as libc::c_ulonglong as uint64_t,
        0x9b0f3a58365d8b21 as libc::c_ulonglong as uint64_t,
        0xac78cdfaadd22c15 as libc::c_ulonglong as uint64_t,
        0xbc95c7e28891a383 as libc::c_ulonglong as uint64_t,
        0x6a927f5f65dab9c3 as libc::c_ulonglong as uint64_t,
        0xc3891d2c1ba0cb9e as libc::c_ulonglong as uint64_t,
        0xeaa92f9f50f8b507 as libc::c_ulonglong as uint64_t,
        0xcf0d9426c9d6e87e as libc::c_ulonglong as uint64_t,
        0xca6e3baf1a7eb636 as libc::c_ulonglong as uint64_t,
        0xab25247059980786 as libc::c_ulonglong as uint64_t,
        0x69b31ad3df4978fb as libc::c_ulonglong as uint64_t,
        0xe2512a93cc577c4c as libc::c_ulonglong as uint64_t,
        0xff278a0ea61364d9 as libc::c_ulonglong as uint64_t,
        0x71a615c766a53e26 as libc::c_ulonglong as uint64_t,
        0x89dc764334fc716c as libc::c_ulonglong as uint64_t,
        0xf87a638452594f4a as libc::c_ulonglong as uint64_t,
        0xf2bc208be914f3da as libc::c_ulonglong as uint64_t,
        0x8766b94ac1682757 as libc::c_ulonglong as uint64_t,
        0xbbc82e687cdb8810 as libc::c_ulonglong as uint64_t,
        0x626a7a53f9757088 as libc::c_ulonglong as uint64_t,
        0xa2c202f358467a2e as libc::c_ulonglong as uint64_t,
        0x4d0882e5db169161 as libc::c_ulonglong as uint64_t,
        0x9e7268301de7da8 as libc::c_ulonglong as uint64_t,
        0xe897699c771ac0dc as libc::c_ulonglong as uint64_t,
        0xc8507dac3d9cc3ed as libc::c_ulonglong as uint64_t,
        0xc0a878a0a1330aa6 as libc::c_ulonglong as uint64_t,
        0x978bb352e42ba8c1 as libc::c_ulonglong as uint64_t,
        0xe9884a13ea6b743f as libc::c_ulonglong as uint64_t,
        0x279afdbabecc28a2 as libc::c_ulonglong as uint64_t,
        0x47c8c064ed9eaab as libc::c_ulonglong as uint64_t,
        0x507e2278b15289f4 as libc::c_ulonglong as uint64_t,
        0x599904fbb08cf45c as libc::c_ulonglong as uint64_t,
        0xbd8ae46d15e01760 as libc::c_ulonglong as uint64_t,
        0x31353da7f2b43844 as libc::c_ulonglong as uint64_t,
        0x8558ff49e68a528c as libc::c_ulonglong as uint64_t,
        0x76fbfc4d92ef15b5 as libc::c_ulonglong as uint64_t,
        0x3456922e211c660c as libc::c_ulonglong as uint64_t,
        0x86799ac55c1993b4 as libc::c_ulonglong as uint64_t,
        0x3e90d1219a51da9c as libc::c_ulonglong as uint64_t,
        0x2d5cbeb505819432 as libc::c_ulonglong as uint64_t,
        0x982e5fd48cce4a19 as libc::c_ulonglong as uint64_t,
        0xdb9c1238a24c8d43 as libc::c_ulonglong as uint64_t,
        0xd439febecaa96f9b as libc::c_ulonglong as uint64_t,
        0x418c0bef0960b281 as libc::c_ulonglong as uint64_t,
        0x158ea591f6ebd1de as libc::c_ulonglong as uint64_t,
        0x1f48e69e4da66d4e as libc::c_ulonglong as uint64_t,
        0x8afd13cf8e6fb054 as libc::c_ulonglong as uint64_t,
        0xf5e1c9011d5ed849 as libc::c_ulonglong as uint64_t,
        0xe34e091c5126c8af as libc::c_ulonglong as uint64_t,
        0xad67ee7530a398f6 as libc::c_ulonglong as uint64_t,
        0x43b24dec2e82c75a as libc::c_ulonglong as uint64_t,
        0x75da99c1287cd48d as libc::c_ulonglong as uint64_t,
        0x92e81cdb3783f689 as libc::c_ulonglong as uint64_t,
        0xa3dd217cc537cecd as libc::c_ulonglong as uint64_t,
        0x60543c50de970553 as libc::c_ulonglong as uint64_t,
        0x93f73f54aaf2426a as libc::c_ulonglong as uint64_t,
        0xa91b62737e7a725d as libc::c_ulonglong as uint64_t,
        0xf19d4507538732e2 as libc::c_ulonglong as uint64_t,
        0x77e4dfc20f9ea156 as libc::c_ulonglong as uint64_t,
        0x7d229ccdb4d31dc6 as libc::c_ulonglong as uint64_t,
        0x1b346a98037f87e5 as libc::c_ulonglong as uint64_t,
        0xedf4c615a4b29e94 as libc::c_ulonglong as uint64_t,
        0x4093286094110662 as libc::c_ulonglong as uint64_t,
        0xb0114ee85ae78063 as libc::c_ulonglong as uint64_t,
        0x6ff1d0d6b672e78b as libc::c_ulonglong as uint64_t,
        0x6dcf96d591909250 as libc::c_ulonglong as uint64_t,
        0xdfe09e3eec9567e8 as libc::c_ulonglong as uint64_t,
        0x3214582b4827f97c as libc::c_ulonglong as uint64_t,
        0xb46dc2ee143e6ac8 as libc::c_ulonglong as uint64_t,
        0xf6c0ac8da7cd1971 as libc::c_ulonglong as uint64_t,
        0xebb60c10cd8901e4 as libc::c_ulonglong as uint64_t,
        0xf7df8f023abcad92 as libc::c_ulonglong as uint64_t,
        0x9c52d3d2c217a0b2 as libc::c_ulonglong as uint64_t,
        0x6b8d5cd0f8ab0d20 as libc::c_ulonglong as uint64_t,
        0x3777f7a29b8fa734 as libc::c_ulonglong as uint64_t,
        0x11f238f9d71b4e3 as libc::c_ulonglong as uint64_t,
        0xc1b75b2f3c42be45 as libc::c_ulonglong as uint64_t,
        0x5de588fdfe551ef7 as libc::c_ulonglong as uint64_t,
        0x6eeef3592b035368 as libc::c_ulonglong as uint64_t,
        0xaa3a07ffc4e9b365 as libc::c_ulonglong as uint64_t,
        0xecebe59a39c32a77 as libc::c_ulonglong as uint64_t,
        0x5ba742f8976e8187 as libc::c_ulonglong as uint64_t,
        0x4b4a48e0b22d0e11 as libc::c_ulonglong as uint64_t,
        0xddded83dcb771233 as libc::c_ulonglong as uint64_t,
        0xa59feb79ac0c51bd as libc::c_ulonglong as uint64_t,
        0xc7f5912a55792135 as libc::c_ulonglong as uint64_t,
    ],
    [
        0x6d6ae04668a9b08a as libc::c_ulonglong as uint64_t,
        0x3ab3f04b0be8c743 as libc::c_ulonglong as uint64_t,
        0xe51e166b54b3c908 as libc::c_ulonglong as uint64_t,
        0xbe90a9eb35c2f139 as libc::c_ulonglong as uint64_t,
        0xb2c7066637f2bec1 as libc::c_ulonglong as uint64_t,
        0xaa6945613392202c as libc::c_ulonglong as uint64_t,
        0x9a28c36f3b5201eb as libc::c_ulonglong as uint64_t,
        0xddce5a93ab536994 as libc::c_ulonglong as uint64_t,
        0xe34133ef6382827 as libc::c_ulonglong as uint64_t,
        0x52a02ba1ec55048b as libc::c_ulonglong as uint64_t,
        0xa2f88f97c4b2a177 as libc::c_ulonglong as uint64_t,
        0x8640e513ca2251a5 as libc::c_ulonglong as uint64_t,
        0xcdf1d36258137622 as libc::c_ulonglong as uint64_t,
        0xfe6cb708dedf8ddb as libc::c_ulonglong as uint64_t,
        0x8a174a9ec8121e5d as libc::c_ulonglong as uint64_t,
        0x679896036b81560e as libc::c_ulonglong as uint64_t,
        0x59ed033395795fee as libc::c_ulonglong as uint64_t,
        0x1dd778ab8b74edaf as libc::c_ulonglong as uint64_t,
        0xee533ef92d9f926d as libc::c_ulonglong as uint64_t,
        0x2a8c79baf8a8d8f5 as libc::c_ulonglong as uint64_t,
        0x6bcf398e69b119f6 as libc::c_ulonglong as uint64_t,
        0xe20491742fafdd95 as libc::c_ulonglong as uint64_t,
        0x276488e0809c2aec as libc::c_ulonglong as uint64_t,
        0xea955b82d88f5cce as libc::c_ulonglong as uint64_t,
        0x7102c63a99d9e0c4 as libc::c_ulonglong as uint64_t,
        0xf9763017a5c39946 as libc::c_ulonglong as uint64_t,
        0x429fa2501f151b3d as libc::c_ulonglong as uint64_t,
        0x4659c72bea05d59e as libc::c_ulonglong as uint64_t,
        0x984b7fdccf5a6634 as libc::c_ulonglong as uint64_t,
        0xf742232953fbb161 as libc::c_ulonglong as uint64_t,
        0x3041860e08c021c7 as libc::c_ulonglong as uint64_t,
        0x747bfd9616cd9386 as libc::c_ulonglong as uint64_t,
        0x4bb1367192312787 as libc::c_ulonglong as uint64_t,
        0x1b72a1638a6c44d3 as libc::c_ulonglong as uint64_t,
        0x4a0e68a6e8359a66 as libc::c_ulonglong as uint64_t,
        0x169a5039f258b6ca as libc::c_ulonglong as uint64_t,
        0xb98a2ef44edee5a4 as libc::c_ulonglong as uint64_t,
        0xd9083fe85e43a737 as libc::c_ulonglong as uint64_t,
        0x967f6ce239624e13 as libc::c_ulonglong as uint64_t,
        0x8874f62d3c1a7982 as libc::c_ulonglong as uint64_t,
        0x3c1629830af06e3f as libc::c_ulonglong as uint64_t,
        0x9165ebfd427e5a8e as libc::c_ulonglong as uint64_t,
        0xb5dd81794ceeaa5c as libc::c_ulonglong as uint64_t,
        0xde8f15a7834f219 as libc::c_ulonglong as uint64_t,
        0x70bd98ede3dd5d25 as libc::c_ulonglong as uint64_t,
        0xaccc9ca9328a8950 as libc::c_ulonglong as uint64_t,
        0x56664eda1945ca28 as libc::c_ulonglong as uint64_t,
        0x221db34c0f8859ae as libc::c_ulonglong as uint64_t,
        0x26dbd637fa98970d as libc::c_ulonglong as uint64_t,
        0x1acdffb4f068f932 as libc::c_ulonglong as uint64_t,
        0x4585254f64090fa0 as libc::c_ulonglong as uint64_t,
        0x72de245e17d53afa as libc::c_ulonglong as uint64_t,
        0x1546b25d7c546cf4 as libc::c_ulonglong as uint64_t,
        0x207e0ffffb803e71 as libc::c_ulonglong as uint64_t,
        0xfaaad2732bcf4378 as libc::c_ulonglong as uint64_t,
        0xb462dfae36ea17bd as libc::c_ulonglong as uint64_t,
        0xcf926fd1ac1b11fd as libc::c_ulonglong as uint64_t,
        0xe0672dc7dba7ba4a as libc::c_ulonglong as uint64_t,
        0xd3fa49ad5d6b41b3 as libc::c_ulonglong as uint64_t,
        0x8ba81449b216a3bc as libc::c_ulonglong as uint64_t,
        0x14f9ec8a0650d115 as libc::c_ulonglong as uint64_t,
        0x40fc1ee3eb1d7ce2 as libc::c_ulonglong as uint64_t,
        0x23a2ed9b758ce44f as libc::c_ulonglong as uint64_t,
        0x782c521b14fddc7e as libc::c_ulonglong as uint64_t,
        0x1c68267cf170504e as libc::c_ulonglong as uint64_t,
        0xbcf31558c1ca96e6 as libc::c_ulonglong as uint64_t,
        0xa781b43b4ba6d235 as libc::c_ulonglong as uint64_t,
        0xf6fd7dfe29ff0c80 as libc::c_ulonglong as uint64_t,
        0xb0a4bad5c3fad91e as libc::c_ulonglong as uint64_t,
        0xd199f51ea963266c as libc::c_ulonglong as uint64_t,
        0x414340349119c103 as libc::c_ulonglong as uint64_t,
        0x5405f269ed4dadf7 as libc::c_ulonglong as uint64_t,
        0xabd61bb649969dcd as libc::c_ulonglong as uint64_t,
        0x6813dbeae7bdc3c8 as libc::c_ulonglong as uint64_t,
        0x65fb2ab09f8931d1 as libc::c_ulonglong as uint64_t,
        0xf1e7fae152e3181d as libc::c_ulonglong as uint64_t,
        0xc1a67cef5a2339da as libc::c_ulonglong as uint64_t,
        0x7a4feea8e0f5bba1 as libc::c_ulonglong as uint64_t,
        0x1e0b9acf05783791 as libc::c_ulonglong as uint64_t,
        0x5b8ebf8061713831 as libc::c_ulonglong as uint64_t,
        0x80e53cdbcb3af8d9 as libc::c_ulonglong as uint64_t,
        0x7e898bd315e57502 as libc::c_ulonglong as uint64_t,
        0xc6bcfbf0213f2d47 as libc::c_ulonglong as uint64_t,
        0x95a38e86b76e942d as libc::c_ulonglong as uint64_t,
        0x92e94218d243cba as libc::c_ulonglong as uint64_t,
        0x8339debf453622e7 as libc::c_ulonglong as uint64_t,
        0xb11be402b9fe64ff as libc::c_ulonglong as uint64_t,
        0x57d9100d634177c9 as libc::c_ulonglong as uint64_t,
        0xcc4e8db52217cbc3 as libc::c_ulonglong as uint64_t,
        0x3b0cae9c71ec7aa2 as libc::c_ulonglong as uint64_t,
        0xfb158ca451cbfe99 as libc::c_ulonglong as uint64_t,
        0x2b33276d82ac6514 as libc::c_ulonglong as uint64_t,
        0x1bf5ed77a04bde1 as libc::c_ulonglong as uint64_t,
        0xc5601994af33f779 as libc::c_ulonglong as uint64_t,
        0x75c4a3416cc92e67 as libc::c_ulonglong as uint64_t,
        0xf3844652a6eb7fc2 as libc::c_ulonglong as uint64_t,
        0x3487e375fdd0ef64 as libc::c_ulonglong as uint64_t,
        0x18ae430704609eed as libc::c_ulonglong as uint64_t,
        0x4d14efb993298efb as libc::c_ulonglong as uint64_t,
        0x815a620cb13e4538 as libc::c_ulonglong as uint64_t,
        0x125c354207487869 as libc::c_ulonglong as uint64_t,
        0x9eeea614ce42cf48 as libc::c_ulonglong as uint64_t,
        0xce2d3106d61fac1c as libc::c_ulonglong as uint64_t,
        0xbbe99247bad6827b as libc::c_ulonglong as uint64_t,
        0x71a871f7b1c149d as libc::c_ulonglong as uint64_t,
        0x2e4a1cc10db81656 as libc::c_ulonglong as uint64_t,
        0x77a71ff298c149b8 as libc::c_ulonglong as uint64_t,
        0x6a5d9c80118a97c as libc::c_ulonglong as uint64_t,
        0xad73c27e488e34b1 as libc::c_ulonglong as uint64_t,
        0x443a7b981e0db241 as libc::c_ulonglong as uint64_t,
        0xe3bbcfa355ab6074 as libc::c_ulonglong as uint64_t,
        0xaf276450328e684 as libc::c_ulonglong as uint64_t,
        0x73617a896dd1871b as libc::c_ulonglong as uint64_t,
        0x58525de4ef7de20f as libc::c_ulonglong as uint64_t,
        0xb7be3dcab8e6cd83 as libc::c_ulonglong as uint64_t,
        0x19111dd07e64230c as libc::c_ulonglong as uint64_t,
        0x842359a03e2a367a as libc::c_ulonglong as uint64_t,
        0x103f89f1f3401fb6 as libc::c_ulonglong as uint64_t,
        0xdc710444d157d475 as libc::c_ulonglong as uint64_t,
        0xb835702334da5845 as libc::c_ulonglong as uint64_t,
        0x4320fc876511a6dc as libc::c_ulonglong as uint64_t,
        0xd026abc9d3679b8d as libc::c_ulonglong as uint64_t,
        0x17250eee885c0b2b as libc::c_ulonglong as uint64_t,
        0x90dab52a387ae76f as libc::c_ulonglong as uint64_t,
        0x31fed8d972c49c26 as libc::c_ulonglong as uint64_t,
        0x89cba8fa461ec463 as libc::c_ulonglong as uint64_t,
        0x2ff5421677bcabb7 as libc::c_ulonglong as uint64_t,
        0x396f122f85e41d7d as libc::c_ulonglong as uint64_t,
        0xa09b332430bac6a8 as libc::c_ulonglong as uint64_t,
        0xc888e8ced7070560 as libc::c_ulonglong as uint64_t,
        0xaeaf201ac682ee8f as libc::c_ulonglong as uint64_t,
        0x1180d7268944a257 as libc::c_ulonglong as uint64_t,
        0xf058a43628e7a5fc as libc::c_ulonglong as uint64_t,
        0xbd4c4b8fbbce2b07 as libc::c_ulonglong as uint64_t,
        0xa1246df34abe7b49 as libc::c_ulonglong as uint64_t,
        0x7d5569b79be9af3c as libc::c_ulonglong as uint64_t,
        0xa9b5a705bd9efa12 as libc::c_ulonglong as uint64_t,
        0xdb6b835baa4bc0e8 as libc::c_ulonglong as uint64_t,
        0x5793bac8f147342 as libc::c_ulonglong as uint64_t,
        0x21c1512881848390 as libc::c_ulonglong as uint64_t,
        0xfdb0556c50d357e5 as libc::c_ulonglong as uint64_t,
        0x613d4fcb6a99ff72 as libc::c_ulonglong as uint64_t,
        0x3dce2648e0cda3e as libc::c_ulonglong as uint64_t,
        0xe949b9e6568386f0 as libc::c_ulonglong as uint64_t,
        0xfc0f0bbb2ad7ea04 as libc::c_ulonglong as uint64_t,
        0x6a70675913b5a417 as libc::c_ulonglong as uint64_t,
        0x7f36d5046fe1c8e3 as libc::c_ulonglong as uint64_t,
        0xc57af8d02304ff8 as libc::c_ulonglong as uint64_t,
        0x32223abdfcc84618 as libc::c_ulonglong as uint64_t,
        0x891caf6f720815b as libc::c_ulonglong as uint64_t,
        0xa63eeaec31a26fd4 as libc::c_ulonglong as uint64_t,
        0x2507345374944d33 as libc::c_ulonglong as uint64_t,
        0x49d28ac266394058 as libc::c_ulonglong as uint64_t,
        0xf5219f9aa7f3d6be as libc::c_ulonglong as uint64_t,
        0x2d96fea583b4cc68 as libc::c_ulonglong as uint64_t,
        0x5a31e1571b7585d0 as libc::c_ulonglong as uint64_t,
        0x8ed12fe53d02d0fe as libc::c_ulonglong as uint64_t,
        0xdfade6205f5b0e4b as libc::c_ulonglong as uint64_t,
        0x4cabb16ee92d331a as libc::c_ulonglong as uint64_t,
        0x4c6657bf510cea3 as libc::c_ulonglong as uint64_t,
        0xd73c2cd6a87b8f10 as libc::c_ulonglong as uint64_t,
        0xe1d87310a1a307ab as libc::c_ulonglong as uint64_t,
        0x6cd5be9112ad0d6b as libc::c_ulonglong as uint64_t,
        0x97c032354366f3f2 as libc::c_ulonglong as uint64_t,
        0xd4e0ceb22677552e as libc::c_ulonglong as uint64_t,
        0 as libc::c_ulonglong as uint64_t,
        0x29509bde76a402cb as libc::c_ulonglong as uint64_t,
        0xc27a9e8bd42fe3e4 as libc::c_ulonglong as uint64_t,
        0x5ef7842cee654b73 as libc::c_ulonglong as uint64_t,
        0xaf107ecdbc86536e as libc::c_ulonglong as uint64_t,
        0x3fcacbe784fcb401 as libc::c_ulonglong as uint64_t,
        0xd55f90655c73e8cf as libc::c_ulonglong as uint64_t,
        0xe6c2f40fdabf1336 as libc::c_ulonglong as uint64_t,
        0xe8f6e7312c873b11 as libc::c_ulonglong as uint64_t,
        0xeb2a0555a28be12f as libc::c_ulonglong as uint64_t,
        0xe4a148bc2eb774e9 as libc::c_ulonglong as uint64_t,
        0x9b979db84156bc0a as libc::c_ulonglong as uint64_t,
        0x6eb60222e6a56ab4 as libc::c_ulonglong as uint64_t,
        0x87ffbbc4b026ec44 as libc::c_ulonglong as uint64_t,
        0xc703a5275b3b90a6 as libc::c_ulonglong as uint64_t,
        0x47e699fc9001687f as libc::c_ulonglong as uint64_t,
        0x9c8d1aa73a4aa897 as libc::c_ulonglong as uint64_t,
        0x7cea3760e1ed12dd as libc::c_ulonglong as uint64_t,
        0x4ec80ddd1d2554c5 as libc::c_ulonglong as uint64_t,
        0x13e36b957d4cc588 as libc::c_ulonglong as uint64_t,
        0x5d2b66486069914d as libc::c_ulonglong as uint64_t,
        0x92b90999cc7280b0 as libc::c_ulonglong as uint64_t,
        0x517cc9c56259deb5 as libc::c_ulonglong as uint64_t,
        0xc937b619ad03b881 as libc::c_ulonglong as uint64_t,
        0xec30824ad997f5b2 as libc::c_ulonglong as uint64_t,
        0xa45d565fc5aa080b as libc::c_ulonglong as uint64_t,
        0xd6837201d27f32f1 as libc::c_ulonglong as uint64_t,
        0x635ef3789e9198ad as libc::c_ulonglong as uint64_t,
        0x531f75769651b96a as libc::c_ulonglong as uint64_t,
        0x4f77530a6721e924 as libc::c_ulonglong as uint64_t,
        0x486dd4151c3dfdb9 as libc::c_ulonglong as uint64_t,
        0x5f48dafb9461f692 as libc::c_ulonglong as uint64_t,
        0x375b011173dc355a as libc::c_ulonglong as uint64_t,
        0x3da9775470f4d3de as libc::c_ulonglong as uint64_t,
        0x8d0dcd81b30e0ac0 as libc::c_ulonglong as uint64_t,
        0x36e45fc609d888bb as libc::c_ulonglong as uint64_t,
        0x55baacbe97491016 as libc::c_ulonglong as uint64_t,
        0x8cb29356c90ab721 as libc::c_ulonglong as uint64_t,
        0x76184125e2c5f459 as libc::c_ulonglong as uint64_t,
        0x99f4210bb55edbd5 as libc::c_ulonglong as uint64_t,
        0x6f095cf59ca1d755 as libc::c_ulonglong as uint64_t,
        0x9f51f8c3b44672a9 as libc::c_ulonglong as uint64_t,
        0x3538bda287d45285 as libc::c_ulonglong as uint64_t,
        0x50c39712185d6354 as libc::c_ulonglong as uint64_t,
        0xf23b1885dcefc223 as libc::c_ulonglong as uint64_t,
        0x79930ccc6ef9619f as libc::c_ulonglong as uint64_t,
        0xed8fdc9da3934853 as libc::c_ulonglong as uint64_t,
        0xcb540aaa590bdf5e as libc::c_ulonglong as uint64_t,
        0x5c94389f1a6d2cac as libc::c_ulonglong as uint64_t,
        0xe77daad8a0bbaed7 as libc::c_ulonglong as uint64_t,
        0x28efc5090ca0bf2a as libc::c_ulonglong as uint64_t,
        0xbf2ff73c4fc64cd8 as libc::c_ulonglong as uint64_t,
        0xb37858b14df60320 as libc::c_ulonglong as uint64_t,
        0xf8c96ec0dfc724a7 as libc::c_ulonglong as uint64_t,
        0x828680683f329f06 as libc::c_ulonglong as uint64_t,
        0x941cd051cd6a29cc as libc::c_ulonglong as uint64_t,
        0xc3c5c05cae2b5e05 as libc::c_ulonglong as uint64_t,
        0xb601631dc2e27062 as libc::c_ulonglong as uint64_t,
        0xc01922382027843b as libc::c_ulonglong as uint64_t,
        0x24b86a840e90f0d2 as libc::c_ulonglong as uint64_t,
        0xd245177a276ffc52 as libc::c_ulonglong as uint64_t,
        0xf8b4de98c3c95c6 as libc::c_ulonglong as uint64_t,
        0x3e759530fef809e0 as libc::c_ulonglong as uint64_t,
        0xb4d2892792c5b65 as libc::c_ulonglong as uint64_t,
        0xc4df4743d5374a98 as libc::c_ulonglong as uint64_t,
        0xa5e20888bfaeb5ea as libc::c_ulonglong as uint64_t,
        0xba56cc90c0d23f9a as libc::c_ulonglong as uint64_t,
        0x38d04cf8ffe0a09c as libc::c_ulonglong as uint64_t,
        0x62e1adafe495254c as libc::c_ulonglong as uint64_t,
        0x263bcb3f40867df as libc::c_ulonglong as uint64_t,
        0xcaeb547d230f62bf as libc::c_ulonglong as uint64_t,
        0x6082111c109d4293 as libc::c_ulonglong as uint64_t,
        0xdad4dd8cd04f7d09 as libc::c_ulonglong as uint64_t,
        0xefec602e579b2f8c as libc::c_ulonglong as uint64_t,
        0x1fb4c4187f7c8a70 as libc::c_ulonglong as uint64_t,
        0xffd3e9dfa4db303a as libc::c_ulonglong as uint64_t,
        0x7bf0b07f9af10640 as libc::c_ulonglong as uint64_t,
        0xf49ec14dddf76b5f as libc::c_ulonglong as uint64_t,
        0x8f6e713247066d1f as libc::c_ulonglong as uint64_t,
        0x339d646a86ccfbf9 as libc::c_ulonglong as uint64_t,
        0x64447467e58d8c30 as libc::c_ulonglong as uint64_t,
        0x2c29a072f9b07189 as libc::c_ulonglong as uint64_t,
        0xd8b7613f24471ad6 as libc::c_ulonglong as uint64_t,
        0x6627c8d41185ebef as libc::c_ulonglong as uint64_t,
        0xa347d140beb61c96 as libc::c_ulonglong as uint64_t,
        0xde12b8f7255fb3aa as libc::c_ulonglong as uint64_t,
        0x9d324470404e1576 as libc::c_ulonglong as uint64_t,
        0x9306574eb6763d51 as libc::c_ulonglong as uint64_t,
        0xa80af9d2c79a47f3 as libc::c_ulonglong as uint64_t,
        0x859c0777442e8b9b as libc::c_ulonglong as uint64_t,
        0x69ac853d9db97e29 as libc::c_ulonglong as uint64_t,
    ],
    [
        0xc3407dfc2de6377e as libc::c_ulonglong as uint64_t,
        0x5b9e93eea4256f77 as libc::c_ulonglong as uint64_t,
        0xadb58fdd50c845e0 as libc::c_ulonglong as uint64_t,
        0x5219ff11a75bed86 as libc::c_ulonglong as uint64_t,
        0x356b61cfd90b1de9 as libc::c_ulonglong as uint64_t,
        0xfb8f406e25abe037 as libc::c_ulonglong as uint64_t,
        0x7a5a0231c0f60796 as libc::c_ulonglong as uint64_t,
        0x9d3cd216e1f5020b as libc::c_ulonglong as uint64_t,
        0xc6550fb6b48d8f3 as libc::c_ulonglong as uint64_t,
        0xf57508c427ff1c62 as libc::c_ulonglong as uint64_t,
        0x4ad35ffa71cb407d as libc::c_ulonglong as uint64_t,
        0x6290a2da1666aa6d as libc::c_ulonglong as uint64_t,
        0xe284ec2349355f9f as libc::c_ulonglong as uint64_t,
        0xb3c307c53d7c84ec as libc::c_ulonglong as uint64_t,
        0x5e23c0468365a02 as libc::c_ulonglong as uint64_t,
        0x190bac4d6c9ebfa8 as libc::c_ulonglong as uint64_t,
        0x94bbbee9e28b80fa as libc::c_ulonglong as uint64_t,
        0xa34fc777529cb9b5 as libc::c_ulonglong as uint64_t,
        0xcc7b39f095bcd978 as libc::c_ulonglong as uint64_t,
        0x2426addb0ce532e3 as libc::c_ulonglong as uint64_t,
        0x7e79329312ce4fc7 as libc::c_ulonglong as uint64_t,
        0xab09a72eebec2917 as libc::c_ulonglong as uint64_t,
        0xf8d15499f6b9d6c2 as libc::c_ulonglong as uint64_t,
        0x1a55b8babf8c895d as libc::c_ulonglong as uint64_t,
        0xdb8add17fb769a85 as libc::c_ulonglong as uint64_t,
        0xb57f2f368658e81b as libc::c_ulonglong as uint64_t,
        0x8acd36f18f3f41f6 as libc::c_ulonglong as uint64_t,
        0x5ce3b7bba50f11d3 as libc::c_ulonglong as uint64_t,
        0x114dcc14d5ee2f0a as libc::c_ulonglong as uint64_t,
        0xb91a7fcded1030e8 as libc::c_ulonglong as uint64_t,
        0x81d5425fe55de7a1 as libc::c_ulonglong as uint64_t,
        0xb6213bc1554adeee as libc::c_ulonglong as uint64_t,
        0x80144ef95f53f5f2 as libc::c_ulonglong as uint64_t,
        0x1e7688186db4c10c as libc::c_ulonglong as uint64_t,
        0x3b912965db5fe1bc as libc::c_ulonglong as uint64_t,
        0xc281715a97e8252d as libc::c_ulonglong as uint64_t,
        0x54a5d7e21c7f8171 as libc::c_ulonglong as uint64_t,
        0x4b12535ccbc5522e as libc::c_ulonglong as uint64_t,
        0x1d289cefbea6f7f9 as libc::c_ulonglong as uint64_t,
        0x6ef5f2217d2e729e as libc::c_ulonglong as uint64_t,
        0xe6a7dc819b0d17ce as libc::c_ulonglong as uint64_t,
        0x1b94b41c05829b0e as libc::c_ulonglong as uint64_t,
        0x33d7493c622f711e as libc::c_ulonglong as uint64_t,
        0xdcf7f942fa5ce421 as libc::c_ulonglong as uint64_t,
        0x600fba8b7f7a8ecb as libc::c_ulonglong as uint64_t,
        0x46b60f011a83988e as libc::c_ulonglong as uint64_t,
        0x235b898e0dcf4c47 as libc::c_ulonglong as uint64_t,
        0x957ab24f588592a9 as libc::c_ulonglong as uint64_t,
        0x4354330572b5c28c as libc::c_ulonglong as uint64_t,
        0xa5f3ef84e9b8d542 as libc::c_ulonglong as uint64_t,
        0x8c711e02341b2d01 as libc::c_ulonglong as uint64_t,
        0xb1874ae6a62a657 as libc::c_ulonglong as uint64_t,
        0x1213d8e306fc19ff as libc::c_ulonglong as uint64_t,
        0xfe6d7c6a4d9dba35 as libc::c_ulonglong as uint64_t,
        0x65ed868f174cd4c9 as libc::c_ulonglong as uint64_t,
        0x88522ea0e6236550 as libc::c_ulonglong as uint64_t,
        0x899322065c2d7703 as libc::c_ulonglong as uint64_t,
        0xc01e690bfef4018b as libc::c_ulonglong as uint64_t,
        0x915982ed8abddaf8 as libc::c_ulonglong as uint64_t,
        0xbe675b98ec3a4e4c as libc::c_ulonglong as uint64_t,
        0xa996bf7f82f00db1 as libc::c_ulonglong as uint64_t,
        0xe1daf8d49a27696a as libc::c_ulonglong as uint64_t,
        0x2effd5d3dc8986e7 as libc::c_ulonglong as uint64_t,
        0xd153a51f2b1a2e81 as libc::c_ulonglong as uint64_t,
        0x18caa0ebd690adfb as libc::c_ulonglong as uint64_t,
        0x390e3134b243c51a as libc::c_ulonglong as uint64_t,
        0x2778b92cdff70416 as libc::c_ulonglong as uint64_t,
        0x29f1851691c24a6 as libc::c_ulonglong as uint64_t,
        0x5e7cafeacc133575 as libc::c_ulonglong as uint64_t,
        0xfa4e4cc89fa5f264 as libc::c_ulonglong as uint64_t,
        0x5a5f9f481e2b7d24 as libc::c_ulonglong as uint64_t,
        0x484c47ab18d764db as libc::c_ulonglong as uint64_t,
        0x400a27f2a1a7f479 as libc::c_ulonglong as uint64_t,
        0xaeeb9b2a83da7315 as libc::c_ulonglong as uint64_t,
        0x721c626879869734 as libc::c_ulonglong as uint64_t,
        0x42330a2d2384851 as libc::c_ulonglong as uint64_t,
        0x85f672fd3765aff0 as libc::c_ulonglong as uint64_t,
        0xba446b3a3e02061d as libc::c_ulonglong as uint64_t,
        0x73dd6ecec3888567 as libc::c_ulonglong as uint64_t,
        0xffac70ccf793a866 as libc::c_ulonglong as uint64_t,
        0xdfa9edb5294ed2d4 as libc::c_ulonglong as uint64_t,
        0x6c6aea7014325638 as libc::c_ulonglong as uint64_t,
        0x834a5a0e8c41c307 as libc::c_ulonglong as uint64_t,
        0xcdba35562fb2cb2b as libc::c_ulonglong as uint64_t,
        0xad97808d06cb404 as libc::c_ulonglong as uint64_t,
        0xf3b440cb85aee06 as libc::c_ulonglong as uint64_t,
        0xe5f9c876481f213b as libc::c_ulonglong as uint64_t,
        0x98deee1289c35809 as libc::c_ulonglong as uint64_t,
        0x59018bbfcd394bd1 as libc::c_ulonglong as uint64_t,
        0xe01bf47220297b39 as libc::c_ulonglong as uint64_t,
        0xde68e1139340c087 as libc::c_ulonglong as uint64_t,
        0x9fa3ca4788e926ad as libc::c_ulonglong as uint64_t,
        0xbb85679c840c144e as libc::c_ulonglong as uint64_t,
        0x53d8f3b71d55ffd5 as libc::c_ulonglong as uint64_t,
        0xda45c5dd146caa0 as libc::c_ulonglong as uint64_t,
        0x6f34fe87c72060cd as libc::c_ulonglong as uint64_t,
        0x57fbc315cf6db784 as libc::c_ulonglong as uint64_t,
        0xcee421a1fca0fdde as libc::c_ulonglong as uint64_t,
        0x3d2d0196607b8d4b as libc::c_ulonglong as uint64_t,
        0x642c8a29ad42c69a as libc::c_ulonglong as uint64_t,
        0x14aff010bdd87508 as libc::c_ulonglong as uint64_t,
        0xac74837beac657b3 as libc::c_ulonglong as uint64_t,
        0x3216459ad821634d as libc::c_ulonglong as uint64_t,
        0x3fb219c70967a9ed as libc::c_ulonglong as uint64_t,
        0x6bc28f3bb246cf7 as libc::c_ulonglong as uint64_t,
        0xf2082c9126d562c6 as libc::c_ulonglong as uint64_t,
        0x66b39278c45ee23c as libc::c_ulonglong as uint64_t,
        0xbd394f6f3f2878b9 as libc::c_ulonglong as uint64_t,
        0xfd33689d9e8f8cc0 as libc::c_ulonglong as uint64_t,
        0x37f4799eb017394f as libc::c_ulonglong as uint64_t,
        0x108cc0b26fe03d59 as libc::c_ulonglong as uint64_t,
        0xda4bd1b1417888d6 as libc::c_ulonglong as uint64_t,
        0xb09d1332ee6eb219 as libc::c_ulonglong as uint64_t,
        0x2f3ed975668794b4 as libc::c_ulonglong as uint64_t,
        0x58c0871977375982 as libc::c_ulonglong as uint64_t,
        0x7561463d78ace990 as libc::c_ulonglong as uint64_t,
        0x9876cff037e82f1 as libc::c_ulonglong as uint64_t,
        0x7fb83e35a8c05d94 as libc::c_ulonglong as uint64_t,
        0x26b9b58a65f91645 as libc::c_ulonglong as uint64_t,
        0xef20b07e9873953f as libc::c_ulonglong as uint64_t,
        0x3148516d0b3355b8 as libc::c_ulonglong as uint64_t,
        0x41cb2b541ba9e62a as libc::c_ulonglong as uint64_t,
        0x790416c613e43163 as libc::c_ulonglong as uint64_t,
        0xa011d380818e8f40 as libc::c_ulonglong as uint64_t,
        0x3a5025c36151f3ef as libc::c_ulonglong as uint64_t,
        0xd57095bdf92266d0 as libc::c_ulonglong as uint64_t,
        0x498d4b0da2d97688 as libc::c_ulonglong as uint64_t,
        0x8b0c3a57353153a5 as libc::c_ulonglong as uint64_t,
        0x21c491df64d368e1 as libc::c_ulonglong as uint64_t,
        0x8f2f0af5e7091bf4 as libc::c_ulonglong as uint64_t,
        0x2da1c1240f9bb012 as libc::c_ulonglong as uint64_t,
        0xc43d59a92ccc49da as libc::c_ulonglong as uint64_t,
        0xbfa6573e56345c1f as libc::c_ulonglong as uint64_t,
        0x828b56a8364fd154 as libc::c_ulonglong as uint64_t,
        0x9a41f643e0df7caf as libc::c_ulonglong as uint64_t,
        0xbcf843c985266aea as libc::c_ulonglong as uint64_t,
        0x2b1de9d7b4bfdce5 as libc::c_ulonglong as uint64_t,
        0x20059d79dedd7ab2 as libc::c_ulonglong as uint64_t,
        0x6dabe6d6ae3c446b as libc::c_ulonglong as uint64_t,
        0x45e81bf6c991ae7b as libc::c_ulonglong as uint64_t,
        0x6351ae7cac68b83e as libc::c_ulonglong as uint64_t,
        0xa432e32253b6c711 as libc::c_ulonglong as uint64_t,
        0xd092a9b991143cd2 as libc::c_ulonglong as uint64_t,
        0xcac711032e98b58f as libc::c_ulonglong as uint64_t,
        0xd8d4c9e02864ac70 as libc::c_ulonglong as uint64_t,
        0xc5fc550f96c25b89 as libc::c_ulonglong as uint64_t,
        0xd7ef8dec903e4276 as libc::c_ulonglong as uint64_t,
        0x67729ede7e50f06f as libc::c_ulonglong as uint64_t,
        0xeac28c7af045cf3d as libc::c_ulonglong as uint64_t,
        0xb15c1f945460a04a as libc::c_ulonglong as uint64_t,
        0x9cfddeb05bfb1058 as libc::c_ulonglong as uint64_t,
        0x93c69abce3a1fe5e as libc::c_ulonglong as uint64_t,
        0xeb0380dc4a4bdd6e as libc::c_ulonglong as uint64_t,
        0xd20db1e8f8081874 as libc::c_ulonglong as uint64_t,
        0x229a8528b7c15e14 as libc::c_ulonglong as uint64_t,
        0x44291750739fbc28 as libc::c_ulonglong as uint64_t,
        0xd3ccbd4e42060a27 as libc::c_ulonglong as uint64_t,
        0xf62b1c33f4ed2a97 as libc::c_ulonglong as uint64_t,
        0x86a8660ae4779905 as libc::c_ulonglong as uint64_t,
        0xd62e814a2a305025 as libc::c_ulonglong as uint64_t,
        0x477703a7a08d8add as libc::c_ulonglong as uint64_t,
        0x7b9b0e977af815c5 as libc::c_ulonglong as uint64_t,
        0x78c51a60a9ea2330 as libc::c_ulonglong as uint64_t,
        0xa6adfb733aaae3b7 as libc::c_ulonglong as uint64_t,
        0x97e5aa1e3199b60f as libc::c_ulonglong as uint64_t,
        0 as libc::c_ulonglong as uint64_t,
        0xf4b404629df10e31 as libc::c_ulonglong as uint64_t,
        0x5564db44a6719322 as libc::c_ulonglong as uint64_t,
        0x9207961a59afec0d as libc::c_ulonglong as uint64_t,
        0x9624a6b88b97a45c as libc::c_ulonglong as uint64_t,
        0x363575380a192b1c as libc::c_ulonglong as uint64_t,
        0x2c60cd82b595a241 as libc::c_ulonglong as uint64_t,
        0x7d272664c1dc7932 as libc::c_ulonglong as uint64_t,
        0x7142769faa94a1c1 as libc::c_ulonglong as uint64_t,
        0xa1d0df263b809d13 as libc::c_ulonglong as uint64_t,
        0x1630e841d4c451ae as libc::c_ulonglong as uint64_t,
        0xc1df65ad44fa13d8 as libc::c_ulonglong as uint64_t,
        0x13d2d445bcf20bac as libc::c_ulonglong as uint64_t,
        0xd915c546926abe23 as libc::c_ulonglong as uint64_t,
        0x38cf3d92084dd749 as libc::c_ulonglong as uint64_t,
        0xe766d0272103059d as libc::c_ulonglong as uint64_t,
        0xc7634d5effde7f2f as libc::c_ulonglong as uint64_t,
        0x77d2455012a7ea4 as libc::c_ulonglong as uint64_t,
        0xedbfa82ff16fb199 as libc::c_ulonglong as uint64_t,
        0xaf2a978c39d46146 as libc::c_ulonglong as uint64_t,
        0x42953fa3c8bbd0df as libc::c_ulonglong as uint64_t,
        0xcb061da59496a7dc as libc::c_ulonglong as uint64_t,
        0x25e7a17db6eb20b0 as libc::c_ulonglong as uint64_t,
        0x34aa6d6963050fba as libc::c_ulonglong as uint64_t,
        0xa76cf7d580a4f1e4 as libc::c_ulonglong as uint64_t,
        0xf7ea10954ee338c4 as libc::c_ulonglong as uint64_t,
        0xfcf2643b24819e93 as libc::c_ulonglong as uint64_t,
        0xcf252d0746aeef8d as libc::c_ulonglong as uint64_t,
        0x4ef06f58a3f3082c as libc::c_ulonglong as uint64_t,
        0x563acfb37563a5d7 as libc::c_ulonglong as uint64_t,
        0x5086e740ce47c920 as libc::c_ulonglong as uint64_t,
        0x2982f186dda3f843 as libc::c_ulonglong as uint64_t,
        0x87696aac5e798b56 as libc::c_ulonglong as uint64_t,
        0x5d22bb1d1f010380 as libc::c_ulonglong as uint64_t,
        0x35e14f7d31236f5 as libc::c_ulonglong as uint64_t,
        0x3cec0d30da759f18 as libc::c_ulonglong as uint64_t,
        0xf3c920379cdb7095 as libc::c_ulonglong as uint64_t,
        0xb8db736b571e22bb as libc::c_ulonglong as uint64_t,
        0xdd36f5e44052f672 as libc::c_ulonglong as uint64_t,
        0xaac8ab8851e23b44 as libc::c_ulonglong as uint64_t,
        0xa857b3d938fe1fe2 as libc::c_ulonglong as uint64_t,
        0x17f1e4e76eca43fd as libc::c_ulonglong as uint64_t,
        0xec7ea4894b61a3ca as libc::c_ulonglong as uint64_t,
        0x9e62c6e132e734fe as libc::c_ulonglong as uint64_t,
        0xd4b1991b432c7483 as libc::c_ulonglong as uint64_t,
        0x6ad6c283af163acf as libc::c_ulonglong as uint64_t,
        0x1ce9904904a8e5aa as libc::c_ulonglong as uint64_t,
        0x5fbda34c761d2726 as libc::c_ulonglong as uint64_t,
        0xf910583f4cb7c491 as libc::c_ulonglong as uint64_t,
        0xc6a241f845d06d7c as libc::c_ulonglong as uint64_t,
        0x4f3163fe19fd1a7f as libc::c_ulonglong as uint64_t,
        0xe99c988d2357f9c8 as libc::c_ulonglong as uint64_t,
        0x8eee06535d0709a7 as libc::c_ulonglong as uint64_t,
        0xefa48aa0254fc55 as libc::c_ulonglong as uint64_t,
        0xb4be23903c56fa48 as libc::c_ulonglong as uint64_t,
        0x763f52caabbedf65 as libc::c_ulonglong as uint64_t,
        0xeee1bcd8227d876c as libc::c_ulonglong as uint64_t,
        0xe345e085f33b4dcc as libc::c_ulonglong as uint64_t,
        0x3e731561b369bbbe as libc::c_ulonglong as uint64_t,
        0x2843fd2067adea10 as libc::c_ulonglong as uint64_t,
        0x2adce5710eb1ceb6 as libc::c_ulonglong as uint64_t,
        0xb7e03767ef44ccbd as libc::c_ulonglong as uint64_t,
        0x8db012a48e153f52 as libc::c_ulonglong as uint64_t,
        0x61ceb62dc5749c98 as libc::c_ulonglong as uint64_t,
        0xe85d942b9959eb9b as libc::c_ulonglong as uint64_t,
        0x4c6f7709caef2c8a as libc::c_ulonglong as uint64_t,
        0x84377e5b8d6bbda3 as libc::c_ulonglong as uint64_t,
        0x30895dcbb13d47eb as libc::c_ulonglong as uint64_t,
        0x74a04a9bc2a2fbc3 as libc::c_ulonglong as uint64_t,
        0x6b17ce251518289c as libc::c_ulonglong as uint64_t,
        0xe438c4d0f2113368 as libc::c_ulonglong as uint64_t,
        0x1fb784bed7bad35f as libc::c_ulonglong as uint64_t,
        0x9b80fae55ad16efc as libc::c_ulonglong as uint64_t,
        0x77fe5e6c11b0cd36 as libc::c_ulonglong as uint64_t,
        0xc858095247849129 as libc::c_ulonglong as uint64_t,
        0x8466059b97090a2 as libc::c_ulonglong as uint64_t,
        0x1c10ca6ba0e1253 as libc::c_ulonglong as uint64_t,
        0x6988d6747c040c3a as libc::c_ulonglong as uint64_t,
        0x6849dad2c60a1e69 as libc::c_ulonglong as uint64_t,
        0x5147ebe67449db73 as libc::c_ulonglong as uint64_t,
        0xc99905f4fd8a837a as libc::c_ulonglong as uint64_t,
        0x991fe2b433cd4a5a as libc::c_ulonglong as uint64_t,
        0xf09734c04fc94660 as libc::c_ulonglong as uint64_t,
        0xa28ecbd1e892abe6 as libc::c_ulonglong as uint64_t,
        0xf1563866f5c75433 as libc::c_ulonglong as uint64_t,
        0x4dae7baf70e13ed9 as libc::c_ulonglong as uint64_t,
        0x7ce62ac27bd26b61 as libc::c_ulonglong as uint64_t,
        0x70837a39109ab392 as libc::c_ulonglong as uint64_t,
        0x90988e4b30b3c8ab as libc::c_ulonglong as uint64_t,
        0xb2020b63877296bf as libc::c_ulonglong as uint64_t,
        0x156efcb607d6675b as libc::c_ulonglong as uint64_t,
    ],
    [
        0xe63f55ce97c331d0 as libc::c_ulonglong as uint64_t,
        0x25b506b0015bba16 as libc::c_ulonglong as uint64_t,
        0xc8706e29e6ad9ba8 as libc::c_ulonglong as uint64_t,
        0x5b43d3775d521f6a as libc::c_ulonglong as uint64_t,
        0xbfa3d577035106e as libc::c_ulonglong as uint64_t,
        0xab95fc172afb0e66 as libc::c_ulonglong as uint64_t,
        0xf64b63979e7a3276 as libc::c_ulonglong as uint64_t,
        0xf58b4562649dad4b as libc::c_ulonglong as uint64_t,
        0x48f7c3dbae0c83f1 as libc::c_ulonglong as uint64_t,
        0xff31916642f5c8c5 as libc::c_ulonglong as uint64_t,
        0xcbb048dc1c4a0495 as libc::c_ulonglong as uint64_t,
        0x66b8f83cdf622989 as libc::c_ulonglong as uint64_t,
        0x35c130e908e2b9b0 as libc::c_ulonglong as uint64_t,
        0x7c761a61f0b34fa1 as libc::c_ulonglong as uint64_t,
        0x3601161cf205268d as libc::c_ulonglong as uint64_t,
        0x9e54ccfe2219b7d6 as libc::c_ulonglong as uint64_t,
        0x8b7d90a538940837 as libc::c_ulonglong as uint64_t,
        0x9cd403588ea35d0b as libc::c_ulonglong as uint64_t,
        0xbc3c6fea9ccc5b5a as libc::c_ulonglong as uint64_t,
        0xe5ff733b6d24aeed as libc::c_ulonglong as uint64_t,
        0xceed22de0f7eb8d2 as libc::c_ulonglong as uint64_t,
        0xec8581cab1ab545e as libc::c_ulonglong as uint64_t,
        0xb96105e88ff8e71d as libc::c_ulonglong as uint64_t,
        0x8ca03501871a5ead as libc::c_ulonglong as uint64_t,
        0x76ccce65d6db2a2f as libc::c_ulonglong as uint64_t,
        0x5883f582a7b58057 as libc::c_ulonglong as uint64_t,
        0x3f7be4ed2e8adc3e as libc::c_ulonglong as uint64_t,
        0xfe7be06355cd9c9 as libc::c_ulonglong as uint64_t,
        0xee054e6c1d11be83 as libc::c_ulonglong as uint64_t,
        0x1074365909b903a6 as libc::c_ulonglong as uint64_t,
        0x5dde9f80b4813c10 as libc::c_ulonglong as uint64_t,
        0x4a770c7d02b6692c as libc::c_ulonglong as uint64_t,
        0x5379c8d5d7809039 as libc::c_ulonglong as uint64_t,
        0xb4067448161ed409 as libc::c_ulonglong as uint64_t,
        0x5f5e5026183bd6cd as libc::c_ulonglong as uint64_t,
        0xe898029bf4c29df9 as libc::c_ulonglong as uint64_t,
        0x7fb63c940a54d09c as libc::c_ulonglong as uint64_t,
        0xc5171f897f4ba8bc as libc::c_ulonglong as uint64_t,
        0xa6f28db7b31d3d72 as libc::c_ulonglong as uint64_t,
        0x2e4f3be7716eaa78 as libc::c_ulonglong as uint64_t,
        0xd6771a099e63314 as libc::c_ulonglong as uint64_t,
        0x82076254e41bf284 as libc::c_ulonglong as uint64_t,
        0x2f0fd2b42733df98 as libc::c_ulonglong as uint64_t,
        0x5c9e76d3e2dc49f0 as libc::c_ulonglong as uint64_t,
        0x7aeb569619606cdb as libc::c_ulonglong as uint64_t,
        0x83478b07b2468764 as libc::c_ulonglong as uint64_t,
        0xcfadcb8d5923cd32 as libc::c_ulonglong as uint64_t,
        0x85dac7f05b95a41e as libc::c_ulonglong as uint64_t,
        0xb5469d1b4043a1e9 as libc::c_ulonglong as uint64_t,
        0xb821ecbbd9a592fd as libc::c_ulonglong as uint64_t,
        0x1b8e0b0e798c13c8 as libc::c_ulonglong as uint64_t,
        0x62a57b6d9a0be02e as libc::c_ulonglong as uint64_t,
        0xfcf1b793b81257f8 as libc::c_ulonglong as uint64_t,
        0x9d94ea0bd8fe28eb as libc::c_ulonglong as uint64_t,
        0x4cea408aeb654a56 as libc::c_ulonglong as uint64_t,
        0x23284a47e888996c as libc::c_ulonglong as uint64_t,
        0x2d8f1d128b893545 as libc::c_ulonglong as uint64_t,
        0xf4cbac3132c0d8ab as libc::c_ulonglong as uint64_t,
        0xbd7c86b9ca912eba as libc::c_ulonglong as uint64_t,
        0x3a268eef3dbe6079 as libc::c_ulonglong as uint64_t,
        0xf0d62f6077a9110c as libc::c_ulonglong as uint64_t,
        0x2735c916ade150cb as libc::c_ulonglong as uint64_t,
        0x89fd5f03942ee2ea as libc::c_ulonglong as uint64_t,
        0x1acee25d2fd16628 as libc::c_ulonglong as uint64_t,
        0x90f39bab41181bff as libc::c_ulonglong as uint64_t,
        0x430dfe8cde39939f as libc::c_ulonglong as uint64_t,
        0xf70b8ac4c8274796 as libc::c_ulonglong as uint64_t,
        0x1c53aeaac6024552 as libc::c_ulonglong as uint64_t,
        0x13b410acf35e9c9b as libc::c_ulonglong as uint64_t,
        0xa532ab4249faa24f as libc::c_ulonglong as uint64_t,
        0x2b1251e5625a163f as libc::c_ulonglong as uint64_t,
        0xd7e3e676da4841c7 as libc::c_ulonglong as uint64_t,
        0xa7b264e4e5404892 as libc::c_ulonglong as uint64_t,
        0xda8497d643ae72d3 as libc::c_ulonglong as uint64_t,
        0x861ae105a1723b23 as libc::c_ulonglong as uint64_t,
        0x38a6414991048aa4 as libc::c_ulonglong as uint64_t,
        0x6578dec92585b6b4 as libc::c_ulonglong as uint64_t,
        0x280cfa6acbaeadd as libc::c_ulonglong as uint64_t,
        0x88bdb650c273970a as libc::c_ulonglong as uint64_t,
        0x9333bd5ebbff84c2 as libc::c_ulonglong as uint64_t,
        0x4e6a8f2c47dfa08b as libc::c_ulonglong as uint64_t,
        0x321c954db76cef2a as libc::c_ulonglong as uint64_t,
        0x418d312a72837942 as libc::c_ulonglong as uint64_t,
        0xb29b38bfffcdf773 as libc::c_ulonglong as uint64_t,
        0x6c022c38f90a4c07 as libc::c_ulonglong as uint64_t,
        0x5a033a240b0f6a8a as libc::c_ulonglong as uint64_t,
        0x1f93885f3ce5da6f as libc::c_ulonglong as uint64_t,
        0xc38a537e96988bc6 as libc::c_ulonglong as uint64_t,
        0x39e6a81ac759ff44 as libc::c_ulonglong as uint64_t,
        0x29929e43cee0fce2 as libc::c_ulonglong as uint64_t,
        0x40cdd87924de0ca2 as libc::c_ulonglong as uint64_t,
        0xe9d8ebc8a29fe819 as libc::c_ulonglong as uint64_t,
        0xc2798f3cfbb46f4 as libc::c_ulonglong as uint64_t,
        0x55e484223e53b343 as libc::c_ulonglong as uint64_t,
        0x4650948ecd0d2fd8 as libc::c_ulonglong as uint64_t,
        0x20e86cb2126f0651 as libc::c_ulonglong as uint64_t,
        0x6d42c56baf5739e7 as libc::c_ulonglong as uint64_t,
        0xa06fc1405ace1e08 as libc::c_ulonglong as uint64_t,
        0x7babbfc54f3d193b as libc::c_ulonglong as uint64_t,
        0x424d17df8864e67f as libc::c_ulonglong as uint64_t,
        0xd8045870ef14980e as libc::c_ulonglong as uint64_t,
        0xc6d7397c85ac3781 as libc::c_ulonglong as uint64_t,
        0x21a885e1443273b1 as libc::c_ulonglong as uint64_t,
        0x67f8116f893f5c69 as libc::c_ulonglong as uint64_t,
        0x24f5efe35706cff6 as libc::c_ulonglong as uint64_t,
        0xd56329d076f2ab1a as libc::c_ulonglong as uint64_t,
        0x5e1eb9754e66a32d as libc::c_ulonglong as uint64_t,
        0x28d2771098bd8902 as libc::c_ulonglong as uint64_t,
        0x8f6013f47dfdc190 as libc::c_ulonglong as uint64_t,
        0x17a993fdb637553c as libc::c_ulonglong as uint64_t,
        0xe0a219397e1012aa as libc::c_ulonglong as uint64_t,
        0x786b9930b5da8606 as libc::c_ulonglong as uint64_t,
        0x6e82e39e55b0a6da as libc::c_ulonglong as uint64_t,
        0x875a0856f72f4ec3 as libc::c_ulonglong as uint64_t,
        0x3741ff4fa458536d as libc::c_ulonglong as uint64_t,
        0xac4859b3957558fc as libc::c_ulonglong as uint64_t,
        0x7ef6d5c75c09a57c as libc::c_ulonglong as uint64_t,
        0xc04a758b6c7f14fb as libc::c_ulonglong as uint64_t,
        0xf9acdd91ab26ebbf as libc::c_ulonglong as uint64_t,
        0x7391a467c5ef9668 as libc::c_ulonglong as uint64_t,
        0x335c7c1ee1319aca as libc::c_ulonglong as uint64_t,
        0xa91533b18641e4bb as libc::c_ulonglong as uint64_t,
        0xe4bf9a683b79db0d as libc::c_ulonglong as uint64_t,
        0x8e20faa72ba0b470 as libc::c_ulonglong as uint64_t,
        0x51f907737b3a7ae4 as libc::c_ulonglong as uint64_t,
        0x2268a314bed5ec8c as libc::c_ulonglong as uint64_t,
        0xd944b123b949edee as libc::c_ulonglong as uint64_t,
        0x31dcb3b84d8b7017 as libc::c_ulonglong as uint64_t,
        0xd3fe65279f218860 as libc::c_ulonglong as uint64_t,
        0x97af2f1dc8ffab3 as libc::c_ulonglong as uint64_t,
        0x9b09a6fc312d0b91 as libc::c_ulonglong as uint64_t,
        0xcc6ded78a3c4520f as libc::c_ulonglong as uint64_t,
        0x3481d9ba5ebfcc50 as libc::c_ulonglong as uint64_t,
        0x4f2a667f1182d56b as libc::c_ulonglong as uint64_t,
        0xdfd9fdd4509ace94 as libc::c_ulonglong as uint64_t,
        0x26752045fbbc252b as libc::c_ulonglong as uint64_t,
        0xbffc491f662bc467 as libc::c_ulonglong as uint64_t,
        0xdd593272fc202449 as libc::c_ulonglong as uint64_t,
        0x3cbbc218d46d4303 as libc::c_ulonglong as uint64_t,
        0x91b372f817456e1f as libc::c_ulonglong as uint64_t,
        0x681faf69bc6385a0 as libc::c_ulonglong as uint64_t,
        0xb686bbeebaa43ed4 as libc::c_ulonglong as uint64_t,
        0x1469b5084cd0ca01 as libc::c_ulonglong as uint64_t,
        0x98c98009cbca94ac as libc::c_ulonglong as uint64_t,
        0x6438379a73d8c354 as libc::c_ulonglong as uint64_t,
        0xc2caba2dc0c5fe26 as libc::c_ulonglong as uint64_t,
        0x3e3b0dbe78d7a9de as libc::c_ulonglong as uint64_t,
        0x50b9ee202d670f04 as libc::c_ulonglong as uint64_t,
        0x4590b27b37eab0e5 as libc::c_ulonglong as uint64_t,
        0x6025b4cb36b10af3 as libc::c_ulonglong as uint64_t,
        0xfb2c1237079c0162 as libc::c_ulonglong as uint64_t,
        0xa12f28130c936be8 as libc::c_ulonglong as uint64_t,
        0x4b37e52e54eb1ccc as libc::c_ulonglong as uint64_t,
        0x83a1ba28ad28f53 as libc::c_ulonglong as uint64_t,
        0xc10a9cd83a22611b as libc::c_ulonglong as uint64_t,
        0x9f1425ad7444c236 as libc::c_ulonglong as uint64_t,
        0x69d4cf7e9d3237a as libc::c_ulonglong as uint64_t,
        0xedc56899e7f621be as libc::c_ulonglong as uint64_t,
        0x778c273680865fcf as libc::c_ulonglong as uint64_t,
        0x309c5aeb1bd605f7 as libc::c_ulonglong as uint64_t,
        0x8de0dc52d1472b4d as libc::c_ulonglong as uint64_t,
        0xf8ec34c2fd7b9e5f as libc::c_ulonglong as uint64_t,
        0xea18cd3d58787724 as libc::c_ulonglong as uint64_t,
        0xaad515447ca67b86 as libc::c_ulonglong as uint64_t,
        0x9989695a9d97e14c as libc::c_ulonglong as uint64_t,
        0 as libc::c_ulonglong as uint64_t,
        0xf196c63321f464ec as libc::c_ulonglong as uint64_t,
        0x71116bc169557cb5 as libc::c_ulonglong as uint64_t,
        0xaf887f466f92c7c1 as libc::c_ulonglong as uint64_t,
        0x972e3e0ffe964d65 as libc::c_ulonglong as uint64_t,
        0x190ec4a8d536f915 as libc::c_ulonglong as uint64_t,
        0x95aef1a9522ca7b8 as libc::c_ulonglong as uint64_t,
        0xdc19db21aa7d51a9 as libc::c_ulonglong as uint64_t,
        0x94ee18fa0471d258 as libc::c_ulonglong as uint64_t,
        0x8087adf248a11859 as libc::c_ulonglong as uint64_t,
        0xc457f6da2916dd5c as libc::c_ulonglong as uint64_t,
        0xfa6cfb6451c17482 as libc::c_ulonglong as uint64_t,
        0xf256e0c6db13fbd1 as libc::c_ulonglong as uint64_t,
        0x6a9f60cf10d96f7d as libc::c_ulonglong as uint64_t,
        0x4daaa9d9bd383fb6 as libc::c_ulonglong as uint64_t,
        0x3c026f5fae79f3d as libc::c_ulonglong as uint64_t,
        0xde99148706c7bb74 as libc::c_ulonglong as uint64_t,
        0x2a52b8b6340763df as libc::c_ulonglong as uint64_t,
        0x6fc20acd03edd33a as libc::c_ulonglong as uint64_t,
        0xd423c08320afdefa as libc::c_ulonglong as uint64_t,
        0xbbe1ca4e23420dc0 as libc::c_ulonglong as uint64_t,
        0x966ed75ca8cb3885 as libc::c_ulonglong as uint64_t,
        0xeb58246e0e2502c4 as libc::c_ulonglong as uint64_t,
        0x55d6a021334bc47 as libc::c_ulonglong as uint64_t,
        0xa47242111fa7d7af as libc::c_ulonglong as uint64_t,
        0xe3623fcc84f78d97 as libc::c_ulonglong as uint64_t,
        0x81c744a11efc6db9 as libc::c_ulonglong as uint64_t,
        0xaec8961539cfb221 as libc::c_ulonglong as uint64_t,
        0xf31609958d4e8e31 as libc::c_ulonglong as uint64_t,
        0x63e5923ecc5695ce as libc::c_ulonglong as uint64_t,
        0x47107ddd9b505a38 as libc::c_ulonglong as uint64_t,
        0xa3afe7b5a0298135 as libc::c_ulonglong as uint64_t,
        0x792b7063e387f3e6 as libc::c_ulonglong as uint64_t,
        0x140e953565d75e0 as libc::c_ulonglong as uint64_t,
        0x12f4f9ffa503e97b as libc::c_ulonglong as uint64_t,
        0x750ce8902c3cb512 as libc::c_ulonglong as uint64_t,
        0xdbc47e8515f30733 as libc::c_ulonglong as uint64_t,
        0x1ed3610c6ab8af8f as libc::c_ulonglong as uint64_t,
        0x5239218681dde5d9 as libc::c_ulonglong as uint64_t,
        0xe222d69fd2aaf877 as libc::c_ulonglong as uint64_t,
        0xfe71783514a8bd25 as libc::c_ulonglong as uint64_t,
        0xcaf0a18f4a177175 as libc::c_ulonglong as uint64_t,
        0x61655d9860ec7f13 as libc::c_ulonglong as uint64_t,
        0xe77fbc9dc19e4430 as libc::c_ulonglong as uint64_t,
        0x2ccff441ddd440a5 as libc::c_ulonglong as uint64_t,
        0x16e97aaee06a20dc as libc::c_ulonglong as uint64_t,
        0xa855dae2d01c915b as libc::c_ulonglong as uint64_t,
        0x1d1347f9905f30b2 as libc::c_ulonglong as uint64_t,
        0xb7c652bdecf94b34 as libc::c_ulonglong as uint64_t,
        0xd03e43d265c6175d as libc::c_ulonglong as uint64_t,
        0xfdb15ec0ee4f2218 as libc::c_ulonglong as uint64_t,
        0x57644b8492e9599e as libc::c_ulonglong as uint64_t,
        0x7dda5a4bf8e569a as libc::c_ulonglong as uint64_t,
        0x54a46d71680ec6a3 as libc::c_ulonglong as uint64_t,
        0x5624a2d7c4b42c7e as libc::c_ulonglong as uint64_t,
        0xbebca04c3076b187 as libc::c_ulonglong as uint64_t,
        0x7d36f332a6ee3a41 as libc::c_ulonglong as uint64_t,
        0x3b6667bc6be31599 as libc::c_ulonglong as uint64_t,
        0x695f463aea3ef040 as libc::c_ulonglong as uint64_t,
        0xad08b0e0c3282d1c as libc::c_ulonglong as uint64_t,
        0xb15b1e4a052a684e as libc::c_ulonglong as uint64_t,
        0x44d05b2861b7c505 as libc::c_ulonglong as uint64_t,
        0x15295c5b1a8dbfe1 as libc::c_ulonglong as uint64_t,
        0x744c01c37a61c0f2 as libc::c_ulonglong as uint64_t,
        0x59c31cd1f1e8f5b7 as libc::c_ulonglong as uint64_t,
        0xef45a73f4b4ccb63 as libc::c_ulonglong as uint64_t,
        0x6bdf899c46841a9d as libc::c_ulonglong as uint64_t,
        0x3dfb2b4b823036e3 as libc::c_ulonglong as uint64_t,
        0xa2ef0ee6f674f4d5 as libc::c_ulonglong as uint64_t,
        0x184e2dfb836b8cf5 as libc::c_ulonglong as uint64_t,
        0x1134df0a5fe47646 as libc::c_ulonglong as uint64_t,
        0xbaa1231d751f7820 as libc::c_ulonglong as uint64_t,
        0xd17eaa81339b62bd as libc::c_ulonglong as uint64_t,
        0xb01bf71953771dae as libc::c_ulonglong as uint64_t,
        0x849a2ea30dc8d1fe as libc::c_ulonglong as uint64_t,
        0x705182923f080955 as libc::c_ulonglong as uint64_t,
        0xea757556301ac29 as libc::c_ulonglong as uint64_t,
        0x41d83514569c9a7 as libc::c_ulonglong as uint64_t,
        0xabad4042668658e as libc::c_ulonglong as uint64_t,
        0x49b72a88f851f611 as libc::c_ulonglong as uint64_t,
        0x8a3d79f66ec97dd7 as libc::c_ulonglong as uint64_t,
        0xcd2d042bf59927ef as libc::c_ulonglong as uint64_t,
        0xc930877ab0f0ee48 as libc::c_ulonglong as uint64_t,
        0x9273540deda2f122 as libc::c_ulonglong as uint64_t,
        0xc797d02fd3f14261 as libc::c_ulonglong as uint64_t,
        0xe1e2f06a284d674a as libc::c_ulonglong as uint64_t,
        0xd2be8c74c97cfd80 as libc::c_ulonglong as uint64_t,
        0x9a494faf67707e71 as libc::c_ulonglong as uint64_t,
        0xb3dbd1eca9908293 as libc::c_ulonglong as uint64_t,
        0x72d14d3493b2e388 as libc::c_ulonglong as uint64_t,
        0xd6a30f258c153427 as libc::c_ulonglong as uint64_t,
    ],
];
static mut C16: [[uint64_t; 8]; 12] = [
    [
        0xdd806559f2a64507 as libc::c_ulonglong as uint64_t,
        0x5767436cc744d23 as libc::c_ulonglong as uint64_t,
        0xa2422a08a460d315 as libc::c_ulonglong as uint64_t,
        0x4b7ce09192676901 as libc::c_ulonglong as uint64_t,
        0x714eb88d7585c4fc as libc::c_ulonglong as uint64_t,
        0x2f6a76432e45d016 as libc::c_ulonglong as uint64_t,
        0xebcb2f81c0657c1f as libc::c_ulonglong as uint64_t,
        0xb1085bda1ecadae9 as libc::c_ulonglong as uint64_t,
    ],
    [
        0xe679047021b19bb7 as libc::c_ulonglong as uint64_t,
        0x55dda21bd7cbcd56 as libc::c_ulonglong as uint64_t,
        0x5cb561c2db0aa7ca as libc::c_ulonglong as uint64_t,
        0x9ab5176b12d69958 as libc::c_ulonglong as uint64_t,
        0x61d55e0f16b50131 as libc::c_ulonglong as uint64_t,
        0xf3feea720a232b98 as libc::c_ulonglong as uint64_t,
        0x4fe39d460f70b5d7 as libc::c_ulonglong as uint64_t,
        0x6fa3b58aa99d2f1a as libc::c_ulonglong as uint64_t,
    ],
    [
        0x991e96f50aba0ab2 as libc::c_ulonglong as uint64_t,
        0xc2b6f443867adb31 as libc::c_ulonglong as uint64_t,
        0xc1c93a376062db09 as libc::c_ulonglong as uint64_t,
        0xd3e20fe490359eb1 as libc::c_ulonglong as uint64_t,
        0xf2ea7514b1297b7b as libc::c_ulonglong as uint64_t,
        0x6f15e5f529c1f8b as libc::c_ulonglong as uint64_t,
        0xa39fc286a3d8435 as libc::c_ulonglong as uint64_t,
        0xf574dcac2bce2fc7 as libc::c_ulonglong as uint64_t,
    ],
    [
        0x220cbebc84e3d12e as libc::c_ulonglong as uint64_t,
        0x3453eaa193e837f1 as libc::c_ulonglong as uint64_t,
        0xd8b71333935203be as libc::c_ulonglong as uint64_t,
        0xa9d72c82ed03d675 as libc::c_ulonglong as uint64_t,
        0x9d721cad685e353f as libc::c_ulonglong as uint64_t,
        0x488e857e335c3c7d as libc::c_ulonglong as uint64_t,
        0xf948e1a05d71e4dd as libc::c_ulonglong as uint64_t,
        0xef1fdfb3e81566d2 as libc::c_ulonglong as uint64_t,
    ],
    [
        0x601758fd7c6cfe57 as libc::c_ulonglong as uint64_t,
        0x7a56a27ea9ea63f5 as libc::c_ulonglong as uint64_t,
        0xdfff00b723271a16 as libc::c_ulonglong as uint64_t,
        0xbfcd1747253af5a3 as libc::c_ulonglong as uint64_t,
        0x359e35d7800fffbd as libc::c_ulonglong as uint64_t,
        0x7f151c1f1686104a as libc::c_ulonglong as uint64_t,
        0x9a3f410c6ca92363 as libc::c_ulonglong as uint64_t,
        0x4bea6bacad474799 as libc::c_ulonglong as uint64_t,
    ],
    [
        0xfa68407a46647d6e as libc::c_ulonglong as uint64_t,
        0xbf71c57236904f35 as libc::c_ulonglong as uint64_t,
        0xaf21f66c2bec6b6 as libc::c_ulonglong as uint64_t,
        0xcffaa6b71c9ab7b4 as libc::c_ulonglong as uint64_t,
        0x187f9ab49af08ec6 as libc::c_ulonglong as uint64_t,
        0x2d66c4f95142a46c as libc::c_ulonglong as uint64_t,
        0x6fa4c33b7a3039c0 as libc::c_ulonglong as uint64_t,
        0xae4faeae1d3ad3d9 as libc::c_ulonglong as uint64_t,
    ],
    [
        0x8886564d3a14d493 as libc::c_ulonglong as uint64_t,
        0x3517454ca23c4af3 as libc::c_ulonglong as uint64_t,
        0x6476983284a0504 as libc::c_ulonglong as uint64_t,
        0x992abc52d822c37 as libc::c_ulonglong as uint64_t,
        0xd3473e33197a93c9 as libc::c_ulonglong as uint64_t,
        0x399ec6c7e6bf87c9 as libc::c_ulonglong as uint64_t,
        0x51ac86febf240954 as libc::c_ulonglong as uint64_t,
        0xf4c70e16eeaac5ec as libc::c_ulonglong as uint64_t,
    ],
    [
        0xa47f0dd4bf02e71e as libc::c_ulonglong as uint64_t,
        0x36acc2355951a8d9 as libc::c_ulonglong as uint64_t,
        0x69d18d2bd1a5c42f as libc::c_ulonglong as uint64_t,
        0xf4892bcb929b0690 as libc::c_ulonglong as uint64_t,
        0x89b4443b4ddbc49a as libc::c_ulonglong as uint64_t,
        0x4eb7f8719c36de1e as libc::c_ulonglong as uint64_t,
        0x3e7aa020c6e4141 as libc::c_ulonglong as uint64_t,
        0x9b1f5b424d93c9a7 as libc::c_ulonglong as uint64_t,
    ],
    [
        0x7261445183235adb as libc::c_ulonglong as uint64_t,
        0xe38dc92cb1f2a60 as libc::c_ulonglong as uint64_t,
        0x7b2b8a9aa6079c54 as libc::c_ulonglong as uint64_t,
        0x800a440bdbb2ceb1 as libc::c_ulonglong as uint64_t,
        0x3cd955b7e00d0984 as libc::c_ulonglong as uint64_t,
        0x3a7d3a1b25894224 as libc::c_ulonglong as uint64_t,
        0x944c9ad8ec165fde as libc::c_ulonglong as uint64_t,
        0x378f5a541631229b as libc::c_ulonglong as uint64_t,
    ],
    [
        0x74b4c7fb98459ced as libc::c_ulonglong as uint64_t,
        0x3698fad1153bb6c3 as libc::c_ulonglong as uint64_t,
        0x7a1e6c303b7652f4 as libc::c_ulonglong as uint64_t,
        0x9fe76702af69334b as libc::c_ulonglong as uint64_t,
        0x1fffe18a1b336103 as libc::c_ulonglong as uint64_t,
        0x8941e71cff8a78db as libc::c_ulonglong as uint64_t,
        0x382ae548b2e4f3f3 as libc::c_ulonglong as uint64_t,
        0xabbedea680056f52 as libc::c_ulonglong as uint64_t,
    ],
    [
        0x6bcaa4cd81f32d1b as libc::c_ulonglong as uint64_t,
        0xdea2594ac06fd85d as libc::c_ulonglong as uint64_t,
        0xefbacd1d7d476e98 as libc::c_ulonglong as uint64_t,
        0x8a1d71efea48b9ca as libc::c_ulonglong as uint64_t,
        0x2001802114846679 as libc::c_ulonglong as uint64_t,
        0xd8fa6bbbebab0761 as libc::c_ulonglong as uint64_t,
        0x3002c6cd635afe94 as libc::c_ulonglong as uint64_t,
        0x7bcd9ed0efc889fb as libc::c_ulonglong as uint64_t,
    ],
    [
        0x48bc924af11bd720 as libc::c_ulonglong as uint64_t,
        0xfaf417d5d9b21b99 as libc::c_ulonglong as uint64_t,
        0xe71da4aa88e12852 as libc::c_ulonglong as uint64_t,
        0x5d80ef9d1891cc86 as libc::c_ulonglong as uint64_t,
        0xf82012d430219f9b as libc::c_ulonglong as uint64_t,
        0xcda43c32bcdf1d77 as libc::c_ulonglong as uint64_t,
        0xd21380b00449b17a as libc::c_ulonglong as uint64_t,
        0x378ee767f11631ba as libc::c_ulonglong as uint64_t,
    ],
];
unsafe extern "C" fn LPSX(
    mut out: *mut uint64_t,
    mut a: *const uint64_t,
    mut b: *const uint64_t,
) {
    let mut temp: [uint64_t; 8] = [0; 8];
    temp[0 as libc::c_int
        as usize] = *a.offset(0 as libc::c_int as isize)
        ^ *b.offset(0 as libc::c_int as isize);
    temp[1 as libc::c_int
        as usize] = *a.offset(1 as libc::c_int as isize)
        ^ *b.offset(1 as libc::c_int as isize);
    temp[2 as libc::c_int
        as usize] = *a.offset(2 as libc::c_int as isize)
        ^ *b.offset(2 as libc::c_int as isize);
    temp[3 as libc::c_int
        as usize] = *a.offset(3 as libc::c_int as isize)
        ^ *b.offset(3 as libc::c_int as isize);
    temp[4 as libc::c_int
        as usize] = *a.offset(4 as libc::c_int as isize)
        ^ *b.offset(4 as libc::c_int as isize);
    temp[5 as libc::c_int
        as usize] = *a.offset(5 as libc::c_int as isize)
        ^ *b.offset(5 as libc::c_int as isize);
    temp[6 as libc::c_int
        as usize] = *a.offset(6 as libc::c_int as isize)
        ^ *b.offset(6 as libc::c_int as isize);
    temp[7 as libc::c_int
        as usize] = *a.offset(7 as libc::c_int as isize)
        ^ *b.offset(7 as libc::c_int as isize);
    let mut t: uint64_t = 0;
    t = streebog_table[0 as libc::c_int
        as usize][(temp[0 as libc::c_int as usize] >> 0 as libc::c_int * 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as usize];
    t
        ^= streebog_table[1 as libc::c_int
            as usize][(temp[1 as libc::c_int as usize]
            >> 0 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t
        ^= streebog_table[2 as libc::c_int
            as usize][(temp[2 as libc::c_int as usize]
            >> 0 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t
        ^= streebog_table[3 as libc::c_int
            as usize][(temp[3 as libc::c_int as usize]
            >> 0 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t
        ^= streebog_table[4 as libc::c_int
            as usize][(temp[4 as libc::c_int as usize]
            >> 0 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t
        ^= streebog_table[5 as libc::c_int
            as usize][(temp[5 as libc::c_int as usize]
            >> 0 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t
        ^= streebog_table[6 as libc::c_int
            as usize][(temp[6 as libc::c_int as usize]
            >> 0 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t
        ^= streebog_table[7 as libc::c_int
            as usize][(temp[7 as libc::c_int as usize]
            >> 0 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    *out.offset(0 as libc::c_int as isize) = t;
    let mut t_0: uint64_t = 0;
    t_0 = streebog_table[0 as libc::c_int
        as usize][(temp[0 as libc::c_int as usize] >> 1 as libc::c_int * 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_0
        ^= streebog_table[1 as libc::c_int
            as usize][(temp[1 as libc::c_int as usize]
            >> 1 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_0
        ^= streebog_table[2 as libc::c_int
            as usize][(temp[2 as libc::c_int as usize]
            >> 1 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_0
        ^= streebog_table[3 as libc::c_int
            as usize][(temp[3 as libc::c_int as usize]
            >> 1 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_0
        ^= streebog_table[4 as libc::c_int
            as usize][(temp[4 as libc::c_int as usize]
            >> 1 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_0
        ^= streebog_table[5 as libc::c_int
            as usize][(temp[5 as libc::c_int as usize]
            >> 1 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_0
        ^= streebog_table[6 as libc::c_int
            as usize][(temp[6 as libc::c_int as usize]
            >> 1 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_0
        ^= streebog_table[7 as libc::c_int
            as usize][(temp[7 as libc::c_int as usize]
            >> 1 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    *out.offset(1 as libc::c_int as isize) = t_0;
    let mut t_1: uint64_t = 0;
    t_1 = streebog_table[0 as libc::c_int
        as usize][(temp[0 as libc::c_int as usize] >> 2 as libc::c_int * 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_1
        ^= streebog_table[1 as libc::c_int
            as usize][(temp[1 as libc::c_int as usize]
            >> 2 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_1
        ^= streebog_table[2 as libc::c_int
            as usize][(temp[2 as libc::c_int as usize]
            >> 2 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_1
        ^= streebog_table[3 as libc::c_int
            as usize][(temp[3 as libc::c_int as usize]
            >> 2 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_1
        ^= streebog_table[4 as libc::c_int
            as usize][(temp[4 as libc::c_int as usize]
            >> 2 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_1
        ^= streebog_table[5 as libc::c_int
            as usize][(temp[5 as libc::c_int as usize]
            >> 2 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_1
        ^= streebog_table[6 as libc::c_int
            as usize][(temp[6 as libc::c_int as usize]
            >> 2 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_1
        ^= streebog_table[7 as libc::c_int
            as usize][(temp[7 as libc::c_int as usize]
            >> 2 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    *out.offset(2 as libc::c_int as isize) = t_1;
    let mut t_2: uint64_t = 0;
    t_2 = streebog_table[0 as libc::c_int
        as usize][(temp[0 as libc::c_int as usize] >> 3 as libc::c_int * 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_2
        ^= streebog_table[1 as libc::c_int
            as usize][(temp[1 as libc::c_int as usize]
            >> 3 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_2
        ^= streebog_table[2 as libc::c_int
            as usize][(temp[2 as libc::c_int as usize]
            >> 3 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_2
        ^= streebog_table[3 as libc::c_int
            as usize][(temp[3 as libc::c_int as usize]
            >> 3 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_2
        ^= streebog_table[4 as libc::c_int
            as usize][(temp[4 as libc::c_int as usize]
            >> 3 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_2
        ^= streebog_table[5 as libc::c_int
            as usize][(temp[5 as libc::c_int as usize]
            >> 3 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_2
        ^= streebog_table[6 as libc::c_int
            as usize][(temp[6 as libc::c_int as usize]
            >> 3 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_2
        ^= streebog_table[7 as libc::c_int
            as usize][(temp[7 as libc::c_int as usize]
            >> 3 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    *out.offset(3 as libc::c_int as isize) = t_2;
    let mut t_3: uint64_t = 0;
    t_3 = streebog_table[0 as libc::c_int
        as usize][(temp[0 as libc::c_int as usize] >> 4 as libc::c_int * 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_3
        ^= streebog_table[1 as libc::c_int
            as usize][(temp[1 as libc::c_int as usize]
            >> 4 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_3
        ^= streebog_table[2 as libc::c_int
            as usize][(temp[2 as libc::c_int as usize]
            >> 4 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_3
        ^= streebog_table[3 as libc::c_int
            as usize][(temp[3 as libc::c_int as usize]
            >> 4 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_3
        ^= streebog_table[4 as libc::c_int
            as usize][(temp[4 as libc::c_int as usize]
            >> 4 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_3
        ^= streebog_table[5 as libc::c_int
            as usize][(temp[5 as libc::c_int as usize]
            >> 4 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_3
        ^= streebog_table[6 as libc::c_int
            as usize][(temp[6 as libc::c_int as usize]
            >> 4 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_3
        ^= streebog_table[7 as libc::c_int
            as usize][(temp[7 as libc::c_int as usize]
            >> 4 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    *out.offset(4 as libc::c_int as isize) = t_3;
    let mut t_4: uint64_t = 0;
    t_4 = streebog_table[0 as libc::c_int
        as usize][(temp[0 as libc::c_int as usize] >> 5 as libc::c_int * 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_4
        ^= streebog_table[1 as libc::c_int
            as usize][(temp[1 as libc::c_int as usize]
            >> 5 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_4
        ^= streebog_table[2 as libc::c_int
            as usize][(temp[2 as libc::c_int as usize]
            >> 5 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_4
        ^= streebog_table[3 as libc::c_int
            as usize][(temp[3 as libc::c_int as usize]
            >> 5 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_4
        ^= streebog_table[4 as libc::c_int
            as usize][(temp[4 as libc::c_int as usize]
            >> 5 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_4
        ^= streebog_table[5 as libc::c_int
            as usize][(temp[5 as libc::c_int as usize]
            >> 5 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_4
        ^= streebog_table[6 as libc::c_int
            as usize][(temp[6 as libc::c_int as usize]
            >> 5 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_4
        ^= streebog_table[7 as libc::c_int
            as usize][(temp[7 as libc::c_int as usize]
            >> 5 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    *out.offset(5 as libc::c_int as isize) = t_4;
    let mut t_5: uint64_t = 0;
    t_5 = streebog_table[0 as libc::c_int
        as usize][(temp[0 as libc::c_int as usize] >> 6 as libc::c_int * 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_5
        ^= streebog_table[1 as libc::c_int
            as usize][(temp[1 as libc::c_int as usize]
            >> 6 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_5
        ^= streebog_table[2 as libc::c_int
            as usize][(temp[2 as libc::c_int as usize]
            >> 6 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_5
        ^= streebog_table[3 as libc::c_int
            as usize][(temp[3 as libc::c_int as usize]
            >> 6 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_5
        ^= streebog_table[4 as libc::c_int
            as usize][(temp[4 as libc::c_int as usize]
            >> 6 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_5
        ^= streebog_table[5 as libc::c_int
            as usize][(temp[5 as libc::c_int as usize]
            >> 6 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_5
        ^= streebog_table[6 as libc::c_int
            as usize][(temp[6 as libc::c_int as usize]
            >> 6 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_5
        ^= streebog_table[7 as libc::c_int
            as usize][(temp[7 as libc::c_int as usize]
            >> 6 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    *out.offset(6 as libc::c_int as isize) = t_5;
    let mut t_6: uint64_t = 0;
    t_6 = streebog_table[0 as libc::c_int
        as usize][(temp[0 as libc::c_int as usize] >> 7 as libc::c_int * 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_6
        ^= streebog_table[1 as libc::c_int
            as usize][(temp[1 as libc::c_int as usize]
            >> 7 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_6
        ^= streebog_table[2 as libc::c_int
            as usize][(temp[2 as libc::c_int as usize]
            >> 7 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_6
        ^= streebog_table[3 as libc::c_int
            as usize][(temp[3 as libc::c_int as usize]
            >> 7 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_6
        ^= streebog_table[4 as libc::c_int
            as usize][(temp[4 as libc::c_int as usize]
            >> 7 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_6
        ^= streebog_table[5 as libc::c_int
            as usize][(temp[5 as libc::c_int as usize]
            >> 7 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_6
        ^= streebog_table[6 as libc::c_int
            as usize][(temp[6 as libc::c_int as usize]
            >> 7 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    t_6
        ^= streebog_table[7 as libc::c_int
            as usize][(temp[7 as libc::c_int as usize]
            >> 7 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as usize];
    *out.offset(7 as libc::c_int as isize) = t_6;
}
#[inline]
unsafe extern "C" fn g(
    mut h: *mut uint64_t,
    mut m: *mut uint64_t,
    mut N: *mut uint64_t,
) {
    let mut K: [uint64_t; 8] = [0; 8];
    let mut T: [uint64_t; 8] = [0; 8];
    let mut i: libc::c_int = 0;
    LPSX(K.as_mut_ptr(), h, N);
    LPSX(T.as_mut_ptr(), K.as_mut_ptr(), m);
    LPSX(K.as_mut_ptr(), K.as_mut_ptr(), (C16[0 as libc::c_int as usize]).as_ptr());
    i = 1 as libc::c_int;
    while i < 12 as libc::c_int {
        LPSX(T.as_mut_ptr(), K.as_mut_ptr(), T.as_mut_ptr());
        LPSX(K.as_mut_ptr(), K.as_mut_ptr(), (C16[i as usize]).as_ptr());
        i += 1;
        i;
    }
    let ref mut fresh0 = *h.offset(0 as libc::c_int as isize);
    *fresh0
        ^= T[0 as libc::c_int as usize] ^ K[0 as libc::c_int as usize]
            ^ *m.offset(0 as libc::c_int as isize);
    let ref mut fresh1 = *h.offset(1 as libc::c_int as isize);
    *fresh1
        ^= T[1 as libc::c_int as usize] ^ K[1 as libc::c_int as usize]
            ^ *m.offset(1 as libc::c_int as isize);
    let ref mut fresh2 = *h.offset(2 as libc::c_int as isize);
    *fresh2
        ^= T[2 as libc::c_int as usize] ^ K[2 as libc::c_int as usize]
            ^ *m.offset(2 as libc::c_int as isize);
    let ref mut fresh3 = *h.offset(3 as libc::c_int as isize);
    *fresh3
        ^= T[3 as libc::c_int as usize] ^ K[3 as libc::c_int as usize]
            ^ *m.offset(3 as libc::c_int as isize);
    let ref mut fresh4 = *h.offset(4 as libc::c_int as isize);
    *fresh4
        ^= T[4 as libc::c_int as usize] ^ K[4 as libc::c_int as usize]
            ^ *m.offset(4 as libc::c_int as isize);
    let ref mut fresh5 = *h.offset(5 as libc::c_int as isize);
    *fresh5
        ^= T[5 as libc::c_int as usize] ^ K[5 as libc::c_int as usize]
            ^ *m.offset(5 as libc::c_int as isize);
    let ref mut fresh6 = *h.offset(6 as libc::c_int as isize);
    *fresh6
        ^= T[6 as libc::c_int as usize] ^ K[6 as libc::c_int as usize]
            ^ *m.offset(6 as libc::c_int as isize);
    let ref mut fresh7 = *h.offset(7 as libc::c_int as isize);
    *fresh7
        ^= T[7 as libc::c_int as usize] ^ K[7 as libc::c_int as usize]
            ^ *m.offset(7 as libc::c_int as isize);
}
unsafe extern "C" fn streebog512_compress(
    mut ctx: *mut streebog512_ctx,
    mut input: *const uint8_t,
    mut count: uint64_t,
) {
    let mut M: [uint64_t; 8] = [0; 8];
    let mut cf: uint64_t = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        M[i
            as usize] = (*input.offset(7 as libc::c_int as isize) as uint64_t)
            << 56 as libc::c_int
            | (*input.offset(6 as libc::c_int as isize) as uint64_t) << 48 as libc::c_int
            | (*input.offset(5 as libc::c_int as isize) as uint64_t) << 40 as libc::c_int
            | (*input.offset(4 as libc::c_int as isize) as uint64_t) << 32 as libc::c_int
            | (*input.offset(3 as libc::c_int as isize) as uint64_t) << 24 as libc::c_int
            | (*input.offset(2 as libc::c_int as isize) as uint64_t) << 16 as libc::c_int
            | (*input.offset(1 as libc::c_int as isize) as uint64_t) << 8 as libc::c_int
            | *input.offset(0 as libc::c_int as isize) as uint64_t;
        i += 1;
        i;
        input = input.offset(8 as libc::c_int as isize);
    }
    g(((*ctx).state).as_mut_ptr(), M.as_mut_ptr(), ((*ctx).count).as_mut_ptr());
    (*ctx)
        .count[0 as libc::c_int
        as usize] = ((*ctx).count[0 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(count) as uint64_t as uint64_t;
    if (*ctx).count[0 as libc::c_int as usize] < count {
        i = 1 as libc::c_int;
        while i < 8 as libc::c_int {
            (*ctx).count[i as usize] = ((*ctx).count[i as usize]).wrapping_add(1);
            (*ctx).count[i as usize];
            if (*ctx).count[i as usize] != 0 as libc::c_int as libc::c_ulong {
                break;
            }
            i += 1;
            i;
        }
    }
    (*ctx)
        .sigma[0 as libc::c_int
        as usize] = ((*ctx).sigma[0 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(M[0 as libc::c_int as usize]) as uint64_t as uint64_t;
    cf = ((*ctx).sigma[0 as libc::c_int as usize] < M[0 as libc::c_int as usize])
        as libc::c_int as uint64_t;
    i = 1 as libc::c_int;
    while i < 7 as libc::c_int {
        (*ctx)
            .sigma[i
            as usize] = ((*ctx).sigma[i as usize] as libc::c_ulong).wrapping_add(cf)
            as uint64_t as uint64_t;
        cf = ((*ctx).sigma[i as usize] < cf) as libc::c_int as uint64_t;
        (*ctx)
            .sigma[i
            as usize] = ((*ctx).sigma[i as usize] as libc::c_ulong)
            .wrapping_add(M[i as usize]) as uint64_t as uint64_t;
        cf |= ((*ctx).sigma[i as usize] < M[i as usize]) as libc::c_int as libc::c_ulong;
        i += 1;
        i;
    }
    (*ctx)
        .sigma[7 as libc::c_int
        as usize] = ((*ctx).sigma[7 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add((M[7 as libc::c_int as usize]).wrapping_add(cf)) as uint64_t
        as uint64_t;
}
unsafe extern "C" fn streebog_final(mut ctx: *mut streebog512_ctx) {
    let mut Z: [uint64_t; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut i: libc::c_uint = 0;
    i = (*ctx).index;
    let fresh8 = i;
    i = i.wrapping_add(1);
    (*ctx).block[fresh8 as usize] = 1 as libc::c_int as uint8_t;
    while i < 64 as libc::c_int as libc::c_uint {
        let fresh9 = i;
        i = i.wrapping_add(1);
        (*ctx).block[fresh9 as usize] = 0 as libc::c_int as uint8_t;
    }
    streebog512_compress(
        ctx,
        ((*ctx).block).as_mut_ptr(),
        ((*ctx).index).wrapping_mul(8 as libc::c_int as libc::c_uint) as uint64_t,
    );
    g(((*ctx).state).as_mut_ptr(), ((*ctx).count).as_mut_ptr(), Z.as_mut_ptr());
    g(((*ctx).state).as_mut_ptr(), ((*ctx).sigma).as_mut_ptr(), Z.as_mut_ptr());
}
pub unsafe extern "C" fn nettle_streebog512_init(mut ctx: *mut streebog512_ctx) {
    memset(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
    );
    memset(
        ((*ctx).count).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
    );
    memset(
        ((*ctx).sigma).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
    );
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_streebog512_update(
    mut ctx: *mut streebog512_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block_16: u64;
    if (*ctx).index != 0 {
        let mut __md_left: libc::c_uint = (::std::mem::size_of::<[uint8_t; 64]>()
            as libc::c_ulong)
            .wrapping_sub((*ctx).index as libc::c_ulong) as libc::c_uint;
        if length < __md_left as libc::c_ulong {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            (*ctx)
                .index = ((*ctx).index as libc::c_ulong).wrapping_add(length)
                as libc::c_uint as libc::c_uint;
            current_block_16 = 13056961889198038528;
        } else {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                __md_left as libc::c_ulong,
            );
            streebog512_compress(
                ctx,
                ((*ctx).block).as_mut_ptr(),
                (64 as libc::c_int * 8 as libc::c_int) as uint64_t,
            );
            data = data.offset(__md_left as isize);
            length = (length as libc::c_ulong).wrapping_sub(__md_left as libc::c_ulong)
                as size_t as size_t;
            current_block_16 = 5720623009719927633;
        }
    } else {
        current_block_16 = 5720623009719927633;
    }
    match current_block_16 {
        5720623009719927633 => {
            while length >= ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong {
                streebog512_compress(
                    ctx,
                    data,
                    (64 as libc::c_int * 8 as libc::c_int) as uint64_t,
                );
                data = data
                    .offset(
                        ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong as isize,
                    );
                length = (length as libc::c_ulong)
                    .wrapping_sub(
                        ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong,
                    ) as size_t as size_t;
            }
            memcpy(
                ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            (*ctx).index = length as libc::c_uint;
        }
        _ => {}
    };
}
unsafe extern "C" fn streebog512_write_digest(
    mut ctx: *mut streebog512_ctx,
    mut offset: size_t,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    if offset.wrapping_add(length) <= 64 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"offset + length <= STREEBOG512_DIGEST_SIZE\0" as *const u8
                as *const libc::c_char,
            b"streebog.c\0" as *const u8 as *const libc::c_char,
            1276 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"void streebog512_write_digest(struct streebog512_ctx *, size_t, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8711: {
        if offset.wrapping_add(length) <= 64 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"offset + length <= STREEBOG512_DIGEST_SIZE\0" as *const u8
                    as *const libc::c_char,
                b"streebog.c\0" as *const u8 as *const libc::c_char,
                1276 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"void streebog512_write_digest(struct streebog512_ctx *, size_t, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    streebog_final(ctx);
    _nettle_write_le64(
        length,
        digest,
        ((*ctx).state).as_mut_ptr().offset(offset as isize),
    );
}
pub unsafe extern "C" fn nettle_streebog512_digest(
    mut ctx: *mut streebog512_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    if length <= 64 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= STREEBOG512_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"streebog.c\0" as *const u8 as *const libc::c_char,
            1288 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void nettle_streebog512_digest(struct streebog512_ctx *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8760: {
        if length <= 64 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= STREEBOG512_DIGEST_SIZE\0" as *const u8
                    as *const libc::c_char,
                b"streebog.c\0" as *const u8 as *const libc::c_char,
                1288 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void nettle_streebog512_digest(struct streebog512_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    streebog512_write_digest(ctx, 0 as libc::c_int as size_t, length, digest);
    nettle_streebog512_init(ctx);
}
pub unsafe extern "C" fn nettle_streebog256_init(mut ctx: *mut streebog512_ctx) {
    memset(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int,
        ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
    );
    memset(
        ((*ctx).count).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
    );
    memset(
        ((*ctx).sigma).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
    );
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_streebog256_digest(
    mut ctx: *mut streebog512_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    if length <= 32 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= STREEBOG256_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"streebog.c\0" as *const u8 as *const libc::c_char,
            1310 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void nettle_streebog256_digest(struct streebog512_ctx *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8880: {
        if length <= 32 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= STREEBOG256_DIGEST_SIZE\0" as *const u8
                    as *const libc::c_char,
                b"streebog.c\0" as *const u8 as *const libc::c_char,
                1310 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void nettle_streebog256_digest(struct streebog512_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    streebog512_write_digest(ctx, 4 as libc::c_int as size_t, length, digest);
    nettle_streebog256_init(ctx);
}
