# impl

The _implementation-defining_ keyword.

## Ref:

+ [impl keyword](https://doc.rust-lang.org/beta/std/keyword.impl.html)
+ [method syntax](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html)
+ [implementations reference](https://doc.rust-lang.org/reference/items/implementations.html)


## Hints:

+ used to define implementations on types.
+ is an item that associates items with an _implementing type_.
+ contain functions that belong to an instance of the type that is being
implemented or to the type statically.

## Example:

```rust

// the type
struct Example {
    number: i32,
}

// the type implementation
impl Example {
    fn boo() {
        println!("boo! Example::boo() was called!");
    }

    fn answer(&mut self) {
        self.number += 42;
    }

    fn get_number(&self) -> i32 {
        self.number
    }
}

trait Thingy {
    fn do_thingy(&self);
}

impl Thingy for Example {
    fn do_thingy(&self) {
        println!("doing a thing! also, number is {}!", self.number);
    }
}
```

## Details:

Functions and consts can both be defined in an implementation. A function
defined in an impl block can be standalone, meaning it would be called like
`Foo::bar()` (like a static method?). If the function takes `self`, `&self`, or
`&mut self` as its first argument, it can also be called using _method-call_
syntax, a familiar feature to any object oriented programmer, like `foo.bar()`


There are two types of implementations:

+ inherent implementations
+ trait implementations
