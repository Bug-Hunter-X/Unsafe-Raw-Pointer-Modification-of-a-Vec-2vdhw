fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_mut_ptr();
    unsafe {
        *ptr = 4;
    }

    println!( "{:?}", vec);
}