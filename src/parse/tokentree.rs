use std::iter::FromIterator;
use std::cmp::Ordering;
use crate::parse::{Result, Error, Token, token::{ParenType, Operator, Literal}};
use std::convert::TryFrom;

#[derive(Debug)]
pub enum TokenTree {
	Empty,
	Literal(Literal),
	Group(ParenType, Box<TokenTree>),
	Operator(String, Vec<TokenTree>),
	_Group(ParenType, Vec<TokenTree>),
	_Operator(Operator, Vec<TokenTree>),
}

impl TokenTree {
	// I: 
	// pub fn try_from_iter<I: Iterator<Item=Result<Token>>>(mut iter: I) -> Result<Self> {
	pub fn try_from_iter<T: std::io::Read + std::io::Seek>(mut iter: &mut super::Stream<T>) -> Result<Self> {
		// first get the grouped tokens.
		fn splitify(mut tosplit: Vec<TokenTree>) -> Option<Result<TokenTree>> {
			let split_pos = tosplit.iter().enumerate().max_by(|(_, l), (_, r)| {
				use TokenTree::*;
				match (*l, *r) {
					(_Operator(lhs, ..), _Operator(rhs, ..)) => lhs.cmp(&rhs),
					(_Operator(..), ..) => Ordering::Greater, 
					(.., _Operator(..)) => Ordering::Less,
					(_Group(..), _Group(..)) => Ordering::Equal,
					(_Group(..), _) => Ordering::Greater, // groups are bigger than literals
					(_, _Group(..)) => Ordering::Less,
					(Literal(..), Literal(..)) => Ordering::Equal,

					(Group(..), ..) => unimplemented!(),
					(.., Group(..)) => unimplemented!(),
					(Operator(..), ..) => unimplemented!(),
					(.., Operator(..)) => unimplemented!(),
					(Empty, ..) => unimplemented!(),
					(.., Empty) => unimplemented!(),
				}
			})?.0;
			let mut rhs = tosplit.split_off(split_pos);
			let mut lhs = tosplit;
			debug_assert!(!rhs.is_empty());
			Some(match rhs.remove(0) {
				TokenTree::_Operator(oper, _should_be_blank) => {
					// TODO: all this. it's absolute garbage
					debug_assert!(_should_be_blank.is_empty());
					let mut args = Vec::new();
					if oper.arity() != 2 {
						todo!("arities not eql to 2");
					}

					match splitify(lhs) {
						Some(Ok(tt)) => args.push(tt),
						Some(Err(err)) => panic!("bad splitify for lhs: oper={:?}, err={:?}", oper, err),
						None => panic!("bad splitify for lhs: oper={:?}", oper),
					};
					match splitify(rhs) {
						Some(Ok(tt)) => args.push(tt),
						Some(Err(err)) => panic!("bad splitify for rhs (1): oper={:?}, err={:?}", oper, err),
						None if oper == Operator::Endline => {},
						None => panic!("bad splitify for rhs (1): oper={:?}", oper),
					};
					Ok(TokenTree::Operator(oper.to_string(), args))
				},

				TokenTree::_Group(paren, group) => {
					let token_group = match splitify(group) {
						Some(Ok(tt)) => tt,
						Some(Err(err)) => panic!("bad splitify for rhs (1): paren={:?}, err={:?}", paren, err),
						None => TokenTree::Empty
					};

					assert!(rhs.is_empty());
					assert!(rhs.len() <= 1);

					Ok(if lhs.len() == 1 {
						let mut v = vec![];
						match lhs.pop().unwrap() {
							lit @ TokenTree::Literal(..) => v.push(lit),
							TokenTree::_Group(paren, body) => match splitify(body) {
								Some(Ok(tt)) => v.push(tt),
								Some(Err(err)) => return Some(Err(err)),
								None => {},
							},
							_ => unimplemented!()
						};
						
						v.push(token_group);
						TokenTree::Operator(paren.to_string(), v)
					} else { 
						token_group
					})
					// if lhs.len() == 1 {
					// 	Ok(TokenTree::Operator(paren.to_string(), vec![lhs.pop().unwrap(),
					// 		]))
					// } else if lhs.length() != 1 || !rhs.is_empty() {
					// 	panic!("lhs implied no lhs or rhs: paren={:?}, group={:?}, lhs={:?}, rhs={:?}", paren, group, lhs, rhs);
					// 	// Err(Error::Message(format!("tt implied no lhs or rhs: tt={:?}, lhs={:?}, rhs={:?}", tt, lhs, rhs)))
					// } else {

					// 	Ok(TokenTree::Group(paren, match splitify(group) {
					// 		Some(Ok(tt)) => Box::new(tt),
					// 		Some(Err(err)) => panic!("bad splitify for rhs (1): paren={:?}, err={:?}", paren, err),
					// 		None => panic!("bad splitify for rhs (1): paren={:?}", paren),
					// 	}))
					// }
				}
				tt @ TokenTree::Literal(..) => {
					if !lhs.is_empty() || !rhs.is_empty() {
						panic!("tt implied no lhs or rhs: tt={:?}, lhs={:?}, rhs={:?}", tt, lhs, rhs);
						// Err(Error::Message(format!("tt implied no grp or rhs: tt={:?}, grp={:?}, rhs={:?}", tt, grp, rhs)))
					} else {
						Ok(tt)
					}
				},
				_ => unimplemented!()
			})
		}
		splitify(TokenTree::group_tokens(&mut iter, None)?)
			.unwrap_or_else(|| Err(Error::Message(format!("no tokens given"))))
	}

