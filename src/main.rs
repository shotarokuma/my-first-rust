fn main() {
    let name: String = "Bob".to_string();
    let tuple: (i32, f64, bool) = (1, 2.0, true);
    let array: [i32; 3] = [1, 2, 3]; 
    println!("Hello, {}", name);
    println!("My first tuple is {:?}", tuple);
    println!("My first array is {:?}", array);
    let num : i32 = 5;
    if num < 9
    {
        println!("low!");
    }else{
        println!("high!");
    }

    let mut counter = 0;
    while counter < 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    for number in 1..=5 {
        println!("Number: {}", number);
    }

    let result = add(3, 4);
    println!("Result: {}", result);
}


fn add(a: i32, b: i32) -> i32 {
    a + b
}

