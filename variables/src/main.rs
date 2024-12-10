fn main() {
    // Here difined a variable whos value can't be changed.
    let msg = "Hello World!";
    println!("msg1: {msg}");
    
    // it's not valid. You can't change the value of immutable value.
    // error[E0384]: cannot assign twice to immutable variable `msg`
    // msg = "What'up!";

    // it's valid.
    let msg =  "What's up";
    println!("msg2: {msg}");


    // Here difined a variable can changed the value.
    let mut age = 1;

    // it's valid, you can change age value.
    age = age + 3;
    println!("age: {age}");
}


