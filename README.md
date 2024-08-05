# Ident-Mash

Have you ever had a bunch of identifiers that you "just want to mash together"
inside a macro but don't want to resort to writing a procedural macro?

Ident-Mash has got you covered, because it takes care of the annoying proc macro
part for you!

```rust
macro_rules! create_struct {
    ($name:ident) => {
        struct $name { ... }
        
        ident_mash::mash!{
            module = my_secret_ + :snake_case($name) + _module &
            constant = MY_SECRET_ + :upper_snake_case($name) + _CONST =>
            
            #[doc(hidden)]
            mod $module {
                const $constant: usize = 42;
                /* Secret stuff here */
            } }
        
         
    };
}

create_struct!(MyStruct);
```

## Capabilities

As you can see from the example, creating more than one meta variable is allowed
as well as using certain functions like `:snake_case` and `"string literals"`
instead of idents.
But there are other functions, which all take a list of idents or other functions.
In fact, they support recursion.

| Name                | Function                                                                                     |
|---------------------|----------------------------------------------------------------------------------------------|
| `:upper_case`       | Turns everything uppercase                                                                   |
| `:lower_case`       | Turns everything lowercase                                                                   |
| `:snake_case`       | Uses the `convert_case` crate to convert the ident to `snake_case`                           |
| `:upper_snake_case` | Same working mechanism but converts to `UPPER_SNAKE_CASE`                                    |
| `:pascal_case`      | Same working mechanism but covnerts to `PascalCase`                                          |
| `:hash`             | Computes a hash on the concatenated idents. Useful for generating deterministic random names |

Extreme example:
`:hash(:snake_case(x + y + z) + x) + z + :hash(z)`

## More examples

The `/tests` directory contains tests that show off all the possible use cases of the
macro, without making it too difficult to understand what's going on.

## Bugs

It is possible to break the procedural macros or get it to produce unexpected results.
How the macro works is it searches for the `$meta_var` pattern and replaces it
with the generated ident.

But the `meta_var` name is the parameter you give as the first argument to `mash!`.
For this reason, it is important to avoid giving a dynamic name to the meta variable.

E.G. don't do this:
```rust
macro_rules! dangerous {
    ($name:ident) => {
        struct $name { ... }
        
        ident_mash::mash!{
            $name = my_secret_ + :snake_case($name) + _module &
            constant = MY_SECRET_ + :upper_snake_case($name) + _CONST =>
            
            #[doc(hidden)]
            mod $$name {
                const $constant: usize = 42;
                /* Secret stuff here */
            } }
        
         
    };
}

dangerous!(MaliciousName);
```

This specific example doesn't even compile, but I think it illustrates the possible danger.

## Future plans

More functions may be added in the future. If there is a function you'd like to see
added, feel free to open a PR!