

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyCompilerFlags {
    pub cf_flags: c_int
}

use std::ptr;
use std::os::raw::{c_void, c_int, c_uint, c_ulong, c_char};
use std::os::raw::c_long;

pub enum PyTypeObject { }

pub type Py_ssize_t = c_int;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PyObject {
    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    _ob_next: *mut PyObject,
    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
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

    pub fn PyLong_FromLong(arg1: c_long) -> *mut PyObject;
}