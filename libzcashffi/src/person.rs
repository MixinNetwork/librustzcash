use libc::{c_char, c_uint, c_ulong};
use std::ffi::CStr;

#[repr(C)]
pub struct Person {
    name: *const c_char,
}

#[no_mangle]
pub extern "C" fn set_person_name(ptr: *mut Person, name: *const c_char) {
    unsafe { (*ptr).name = name };
}

#[no_mangle]
pub extern "C" fn get_person_name(ptr: *mut Person) -> *const c_char {
    unsafe { (*ptr).name }
}

#[no_mangle]
pub extern "C" fn hello(ptr: *mut Person) {
    let name = unsafe { (*ptr).name };

    let c_str = unsafe { CStr::from_ptr(name) };
    println!("content {:?}", c_str);
}

#[no_mangle]
pub extern "C" fn hello_vec(length: u32, array_ptr: *mut Person) {
    let items: &mut [Person] = unsafe {
        assert!(!array_ptr.is_null());
        std::slice::from_raw_parts_mut(array_ptr, length as usize)
    };
    for item in items.iter() {
        let c_str = unsafe { CStr::from_ptr(item.name) };
        println!("content {:?}", c_str);
    }
}
