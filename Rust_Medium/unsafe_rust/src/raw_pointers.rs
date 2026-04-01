fn swap_values(a: &mut i32, b: &mut i32) {
    let pa: *mut i32 = a as *mut i32;
    let pb: *mut i32 = b as *mut i32;
    unsafe {
        let temp = *pa;
        *pa = *pb;
        *pb = temp;
    }
}
