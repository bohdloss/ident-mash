use ident_mash::mash;

mash!(name = :snake_case("my mod name") => mod $name { pub const VALUE: usize = 15; });
mash!(name = :upper_snake_case("my const name") => pub const $name: usize = 129;);
mash!(name = :pascal_case("my struct name") => struct $name;);

#[test]
fn main() {
	assert_eq!(my_mod_name::VALUE, 15);
	assert_eq!(MY_CONST_NAME, 129);
	let _: MyStructName = MyStructName;
}