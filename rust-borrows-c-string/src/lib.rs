use std::ffi::CStr;
use std::os::raw::c_char;


#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}


#[no_mangle]
pub extern "C" fn print_string(cptr: *const c_char) {
    println!("got ptr {:?} from C", cptr);

    let my_cstr: &CStr = unsafe { CStr::from_ptr(cptr) };
    println!("made a &CStr from it: {:?}", my_cstr);
    println!("    cstr ptr: {:?}", my_cstr.as_ptr());

    let s: &str = my_cstr.to_str().unwrap();
    println!("made a &str from it: {:?}", s);
    println!("    &str ptr: {:?}", s.as_ptr());

    println!("finally here's the string from rust: {}", s);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
