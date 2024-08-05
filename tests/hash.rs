use ident_mash::mash;

mash!(name = TEST_ + :hash(MY + _ + STATIC + _NAME) => static $name: usize = 42;);
mash!(name = TEST_ + :hash(MY + _OTHER_ + STATIC + _NAME) => static $name: usize = 52;);

#[test]
fn main() {
	let value1 = mash!(name = TEST_ + :hash(MY_STATIC + _ + NAME) => $name);
	let value2 = mash!(name = TE + ST_ + :hash(MY_OTHER + _STATIC + _ + NAME) => $name);
	
	assert_eq!(value1, 42);
	assert_eq!(value2, 52);
}