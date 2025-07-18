use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut nethackflag: libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nlstrans {
    pub from: *mut libc::c_char,
    pub to: *mut libc::c_char,
}
static mut nethacktrans: [nlstrans; 39] = [
    {
        let mut init = nlstrans {
            from: b"Cannot lock terminal - fork failed\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"Cannot fork terminal - lock failed\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Got only %d bytes from %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"You choke on your food: %d bytes from %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Copy mode - Column %d Line %d(+%d) (%d,%d)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"Welcome to hacker's treasure zoo - Column %d Line %d(+%d) (%d,%d)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"First mark set - Column %d Line %d\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"You drop a magic marker - Column %d Line %d\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Copy mode aborted\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"You escaped the dungeon.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Filter removed.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"You have a sad feeling for a moment...\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Window %d (%s) killed.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"You destroy poor window %d (%s).\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Window %d (%s) is now being monitored for all activity.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            to: b"You feel like someone is watching you...\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Window %d (%s) is no longer being monitored for activity.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            to: b"You no longer sense the watcher's presence.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"empty buffer\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"Nothing happens.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"switched to audible bell.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"Suddenly you can't see your bell!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"switched to visual bell.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"Your bell is no longer invisible.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"The window is now being monitored for %d sec. silence.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            to: b"You feel like someone is waiting for %d sec. silence...\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"The window is no longer being monitored for silence.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"You no longer sense the watcher's silence.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"No other window.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"You cannot escape from window %d!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Logfile \"%s\" closed.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"You put away your scroll of logging named \"%s\".\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Error opening logfile \"%s\"\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"You don't seem to have a scroll of logging named \"%s\".\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Creating logfile \"%s\".\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"You start writing on your scroll of logging named \"%s\".\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Appending to logfile \"%s\".\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"You add to your scroll of logging named \"%s\".\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Detach aborted.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"The blast of disintegration whizzes by you!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Empty register.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"Nothing happens.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"[ Passwords don't match - checking turned off ]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"[ Passwords don't match - your armor crumbles away ]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Aborted because of window size change.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"KAABLAMM!!!  You triggered a land mine!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Out of memory.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"Who was that Maude person anyway?\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"getpwuid() can't identify your account!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"An alarm sounds through the dungeon...\nThe Keystone Kops are after you!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Must be connected to a terminal.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"You must play from a terminal.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"No Sockets found in %s.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"This room is empty (%s).\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"New screen...\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"Be careful!  New screen tonight.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Child has been stopped, restarting.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"You regain consciousness.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"There are screens on:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"Your inventory:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"There is a screen on:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"Your inventory:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"There are several screens on:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"Prove thyself worthy or perish:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"There is a suitable screen on:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"You see here a good looking screen:\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"There are several suitable screens on:\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"You may wish for a screen, what do you want?\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"%d socket%s wiped out.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"You hear %d distant explosion%s.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Remove dead screens with 'screen -wipe'.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"The dead screen%s touch%s you. Try 'screen -wipe'.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Illegal reattach attempt from terminal %s.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            to: b"'%s' tries to touch your session, but fails.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: b"Could not write %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            to: b"%s is too hard to dig in\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = nlstrans {
            from: 0 as *const libc::c_char as *mut libc::c_char,
            to: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
pub unsafe extern "C" fn DoNLS(mut from: *const libc::c_char) -> *const libc::c_char {
    let mut t: *mut nlstrans = 0 as *mut nlstrans;
    if nethackflag != 0 {
        t = nethacktrans.as_mut_ptr();
        while !((*t).from).is_null() {
            if strcmp(from, (*t).from) == 0 as libc::c_int {
                return (*t).to;
            }
            t = t.offset(1);
            t;
        }
    }
    return from;
}
