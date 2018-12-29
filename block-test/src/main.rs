fn main() {
    let x = 5;

    // https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}
