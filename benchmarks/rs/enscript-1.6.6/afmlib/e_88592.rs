use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encoding_table_st {
    pub code: libc::c_int,
    pub character: *mut libc::c_char,
}
pub type AFMEncodingTable = encoding_table_st;
pub static mut afm_88592_encoding: [AFMEncodingTable; 257] = [
    {
        let mut init = encoding_table_st {
            code: 0 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x10 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x11 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x12 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x13 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x14 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x15 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x16 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x17 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x18 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x19 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1a as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1b as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1c as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1d as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1e as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x1f as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x20 as libc::c_int,
            character: b"space\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x21 as libc::c_int,
            character: b"exclam\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x22 as libc::c_int,
            character: b"quotedbl\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x23 as libc::c_int,
            character: b"numbersign\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x24 as libc::c_int,
            character: b"dollar\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x25 as libc::c_int,
            character: b"percent\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x26 as libc::c_int,
            character: b"ampersand\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x27 as libc::c_int,
            character: b"quoteright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x28 as libc::c_int,
            character: b"parenleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x29 as libc::c_int,
            character: b"parenright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2a as libc::c_int,
            character: b"asterisk\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2b as libc::c_int,
            character: b"plus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2c as libc::c_int,
            character: b"comma\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2d as libc::c_int,
            character: b"hyphen\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2e as libc::c_int,
            character: b"period\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x2f as libc::c_int,
            character: b"slash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x30 as libc::c_int,
            character: b"zero\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x31 as libc::c_int,
            character: b"one\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x32 as libc::c_int,
            character: b"two\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x33 as libc::c_int,
            character: b"three\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x34 as libc::c_int,
            character: b"four\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x35 as libc::c_int,
            character: b"five\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x36 as libc::c_int,
            character: b"six\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x37 as libc::c_int,
            character: b"seven\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x38 as libc::c_int,
            character: b"eight\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x39 as libc::c_int,
            character: b"nine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3a as libc::c_int,
            character: b"colon\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3b as libc::c_int,
            character: b"semicolon\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3c as libc::c_int,
            character: b"less\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3d as libc::c_int,
            character: b"equal\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3e as libc::c_int,
            character: b"greater\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x3f as libc::c_int,
            character: b"question\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x40 as libc::c_int,
            character: b"at\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x41 as libc::c_int,
            character: b"A\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x42 as libc::c_int,
            character: b"B\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x43 as libc::c_int,
            character: b"C\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x44 as libc::c_int,
            character: b"D\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x45 as libc::c_int,
            character: b"E\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x46 as libc::c_int,
            character: b"F\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x47 as libc::c_int,
            character: b"G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x48 as libc::c_int,
            character: b"H\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x49 as libc::c_int,
            character: b"I\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4a as libc::c_int,
            character: b"J\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4b as libc::c_int,
            character: b"K\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4c as libc::c_int,
            character: b"L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4d as libc::c_int,
            character: b"M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4e as libc::c_int,
            character: b"N\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x4f as libc::c_int,
            character: b"O\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x50 as libc::c_int,
            character: b"P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x51 as libc::c_int,
            character: b"Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x52 as libc::c_int,
            character: b"R\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x53 as libc::c_int,
            character: b"S\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x54 as libc::c_int,
            character: b"T\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x55 as libc::c_int,
            character: b"U\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x56 as libc::c_int,
            character: b"V\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x57 as libc::c_int,
            character: b"W\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x58 as libc::c_int,
            character: b"X\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x59 as libc::c_int,
            character: b"Y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5a as libc::c_int,
            character: b"Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5b as libc::c_int,
            character: b"bracketleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5c as libc::c_int,
            character: b"backslash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5d as libc::c_int,
            character: b"bracketright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5e as libc::c_int,
            character: b"asciicircum\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x5f as libc::c_int,
            character: b"underscore\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x60 as libc::c_int,
            character: b"quoteleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x61 as libc::c_int,
            character: b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x62 as libc::c_int,
            character: b"b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x63 as libc::c_int,
            character: b"c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x64 as libc::c_int,
            character: b"d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x65 as libc::c_int,
            character: b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x66 as libc::c_int,
            character: b"f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x67 as libc::c_int,
            character: b"g\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x68 as libc::c_int,
            character: b"h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x69 as libc::c_int,
            character: b"i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6a as libc::c_int,
            character: b"j\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6b as libc::c_int,
            character: b"k\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6c as libc::c_int,
            character: b"l\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6d as libc::c_int,
            character: b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6e as libc::c_int,
            character: b"n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x6f as libc::c_int,
            character: b"o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x70 as libc::c_int,
            character: b"p\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x71 as libc::c_int,
            character: b"q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x72 as libc::c_int,
            character: b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x73 as libc::c_int,
            character: b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x74 as libc::c_int,
            character: b"t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x75 as libc::c_int,
            character: b"u\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x76 as libc::c_int,
            character: b"v\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x77 as libc::c_int,
            character: b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x78 as libc::c_int,
            character: b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x79 as libc::c_int,
            character: b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7a as libc::c_int,
            character: b"z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7b as libc::c_int,
            character: b"braceleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7c as libc::c_int,
            character: b"bar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7d as libc::c_int,
            character: b"braceright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7e as libc::c_int,
            character: b"tilde\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x7f as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x80 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x81 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x82 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x83 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x84 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x85 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x86 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x87 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x88 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x89 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8a as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8b as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8c as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8d as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8e as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x8f as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x90 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x91 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x92 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x93 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x94 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x95 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x96 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x97 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x98 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x99 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9a as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9b as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9c as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9d as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9e as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0x9f as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa0 as libc::c_int,
            character: b"space\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa1 as libc::c_int,
            character: b"Aogonek\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa2 as libc::c_int,
            character: b"breve\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa3 as libc::c_int,
            character: b"Lslash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa4 as libc::c_int,
            character: b"currency\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa5 as libc::c_int,
            character: b"Lcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa6 as libc::c_int,
            character: b"Sacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa7 as libc::c_int,
            character: b"section\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa8 as libc::c_int,
            character: b"dieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xa9 as libc::c_int,
            character: b"Scaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xaa as libc::c_int,
            character: b"Scedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xab as libc::c_int,
            character: b"Tcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xac as libc::c_int,
            character: b"Zacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xad as libc::c_int,
            character: b"hyphen\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xae as libc::c_int,
            character: b"Zcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xaf as libc::c_int,
            character: b"Zdotaccent\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb0 as libc::c_int,
            character: b"degree\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb1 as libc::c_int,
            character: b"aogonek\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb2 as libc::c_int,
            character: b"ogonek\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb3 as libc::c_int,
            character: b"lslash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb4 as libc::c_int,
            character: b"acute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb5 as libc::c_int,
            character: b"lcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb6 as libc::c_int,
            character: b"sacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb7 as libc::c_int,
            character: b"caron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb8 as libc::c_int,
            character: b"cedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xb9 as libc::c_int,
            character: b"scaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xba as libc::c_int,
            character: b"scedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbb as libc::c_int,
            character: b"tcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbc as libc::c_int,
            character: b"zacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbd as libc::c_int,
            character: b"hungarumlaut\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbe as libc::c_int,
            character: b"zcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xbf as libc::c_int,
            character: b"zdotaccent\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc0 as libc::c_int,
            character: b"Racute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc1 as libc::c_int,
            character: b"Aacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc2 as libc::c_int,
            character: b"Acircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc3 as libc::c_int,
            character: b"Abreve\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc4 as libc::c_int,
            character: b"Adieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc5 as libc::c_int,
            character: b"Lacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc6 as libc::c_int,
            character: b"Cacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc7 as libc::c_int,
            character: b"Ccedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc8 as libc::c_int,
            character: b"Ccaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xc9 as libc::c_int,
            character: b"Eacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xca as libc::c_int,
            character: b"Eogonek\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcb as libc::c_int,
            character: b"Edieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcc as libc::c_int,
            character: b"Ecaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcd as libc::c_int,
            character: b"Iacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xce as libc::c_int,
            character: b"Icircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xcf as libc::c_int,
            character: b"Dcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd0 as libc::c_int,
            character: b"Eth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd1 as libc::c_int,
            character: b"Nacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd2 as libc::c_int,
            character: b"Ncaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd3 as libc::c_int,
            character: b"Oacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd4 as libc::c_int,
            character: b"Ocircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd5 as libc::c_int,
            character: b"Ohungarumlaut\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd6 as libc::c_int,
            character: b"Odieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd7 as libc::c_int,
            character: b"multiply\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd8 as libc::c_int,
            character: b"Rcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xd9 as libc::c_int,
            character: b"Uring\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xda as libc::c_int,
            character: b"Uacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdb as libc::c_int,
            character: b"Uhungarumlaut\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdc as libc::c_int,
            character: b"Udieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdd as libc::c_int,
            character: b"Yacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xde as libc::c_int,
            character: b"Tcedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xdf as libc::c_int,
            character: b"germandbls\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe0 as libc::c_int,
            character: b"racute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe1 as libc::c_int,
            character: b"aacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe2 as libc::c_int,
            character: b"acircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe3 as libc::c_int,
            character: b"abreve\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe4 as libc::c_int,
            character: b"adieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe5 as libc::c_int,
            character: b"lacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe6 as libc::c_int,
            character: b"cacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe7 as libc::c_int,
            character: b"ccedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe8 as libc::c_int,
            character: b"ccaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xe9 as libc::c_int,
            character: b"eacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xea as libc::c_int,
            character: b"eogonek\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xeb as libc::c_int,
            character: b"edieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xec as libc::c_int,
            character: b"ecaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xed as libc::c_int,
            character: b"iacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xee as libc::c_int,
            character: b"icircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xef as libc::c_int,
            character: b"dcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf0 as libc::c_int,
            character: b"dbar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf1 as libc::c_int,
            character: b"nacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf2 as libc::c_int,
            character: b"ncaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf3 as libc::c_int,
            character: b"oacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf4 as libc::c_int,
            character: b"ocircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf5 as libc::c_int,
            character: b"ohungarumlaut\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf6 as libc::c_int,
            character: b"odieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf7 as libc::c_int,
            character: b"divide\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf8 as libc::c_int,
            character: b"rcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xf9 as libc::c_int,
            character: b"uring\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfa as libc::c_int,
            character: b"uacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfb as libc::c_int,
            character: b"uhungarumlaut\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfc as libc::c_int,
            character: b"udieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfd as libc::c_int,
            character: b"yacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xfe as libc::c_int,
            character: b"tcedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0xff as libc::c_int,
            character: b"dotaccent\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
