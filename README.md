# edy

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`edy` is simple and ergonomic library for handling dynamic-typed values, featuring highly-tailorable features, no stack allocations for primitive types, and more.

## Edy at a glance
```rust

fn double_dynamic(value: Dynamic) -> Result<Dynamic, UnsupportedTypeError> {
    match value {
        Dynamic::Str(v) => Ok(format!("{v}{v}").into()),
        Dynamic::Int(v) =>  Ok((v*2).into()),
        Dynamic::Float(v) => Ok((v*2f64).into()),
        _ => Err(UnsupportedTypeError { 
                expected_types: vec![DynamicType::Str, DynamicType::Int, DynamicType::Float], 
                found_type: value.get_type() 
            })
    }
}

fn main() {
    let mut my_int = 2i32;

    my_int = double_dynamic(my_int.into())
                    .unwrap()
                    .try_into()
                    .unwrap();

    assert_eq!(my_int, 4)
}

```

### Features
- Simple, lightweight enumeration over rust primitives
- Optional support for some types from other popular libraries

### Documentation
Currently in progress.

## FAQ
### How fast is `edy`?
I have not run any benchmark, but by desing `edy` has significant advantages over other options based on `Any`, as it will not run any non-needed stack allocation. 
This certainly comes at the cost of flexibility.

### Is it production ready?
Although the API will likely not change much, the internals are just a draft and will likely feature major changes. `edy` is not reccomended for production yet. 

### Why the name?
Simple: **e**num-based **dy**namic. **edy**!

### Can I contribute?
Not at the moment, but it'll be certainly possible after I reach a better degree of completion in the internals.
