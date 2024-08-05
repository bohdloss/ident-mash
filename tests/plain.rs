use ident_mash::mash;

mash!(name = my + _ + "very_complicated" + _name => mod $name { pub const VALUE: usize = 15; });

#[test]
fn main() {
	assert_eq!(my_very_complicated_name::VALUE, 15);
}