use std::ptr;
use std::ffi::CString;

mod ffi;

use ffi::{rpython_startup_code, pypy_setup_home, pypy_execute_source};
use ffi::Py_Initialize;
use ffi::PyRun_SimpleStringFlags;
use ffi::Py_Finalize;
use ffi::PyLong_FromLong;

fn main() {
    let script = CString::new("import this; import sys; print(sys.version)").unwrap();

    unsafe {
//        Py_Initialize();
//        PyRun_SimpleStringFlags(
//            script.as_ptr(),
//            ptr::null_mut()
//        );
//        Py_Finalize();
        // Works if you put the binary in the folder that has ./lib/libpypy...
        rpython_startup_code();
        pypy_setup_home(0, 1);
        pypy_execute_source(script.as_ptr());
        PyLong_FromLong(3);
    }
}