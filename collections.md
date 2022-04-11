- collections are stored in heap

## Vector

```rs
fn main(){
    let a=[1,2,3];
    // vector can grow in size
    let mut v:Vec<i32>=Vec::new();
    v.push(1);
    v.push(2);
    // intialize vector with values using vec! macro
    let v2=vec![1,2,3];
    // if v3 is out of scope, rust drops it
    {
         let v3=vec![112,221,23];
    }
}
```

- accessing elements in vector. there are two ways
  1- directly reference an index in the vector. the problem with this, we could specify invalid index. with arrays we know the size of the array so we get the error in compile time. But with vector we dont know the size at compile time. so accessing this way, you accept that your app might crash at run time.

      `let third=&v[2]`

2- Using the `get` method. this is the safest way. this returns an `option`

```rs
fn main(){
     let v=vec![1,2,3];
     let third=&v[2]

     match v.get(2){
         Some(third)=>println!("the third is{}",third),
         None => println!("there is no third element)
     }
}
```

- when we access elements in a vector we are getting a reference to that element. some ownership rules, you cannot have a immutable reference and mutable reference to the same thing at the same time.
- when we push a new element into a vector, we might need to allocate more memory to make room for that new value. when we do that, we need to move all the elements in our vector to new memory locations. If there were happen, then our variable we declared here.

```rs
fn main(){
     let mut v=vec![1,2,3];
     // we take immutable reference
     let third=&v[2]
     // we take mutable reference here
    v.push(6)
    // Error. cannot borrow 'v' as mutable because it is also borrowed as immutable.
    println!("the third is {}",third)
}
```

- for loop in vector

```rs
fn main(){
     let mut v=vec![1,2,3];
     for i in &v{
         println!("{}",i)
     }
     // we could get mutable reference, too
     let mut v=vec![1,2,3];
     for i in &mut v{
         // * is dereferencing operator to get the underlying value and add 50 to it.
         *i+=50
         println!("{}",i)
     }
     for i in &v{
         // 51,52,...
         println!("{}",i)
     }
}
```

- we can store enum values inside vectors. vectors can store only one type of data. for example row of cells in a spreadsheet.

```rs

fn main(){
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row=vec![SpreadsheetCell::Int(3),
                 SpreadsheetCell::Text(String::from('blue')),
                 SpreadsheetCell::Float(10.12)
                ]
    // when you reference a specific element inside a vector, you have to use match expression to figure out which variant of enum it is.
    match &row[1]{
        SpreadsheetCell::Int(i)=>println!("{}",i),
        _ => println!("Not a integer!")
    }
}
```

## STRINGS

- In higher programming languages, complexity of strings is abstracted away from the programmer but in lower programming languages such as rust, we have to deal with that complexity.
- in rust, strings are stored as a collection of utf-8 encoded bytes. In memory, strings are just collection of 1's and 0's. a program needs to be able to interpret those 1's and 0's and print out the correct characters. that's where encoding comes into play.
- Ascii is a string encoding, it defines how to take 1's and 0's and turn it into a string, or take a string and turn it into ones and zeroes. the problem is each asci character is stored as a byte and only 7 bits of that byte are used to represent the character. that means ascii can represent only 128 unique characters and so ascii only represents english alphabet some special characters and a few commands.
- to encode all the chars in all languages, `unicode` is created. Universal Character Set. Unicode is backwards compatible with ascii and that is because the first 128 symbols of unicode are ascii characters.
- utf-8 is a variable-width character encoding for unicode. Variable-width because each character in utf-8 could be represented as one byte,two bytes etc. remember in ascii each character is represented by 1 byte but with utf-8, each character could be different size in terms of bytes.

```rs
fn main(){
    // empty string
    let s1:String=String::new();
    let s2:&str="initial contents";
    let s3:String=s2.to_string();
    let s4:String=String::from("initial contents")
}
```

- Just like a vector, a string can grow or shrink.

```rs
   let mut s:String=String::from("foo")
   // push_str takes a string slice. we dont want to take ownership of the string being passed in.
   s.push_str("bar")
   s.push("!")
    // we could also add string with + operator
    let s1=String::from("Hello, ")
    let s2=String::from("world!")
    // we are moving the ownership of s1 into s3 and then we are taking all the characters in s2 and appending them to the end of s3. this saves a liitle bit of memeory compared to copying both strings
    // because we moved the ownership of s1, if we try to use "s1" after we have declared s3, we will get an error. "We cannot borrow a value after it has been moved"
    let s3=s1+&s2
```

**INDEXING**

```rs
   fn main(){
       let hello:String=String::from("Hello")
       // we could this in higher programming languages. in rust we get error. cannot be indexed by an integer
       // becuase string is a collection of bytes. so what is the lenght of our "hello". Because some chars can be 1 to 4 bytes long. If you want to get the first char in string, using the indexing, hello[0] will specify the first byte.
       let c:char=hello[0]
   }
```

- 3 relevant ways a word in represented in unicode:

  Bytes, scalar values,
  graphene clusters: each char in string.

  another problem with indexing, rust does not know what we will receive. Bytes, scalar value or graphene clusters. so we have to use more specific methods.

- In order to access the bytes of strings, we can use the bytes method.

```rs
   for b in "dsfsd".bytes(){
       // bytes method returns a collection of bytes and here we are iterating over every byte and printing it out
       println!("{}",b)
   }

   // we could iterate over scalar values using char methods
   for c in "kjdskj".chars(){
       println!("{}",c)
   }
```

**Grapheme Clusters**

```rs
   // in order to keep rust standard library lean, the ability iterate over graphene clusters is not included by default. we need to import a crate
   // in cargo.toml
   [dependencies]
   unicode-segmentation="1.7.1"

   use unicode_segmentation::UnicodeSegmentation;
   // we pass true to get extended grapheme clusters
   for g in "dada"graphemes(true){
       println!("{}",g)
   }

```

## HASH MAPS

```rs

use std::collections::HashMap;

fn main(){
    let blue=String::from("Blue");
    let yellow=String::from("Yellow")

    let mut scores=HashMap::new();
}
```
