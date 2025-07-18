use ::libc;
extern "C" {
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type config_directive = libc::c_uint;
pub const CD_loglevel: config_directive = 40;
pub const CD_upstream: config_directive = 39;
pub const CD_reversepath: config_directive = 38;
pub const CD_reversemagic: config_directive = 37;
pub const CD_reverseonly: config_directive = 36;
pub const CD_reversebaseurl: config_directive = 35;
pub const CD_filtercasesensitive: config_directive = 34;
pub const CD_filterdefaultdeny: config_directive = 33;
pub const CD_filterextended: config_directive = 32;
pub const CD_filtertype: config_directive = 31;
pub const CD_filterurls: config_directive = 30;
pub const CD_filter: config_directive = 29;
pub const CD_addheader: config_directive = 28;
pub const CD_errorfile: config_directive = 27;
pub const CD_basicauth: config_directive = 26;
pub const CD_bind: config_directive = 25;
pub const CD_deny: config_directive = 24;
pub const CD_allow: config_directive = 23;
pub const CD_listen: config_directive = 22;
pub const CD_group: config_directive = 21;
pub const CD_user: config_directive = 20;
pub const CD_connectport: config_directive = 19;
pub const CD_timeout: config_directive = 18;
pub const CD_maxrequestsperchild: config_directive = 17;
pub const CD_startservers: config_directive = 16;
pub const CD_minspareservers: config_directive = 15;
pub const CD_maxspareservers: config_directive = 14;
pub const CD_maxclients: config_directive = 13;
pub const CD_port: config_directive = 12;
pub const CD_disableviaheader: config_directive = 11;
pub const CD_bindsame: config_directive = 10;
pub const CD_syslog: config_directive = 9;
pub const CD_xtinyproxy: config_directive = 8;
pub const CD_stathost: config_directive = 7;
pub const CD_statfile: config_directive = 6;
pub const CD_defaulterrorfile: config_directive = 5;
pub const CD_viaproxyname: config_directive = 4;
pub const CD_anonymous: config_directive = 3;
pub const CD_pidfile: config_directive = 2;
pub const CD_logfile: config_directive = 1;
pub const CD_NIL: config_directive = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_directive_entry {
    pub name: *const libc::c_char,
    pub value: config_directive,
}
pub unsafe extern "C" fn config_directive_find(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *const config_directive_entry {
    let mut i: size_t = 0;
    static mut wordlist: [config_directive_entry; 42] = [
        {
            let mut init = config_directive_entry {
                name: b"\0" as *const u8 as *const libc::c_char,
                value: CD_NIL,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"\0" as *const u8 as *const libc::c_char,
                value: CD_NIL,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"allow\0" as *const u8 as *const libc::c_char,
                value: CD_allow,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"stathost\0" as *const u8 as *const libc::c_char,
                value: CD_stathost,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"listen\0" as *const u8 as *const libc::c_char,
                value: CD_listen,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"timeout\0" as *const u8 as *const libc::c_char,
                value: CD_timeout,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"statfile\0" as *const u8 as *const libc::c_char,
                value: CD_statfile,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"pidfile\0" as *const u8 as *const libc::c_char,
                value: CD_pidfile,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"bindsame\0" as *const u8 as *const libc::c_char,
                value: CD_bindsame,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"reversebaseurl\0" as *const u8 as *const libc::c_char,
                value: CD_reversebaseurl,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"viaproxyname\0" as *const u8 as *const libc::c_char,
                value: CD_viaproxyname,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"upstream\0" as *const u8 as *const libc::c_char,
                value: CD_upstream,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"anonymous\0" as *const u8 as *const libc::c_char,
                value: CD_anonymous,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"group\0" as *const u8 as *const libc::c_char,
                value: CD_group,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"defaulterrorfile\0" as *const u8 as *const libc::c_char,
                value: CD_defaulterrorfile,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"startservers\0" as *const u8 as *const libc::c_char,
                value: CD_startservers,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"filtercasesensitive\0" as *const u8 as *const libc::c_char,
                value: CD_filtercasesensitive,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"filtertype\0" as *const u8 as *const libc::c_char,
                value: CD_filtertype,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"filterurls\0" as *const u8 as *const libc::c_char,
                value: CD_filterurls,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"filter\0" as *const u8 as *const libc::c_char,
                value: CD_filter,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"reversemagic\0" as *const u8 as *const libc::c_char,
                value: CD_reversemagic,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"errorfile\0" as *const u8 as *const libc::c_char,
                value: CD_errorfile,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"minspareservers\0" as *const u8 as *const libc::c_char,
                value: CD_minspareservers,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"user\0" as *const u8 as *const libc::c_char,
                value: CD_user,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"disableviaheader\0" as *const u8 as *const libc::c_char,
                value: CD_disableviaheader,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"deny\0" as *const u8 as *const libc::c_char,
                value: CD_deny,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"xtinyproxy\0" as *const u8 as *const libc::c_char,
                value: CD_xtinyproxy,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"reversepath\0" as *const u8 as *const libc::c_char,
                value: CD_reversepath,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"bind\0" as *const u8 as *const libc::c_char,
                value: CD_bind,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"maxclients\0" as *const u8 as *const libc::c_char,
                value: CD_maxclients,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"reverseonly\0" as *const u8 as *const libc::c_char,
                value: CD_reverseonly,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"port\0" as *const u8 as *const libc::c_char,
                value: CD_port,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"maxspareservers\0" as *const u8 as *const libc::c_char,
                value: CD_maxspareservers,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"syslog\0" as *const u8 as *const libc::c_char,
                value: CD_syslog,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"filterdefaultdeny\0" as *const u8 as *const libc::c_char,
                value: CD_filterdefaultdeny,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"loglevel\0" as *const u8 as *const libc::c_char,
                value: CD_loglevel,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"filterextended\0" as *const u8 as *const libc::c_char,
                value: CD_filterextended,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"connectport\0" as *const u8 as *const libc::c_char,
                value: CD_connectport,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"logfile\0" as *const u8 as *const libc::c_char,
                value: CD_logfile,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"basicauth\0" as *const u8 as *const libc::c_char,
                value: CD_basicauth,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"addheader\0" as *const u8 as *const libc::c_char,
                value: CD_addheader,
            };
            init
        },
        {
            let mut init = config_directive_entry {
                name: b"maxrequestsperchild\0" as *const u8 as *const libc::c_char,
                value: CD_maxrequestsperchild,
            };
            init
        },
    ];
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[config_directive_entry; 42]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<config_directive_entry>() as libc::c_ulong,
            )
    {
        if strcasecmp(str, wordlist[i as usize].name) == 0 {
            return &*wordlist.as_ptr().offset(i as isize)
                as *const config_directive_entry;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const config_directive_entry;
}
