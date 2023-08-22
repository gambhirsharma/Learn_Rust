# variables

## let

`let` is immutable by default in Rust. You can't change the value of it directly.
`mut` is added with let to make it mutable.

example
```
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

## const

`const` is also immutable but you can't use `mut` to make it mutable. They are always immutable.
You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.

example:
```
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Rust’s naming convention for constants is to use all uppercase with underscores between words.

## mut vs shadowing