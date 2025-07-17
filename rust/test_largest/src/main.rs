fn largest_u32(list: Vec<u32>) -> u32 {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: Vec<char>) -> char {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 100, 65];

    let result = largest_u32(number_list);
    println!("the largest number is {}", result);

    let number_char = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(number_char);
    println!("the largest char is {}", result);

    let number_list = vec![34, 50, 100, 65];
    let result = largest(number_list);
    println!("02 the largest number is {}", result);

    let number_char = vec!['y', 'm', 'a', 'q'];
    let result = largest(number_char);
    println!("02 the largest char is {}", result);

    println!("Hello, world!");
}
