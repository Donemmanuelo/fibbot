## Pattern matching 
Pattern matching is a powerful tool in rust, allowing for concise and readable way of handling complex data structure.

### Types of Pattern matching

Pattern matching using: 
- if let 
- match

### If let expression pattern matching 
Syntactic sugar is a syntax in programming language that is designed to make certain contructs easier to write and read, without changing the underlying functionalities. If let a prime example of syntactic sugar and here is how they are related:
1. Concise

**Before**
```sh
let some_value = Some(42);

match some_value {
    Some(value) => println!("The value is: {}", value),
    None => (),
}
```

**After**
```sh
let some_value = Some(42);

if let Some(value) = some_value {
    println!("The value is: {}", value);
}
```

2. Readability

- with if-let
```sh
let some_value = Some(42);

if let Some(value) = some_value {
    println!("The value is: {}", value);
} else {
    println!("No value found");
}
```