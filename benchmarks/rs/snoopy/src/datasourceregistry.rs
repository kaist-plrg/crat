use ::libc;
extern "C" {
    fn snoopy_genericregistry_getCount(regArray: *mut *mut libc::c_char) -> libc::c_int;
    fn snoopy_genericregistry_doesIdExist(
        regArray: *mut *mut libc::c_char,
        itemId: libc::c_int,
    ) -> libc::c_int;
    fn snoopy_genericregistry_doesNameExist(
        regArray: *mut *mut libc::c_char,
        itemName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_genericregistry_getIdFromName(
        regArray: *mut *mut libc::c_char,
        itemName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_genericregistry_getName(
        regArray: *mut *mut libc::c_char,
        itemId: libc::c_int,
    ) -> *mut libc::c_char;
    fn snoopy_datasource_cgroup(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_cmdline(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_cwd(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_datetime(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_domain(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_egid(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_egroup(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_env(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_env_all(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_euid(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_eusername(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_filename(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_gid(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_group(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_hostname(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_login(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_pid(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_ppid(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_rpname(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_sid(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_snoopy_configure_command(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_snoopy_literal(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_snoopy_threads(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_snoopy_version(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_systemd_unit_name(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_tid(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_tid_kernel(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_timestamp(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_timestamp_ms(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_timestamp_us(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_tty(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_tty_uid(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_tty_username(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_uid(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_username(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_failure(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasource_noop(
        result: *mut libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
}
pub static mut snoopy_datasourceregistry_names: [*mut libc::c_char; 38] = [
    b"cgroup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cmdline\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cwd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"datetime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"domain\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"egid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"egroup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"env\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"env_all\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"euid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"eusername\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"filename\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"group\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hostname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"login\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ppid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rpname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"snoopy_configure_command\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"snoopy_literal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"snoopy_threads\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"snoopy_version\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"systemd_unit_name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tid_kernel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"timestamp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"timestamp_ms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"timestamp_us\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tty\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tty_uid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tty_username\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"uid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"username\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"failure\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"noop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut snoopy_datasourceregistry_ptrs: [Option::<
    unsafe extern "C" fn(*mut libc::c_char, *const libc::c_char) -> libc::c_int,
>; 37] = unsafe {
    [
        Some(
            snoopy_datasource_cgroup
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_cmdline
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_cwd
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_datetime
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_domain
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_egid
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_egroup
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_env
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_env_all
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_euid
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_eusername
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_filename
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_gid
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_group
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_hostname
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_login
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_pid
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_ppid
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_rpname
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_sid
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_snoopy_configure_command
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_snoopy_literal
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_snoopy_threads
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_snoopy_version
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_systemd_unit_name
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_tid
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_tid_kernel
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_timestamp
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_timestamp_ms
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_timestamp_us
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_tty
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_tty_uid
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_tty_username
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_uid
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_username
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_failure
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_datasource_noop
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    ]
};
pub unsafe extern "C" fn snoopy_datasourceregistry_getCount() -> libc::c_int {
    return snoopy_genericregistry_getCount(snoopy_datasourceregistry_names.as_mut_ptr());
}
pub unsafe extern "C" fn snoopy_datasourceregistry_doesIdExist(
    mut datasourceId: libc::c_int,
) -> libc::c_int {
    return snoopy_genericregistry_doesIdExist(
        snoopy_datasourceregistry_names.as_mut_ptr(),
        datasourceId,
    );
}
pub unsafe extern "C" fn snoopy_datasourceregistry_doesNameExist(
    datasourceName: *const libc::c_char,
) -> libc::c_int {
    return snoopy_genericregistry_doesNameExist(
        snoopy_datasourceregistry_names.as_mut_ptr(),
        datasourceName,
    );
}
pub unsafe extern "C" fn snoopy_datasourceregistry_getIdFromName(
    datasourceName: *const libc::c_char,
) -> libc::c_int {
    return snoopy_genericregistry_getIdFromName(
        snoopy_datasourceregistry_names.as_mut_ptr(),
        datasourceName,
    );
}
pub unsafe extern "C" fn snoopy_datasourceregistry_getName(
    mut datasourceId: libc::c_int,
) -> *mut libc::c_char {
    return snoopy_genericregistry_getName(
        snoopy_datasourceregistry_names.as_mut_ptr(),
        datasourceId,
    );
}
pub unsafe extern "C" fn snoopy_datasourceregistry_callById(
    mut datasourceId: libc::c_int,
    result: *mut libc::c_char,
    datasourceArg: *const libc::c_char,
) -> libc::c_int {
    if 0 as libc::c_int == snoopy_datasourceregistry_doesIdExist(datasourceId) {
        return -(1 as libc::c_int);
    }
    return (snoopy_datasourceregistry_ptrs[datasourceId as usize])
        .unwrap()(result, datasourceArg);
}
pub unsafe extern "C" fn snoopy_datasourceregistry_callByName(
    datasourceName: *const libc::c_char,
    result: *mut libc::c_char,
    datasourceArg: *const libc::c_char,
) -> libc::c_int {
    let mut datasourceId: libc::c_int = 0;
    datasourceId = snoopy_datasourceregistry_getIdFromName(datasourceName);
    if datasourceId == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return (snoopy_datasourceregistry_ptrs[datasourceId as usize])
        .unwrap()(result, datasourceArg);
}
