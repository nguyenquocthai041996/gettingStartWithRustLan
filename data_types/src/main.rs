use std::io;
fn main() {
    data_type();
    tuple_type();
    array_type();
    accessing_array_elements();
    invalid_array_element_access();
}

fn data_type() {
    println!("========DATA TYPE========");
    let guess: u32 = "42".parse().expect("Not a number");
    println!("Guess: {}", guess);

    let x = 2.0; //f64
    println!("x: {}", x);

    let y: f32 = 3.0; //f32
    println!("y: {}", y);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);
}

fn tuple_type() {
    println!("========TUPLE TYPE========");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of x, y, z is: {},{}, {}", _x, y, _z);

    let five_hundred = tup.0;
    println!("The value of five_hundred is: {}", five_hundred);

    let six_point_four = tup.1;
    println!("The value of six_point_four is: {}", six_point_four);

    let one = tup.2;
    println!("The value of one is: {}", one);

    println!("The value of tup is: {:?}", tup);
}

fn array_type() {
    println!("========ARRAY TYPE========");
    let a = [1, 2, 3, 4, 5];
    println!("The value of array is: {:?}", a);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("The value of months is: {:?}", months);

    //i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array is: {:?}", a);

    let a = [3; 5];
    println!("The value of array is: {:?}", a);
}

fn accessing_array_elements() {
    println!("========ACCESSING ARRAY ELEMENTS========");
    let a = [1, 2, 3, 4, 5];
    println!("The value of array is: {:?}", a);
    let first = a[0];
    let second = a[1];
    println!("The value 1: {}, value 2: {}", first, second);
}

fn invalid_array_element_access() {
    println!("========INVALID ARRAY ELEMENT ACCESS========");
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
