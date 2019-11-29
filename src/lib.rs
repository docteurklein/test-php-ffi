#[no_mangle]
pub extern fn hello_from_rust(p: Point) {
    println!("Hello from Rust! {:?}", p);
}

/// cbindgen:field-names=[x, y]
#[repr(C)]
#[derive(Debug)]
pub struct Point(pub i32, pub i32);
