use ::libc;
extern "C" {
    pub type PKVM;
    fn registerModuleMath(vm: *mut PKVM);
    fn registerModuleTypes(vm: *mut PKVM);
    fn registerModuleTime(vm: *mut PKVM);
    fn registerModuleIO(vm: *mut PKVM);
    fn registerModulePath(vm: *mut PKVM);
    fn registerModuleOS(vm: *mut PKVM);
    fn registerModuleJson(vm: *mut PKVM);
    fn registerModuleDummy(vm: *mut PKVM);
    fn registerModuleTerm(vm: *mut PKVM);
    fn cleanupModuleTerm(vm: *mut PKVM);
}
pub unsafe extern "C" fn registerLibs(mut vm: *mut PKVM) {
    registerModuleMath(vm);
    registerModuleTypes(vm);
    registerModuleTime(vm);
    registerModuleIO(vm);
    registerModulePath(vm);
    registerModuleOS(vm);
    registerModuleJson(vm);
    registerModuleDummy(vm);
    registerModuleTerm(vm);
}
pub unsafe extern "C" fn cleanupLibs(mut vm: *mut PKVM) {
    cleanupModuleTerm(vm);
}
