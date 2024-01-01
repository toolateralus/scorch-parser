pub mod ast;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use lexer::{create_tokenizer, TokenProcessor};

    use super::*;

    #[test]
    fn test_main() {
        // todo: make testing much much better in this project.
        // tests for both tokenizer and parser, individual rules & actual unit size tests.
        let input = "

const x := 0
const y := 1020
var y := 100
var z := 100

struct X |
    const z := 10
	const y : int = 0
	const n : double
	var z := new X()
|

X :: {
    funcy_test :=  {
        break 100
    }
}
        
ff_addition 		:= 5.3 + 6.2
ff_subtraction 		:= 5.3 - 6.2
ff_multiplcation 	:= 5.3 * 6.2
ff_division 		:= 5.3 / 6.2
ff_parenthesis_1 	:= (5.3 + 6.2) * 2.5
ff_parenthesis_2 	:= 5.3 - (6.2 * 3.1)
ff_complex_1 		:= (5.3 + 6.2) / (3.1 - 2.0)
ff_complex_2 		:= 5.3 + (6.2 * 3.1) / 2.5
ff_complex_3 		:= (5.3 - 6.2) * 2.5 / 3.1
ff_complex_4 		:= 5.3 / (6.2 + 3.1) * 2.5
ff_complex_5 		:= 5.3 + 6.2 - 3.1 * 2.0 / 1.5
ff_complex_6 		:= ((5.3 * 2.5) - 6.2) / 3.1 + 1.0
empty_implicit 			:= []

assert(len(empty_implicit) == 0, 'empty_implicit array failed to be empty')

// # todo implement array comparison
// # assert(len(empty_implicit) == 0, 'empty_implicit array failed to compare to []')

empty_explicit 			: Array = []
assert(len(empty_explicit) == 0, 'empty_explicit array failed to be empty')

single_float_implicit 	:= [1.0]
assert(len(single_float_implicit) == 1, 'single_float_implicit array failed to have length of 1')

single_float_explicit 	: Array = [1.0]
assert(len(single_float_explicit) == 1, 'single_float_explicit array failed to have length of 1')

plural_float_implicit  	:= [1.0, 2.0]
assert(len(plural_float_implicit) == 2, 'plural_float_implicit array failed to have length of 2')

plural_float_explicit 	: Array = [1.0, 2.0]
assert(len(plural_float_explicit) == 2, 'plural_float_explicit array failed to have length of 2')

assignment 				:= single_float_implicit
assert(len(assignment) == 1, 'assignment array failed to have length of 1')

single_element_access 	:= single_float_implicit[0]
assert(single_element_access == 1.0, 'single_element_access failed to equal expected value')

first_element_access 	:= plural_float_implicit[0]
assert(first_element_access == 1.0, 'first_element_access failed to equal expected value')

second_element_access 	:= plural_float_implicit[1]
assert(second_element_access == 2.0, 'second_element_access failed to equal expected value')

accessor_assignment		:= [1.0, 2.0]
assert(len(accessor_assignment) == 2, 'accessor_assignment array failed to have length of 2')

accessor_assignment[0] = 3.0
assert(accessor_assignment[0] == 3.0, 'accessor_assignment[0] failed to equal expected value')
x := 100.0

assert(100.0 == x, 'x failed to equal expected value')

vvz := -10

assert(-10 == vvz, 'x failed to equal expected value')

not_true := !true

assert(false == not_true, 'not_true failed to equal expected value')

not_false := !false

assert(true == not_false, 'not_false failed to equal expected value')

my_Bool := true

assert(true == my_Bool, 'my_Bool failed to equal expected value')

first_name := 'Cyitlec'

assert('Cyitlec' == first_name, 'first_name failed to equal expected value')

last_name := 'Kivals'

assert('Kivals' == last_name, 'last_name failed to equal expected value')

full_name := first_name + ' ' + last_name

assert('Cyitlec Kivals' == full_name, 'full_name failed to equal expected value')

birthplace := 'The
Most
American
American 
City 
in 
America'

assert('The
Most
American
American 
City 
in 
America' == birthplace, 'birthplace failed to equal expected value, instead got' + tostr(birthplace))

age := 'mid 20s'

height := '6 feet, 3.758168 inches'
var status := 'failed'

implicit_fn_test_params := (a: String) {
    println('recursive func ', a)
    status = 'passed'
}

implicit_fn_test_no_params := {
    status = 'passed'
    println('recursive func paramless')
}

implicit_fn_test_params('my argument')

assert(status == 'passed', 'failed to pass functions.scorch')

implicit_fn_test_no_params()

var status := 'failed'

fail_status := 'failed'
pass_status := 'passed'

if false {
    status = fail_status
} else false {
    status = fail_status
} else {
    status = pass_status
}

assert(status == 'passed', 'failed to pass if_else.scorch')

if true {
    status = pass_status
} else false {
    status = fail_status
} else {
    status = fail_status
}

assert(status == 'passed', 'failed to pass if_else.scorch')

xx := false
xy := true

if xx != xy {
    status = pass_status
} else {
    status = fail_status
}

assert(status == 'passed', 'failed to pass if_else.scorch')

if xx == xy {
    status = fail_status
} else {
    status = pass_status
}

xxy := 5
xyy := 10

assert(status == 'passed', 'failed to pass if_else.scorch')

if xxy < xyy && (xx != xy) {
    status = pass_status
} else {
    status = fail_status
}

assert(status == 'passed', 'failed to pass if_else.scorch')

if xxy > xyy && (xx != xy) {
    status = fail_status
} else {
    status = pass_status
}


assert(status == 'passed', 'failed to pass if_else.scorch')
var result := false
var result1 := false
var result2 := false
var i := 0

repeat i < 10000 {
    result = true
}

assert(result && i == 10000, 'Test: Cached repeat with condition failed, expected 10000 got ' + tostr(i))

repeat z < 250000 {
    if z == 249999 {
        result1 = true
        break
    }
} 

assert(result1, 'Test: Implicitly declared iterator repeat with condition failed.')

repeat {
    break
    println('test: repeat without condition failing.. result is this infinite loop. please exit.')
}

result2 = true
assert(result2, 'Test: Repeat without condition failed.')

rel_t1 := 5 < 10

assert(rel_t1 == true, '5 < 10 returned true')

rel_t2 := 5 > 10

assert(rel_t2 == false, '5 > 10 returned true')

rel_t3 := 5 <= 10

assert(rel_t3 == true, '5 <= 10 returned false')

rel_t4 := 5 >= 10

assert(rel_t4 == false, '5 >= 10 returned true')

rel_t5 := 5 == 10

assert(rel_t5 == false, '5 == 10 returned true')

rel_t6 := 5 != 10

assert(rel_t6 == true, '5 != 10 returned false')

rel_t7 := 5 == 5

assert(rel_t7 == true, '5 == 5 returned false')

rel_t8 := 5 != 5

assert(rel_t8 == false, '5 != 5 returned true')

rel_t9  := 5 <= 5

assert(rel_t9 == true, '5 <= 5 returned false')

rel_t10 := 5 >= 5

assert(rel_t10 == true, '5 >= 5 returned false')

rel_t11 := 5 < 5

assert(rel_t11 == false, '5 < 5 returned true')

rel_t12 := ((5 < 5) && (5 > 5))

assert(rel_t12 == false, '5 < 5 && 5 > 5 returned true')
var status := false

//# simple type
struct Vec2 |
    x := 0.0
    y := 0.0
|

vector2 := new Vec2 { 0.0, 1250.050 }

assert_eq(0.0, vector2.x, 'Vec2.x failed to equal expected value')
assert_eq(1250.050, vector2.y, 'Vec2.y failed to equal expected value')


// complex type
struct Vec3 |
    xy := new Vec2{0.0, 0.0}
    z := 0.0
|

vector3 := new Vec3 {
    new Vec2 {
        0.0,
        1250.050
    },
    100.0
}

//# temp while chained field access isn't working.
xy := vector3.xy

assert_eq(0.0, xy.x, 'Vec3.xy.x failed to equal expected value')
assert_eq(1250.050, xy.y, 'Vec3.xy.y failed to equal expected value')
assert_eq(100.0, vector3.z, 'Vec3.z failed to equal expected value')

status = true
        ";

        let mut tokenizer = create_tokenizer();

        tokenizer.tokenize(input);

        let ast_root = parser::parse_program(&tokenizer.tokens);

        dbg!(&ast_root);
    }
}
