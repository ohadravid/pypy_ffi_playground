use std::os::raw::c_int;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyCompilerFlags {
    pub cf_flags : c_int
}

extern "C" {
    pub fn Py_Initialize() -> ();
    pub fn Py_Finalize() -> ();
    pub fn PyRun_SimpleStringFlags(arg1: *const c_char,
                                   arg2: *mut PyCompilerFlags)
                                   -> c_int;
    pub fn rpython_startup_code() -> ();
    pub fn pypy_setup_home(arg1: c_int, arg2: c_int) -> c_int;
    pub fn pypy_execute_source(arg1: *const c_char) -> c_int;

}