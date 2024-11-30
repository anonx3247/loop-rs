# loop-rs
Implementation of Loop in Rust

Loop is a modern programming language that combines simplicity with power. It supports both procedural/functional and object-oriented paradigms while providing helpful features for rapid development.

## Key Features

- Clean, expressive syntax
- Type inference with optional type annotations 
- First-class async support
- Pattern matching and exhaustive matches
- Immutable by default with opt-in mutability
- Git-based package management
- Both interpretable and compilable

## Code Examples


```loop
-- loop has can both be a simple language and a powerful one.
-- similarly to C++ you can do entirely procedural programming (or in this case functional) or OOP
-- loop also comes with a lot of niceties and simple modules to get started very quickly.

import {
    'git:github.com/anonx3247/mymodule' -- as you can see you can add git repos as modules
    'std:os'
    'std:geometry' 
    'std:http'
}

from tensor import Tensor

enum Color {
    (u8, u8, u8)     -- RGB
    (u8, u8, u8, u8) -- RGBA
}

response := async http.get(
    server: '192.168.0.1'  -- named parameters
    type: http.GetType.Text
)?  -- optional propagation

red : Color = (255, 0, 0)
mut my_fav_color := red -- mutable variable

struct Person {
    required name: str
    lastname: str = 'Smith'
    age: uint
}

anas := Person(name: 'Anas', lastname: 'Lecaillon', age: 22)

-- Pattern matching
print(match my_fav_color {
    red => 'copycat!'
    (r, g, b, a) => match a {
        a == 255 => 'not transparent'
        a != 255 => 'transparent'
    }
    (r, g, b) => 'transparent'
})

```

