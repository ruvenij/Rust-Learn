pub fn run() {
    let arr1 = [1, 2, 3];
    let arr2 = &arr1;

    println!("Array1: {:?}", arr1);
    println!("Address of array1: {:p}", &arr1);

    println!("Array2: {:?}", arr2);
    println!("Address of array2: {:p}", &arr2);

    println!("arr2 points to same as arr1: {}", std::ptr::eq(&arr1, arr2));

    // vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2));
}
