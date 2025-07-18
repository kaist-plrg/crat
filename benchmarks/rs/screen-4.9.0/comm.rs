use ::libc;
pub type AclBits = *mut libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comm {
    pub name: *mut libc::c_char,
    pub flags: libc::c_int,
    pub userbits: [AclBits; 1],
}
pub static mut comms: [comm; 190] = [
    {
        let mut init = comm {
            name: b"acladd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"aclchg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 2 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"acldel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"aclgrp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"aclumask\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"activity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"addacl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"allpartial\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"altscreen\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"at\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"attrcolor\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"autodetach\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"autonuke\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"backtick\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"bce\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"bell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"bell_msg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"bind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"bindkey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"blanker\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"blankerprg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"break\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"breaktype\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"bufferfile\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"bumpleft\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"bumpright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"c1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"caption\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"chacl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 2 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"charset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int | 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"chdir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"cjkwidth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"clear\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"collapse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"colon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"command\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"compacthist\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"console\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"copy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"crlf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"debug\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defautonuke\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defbce\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defbreaktype\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defc1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defcharset\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defdynamictitle\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defencoding\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defescape\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defflow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defgr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defhstatus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defkanji\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"deflog\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"deflogin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defmode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defmonitor\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defmousetrack\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defnonblock\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defobuflimit\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defscrollback\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defshell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defsilence\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defslowpaste\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defutf8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defwrap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"defwritelock\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"detach\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"digraph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"dinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"displays\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 8 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"dumptermcap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"dynamictitle\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"echo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"encoding\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"escape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"eval\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"exec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"fit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"flow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"focus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"focusminsize\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"gr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"group\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"hardcopy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"hardcopy_append\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"hardcopydir\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"hardstatus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"history\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"hstatus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int | 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"idle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"ignorecase\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"info\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"kanji\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"kill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"lastmsg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"layout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"license\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 8 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"lockscreen\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"log\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"logfile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"login\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"logtstamp\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"mapdefault\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"mapnotnext\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"maptimeout\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"markkeys\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"maxwin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"meta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 8 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"monitor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"mousetrack\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"msgminwait\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"msgwait\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"multiuser\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"nethack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"next\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"nonblock\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"number\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"obuflimit\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"only\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"other\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"partial\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"password\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"paste\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"pastefont\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"pow_break\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"pow_detach\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"pow_detach_msg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"prev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"printcmd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"process\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"readbuf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"readreg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"redisplay\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"register\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 2 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"remove\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"removebuf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"rendition\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 2 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"reset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"resize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"screen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"scrollback\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int | 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"select\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"sessionname\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"setenv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"setsid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"shell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"shelltitle\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"silence\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"silencewait\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"sleep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"slowpaste\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"sorendition\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"sort\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"source\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"split\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"startup_message\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"stuff\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"su\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"suspend\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"term\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"termcap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 2 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"termcapinfo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 2 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"terminfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 2 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"time\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"title\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"umask\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"unbindall\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"unsetenv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"utf8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"vbell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"vbell_msg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"vbellwait\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"verbose\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"version\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"wall\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 7 as libc::c_int | 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"windowlist\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"windows\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"wrap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"writebuf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"writelock\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int),
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"xoff\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 8 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"xon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: (1 as libc::c_int) << 8 as libc::c_int | 0 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"zmodem\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"zombie\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            flags: 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
    {
        let mut init = comm {
            name: b"zombie_timeout\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            flags: 1 as libc::c_int,
            userbits: [0 as *const libc::c_uchar as *mut libc::c_uchar; 1],
        };
        init
    },
];
