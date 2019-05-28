# mbti

A library to work with Myer-Briggs personality types

[![Build Status](https://travis-ci.org/Pomettini/mbti.svg?branch=master)](https://travis-ci.org/Pomettini/mbti)
[![Build status](https://ci.appveyor.com/api/projects/status/2aq98uydgbamd12t?svg=true)](https://ci.appveyor.com/project/Pomettini/mbti)
[![Coverage Status](https://coveralls.io/repos/github/Pomettini/mbti/badge.svg?branch=master)](https://coveralls.io/github/Pomettini/mbti?branch=master)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Usage

Add this to your `Cargo.toml`

```toml
[dependencies]
mbti = ">=0.1.1"
```

## Examples

Get a function from a MBTI type:

```rust
extern crate mbti;

use mbti::{get_function, Function, Role, Type};

fn main() {
    let primary = get_function(Type::INTP, Role::Primary);
    assert_eq!(primary, Function::Ti);
}

```

Get all the functions from a MBTI type:

```rust
extern crate mbti;

use mbti::{get_functions_from_type, Function, Type};

fn main() {
    let functions = get_functions_from_type(Type::INTP);
    assert_eq!(
        functions,
        vec![Function::Ti, Function::Ne, Function::Si, Function::Fe]
    );
}
```

Get all the MBTI types from a function role:

```rust
extern crate mbti;

use mbti::{get_types_from_function_role, Function, Role, Type};
use std::collections::HashSet;
#[macro_use]
extern crate maplit;

fn main() {
    let types = get_types_from_function_role(Function::Fe, Role::Primary);
    assert_eq!(types, hashset![Type::ESFJ, Type::ENFJ]);
}

```

Get a MBTI types from a set of functions:

```rust
extern crate mbti;

use mbti::{get_type_from_functions, Function, Type};

fn main() {
    let functions =
        get_type_from_functions(&[Function::Ti, Function::Ne, Function::Si, Function::Fe]);
    assert_eq!(functions, Some(Type::INTP));
}
```

Get compatibility between two MBTI types:

```rust
extern crate mbti;

use mbti::{check_compatibility, Compatibility, Type};

fn main() {
    let compatibility = check_compatibility(Type::INTP, Type::INFP);
    assert_eq!(compatibility, Compatibility::Positive);
}

```

## License

The MIT License (MIT)

Copyright © `2019` `Giorgio Pomettini`

Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation
files (the “Software”), to deal in the Software without
restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

