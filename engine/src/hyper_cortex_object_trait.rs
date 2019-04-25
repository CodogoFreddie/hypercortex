use crate::primitives::{Id, Mutation, Query};

pub trait HyperCortexObject {
    fn apply_mutation(&mut self, mutation: &Mutation) -> &mut Self;
    fn get_id(&mut self) -> &Id;
    fn satistifes_query(&self, query: &Query) -> bool;
}
