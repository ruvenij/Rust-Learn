use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [7, 5, 3, 8, 1];

    println!("{:?}", numbers);
    println!("Second element is {}", numbers[1]);

    numbers[1] = 10;
    println!("After change, second element is {}", numbers[1]);

    println!("Array length is {}", numbers.len());

    println!("Mem allocation {}", mem::size_of_val(&numbers));

    // copy slice to another variable
    let new_slice: &[i32] = &numbers[1..4];
    println!("New slice: {:?}", new_slice);

    // numbers[0] = 20;
    // println!("After change, second element is {}", numbers[1]);
    // println!("After change, new slice is {:?}", new_slice);

    let my_array: [i32; 5];
    my_array = [0; 5];

    println!("Last element is {}", my_array[4]);

    let array_index = my_array.len();
    println!(
        "Last value is {}, index : {}",
        my_array[array_index - 1],
        array_index - 1
    );

    let parking_lot = [[1, 2, 3], [8, 9, 10]];
    println!("Parking lot at 0,0 is {}", parking_lot[0][0]);
}
