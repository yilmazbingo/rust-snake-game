Rust has three kinds of struct types, named-field, tuple-like, and unit-like, which differ in how you refer to their components: a named-field struct gives a name to each component, whereas a tuple-like struct identifies them by the order in which they appear. Unit-like structs have no components at all; these are not common, but more useful than you might think.

## Named-Field Structs

```rs
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}
```

- The struct expression `GrayscaleMap { pixels, size }` is short for
  `GrayscaleMap { pixels: pixels, size: size }`

- Like all other items, structs are private by default
- Even if a struct is declared pub , its fields can be private:

## Tuple-Like Structs

The second kind of struct type is called a tuple-like struct, because it resembles a tuple:
`struct Bounds(usize, usize)`

## Unit-Like Structs

The third kind of struct is a little obscure: it declares a struct type with no elements at all:
`struct Onesuch`
A value of such a type occupies no memory, much like the unit type () . Rust doesn’t bother actually storing unit-like struct values in memory or generating code to operate on them, because it can tell everything it might need to know about the value from its type alone. But logically, an empty struct is a type with values like any other—or more precisely, a type of which there is only a single value:

    `let o = Onesuch`

- Unit-like structs can also be useful when working with traits,

## Defining Methods with impl

- Functions defined in an impl block are called `associated functions`, since they’re associated with a specific type. The opposite of an associated function is a `free function`,
  one that is not defined as an impl block’s item.
