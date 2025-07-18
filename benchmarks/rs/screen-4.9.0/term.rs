use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term {
    pub tcname: *mut libc::c_char,
    pub type_0: libc::c_int,
}
pub static mut term: [term; 201] = [
    {
        let mut init = term {
            tcname: b"li\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"co\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"hc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"os\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ns\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"cm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ho\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"cr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"up\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"UP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"do\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"DO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"bs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"bc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"le\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"LE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"nd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"RI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"cs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"nl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"sf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"sr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"al\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"AL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"dl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"DL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"in\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"im\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ei\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"IC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"dc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"DC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ut\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"cl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"cd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"CD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ce\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"cb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"is\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ti\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"te\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"bl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"vb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"WS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"Z0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"Z1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"mh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"us\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"md\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"mr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"so\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"mb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"se\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"me\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"sg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ug\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"sa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"AF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"AB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"Sf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"Sb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"op\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"Co\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"be\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"AX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"C8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ks\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ke\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"CS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"CE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"po\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"pf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"hs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ws\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"fs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ds\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"vi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"vs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"am\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"xv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"xn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"OP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"LP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"NF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"nx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"AN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"OL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"KJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"VR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"VN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"TF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"XT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"G0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"S0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"E0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"C0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"as\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ae\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ac\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"eA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"XC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"Km\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"k0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"k1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"k2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"k3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"k4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"k5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"k6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"k7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"k8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"k9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"k;\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"F1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"F2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"F3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"F4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"F5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"F6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"F7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"F8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"F9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"FA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"FB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"FC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"FD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"FE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"K1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"K2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"K3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"K4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"K5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ka\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"*4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"*7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"#2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"#3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"#4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"%c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"%e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"@1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"@7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ku\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"kl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f+\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"fq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"f,\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"fe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"km\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"ko\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"l0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"l1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"l2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"l3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"l4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"l5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"l6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"l7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"l8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"l9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = term {
            tcname: b"la\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
];
