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

	var arr = Array();

	var var1 = asdasd_asdfhnsdfhsd_asdasd;

	var var2 = 2 + 5;
	var var3 = (var2 * 1.5) / 3 + a[0].hello(1, 2, 3, "hello", []) * a[0][1]->bye->a.asd(123123123 * 2, []) + 2;

	return [var3, var2];
};