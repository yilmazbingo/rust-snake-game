## Rust Macros

Macros provide functionality similar to functions but without the runtime cost. There is some compile-time cost, however, since macros are expanded during compile time.

1- **Declarative macros** enable you to write something similar to a match expression that operates on the Rust code you provide as arguments. It uses the code you provide to generate code that replaces the macro invocation

These macros are declared using macro_rules!. Declarative macros are a bit less powerful but provide an easy to use interface for creating macros to remove duplicate code. One of the common declarative macro is println!. Declarative macros provide a match like an interface where on match the macro is replaced with code inside the matched arm.

```rs
// use macro_rules! <name of macro>{<Body>}
macro_rules! add{
 // macth like arm for macro
    ($a:expr,$b:expr)=>{
 // macro expand to this code
        {
// $a and $b will be templated using the value/variable provided to macro
            $a+$b
        }
    }
}
fn main(){
 // call to macro, $a=1 and $b=2
    add!(1,2);
}

```

2- **Procedural macros in Rust**
Allow you to operate on the abstract syntax tree (AST) of the Rust code it is given. A proc macro is a function from a TokenStream (or two) to another TokenStream, where the output replaces the macro invocation.
Procedural macros are a more advanced version of macros. Procedural macros allow you to expand the existing syntax of Rust. It takes arbitrary input and returns valid Rust code. Procedural macros are functions that take a TokenStream as input and return another Token Stream. Procedural macros manipulate the input TokenStream to produce an output stream.

There are three types of procedural macros:

        - Attribute-like macros
        - Derive macros
        - Function-like macros
