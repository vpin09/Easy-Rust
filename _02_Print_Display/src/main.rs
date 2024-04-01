fn main() {
    println!("Hello, world! number {} and {}", 8,9);
    println!("number {}",number());
   let multi= multiply(10, 12);
   println!("multi======{}",multi);
   let my_number = {
    let second_number = 8;
        second_number + 9 
    };
    let doesnt_print=();
    println!("this wil noit print: {}", i8::MAX);

    println!("My number is: {}", my_number);

}
fn number() ->i32 {
    8
}
fn multiply(number_one:i32, number_two:i32)->i32 {
    let res=number_one*number_two;
    println!("Result multiply ={}",res);
    return  res;
}
