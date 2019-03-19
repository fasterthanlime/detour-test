extern crate detour;
use detour::GenericDetour;

extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

extern "C" fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    let mut hook = unsafe {
        GenericDetour::<extern "C" fn(i32, i32) -> i32>::new(add, subtract).unwrap()
    };

    println!("add(1, 5) = {}", add(1, 5));

    unsafe { hook.enable().unwrap(); }

    println!("add(1, 5) = {}", add(1, 5));
}
