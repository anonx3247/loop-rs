-- loop has can both be a simple language and a powerful one.
-- similarly to C++ you can do entirely procedural programming (or in this case functional) or OOP
-- loop also comes with a lot of niceties and simple modules to get started very quickly.
-- loop will both be interpretable and compilable


import {
	'git:github.com/anonx3247/mymodule' -- as you can see you can add git repos as modules. they will be imported as submodules.
	'std:os'
	'std:geometry'
	'std:http'
	'my/other/module/in/this/path'
}

from tensor import Tensor

type Color { -- sumtypes / types à la Rust
	(u8, u8, u8) -- RGB
	(u8, u8, u8, u8) -- RGBA -- as you can see we also have tuples
}

response := async http.get( -- note that calling async functions requires the async keyword as well to make sure we know the value is async. (also any async method calling async methods is by definition async)
	server: '192.168.0.1' -- named parameters
	type: http.GetType.Text -- note that newlines serve the same purpose as ','
)? 
-- optional propagation (you cannot simply give a value that has indeterminate type)
-- this allows always knowing if a value returned is optional or not
-- the same thing happens for errors with '!'
-- note this was a multiline comment.

red : Color = (255, 0, 0) -- we specify the type here in this variable declaration


mut my_fav_color := red -- all vars are immutable by default (also type inference)

comp Person {
	name: str 
	lastname: str = 'Smith' -- you can also specify a default value
	age : uint -- u32 for 32bit and u64 for 64bit machines
}

anas := Person(name: 'Anas', lastname: 'Lecaillon', age: 22) -- comp instantiation

x := 3

-- everything is an expression, not an instruction
y := if 0 <= x <= 10 { 	 -- note how you can chain inequalities like this
	'low'
} else {
	'high'
}


print(match my_fav_color { -- no switches but match expressions
	red => 'copycat!'
	(r, g, b, a) => match a { -- nested matches
		a == 255 => 'not transparent'
		a != 255 => 'transparent'
	}
	(r, g, b) => 'transparent' -- match expressions verify exhaustiveness
})


fn greet(name: str) { -- parameter types must be specified for function defs
	print('Hello {name}!') -- string interpolation
}

fn greet({name: str?}) { -- implicit comp
	print('Hello {name}!')
}

-- the use of structs makes us use named parameters, basically:
greet(name: 'Anas')
-- is short for
greet({name: 'Anas'}) -- you can have multiple definitions of a function as long as they do not contradict each other (i.e. they do not have the same definition)

greet(anas) -- this works thanks to the named parameters of the function which match those of the comp or comp

fn origin() -> {x: int, y: int} { -- implicit comp definition and return type
	ret {x: 0, y: 0} -- implicit comp (return statement)
}

say_bye := (name) -> 'Bye {name}!' -- lambda expression (the types are inferred here)

abs comp GeometryObject {
	mut position: (i64, i64) -- implicit constructor (as you can see mutable parameters must be declared as such)

	fn move_to_origin() -> position = (0, 0)
	fn move_to({x: i64, y: i64}) -> position = (x, y)
}

comp Point impl GeometryObject

comp Rectangle impl GeometryObject {
	mut width: i64
	mut height: i64
	mut _initial_area: i64 -- the _ makes it private
	init { -- added constructor logic done on default constructor
		if width == 0 and height == 0 {
			width, height = 1, 1 -- assign two variables at once, syntax sugar for: (width, height) = (1, 1)
			print('Cannot have empty rectangle! Set width and height to 1!')
		}
		_initial_area = width * height -- must be given a value in the constructor, unless made optional
	}

	Rectangle({width: i64, area: i64}) { -- alternate constructor
		.width = width
		height = area // width
		_initial_area = area
	}

	fn area() -> i64 {
		width * height -- note since everything is an expression the last line is a return value by default (this is true in any function definiton)
	}
}

mut my_rect := Rectangle(width: 10, height: 5) -- instantiation
print(my_rect.x) -- 0
my_rect.get(area) -- 50 (this is the notation used for impure getters)
my_rect.set(area: 30) -- height = 3 -- and this is the notation used for setters with side effects

-- by default types are inferred restrictively (to a point), which means that it will take the smallest type type possible

x := 5 -- this assumes 'int' (int is a BigInt by default, which means it can be arbitrarily large)
y := if x > 3 {'hello'} else {3} -- this is a type of str and int

my_list := [1, 2, 3] -- List<int>
my_list := [1, 2, 'hello'] -- List<type {str, int}>
my_list : List<Any> = [1, 5, 3] -- List<Any> (Any is for any type)

fn sum<T implements Addition> (items: List<T>) -> { -- generics
	s := T.zero -- a getter on the 'Addition' comp
	for item in items {
		s += item
	}
	s
}


sum([1, 2, 4]) -- 7
sum(['a', 'b', 'c']) -- 'abc'
sum([3.4, 1.2, 0.0]) -- 4.6

-- generics also work on compes of course

comp Set<T> implements Collection, Addition { -- compes can only inherit from one comp at a time, but they can 'implement' multiple abstract compes.
	Set(
		required mut elements: List<T>
		let _elements : Map<T, bool>
	) {
		pruned : List<T> = []
		for element in elements {
			if element not in pruned {
				pruned.append(element)
			}
		}
		for element in pruned {
			_elements[element] = true
		}
	}

	fn in(element: T) -> _elements[element] except false -- here we are overloading the Collection comp's 'in' member, this overloads the 'in keyword'

	fn insert(element: T) {
		if element not in self { -- now we can use our overloaded 'in' here
			_elements[element] = true
		}
	}

	fn remove(element: T) {
		match _elements[element] {
			error or false => _
			true => _elements[element] = false
		}
	}

	fn add(Set<T> b) -> Set(elements: _elements.keys + b.keys)

	get zero -> Set<T>(elements: [])
}

N = Set([2*i for i in 0..int.max//2]) -- list comprehension and ranges

a[0..3] -- slices


3 // 2 == 1 -- integer division

3 / 2  -- returns a 'rational' type

a := Tensor([1, 2, 3]) -- tensor types (unlike lists which are linked lists and can change size, tensors are fixed in size and much faster)

a * 3 -- Tensor([3, 6, 9]) -- for all intents and purposes think of this as a numpy array
a = a.T -- Tensor([[3], [6], [9]])

b := Tensor<f64>.vector([3, 8, 7]) -- Tensor([[3.0], [8.0], [7.0]])

a.dot(b) -- 3*3+6*8+9*7 == 120


comp ViewRect {
	min_width: f32 = 0
	min_height: f32 = 0
	max_width := f32.inf
	max_height := f32.inf
}

ViewRect impl Widget {
	fn draw() {
		...
	}
}

