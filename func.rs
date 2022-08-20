#[no_mangle]
unsafe fn array_offset(arr: [i32; 5]) -> i32{
    let ptr = arr.as_ptr().offset(2); 
    *ptr
}