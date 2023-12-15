# edy

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`edy` is simple and ergonomic library for handling dynamic-typed values, featuring highly-tailorable features, no stack allocations for primitive types, and more.

## Edy at a glance
```rust

fn double_dynamic(value: Dynamic) -> Result<Dynamic, Error> {
    match value {
        Dynamic::Str(v) => Ok(format!("{v}{v}").into()),
        Dynamic::Int(v) =>  Ok((v*2).into()),
        Dynamic::Float(v) => Ok((v*2f64).into()),
        _ => Err(UnsupportedTypeError { 
                expected_types: vec![DynamicType::Str, DynamicType::Int, DynamicType::Float], 
                found_type: value.get_type() 
            }.into())
    }
}

fn main() {
    let dy_float = 2i32;

    if let Ok(Dynamic::Int(v)) = double_dynamic(dy_float.into()) {
        // use this value somehow...
    }
    else {
        // handle the error
    }
    
}
```

### Features
- Simple, lightweight enumeration over rust primitives
- Optional support for some types from other popular libraries

### Documentation
Currently in progress.

## Why `edy`?
`speare` is a minimal abstraction layer over [tokio green threads](https://tokio.rs/tokio/tutorial/spawning#tasks) and [flume channels](https://github.com/zesterer/flume), offering functionality to manage these threads, and pass messages between these in a more practical manner. The question instead should be: *"why message passing (channels) instead of sharing state (e.g. `Arc<Mutex<T>>`)?"*

- **Easier reasoning**: With message passing, each piece of data is owned by a single thread at a time, making the flow of data and control easier to reason about.
- **Deadlock Prevention**: Shared state with locks (like mutexes) can lead to deadlocks if not managed carefully. Message passing, especially in Rust, is less prone to deadlocks as it doesn’t involve traditional locking mechanisms.
- **Encouragement of Decoupled Design**: Message passing promotes a more modular, decoupled design where components communicate through well-defined interfaces (channels), enhancing maintainability and scalability of the code.

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