use ident_mash::mash;

mash!(name = :upper_case(upper + _ + case + _ + test) => static $name: usize = 42;);
mash!(name = test_ + :lower_case(LOWER_CASE) => mod $name { pub static VALUE: usize = 100; });
mash!(name = :pascal_case(my_struct) => struct $name;);
mash!(name = :snake_case(SnakeMyCaseDown) => mod $name { pub static VALUE: usize = 200; } );
mash!(name = :upper_snake_case(SnakeMyCaseUp) => pub static $name: usize = 512; );

#[test]
fn main() {
	assert_eq!(UPPER_CASE_TEST, 42);
	assert_eq!(test_lower_case::VALUE, 100);
	let _: MyStruct = MyStruct;
	assert_eq!(snake_my_case_down::VALUE, 200);
	assert_eq!(SNAKE_MY_CASE_UP, 512)
}