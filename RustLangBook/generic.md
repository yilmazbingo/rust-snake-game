```rs
// we use traits to show that our type comparable and copyable. ints and chars are copyable
fn get_largest<T:PartialOrd + Copy>(number_list:Vec<T>)->T{
    let mut largest=number_list[0];
    for number in number_list{
        if number > largest{
            largest=number
        }
    }
}
```

- generics in struct

```rs
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // this will throw error becasue x is int and y is float number
    // they are generic but they have to be same type.
    let integer_and_float = Point { x: 5, y: 4.0 }
}
```

To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters.

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

- Generics in Enum

```rs
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
