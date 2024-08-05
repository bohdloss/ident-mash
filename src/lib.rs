use proc_macro::TokenStream as TokenStream1;
use std::fmt::{Display, Formatter};
use std::hash::{DefaultHasher, Hash, Hasher};

use convert_case::{Case, Casing};
use proc_macro2::{Group, Ident, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote};
use syn::{Error, LitStr, parenthesized, parse_macro_input, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Paren;

fn ident_replace(tokens: TokenStream2, search_for: Ident, replace_with: Ident) -> TokenStream2 {
	let mut tokens: Vec<TokenTree> = tokens.into_iter().collect();
	let mut indices = Vec::new();
	let mut substitute = false;
	for (i, tt) in tokens.iter_mut().enumerate() {
		if let TokenTree::Group(group) = tt {
			let new_stream = ident_replace(group.stream(), search_for.clone(), replace_with.clone());
			*group = Group::new(group.delimiter(), new_stream);
			continue;
		};
		if substitute {
			substitute = false;

			match tt {
				TokenTree::Ident(ident) if *ident == search_for => {
					*ident = replace_with.clone();
					indices.push(i - 1);
				}
				_ => {}
			}
		} else {
			match tt {
				TokenTree::Punct(p) if p.as_char() == '$' => { substitute = true; }
				_ => {}
			}
		}
	}

	for (sub, &idx) in indices.iter().enumerate() {
		tokens.remove(idx - sub);
	}

	tokens.into_iter().collect()
}

fn ident_no_prefix(ident: &Ident) -> String {
	let string = ident.to_string();
	if string.starts_with("r#") {
		string[2..].to_owned()
	} else {
		string
	}
}

mod kw {
	use syn::custom_keyword;

	custom_keyword!(hash);
	custom_keyword!(lower_case);
	custom_keyword!(upper_case);
	custom_keyword!(snake_case);
	custom_keyword!(pascal_case);
	custom_keyword!(upper_snake_case);
}

#[allow(dead_code)]
enum Operation {
	Plain(Plain),
	Hash {
		colon: Token![:],
		hash: kw::hash,
		paren: Paren,
		ops: Punctuated<Operation, Token![+]>,
	},
	Lower {
		colon: Token![:],
		hash: kw::lower_case,
		paren: Paren,
		ops: Punctuated<Operation, Token![+]>,
	},
	Upper {
		colon: Token![:],
		hash: kw::upper_case,
		paren: Paren,
		ops: Punctuated<Operation, Token![+]>,
	},
	Snake {
		colon: Token![:],
		hash: kw::snake_case,
		paren: Paren,
		ops: Punctuated<Operation, Token![+]>,
	},
	Pascal {
		colon: Token![:],
		hash: kw::pascal_case,
		paren: Paren,
		ops: Punctuated<Operation, Token![+]>,
	},
	ScreamingSnake {
		colon: Token![:],
		hash: kw::upper_snake_case,
		paren: Paren,
		ops: Punctuated<Operation, Token![+]>,
	},
}

impl Display for Operation {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Operation::Plain(ident) => ident.fmt(f),
			Operation::Hash { ops, .. } => {
				let mut string = String::new();
				for op in ops.iter() {
					string += &op.to_string();
				}
				let mut hasher = DefaultHasher::new();
				string.hash(&mut hasher);
				write!(f, "{}", hasher.finish())
			}
			Operation::Lower { ops, .. } => {
				for op in ops.iter() {
					write!(f, "{}", op.to_string().to_lowercase())?;
				}
				Ok(())
			}
			Operation::Upper { ops, .. } => {
				for op in ops.iter() {
					write!(f, "{}", op.to_string().to_uppercase())?;
				}
				Ok(())
			}
			Operation::Snake { ops, .. } => {
				for op in ops.iter() {
					write!(f, "{}", op.to_string().to_case(Case::Snake))?;
				}
				Ok(())
			}
			Operation::ScreamingSnake { ops, .. } => {
				for op in ops.iter() {
					write!(f, "{}", op.to_string().to_case(Case::ScreamingSnake))?;
				}
				Ok(())
			}
			Operation::Pascal { ops, .. } => {
				for op in ops.iter() {
					write!(f, "{}", op.to_string().to_case(Case::Pascal))?;
				}
				Ok(())
			}
		}
	}
}

