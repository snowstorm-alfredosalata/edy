# edy
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)
[![Semantic Versioning](https://img.shields.io/badge/Semantic%20Versioning-2.0.0-%234c79c0?logo=semver&logoColor=white)](https://semver.com)
[![Conventional Commits](https://img.shields.io/badge/Keep%20a%20Changelog-1.1.0-%23e5534b?logo=keepachangelog&logoColor=white)](https://keepachangelog.com/en/1.1.0/)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`edy` is simple and ergonomic library for handling dynamic-typed values, featuring highly-tailorable features, no heap allocations for primitive types, and more.

## Edy at a glance
```rust
use edy::prelude::*;
 
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
 
fn main() -> Result<(), Error> {
    let dy: Dynamic = 2i32.into();
    let v: i32 = double_dynamic(dy)?
                 .try_into()?;
 
    assert_eq!(4, v);
 
    let dy: Dynamic = "2".into();
    let v: String = double_dynamic(dy)?
                    .try_into()?;
 
    assert_eq!("22", &v);
 
    Ok(())
}
```

### Features
- Simple, lightweight enumeration over rust primitives
- Optional support for some types from other popular libraries

### Documentation
Currently in progress.

## FAQ
### How fast is `edy`?
I have not run any benchmark, but by design `edy` has significant advantages over other options based on `Any`, as it will not run any non-needed heap allocation. 
This certainly comes at the cost of flexibility.

### Is it production ready?
Although the API will likely not change much, the internals are just a draft and will likely feature major changes. `edy` is not reccomended for production yet. 

### Why the name?
Simple: **e**num-based **dy**namic. **edy**!

### Can I contribute?
Not at the moment, but it'll be certainly possible after I reach a better degree of completion and doc coverage in the internals.
