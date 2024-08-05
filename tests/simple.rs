use ident_mash::mash;

mash!(name = MY + _ + STATIC + _NAME => static $name: usize = 42;);

#[test]
fn main() {
	assert_eq!(MY_STATIC_NAME, 42)
}