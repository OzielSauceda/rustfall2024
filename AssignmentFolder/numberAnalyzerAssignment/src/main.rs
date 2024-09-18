fn is_even(n: i32) -> bool {
    n % 2 == 0  
}

fn main() {
    let new_arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15];

    for &number in new_arr.iter() {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        }
        else if number % 5 == 0 {
            println!("Buzz");
        }
        else if number % 3 == 0 {
            println!("Fizz");
        }
        else {
            if is_even(number){
                println!("Even");
            }
            else {
                println!("Odd");
            }
        }
    }

    let mut total: i32 = 0;
    let mut index = 0;
    while index < new_arr.len(){
        total += new_arr[index];
        index += 1;
    }
    println!("Total Sum: {}", total);

    let mut biggestInt: i32 = 0;
    for number in new_arr {
        if number > biggestInt {
            biggestInt = number;
        }
    }
    println!("The biggest number was {}", biggestInt);

}
