use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ag_malloc(size: size_t) -> *mut libc::c_void;
    fn ag_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_spec_t {
    pub name: *const libc::c_char,
    pub extensions: [*const libc::c_char; 12],
}
pub static mut langs: [lang_spec_t; 138] = [
    {
        let mut init = lang_spec_t {
            name: b"actionscript\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"as\0" as *const u8 as *const libc::c_char,
                b"mxml\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"ada\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ada\0" as *const u8 as *const libc::c_char,
                b"adb\0" as *const u8 as *const libc::c_char,
                b"ads\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"asciidoc\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"adoc\0" as *const u8 as *const libc::c_char,
                b"ad\0" as *const u8 as *const libc::c_char,
                b"asc\0" as *const u8 as *const libc::c_char,
                b"asciidoc\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"apl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"apl\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"asm\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"asm\0" as *const u8 as *const libc::c_char,
                b"s\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"asp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"asp\0" as *const u8 as *const libc::c_char,
                b"asa\0" as *const u8 as *const libc::c_char,
                b"aspx\0" as *const u8 as *const libc::c_char,
                b"asax\0" as *const u8 as *const libc::c_char,
                b"ashx\0" as *const u8 as *const libc::c_char,
                b"ascx\0" as *const u8 as *const libc::c_char,
                b"asmx\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"aspx\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"asp\0" as *const u8 as *const libc::c_char,
                b"asa\0" as *const u8 as *const libc::c_char,
                b"aspx\0" as *const u8 as *const libc::c_char,
                b"asax\0" as *const u8 as *const libc::c_char,
                b"ashx\0" as *const u8 as *const libc::c_char,
                b"ascx\0" as *const u8 as *const libc::c_char,
                b"asmx\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"batch\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"bat\0" as *const u8 as *const libc::c_char,
                b"cmd\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"bazel\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"bazel\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"bitbake\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"bb\0" as *const u8 as *const libc::c_char,
                b"bbappend\0" as *const u8 as *const libc::c_char,
                b"bbclass\0" as *const u8 as *const libc::c_char,
                b"inc\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"cc\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"c\0" as *const u8 as *const libc::c_char,
                b"h\0" as *const u8 as *const libc::c_char,
                b"xs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"cfmx\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cfc\0" as *const u8 as *const libc::c_char,
                b"cfm\0" as *const u8 as *const libc::c_char,
                b"cfml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"chpl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"chpl\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"clojure\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"clj\0" as *const u8 as *const libc::c_char,
                b"cljs\0" as *const u8 as *const libc::c_char,
                b"cljc\0" as *const u8 as *const libc::c_char,
                b"cljx\0" as *const u8 as *const libc::c_char,
                b"edn\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"coffee\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"coffee\0" as *const u8 as *const libc::c_char,
                b"cjsx\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"config\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"config\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"coq\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"coq\0" as *const u8 as *const libc::c_char,
                b"g\0" as *const u8 as *const libc::c_char,
                b"v\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"cpp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cpp\0" as *const u8 as *const libc::c_char,
                b"cc\0" as *const u8 as *const libc::c_char,
                b"C\0" as *const u8 as *const libc::c_char,
                b"cxx\0" as *const u8 as *const libc::c_char,
                b"m\0" as *const u8 as *const libc::c_char,
                b"hpp\0" as *const u8 as *const libc::c_char,
                b"hh\0" as *const u8 as *const libc::c_char,
                b"h\0" as *const u8 as *const libc::c_char,
                b"H\0" as *const u8 as *const libc::c_char,
                b"hxx\0" as *const u8 as *const libc::c_char,
                b"tpp\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"crystal\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cr\0" as *const u8 as *const libc::c_char,
                b"ecr\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"csharp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cs\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"cshtml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cshtml\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"css\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"css\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"cython\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pyx\0" as *const u8 as *const libc::c_char,
                b"pxd\0" as *const u8 as *const libc::c_char,
                b"pxi\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"delphi\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pas\0" as *const u8 as *const libc::c_char,
                b"int\0" as *const u8 as *const libc::c_char,
                b"dfm\0" as *const u8 as *const libc::c_char,
                b"nfm\0" as *const u8 as *const libc::c_char,
                b"dof\0" as *const u8 as *const libc::c_char,
                b"dpk\0" as *const u8 as *const libc::c_char,
                b"dpr\0" as *const u8 as *const libc::c_char,
                b"dproj\0" as *const u8 as *const libc::c_char,
                b"groupproj\0" as *const u8 as *const libc::c_char,
                b"bdsgroup\0" as *const u8 as *const libc::c_char,
                b"bdsproj\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"dlang\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"d\0" as *const u8 as *const libc::c_char,
                b"di\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"dot\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"dot\0" as *const u8 as *const libc::c_char,
                b"gv\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"dts\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"dts\0" as *const u8 as *const libc::c_char,
                b"dtsi\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"ebuild\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ebuild\0" as *const u8 as *const libc::c_char,
                b"eclass\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"elisp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"el\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"elixir\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ex\0" as *const u8 as *const libc::c_char,
                b"eex\0" as *const u8 as *const libc::c_char,
                b"exs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"elm\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"elm\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"erlang\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"erl\0" as *const u8 as *const libc::c_char,
                b"hrl\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"factor\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"factor\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"fortran\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"f\0" as *const u8 as *const libc::c_char,
                b"F\0" as *const u8 as *const libc::c_char,
                b"f77\0" as *const u8 as *const libc::c_char,
                b"f90\0" as *const u8 as *const libc::c_char,
                b"F90\0" as *const u8 as *const libc::c_char,
                b"f95\0" as *const u8 as *const libc::c_char,
                b"f03\0" as *const u8 as *const libc::c_char,
                b"for\0" as *const u8 as *const libc::c_char,
                b"ftn\0" as *const u8 as *const libc::c_char,
                b"fpp\0" as *const u8 as *const libc::c_char,
                b"FPP\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"fsharp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"fs\0" as *const u8 as *const libc::c_char,
                b"fsi\0" as *const u8 as *const libc::c_char,
                b"fsx\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"gettext\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"po\0" as *const u8 as *const libc::c_char,
                b"pot\0" as *const u8 as *const libc::c_char,
                b"mo\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"glsl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vert\0" as *const u8 as *const libc::c_char,
                b"tesc\0" as *const u8 as *const libc::c_char,
                b"tese\0" as *const u8 as *const libc::c_char,
                b"geom\0" as *const u8 as *const libc::c_char,
                b"frag\0" as *const u8 as *const libc::c_char,
                b"comp\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"go\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"go\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"gradle\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"gradle\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"groovy\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"groovy\0" as *const u8 as *const libc::c_char,
                b"gtmpl\0" as *const u8 as *const libc::c_char,
                b"gpp\0" as *const u8 as *const libc::c_char,
                b"grunit\0" as *const u8 as *const libc::c_char,
                b"gradle\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"haml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"haml\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"handlebars\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"hbs\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"haskell\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"hs\0" as *const u8 as *const libc::c_char,
                b"hsig\0" as *const u8 as *const libc::c_char,
                b"lhs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"haxe\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"hx\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"hh\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"h\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"html\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"htm\0" as *const u8 as *const libc::c_char,
                b"html\0" as *const u8 as *const libc::c_char,
                b"shtml\0" as *const u8 as *const libc::c_char,
                b"xhtml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"idris\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"idr\0" as *const u8 as *const libc::c_char,
                b"ipkg\0" as *const u8 as *const libc::c_char,
                b"lidr\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"ini\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ini\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"ipython\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ipynb\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"isabelle\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"thy\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"j\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ijs\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"jade\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"jade\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"java\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"java\0" as *const u8 as *const libc::c_char,
                b"properties\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"jinja2\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"j2\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"js\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"es6\0" as *const u8 as *const libc::c_char,
                b"js\0" as *const u8 as *const libc::c_char,
                b"jsx\0" as *const u8 as *const libc::c_char,
                b"vue\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"json\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"json\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"jsp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"jsp\0" as *const u8 as *const libc::c_char,
                b"jspx\0" as *const u8 as *const libc::c_char,
                b"jhtm\0" as *const u8 as *const libc::c_char,
                b"jhtml\0" as *const u8 as *const libc::c_char,
                b"jspf\0" as *const u8 as *const libc::c_char,
                b"tag\0" as *const u8 as *const libc::c_char,
                b"tagf\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"julia\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"jl\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"kotlin\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"kt\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"less\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"less\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"liquid\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"liquid\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"lisp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"lisp\0" as *const u8 as *const libc::c_char,
                b"lsp\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"log\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"log\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"lua\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"lua\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"m4\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m4\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"make\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"Makefiles\0" as *const u8 as *const libc::c_char,
                b"mk\0" as *const u8 as *const libc::c_char,
                b"mak\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"mako\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"mako\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"markdown\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"markdown\0" as *const u8 as *const libc::c_char,
                b"mdown\0" as *const u8 as *const libc::c_char,
                b"mdwn\0" as *const u8 as *const libc::c_char,
                b"mkdn\0" as *const u8 as *const libc::c_char,
                b"mkd\0" as *const u8 as *const libc::c_char,
                b"md\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"mason\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"mas\0" as *const u8 as *const libc::c_char,
                b"mhtml\0" as *const u8 as *const libc::c_char,
                b"mpl\0" as *const u8 as *const libc::c_char,
                b"mtxt\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"matlab\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"mathematica\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m\0" as *const u8 as *const libc::c_char,
                b"wl\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"md\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"markdown\0" as *const u8 as *const libc::c_char,
                b"mdown\0" as *const u8 as *const libc::c_char,
                b"mdwn\0" as *const u8 as *const libc::c_char,
                b"mkdn\0" as *const u8 as *const libc::c_char,
                b"mkd\0" as *const u8 as *const libc::c_char,
                b"md\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"mercury\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m\0" as *const u8 as *const libc::c_char,
                b"moo\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"naccess\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"asa\0" as *const u8 as *const libc::c_char,
                b"rsa\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"nim\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"nim\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"nix\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"nix\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"objc\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m\0" as *const u8 as *const libc::c_char,
                b"h\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"objcpp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"mm\0" as *const u8 as *const libc::c_char,
                b"h\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"ocaml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ml\0" as *const u8 as *const libc::c_char,
                b"mli\0" as *const u8 as *const libc::c_char,
                b"mll\0" as *const u8 as *const libc::c_char,
                b"mly\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"octave\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"org\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"org\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"parrot\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pir\0" as *const u8 as *const libc::c_char,
                b"pasm\0" as *const u8 as *const libc::c_char,
                b"pmc\0" as *const u8 as *const libc::c_char,
                b"ops\0" as *const u8 as *const libc::c_char,
                b"pod\0" as *const u8 as *const libc::c_char,
                b"pg\0" as *const u8 as *const libc::c_char,
                b"tg\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"pdb\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pdb\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"perl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pl\0" as *const u8 as *const libc::c_char,
                b"pm\0" as *const u8 as *const libc::c_char,
                b"pm6\0" as *const u8 as *const libc::c_char,
                b"pod\0" as *const u8 as *const libc::c_char,
                b"t\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"php\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"php\0" as *const u8 as *const libc::c_char,
                b"phpt\0" as *const u8 as *const libc::c_char,
                b"php3\0" as *const u8 as *const libc::c_char,
                b"php4\0" as *const u8 as *const libc::c_char,
                b"php5\0" as *const u8 as *const libc::c_char,
                b"phtml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"pike\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pike\0" as *const u8 as *const libc::c_char,
                b"pmod\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"plist\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"plist\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"plone\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pt\0" as *const u8 as *const libc::c_char,
                b"cpt\0" as *const u8 as *const libc::c_char,
                b"metadata\0" as *const u8 as *const libc::c_char,
                b"cpy\0" as *const u8 as *const libc::c_char,
                b"py\0" as *const u8 as *const libc::c_char,
                b"xml\0" as *const u8 as *const libc::c_char,
                b"zcml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"powershell\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ps1\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"proto\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"proto\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"ps1\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ps1\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"pug\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pug\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"puppet\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pp\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"python\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"py\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"qml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"qml\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"racket\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rkt\0" as *const u8 as *const libc::c_char,
                b"ss\0" as *const u8 as *const libc::c_char,
                b"scm\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"rake\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"Rakefile\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"razor\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cshtml\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"restructuredtext\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rst\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"rs\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rs\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"r\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"r\0" as *const u8 as *const libc::c_char,
                b"R\0" as *const u8 as *const libc::c_char,
                b"Rmd\0" as *const u8 as *const libc::c_char,
                b"Rnw\0" as *const u8 as *const libc::c_char,
                b"Rtex\0" as *const u8 as *const libc::c_char,
                b"Rrst\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"rdoc\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rdoc\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"ruby\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rb\0" as *const u8 as *const libc::c_char,
                b"rhtml\0" as *const u8 as *const libc::c_char,
                b"rjs\0" as *const u8 as *const libc::c_char,
                b"rxml\0" as *const u8 as *const libc::c_char,
                b"erb\0" as *const u8 as *const libc::c_char,
                b"rake\0" as *const u8 as *const libc::c_char,
                b"spec\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"rust\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rs\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"salt\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"sls\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"sass\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"sass\0" as *const u8 as *const libc::c_char,
                b"scss\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"scala\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"scala\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"scheme\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"scm\0" as *const u8 as *const libc::c_char,
                b"ss\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"shell\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"sh\0" as *const u8 as *const libc::c_char,
                b"bash\0" as *const u8 as *const libc::c_char,
                b"csh\0" as *const u8 as *const libc::c_char,
                b"tcsh\0" as *const u8 as *const libc::c_char,
                b"ksh\0" as *const u8 as *const libc::c_char,
                b"zsh\0" as *const u8 as *const libc::c_char,
                b"fish\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"smalltalk\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"st\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"sml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"sml\0" as *const u8 as *const libc::c_char,
                b"fun\0" as *const u8 as *const libc::c_char,
                b"mlb\0" as *const u8 as *const libc::c_char,
                b"sig\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"sql\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"sql\0" as *const u8 as *const libc::c_char,
                b"ctl\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"stata\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"do\0" as *const u8 as *const libc::c_char,
                b"ado\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"stylus\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"styl\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"swift\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"swift\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"tcl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"tcl\0" as *const u8 as *const libc::c_char,
                b"itcl\0" as *const u8 as *const libc::c_char,
                b"itk\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"terraform\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"tf\0" as *const u8 as *const libc::c_char,
                b"tfvars\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"tex\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"tex\0" as *const u8 as *const libc::c_char,
                b"cls\0" as *const u8 as *const libc::c_char,
                b"sty\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"thrift\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"thrift\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"tla\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"tla\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"tt\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"tt\0" as *const u8 as *const libc::c_char,
                b"tt2\0" as *const u8 as *const libc::c_char,
                b"ttml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"toml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"toml\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"ts\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ts\0" as *const u8 as *const libc::c_char,
                b"tsx\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"twig\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"twig\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"vala\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vala\0" as *const u8 as *const libc::c_char,
                b"vapi\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"vb\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"bas\0" as *const u8 as *const libc::c_char,
                b"cls\0" as *const u8 as *const libc::c_char,
                b"frm\0" as *const u8 as *const libc::c_char,
                b"ctl\0" as *const u8 as *const libc::c_char,
                b"vb\0" as *const u8 as *const libc::c_char,
                b"resx\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"velocity\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vm\0" as *const u8 as *const libc::c_char,
                b"vtl\0" as *const u8 as *const libc::c_char,
                b"vsl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"verilog\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"v\0" as *const u8 as *const libc::c_char,
                b"vh\0" as *const u8 as *const libc::c_char,
                b"sv\0" as *const u8 as *const libc::c_char,
                b"svh\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"vhdl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vhd\0" as *const u8 as *const libc::c_char,
                b"vhdl\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"vim\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vim\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"vue\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vue\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"wix\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"wxi\0" as *const u8 as *const libc::c_char,
                b"wxs\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"wsdl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"wsdl\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"wadl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"wadl\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"xml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"xml\0" as *const u8 as *const libc::c_char,
                b"dtd\0" as *const u8 as *const libc::c_char,
                b"xsl\0" as *const u8 as *const libc::c_char,
                b"xslt\0" as *const u8 as *const libc::c_char,
                b"xsd\0" as *const u8 as *const libc::c_char,
                b"ent\0" as *const u8 as *const libc::c_char,
                b"tld\0" as *const u8 as *const libc::c_char,
                b"plist\0" as *const u8 as *const libc::c_char,
                b"wsdl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"yaml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"yaml\0" as *const u8 as *const libc::c_char,
                b"yml\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"zeek\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"zeek\0" as *const u8 as *const libc::c_char,
                b"bro\0" as *const u8 as *const libc::c_char,
                b"bif\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = lang_spec_t {
            name: b"zephir\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"zep\0" as *const u8 as *const libc::c_char,
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
            ],
        };
        init
    },
];
pub unsafe extern "C" fn get_lang_count() -> size_t {
    return (::std::mem::size_of::<[lang_spec_t; 138]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<lang_spec_t>() as libc::c_ulong);
}
pub unsafe extern "C" fn make_lang_regex(
    mut ext_array: *mut libc::c_char,
    mut num_exts: size_t,
) -> *mut libc::c_char {
    let mut regex_capacity: libc::c_int = 100 as libc::c_int;
    let mut regex: *mut libc::c_char = ag_malloc(regex_capacity as size_t)
        as *mut libc::c_char;
    let mut regex_length: libc::c_int = 3 as libc::c_int;
    let mut subsequent: libc::c_int = 0 as libc::c_int;
    let mut extension: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    strcpy(regex, b"\\.(\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i < num_exts {
        extension = ext_array
            .offset(i.wrapping_mul(20 as libc::c_int as libc::c_ulong) as isize);
        let mut extension_length: libc::c_int = strlen(extension) as libc::c_int;
        while regex_length + extension_length + 3 as libc::c_int + subsequent
            > regex_capacity
        {
            regex_capacity *= 2 as libc::c_int;
            regex = ag_realloc(regex as *mut libc::c_void, regex_capacity as size_t)
                as *mut libc::c_char;
        }
        if subsequent != 0 {
            let fresh0 = regex_length;
            regex_length = regex_length + 1;
            *regex.offset(fresh0 as isize) = '|' as i32 as libc::c_char;
        } else {
            subsequent = 1 as libc::c_int;
        }
        strcpy(regex.offset(regex_length as isize), extension);
        regex_length += extension_length;
        i = i.wrapping_add(1);
        i;
    }
    let fresh1 = regex_length;
    regex_length = regex_length + 1;
    *regex.offset(fresh1 as isize) = ')' as i32 as libc::c_char;
    let fresh2 = regex_length;
    regex_length = regex_length + 1;
    *regex.offset(fresh2 as isize) = '$' as i32 as libc::c_char;
    let fresh3 = regex_length;
    regex_length = regex_length + 1;
    *regex.offset(fresh3 as isize) = 0 as libc::c_int as libc::c_char;
    return regex;
}
pub unsafe extern "C" fn combine_file_extensions(
    mut extension_index: *mut size_t,
    mut len: size_t,
    mut exts: *mut *mut libc::c_char,
) -> size_t {
    let mut ext_capacity: size_t = 100 as libc::c_int as size_t;
    *exts = ag_malloc(ext_capacity.wrapping_mul(20 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    memset(
        *exts as *mut libc::c_void,
        0 as libc::c_int,
        ext_capacity.wrapping_mul(20 as libc::c_int as libc::c_ulong),
    );
    let mut num_of_extensions: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        let mut j: size_t = 0 as libc::c_int as size_t;
        let mut ext: *const libc::c_char = langs[*extension_index.offset(i as isize)
                as usize]
            .extensions[j as usize];
        while !(num_of_extensions == ext_capacity) {
            let mut pos: *mut libc::c_char = (*exts)
                .offset(
                    num_of_extensions.wrapping_mul(20 as libc::c_int as libc::c_ulong)
                        as isize,
                );
            strncpy(pos, ext, strlen(ext));
            num_of_extensions = num_of_extensions.wrapping_add(1);
            num_of_extensions;
            j = j.wrapping_add(1);
            ext = langs[*extension_index.offset(i as isize) as usize]
                .extensions[j as usize];
            if ext.is_null() {
                break;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return num_of_extensions;
}
