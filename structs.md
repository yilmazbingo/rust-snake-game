```rs
struct User {
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}
```

- struct has benfit of naming.

```rs
    let mut user1= User {
        email:String::from("dhfj@hotmail.com"),
        username:String::from("new_name"),
        active:true,
        sign_in_count:1
    }
    let name:String=user1.username
    user1.username=String::from("changed")

    let user2:User=User{
        email:String::from("dshfk@hotmail.com"),
        // get the rest of user1
        ..user1
    }
```

- we could also create tuple structs

```rs
    // Both have same field types
    struct Color(i32,i32,i32)
    struct Point(i32,i32,i32)
```

- Primitive types such as integers implement Display trait by default because there is only way to print the integer. But for custom types like structs, we have to implement it ourself.

```rs
    // Rectangle struct does not implement 'Debug`
    println!("rect: {:?}",rect)
```

## ENUMS

```rs
    enum IpAddrKind{
        v4,
        v6,
    }

struct IpAddr{
    kind:IpAddrKind,
    address:String,
}
fn main(){
    let four:IpAddrKind=IpAddrKind::V4;
    let six :IpAddrKind=IpAddrKind::V6;
    let localhost:IpAddr=IpAddr{
        kind:IpAddrKind::V4,
        addresss:String::from("127.0.0.1")
    }
}
```

- to store data inside enum variances

```rs
    enum IpAddrKind{
        // specify what type data we want to store
        v4(String),
        v6(String),
    }

fn main(){
    let localhost:IpAddrKind=IpAddrKind::V4(String::from("127.0.0.1"))
}
```

- enum can store wide variety of types.

## Option Enum

- the problem with null values, the type system can not guarantee that if you use a value, it is not null. in Rust, there are no `null` value. Instead we have `Option` enum.

```rs
fn main(){
    enum Option<T>{
        Some(T),
        None,
    }
}
```

- this allows type system to enforce that we handle the none case if the value does not exist and guarantee that in some cases our value is present. Optinal is available in gloabl scope.

```rs
fn main(){
    let x:i8=5;
    let y:Option<i8>=Some(5);
    // cannot sum. they are different types
    let sum=x+y
    // in order to make this work we have to extract our value from Some integer.
    // in this case default value is 0
    let sum=x+y.unwrap_or(0)
}
```
