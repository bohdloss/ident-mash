use ident_mash::mash;

mash!(
	mod_name = :snake_case("my mod name") &
	const_name = :upper_snake_case("my const name") &
	struct_name = :pascal_case("my struct name") => 
	mod $mod_name { pub const VALUE: usize = 15; }
	pub const $const_name: usize = 129;
	struct $struct_name;
);

#[test]
fn main() {
	assert_eq!(my_mod_name::VALUE, 15);
	assert_eq!(MY_CONST_NAME, 129);
	let _: MyStructName = MyStructName;
}