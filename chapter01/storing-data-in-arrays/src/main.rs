fn main() {
    let int_array: [i32; 3] = [1, 2, 3];
    for i in int_array {
        println!("{}", i);
    }
    println!("{}", int_array[1]);

    let mut mutable_array: [i32; 3] = [1, 2, 0];
    mutable_array[2] = 3;
    println!("{:?}", mutable_array);
    println!("{}", mutable_array.len());

    let slice_array: [i32; 100] = [0; 100];
    println!("length: {}", slice_array.len());
    println!("slice: {:?}", &slice_array[5 .. 8]);

}