impl Parse for Operation {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(if let Ok(colon) = input.parse::<Token![:]>() {
			if input.peek(kw::hash) {
				let inside;
				Operation::Hash {
					colon,
					hash: input.parse()?,
					paren: parenthesized!(inside in input),
					ops: Punctuated::parse_separated_nonempty(&inside)?,
				}
			} else if input.peek(kw::lower_case) {
				let inside;
				Operation::Lower {
					colon,
					hash: input.parse()?,
					paren: parenthesized!(inside in input),
					ops: Punctuated::parse_separated_nonempty(&inside)?,
				}
			} else if input.peek(kw::upper_case) {
				let inside;
				Operation::Upper {
					colon,
					hash: input.parse()?,
					paren: parenthesized!(inside in input),
					ops: Punctuated::parse_separated_nonempty(&inside)?,
				}
			} else if input.peek(kw::snake_case) {
				let inside;
				Operation::Snake {
					colon,
					hash: input.parse()?,
					paren: parenthesized!(inside in input),
					ops: Punctuated::parse_separated_nonempty(&inside)?,
				}
			} else if input.peek(kw::pascal_case) {
				let inside;
				Operation::Pascal {
					colon,
					hash: input.parse()?,
					paren: parenthesized!(inside in input),
					ops: Punctuated::parse_separated_nonempty(&inside)?,
				}
			} else if input.peek(kw::upper_snake_case) {
				let inside;
				Operation::ScreamingSnake {
					colon,
					hash: input.parse()?,
					paren: parenthesized!(inside in input),
					ops: Punctuated::parse_separated_nonempty(&inside)?,
				}
			} else {
				return Err(Error::new(colon.span, "Unknown operation"))
			}
		} else {
			Operation::Plain(input.parse()?)
		})
	}
}

#[allow(dead_code)]
enum Plain {
	Ident(Ident),
	Underscore(Token![_]),
	LitStr(LitStr),
}

impl Parse for Plain {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(if let Ok(underscore) = input.parse::<Token![_]>() {
			Plain::Underscore(underscore)
		} else if let Ok(lit_str) = input.parse::<LitStr>() {
			Plain::LitStr(lit_str)
		} else {
			Plain::Ident(input.parse()?)
		}
		)
	}
}

impl Display for Plain {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Plain::Ident(ident) => write!(f, "{}", ident_no_prefix(ident)),
			Plain::Underscore(_) => write!(f, "_"),
			Plain::LitStr(lit) => write!(f, "{}", lit.value()),
		}
	}
}

#[allow(dead_code)]
struct MetaVar {
	name: Ident,
	eq: Token![=],
	ops: Punctuated<Operation, Token![+]>,
}

impl Parse for MetaVar {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self {
			name: input.parse()?,
			eq: input.parse()?,
			ops: Punctuated::parse_separated_nonempty(input)?,
		})
	}
}

#[allow(dead_code)]
struct Arguments {
	vars: Punctuated<MetaVar, Token![&]>,
	rarrow: Token![=>],
	tt: TokenStream2
}

impl Parse for Arguments {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self {
			vars: Punctuated::parse_separated_nonempty(input)?,
			rarrow: input.parse()?,
			tt: input.parse()?,
		})
	}
}

#[proc_macro]
pub fn mash(input: TokenStream1) -> TokenStream1 {
	let input = parse_macro_input!(input as Arguments);
	let mut result = input.tt;
	
	let mut found = Vec::new();
	for var in input.vars.iter() {
		if found.contains(&ident_no_prefix(&var.name)) {
			return Error::new(var.name.span(), "meta-variable name defined twice")
				.into_compile_error()
				.into()
		}
		found.push(ident_no_prefix(&var.name));
	}
	
	// For every meta-variable
	for var in input.vars {
		let mut processed = String::new();
		
		// For every + concatenated operation
		for op in var.ops {
			processed += &op.to_string();
		}
		let processed = format_ident!("r#{processed}");
		result = ident_replace(result, var.name, processed);
	}

	quote!(
		#result
	).into()
}