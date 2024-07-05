extern crate alloc;

use alloc::vec::Vec;
use std::mem::MaybeUninit;
use std::slice;
use shared::CreateHostParams;

/* 
 ❗ partly a copy of ❗
  https://github.com/tetratelabs/wazero/blob/main/examples/allocation/rust/testdata/greet.rs
*/

fn validate_create_host_params(value: &str) -> String {
    let res = CreateHostParams::parse_and_validate(value);
    serde_json::to_string(&res).unwrap()
}

#[cfg_attr(all(target_arch = "wasm32"), export_name = "validate_create_host_params")]
#[no_mangle]
pub unsafe extern "C" fn _validate_create_host_params(ptr: u32, len: u32) -> u64 {
    let payload = &ptr_to_string(ptr, len);
    
    let result = validate_create_host_params(payload);
    let (ptr, len) = string_to_ptr(&result);
    std::mem::forget(result);
    ((ptr as u64) << 32) | len as u64
}

unsafe fn ptr_to_string(ptr: u32, len: u32) -> String {
    let slice = slice::from_raw_parts_mut(ptr as *mut u8, len as usize);
    let utf8 = std::str::from_utf8_unchecked_mut(slice);
    String::from(utf8)
}

unsafe fn string_to_ptr(s: &String) -> (u32, u32) {
    (s.as_ptr() as u32, s.len() as u32)
}

#[cfg_attr(all(target_arch = "wasm32"), export_name = "allocate")]
#[no_mangle]
pub extern "C" fn _allocate(size: u32) -> *mut u8 {
    allocate(size as usize)
}

fn allocate(size: usize) -> *mut u8 {
    // Allocate the amount of bytes needed.
    let vec: Vec<MaybeUninit<u8>> = Vec::with_capacity(size);

    // into_raw leaks the memory to the caller.
    Box::into_raw(vec.into_boxed_slice()) as *mut u8
}


#[cfg_attr(all(target_arch = "wasm32"), export_name = "deallocate")]
#[no_mangle]
pub unsafe extern "C" fn _deallocate(ptr: u32, size: u32) {
    deallocate(ptr as *mut u8, size as usize);
}

unsafe fn deallocate(ptr: *mut u8, size: usize) {
    let _ = Vec::from_raw_parts(ptr, 0, size);
}