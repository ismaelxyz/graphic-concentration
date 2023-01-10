use std::os::raw::c_int;
#[cfg(target_os = "emscripten")]
use std::os::raw::c_void;

#[cfg(target_os = "emscripten")]
#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern "C" fn(context: *mut c_void);

#[cfg(target_os = "emscripten")]
extern "C" {
    pub fn emscripten_set_main_loop_arg(
        func: em_callback_func,
        arg: *mut c_void,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );

    pub fn emscripten_cancel_main_loop();
}

pub fn setup_mainloop<F: FnMut() -> bool + 'static>(
    fps: c_int,
    simulate_infinite_loop: bool,
    callback: F,
) {
    #[cfg(not(target_os = "emscripten"))]
    let (_fps, mut callback) = (fps, callback); // TODO

    #[cfg(not(target_os = "emscripten"))]
    while simulate_infinite_loop && callback() {}

    #[cfg(target_os = "emscripten")]
    extern "C" fn wrapper<F: FnMut() -> bool + 'static>(untyped_pointer: *mut c_void) {
        let leaked_pointer = untyped_pointer as *mut F;
        let callback_ref = unsafe { &mut *leaked_pointer };

        if !callback_ref() {
            unsafe {
                emscripten_cancel_main_loop();
            }
        }
    }

    #[cfg(target_os = "emscripten")]
    unsafe {
        let on_the_heap = Box::new(callback);
        let leaked_pointer = Box::into_raw(on_the_heap);
        let untyped_pointer = leaked_pointer as *mut c_void;

        emscripten_set_main_loop_arg(
            wrapper::<F>,
            untyped_pointer,
            fps,
            simulate_infinite_loop as c_int,
        )
    }
}
