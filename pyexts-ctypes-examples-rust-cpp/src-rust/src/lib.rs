extern crate libc;
extern crate serde_json;

use libc::{size_t,int32_t, c_char};
use std::ffi::{CString, CStr};
use std::thread;
use std::mem;
use std::slice;
use serde_json::Value;


#[no_mangle]
pub extern fn test_json_interface(json: *const c_char) -> *const c_char {
    let data = cstr_from_raw(json);
    let parsed: Value = serde_json::from_str(&data).unwrap();
    println!("{:?}", parsed);
    into_raw_str(&data)
}

#[no_mangle]
pub extern fn test_parallel_list_sum(list_a: *const int32_t, n_a: size_t, list_b: *const int32_t, n_b: size_t) -> int32_t {
    let l_a = get_slice(list_a, n_a);
    let l_b = get_slice(list_b, n_b);
    let list_a_sum_handle = thread::spawn(move ||{ sum_list(l_a) });
    let list_b_sum_handle = thread::spawn(move ||{ sum_list(l_b) });
    let s_a = list_a_sum_handle.join().expect("__COULD_NOT_JOIN__:a");
    let s_b = list_b_sum_handle.join().expect("__COULD_NOT_JOIN__:b");
    s_a + s_b
}

#[no_mangle]
pub extern fn test_cstr_interface(array: *const *const c_char, length: size_t) -> *const c_char {
    let args:Vec<String> = into_vec_string(array, length);
    println!("{:?}", args);
    let test = "hello python".to_string();
    into_raw_str(&test)
}
    
#[no_mangle]
pub extern fn consume_cstr(p: *mut c_char) {
    unsafe { CString::from_raw(p) };
}

fn into_vec_string(array: *const *const c_char, length: size_t) -> Vec<String> {
    let args:&[*const c_char] = unsafe { slice::from_raw_parts(array, length as usize) };
    args.iter().map(|&ptr| {cstr_from_raw(ptr)}).collect()
}

fn into_raw_str(s:&str) -> *const c_char {
    CString::new(s).unwrap().into_raw()
    /*let c_string = CString::new(s).unwrap();
    let raw_ptr:*const c_char = c_string.as_ptr();    
    mem::forget(c_string);              
    raw_ptr*/
}

fn cstr_from_raw(raw_str:*const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(raw_str) };
    let c_str_slice = c_str.to_str().expect("__STR_NOT_UTF__");
    c_str_slice.to_string()
} 

fn sum_list(nums: &[int32_t]) -> int32_t {
    nums.iter().fold(0, |acc, i| acc + i)
}

fn get_slice<'a>(data: *const int32_t, length: size_t) -> &'a [i32] {
    unsafe { slice::from_raw_parts(data, length as usize) }
} 