	fn group_tokens<T>(iter: &mut super::Stream<T>, delim: Option<ParenType>) -> Result<Vec<TokenTree>>
	where T: std::io::Read + std::io::Seek {
		let mut group = Vec::with_capacity(iter.size_hint().0);
		let mut did_find_endline = false;

		while let Some(token) = iter.next().transpose()? {
			match token {
				Token::Literal(lit) => group.push(TokenTree::Literal(lit)),
				Token::Operator(oper) => group.push(TokenTree::_Operator(oper, Vec::new())),
				Token::Right(paren) if Some(paren) == delim => { did_find_endline = true; break },
				Token::Right(paren) => return Err(Error::Message(format!("bad parens: open({:?}) vs close({:?})", delim, paren))),
				Token::Left(paren) => group.push(TokenTree::_Group(paren, TokenTree::group_tokens(iter, Some(paren))?))
			}
		} 

		if delim.is_some() && !did_find_endline {
			Err(Error::Message(format!("missing closing paren for {:?}", delim)))
		} else {
			group.shrink_to_fit();
			Ok(group)
		}
	}
	// fn group_tokens<T>(iter: &mut super::Stream<T>, ending: Option<ParenType>) -> impl Iterator<Item=Result<TokenTree>>
	// where T: std::io::Read + std::io::Seek {
	// 	struct TokenGrouper<'a, T: Iterator<Item=Result<Token>>>(&'a mut T, Option<ParenType>);
	// 	impl<'a, T: Iterator<Item=Result<Token>>> Iterator for TokenGrouper<'a, T> {
	// 		type Item = Result<Token>;
	// 	}
	// }


	// NOTE: this is currently a very basic token-to-tree parsing system. In the future, better error handling should be
	// implemented, along with a better way to check arity, etc. This is based off the shunting-yard algorithm.
	// (e.g. it allows `1 2 +`)...
	// fn try_from_iter_rparen<I: Iterator<Item=Result<Token>>>(mut iter: I, delim: Option<ParenType>) -> Result<Vec<TokenTree>> {
	fn try_from_iter_rparen_shunting<T: std::io::Read + std::io::Seek>(mut iter: &mut super::Stream<T>, delim: Option<ParenType>) -> Result<Vec<TokenTree>> {
		let mut token_out: Vec<TokenTree> = Vec::new(); // TODO: use iter's size_hint to make more efficient
		let mut op_list: Vec<Operator> = Vec::new();
		while let Some(token) = iter.next().transpose()? {
			match token {
				Token::Literal(lit) => token_out.push(TokenTree::Literal(lit)),
				Token::Operator(oper) => {
					while !op_list.is_empty() && op_list[op_list.len() - 1] < oper {
						let curr_oper = op_list.pop().unwrap();
						if token_out.len() < curr_oper.arity() {
							return Err(Error::Message(format!("missing args for {:?}!", curr_oper)));
						}

						let mut args: Vec<TokenTree> = token_out.split_off(token_out.len() - curr_oper.arity());
						// args.reverse();
						token_out.push(TokenTree::_Operator(curr_oper, args));
					}
					op_list.push(oper);
				}
				Token::Right(paren) if Some(paren) == delim => break,
				Token::Right(paren) => return Err(Error::Message(format!("dangling paren: {:?}", paren))),
				Token::Left(paren) => token_out.push(TokenTree::_Group(paren,
					TokenTree::try_from_iter_rparen_shunting(&mut iter, Some(paren))?))
			}
		}

		while let Some(curr_oper) = op_list.pop() {
			if token_out.len() < curr_oper.arity() {
				return Err(Error::Message(format!("missing arg for {:?}!", curr_oper)));
			}
			let mut args: Vec<TokenTree> = token_out.split_off(token_out.len() - curr_oper.arity());
			// args.reverse();
			token_out.push(TokenTree::_Operator(curr_oper, args));
		}

		Ok(token_out)
	}
}