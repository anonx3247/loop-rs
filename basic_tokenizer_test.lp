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

enum Color { -- sumtypes / enums à la Rust
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

struct Person {
	required name: str -- every type has a 'default' value, this ensures it must be specified.
	lastname: str = 'Smith' -- you can also specify a default value
	age : uint -- u32 for 32bit and u64 for 64bit machines
}

anas := Person(name: 'Anas', lastname: 'Lecaillon', age: 22) -- struct instantiation

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

fn greet({name: str?}) { -- implicit struct
	print('Hello {name}!')
}

-- the use of structs makes us use named parameters, basically:
greet(name: 'Anas')
-- is short for
greet({name: 'Anas'}) -- you can have multiple definitions of a function as long as they do not contradict each other (i.e. they do not have the same definition)

greet(anas) -- this works thanks to the named parameters of the function which match those of the struct or class

fn origin() -> {x: int, y: int} { -- implicit struct definition and return type
	ret {x: 0, y: 0} -- implicit struct (return statement)
}

say_bye := (name) -> 'Bye {name}!' -- lambda expression (the types are inferred here)

abs class GeometryObject { -- a class can contain both methods and parameters or either (note this is an abstract class so it cannot be instantiated)
	GeometryObject(
		mut .position: (i64, i64) -- implicit constructor (as you can see mutable parameters must be declared as such)
	)
	-- this implicit constructor is short for
	let mut position : (i64, i64) -- let statements declare variables without assigning a value, thus they need a type
	GemoetryObject init({position: (i64, i64)}) -> this.position = position

	fn move_to_origin() -> position = (0, 0)
	fn move_to({x: i64, y: i64}) -> position = (x, y)

	get x -> position[0] -- you can define getters this way. (also tuples are indexable)
	-- not unlike in other languages
	-- a 'get' can only be pure functions
	-- they cannot change the state of the object
	-- to do so you will have to make new 'getter's with method notation
	-- this ensures that getters truly work like parameters
	-- and we can see when state changes on a get
	set x -> position[0] -- 'set' can only be defined this way with a clear reference to a mutable parameter of the class
	get set y -> position[1] 	 -- as you can see you can also define them both at the same time here
}

class Point of GeometryObject -- inheritance

class Rectangle of GeometryObject {
	Rectangle(
		mut .width, .height : i64 -- note we put a '.' when they are parameters of the class
		mut ._last_area : i64 -- we declare a private parameter (marked as private with the '_' prefix) it cannot be assigned here so it must be in the constructor body.
		super -- this adds all the super parameters
	) { -- added constructor logic
		if width == 0 and height == 0 {
			width, height = 1, 1 -- assign two variables at once, syntax sugar for: (width, height) = (1, 1)
			print('Cannot have empty rectangle! Set width and height to 1!')
		}
		_last_area = area
	}

	getter area { -- since this function is impure, we use 'getter' instead of 'get'
		_last_area = area
		width * height -- note since everything is an expression the last line is a return value by default (this is true in any function definiton)
	}

	setter area(new: i64) { -- again since this doesn't simply reference a parameter we use a 'setter' instead of 'set'
		if width > height {
			height = (new / width) except { -- or statements catch errors on a result type (in this case the / symbol has a possible error return value)
				print('canot divide by zero')
				width = 1
				new
			}
		} else {
			width = (new / height) except {
				print('cannot divide by zero')
				height = 1
				new
			}
		}
	}
	
}

mut my_rect := Rectangle(width: 10, height: 5) -- instantiation
print(my_rect.x) -- 0
my_rect.get(area) -- 50 (this is the notation used for impure getters)
my_rect.set(area: 30) -- height = 3 -- and this is the notation used for setters with side effects

-- by default types are inferred restrictively (to a point), which means that it will take the smallest enum type possible

x := 5 -- this assumes 'int' (int is a BigInt by default, which means it can be arbitrarily large)
y := if x > 3 {'hello'} else {3} -- this is a enum of str and int

my_list := [1, 2, 3] -- List<int>
my_list := [1, 2, 'hello'] -- List<enum {str, int}>
my_list : List<Any> = [1, 5, 3] -- List<Any> (Any is for any type)

fn sum<T implements Addition> (items: List<T>) -> { -- generics
	s := T.zero -- a getter on the 'Addition' class
	for item in items {
		s += item
	}
	s
}


sum([1, 2, 4]) -- 7
sum(['a', 'b', 'c']) -- 'abc'
sum([3.4, 1.2, 0.0]) -- 4.6

-- generics also work on classes of course

class Set<T> implements Collection, Addition { -- classes can only inherit from one class at a time, but they can 'implement' multiple abstract classes.
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

	@overload
	fn in(element: T) -> _elements[element] except false -- here we are overloading the Collection class's 'in' member, this overloads the 'in keyword'

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

	@overload
	fn add(Set<T> b) -> Set(elements: _elements.keys + b.keys)

	@overload
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
