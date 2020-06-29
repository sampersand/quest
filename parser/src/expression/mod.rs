mod expression;
mod constructor;
mod bound_operator;

pub trait Executable {
	fn execute(&self) -> quest_core::Result<quest_core::Object>;
}

pub trait PutBack : Iterator {
	fn put_back(&mut self, item: Self::Item);
}

pub trait Constructable {
	type Item: Into<Expression>;
	fn try_construct_primary<C>(ctor: &mut C) -> crate::Result<Option<Self::Item>>
	where
		C: Iterator<Item=crate::Result<crate::token::Token>> + PutBack + crate::stream::Contexted;
}

pub(crate) use constructor::Constructor;
pub use bound_operator::BoundOperator;
pub use expression::Expression;