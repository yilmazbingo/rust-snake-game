- ownerhsip model is a way to manage memory.
  in garbage collector system, we dont need to worry about it. It is error free (because you are not writing code to manage the memory) and gives you faster write time because you are not writing code for memory.
  cons: no control over memory, slower (because we cannot manually optimize memory) and unpredictable (garbage collector could choose to clean up memory at any point in time and when it does so it slows down our program.) runtime performance, larger program size.

- in c or c++, we have to allocate and deallocate memory manually. Manual memory management.Control over memory, faster runtime, smaller program size becuase you dont need to include garbage collector. cons are error prone (many bugs and security issues are caused by incorrect memory management) and slower write time.

between garbage collection and manual memory management pros and cons are opposite. we are doing opposite tradeoffs.

## Ownership

rust is sytem programming language so it does care about runtime performance and program size.

- control over memory
- ERROR FREE. the way how rust achives this, by doing a bunch of compile time checks to make sure you are using memory in safe way. you can opt out with `unsafe` keyword.
  -faster runtime
- smaller program size

con is slower write time. learning curve.

There are 3 ownerhsip rules crucial to remember

- Each value in rust has a variable that is called its owner
- There can only be one owner at a time.
- when the owner goes out of scope, the value will be dropped

## STACK AND HEAP

during the runtime, our program has access to stack and heap. stack is fixed size and cannot grow or shrink during runtime. stack also stores stack frames which are created for every function that executes. stack frame stores the local variables of the function being executed.
the size of the stack frame is calculated at compile time. that means variables has to be known fixed sizes. Variables inside of a stack frame also only live as long as the stack frame lives.

- heap is less organized, it could grow or shring at runtime and data stored in heap could be dynamic in size, could be larger amounts of data and we control the lifetime of the data.

- String data type is stored in heap. we keep the pointer on the stack.

  pointer=(ptr,len,capacity)

```rs
   fn b(){
     let x: String= String::from("world")
   }
```

when this function gets executed, since String has dynamic size, we store it in heap, then heap passes back a pointer. the pointer is what we actually store on the stack

- pushing to stack is faster than allocating on the heap because the heap has to spend time looking for a place to store the new data. Also accessing data on the stack is faster than accessing data on the heap because with the heap u have to follow the pointer.

```rs
let s: &str ="hello";
```

- string literals are stored directly in binary and are fixed. so stored in stack

## MOVE

```rs
let s1:String= String::from("hello");
let s2:String= s1;
```

To ensure memory safety, rust invalidates s1, so instead of being shallow copy, this called a `Move`

```rs
fn main() {
  let s=String::from('hello')
  takes_ownership(s)
  println!("{}",s)
  // Error: borrow of moved value "s". value borrowed here after move. so s cannot be borrowed after a move
  // when we pass a parameter into a function it is the same as if we were to assign s to another variable. Passing 's' moves s into the 'some_string' variable then `println!("{}",some_string)` executed, "some_string" printed out. After this scope is done, some_string gets dropped.

  let x:i32 = 5;
  makes_copy(x)
  // instead of being moved, integers are copied. we can still use "x" after the function
  println("{}",x)
}

fn takes_ownership(some_string:String){
  println!('{}',some_string);
}

fn makes_copy(some_integer:i32){
  println!("{}", some_integer)
}
```

## Copy

```rs
let s1:String= String::from("hello");
let s2:String= s1.clone();
```

Rust defaults to moving a value. If you want to perform more expensive clone operation use .clone

## Copy trait:

```rs
let x: i32 = 5;
let y: i32= x;
```

Integers, booleans and characters are stored on stack. this are copied instead of move.

## REFERENCES

```rs
fn main(){
  let s1=String::from("hello");
  // passing reference
  let len= calculate_lenght(&s1);
  println!("the lenght of '{}' is {}.",s1,len )
}

// we have to pass a reference otherwise we would have borrow of moved value error.
// references do not take the ownership of the underlying value
fn calculate_length(s:&String)->usize{
  let length=s.len();
  length
}
```

- passing references as function parameters is called `BORROWING`. We are borrwoing but we are not actually taking owner ship of it.
- References are immutable my default. If I try to modify `s` inside `calculate_length` I will get error. "We cannot borrow `*s` value as mutable, as it is behind a `&` reference `s` is a `&` reference. " Using `mut`

```rs
fn main(){
  let mut s1=String::from("hello");
  change(&mut s1)
}

fn change(some_string: &mut String){
  some_string.push_str("world")
}
```

- mutable references has a big restriction which is you can have only one mutable reference to a particular piece of data in a particular scope.

```rs
fn main(){
  let mut s=String::from("hello")

  let r1= &mut s;
  // Error cannot borrow "s" as mutable more than once at a time. Second mutable borrow occurs.
  let r2= &mut s;
}
```

- the big benefit of this restriction that rust can prevent data races at compile time. a data race occurs if we have tow pointers pointing to the same piece of data and one of those pointers is used to write to the data and there is no mechanism to synchronize data access between those pointers. in that situation, u could imagine one pointer will read the data and in the middle, other pointer modifying the data. in that case we are gonna get corrupted data back. to fix this error we can switch these references bcak to be immutable reference.

```rs
fn main(){
  let  mut s=String::from("hello")

  let r1= &mut s;
  // Error cannot borrow "s" as mutable more than once at a time. Second mutable borrow occurs.
  let r2= &mut s;
}
```

## Mix immutable references with mutable references

```rs
fn main(){
  let mut s=String::from("hello")

  let r1= s;
  let r2= s;
  let r3=&mut s;
  // cannot borrow 's' as mutable because it is also borrowed as immutable
}
```

- Immutable references do not expect the underlying value to change and this is problematic if we do have a mutable reference. You can however have multiple immutable references because underlying data is not gonna change.

## Dangling References

- what happens if we have a reference that points to invalid data

```rs
fn main(){
  let reference_to_nothing=dangle()
}

// Error: this function's return type contains a borrowed value, but there is no value for it to be borrowed.
fn dangle() -> String{
  let s= String::from("hello");
  // s is defined in this scope. when this function finishes executing rust will drop our string. it will deallocate string from heap
  // our reference will point to invalid memory
  &s
}
```

- recommendation is to use "lifetime".
  the rules of References

- at any given time, you can either have one mutable reference or any number of immutable references
- References must always be valid.

## Slices

Slices lets you reference a contiguous sequence of elements within a collection instead of referencing the entire collection. Slices do not take the ownership of the underlying the data.

```rs
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
      // this signifies the end of the string
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
```
