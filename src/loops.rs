pub fn run() {
    // inifinite loop
    let mut count = 0;
    loop {
        count += 1;
        println!("New number is {}", count);

        if count == 10 {
            break;
        }
    }

    // while loop
    while count <= 20 {
        if count % 15 == 0 {
            println!("{} fizzbuzz", count);
        } else if count % 3 == 0 {
            println!("{} fizz", count);
        } else if count % 5 == 0 {
            println!("{} buzz", count);
        }

        count += 1;
    }

    // for range loop
    for x in 0..20 {
        if x % 15 == 0 {
            println!("{} fizzbuzz", x);
        } else if x % 3 == 0 {
            println!("{} fizz", x);
        } else if x % 5 == 0 {
            println!("{} buzz", x);
        }
    }

    count = 0;
    let result = loop {
        if count == 10 {
            break count * 10;
        }

        count += 1;
    };

    println!("After the loop, count is {}, result is {}", count, result);

    let message_array = ['A', 'B', 'C'];
    for item in message_array {
        println!("Item is {}", item);
    }

    for (index, item) in message_array.iter().enumerate() {
        println!("Item {} is {}", index, item);
    }

    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix {
        for value in row {
            println!("Matrix value is {}", value);
        }
    }

    let input = vec![1, 2, 3, 4, 5];
    let (min_value, max_value, mean_value) = calculate_metrics(input);
    println!(
        "Calculated metrics, Min: {}, Max: {}, Mean: {}",
        min_value, max_value, mean_value
    );
}

fn calculate_metrics(input: Vec<i32>) -> (i32, i32, f64) {
    let mut min_value = input[0];
    let mut max_value = input[0];
    let mut sum = 0;
    let element_count = input.len();

    for value in input {
        if value < min_value {
            min_value = value;
        }

        if value > max_value {
            max_value = value;
        }

        sum += value;
    }

    let mean_value = sum as f64 / element_count as f64;
    (min_value, max_value, mean_value)
}
