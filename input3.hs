var a = 2 + 2 * 2;

var b = some_func() + 2 / 2 * (12312312);

var c = other_func()()()()()() + 1337 * 323289.2 - 1;

var d = array[0][1][2][3][4][5] - 1 + idk_anymore();

var e = array[0]("hello, world!" + "!")("asd")[4 - 2];

var f = some_class->some_property + some_class->some_function();

var e = something.something_else / something.something_more();

var f = asd->2(); // this should be handled later on i guess?

type SomeType<T> = T[];

// single-line comments work // asdasdasdasd

/*
multi-line ones work too
*/

function world() -> str {
	return "hello";
}

class ClassDefinition {
	a: int[][] = "default value";
	b: SomeType<int>[];

	#[static, constructor]
	function some_function() -> bool {

	};

	c: bool;

	function another_function() -> int {

	}
};

declare class ClassDeclaraction {
	a: int;

	#[static, constructor]
	function some_function() -> bool;

	function other_function() -> int;
}

declare class Array<T> {
	#[get]
	function length() -> int;

	function append(_elem: T) -> void;
	function copy() -> this;
}

function hello() -> SomeType<int> {
	world(123);
	asdasd.world(123, ["hello", "world!"]);

	var arr = Array();

	var var1 = asdasd_asdfhnsdfhsd_asdasd;

	var var2 = 2 + 5;
	var var3 = (var2 * 1.5) / 3 + a[0].hello(1, 2, 3, "hello", []) * a[0][1]->bye->a.asd(123123123 * 2, []) + 2;

	return [var3, var2];
};