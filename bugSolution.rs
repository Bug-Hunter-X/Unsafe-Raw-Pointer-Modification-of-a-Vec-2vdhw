fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    vec[0] = 4; // Correct way to modify the first element

    println!( "{:?}", vec);
}