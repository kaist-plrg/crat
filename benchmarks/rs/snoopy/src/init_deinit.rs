use ::libc;
extern "C" {
    fn snoopy_configuration_ctor();
    fn snoopy_configuration_dtor();
    fn snoopy_inputdatastorage_ctor();
    fn snoopy_inputdatastorage_dtor();
    fn snoopy_tsrm_ctor();
    fn snoopy_tsrm_dtor();
}
pub unsafe extern "C" fn snoopy_init() {
    snoopy_tsrm_ctor();
    snoopy_configuration_ctor();
    snoopy_inputdatastorage_ctor();
}
pub unsafe extern "C" fn snoopy_cleanup() {
    snoopy_inputdatastorage_dtor();
    snoopy_configuration_dtor();
    snoopy_tsrm_dtor();
}
