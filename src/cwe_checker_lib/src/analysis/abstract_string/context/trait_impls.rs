use super::*;
use crate::{
    abstract_domain::{AbstractDomain, HasTop},
    analysis::abstract_string::state::State,
    intermediate_representation::*,
};

impl<'a, T: AbstractDomain + HasTop> crate::analysis::forward_interprocedural_fixpoint::Context<'a>
    for Context<'a, T>
{
    type Value = State<T>;

    fn get_graph(&self) -> &Graph<'a> {
        &self.graph
    }

    fn merge(&self, state1: &Self::Value, state2: &Self::Value) -> Self::Value {
        state1.merge(state2)
    }

    fn update_def(&self, state: &Self::Value, def: &Term<Def>) -> Option<Self::Value> {
        todo!()
    }

    fn update_jump(
        &self,
        value: &Self::Value,
        jump: &Term<Jmp>,
        untaken_conditional: Option<&Term<Jmp>>,
        target: &Term<Blk>,
    ) -> Option<Self::Value> {
        todo!()
    }

    fn update_call(
        &self,
        value: &Self::Value,
        call: &Term<Jmp>,
        target: &crate::analysis::graph::Node,
    ) -> Option<Self::Value> {
        todo!()
    }

    fn update_return(
        &self,
        value: Option<&Self::Value>,
        value_before_call: Option<&Self::Value>,
        call_term: &Term<Jmp>,
        return_term: &Term<Jmp>,
    ) -> Option<Self::Value> {
        todo!()
    }

    fn update_call_stub(&self, value: &Self::Value, call: &Term<Jmp>) -> Option<Self::Value> {
        todo!()
    }

    fn specialize_conditional(
        &self,
        value: &Self::Value,
        condition: &Expression,
        block_before_condition: &Term<Blk>,
        is_true: bool,
    ) -> Option<Self::Value> {
        todo!()
    }
}
