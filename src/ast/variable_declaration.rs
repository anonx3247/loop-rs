use crate::ast::{ASTNode};
use crate::ast::value::Value;
use crate::environment::environment::Environment;
use crate::ast::type_node::Type;
use crate::ast::tuple::{Tuple, TupleLike};
use crate::Error;

#[derive(Debug)]
pub struct VariableDeclaration {
    pub mutable: bool,
    pub type_: Type,
    pub name: Tuple<String>,
}

impl ASTNode for VariableDeclaration {
    fn element(&self) -> String {
        let mutable = if self.mutable { "mut" } else { "const" };
        format!("{} {} : {:?}", mutable, self.name, self.type_)
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![]
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(VariableDeclaration {
            mutable: self.mutable,
            type_: self.type_.clone(),
            name: self.name.clone(),
        })
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let types = match self.name.apply_structure(self.type_.to_tuple()) {
            Ok(t) => t,
            Err(e) => {
                return Err(Error::TupleError(e));
            }
        };

        let name_type_pairs = match self.name.pair_up_left(types.clone()) {
            Ok(pairs) => pairs,
            Err(e) => {
                return Err(Error::TupleError(e));
            }
        };

        for (name, type_) in name_type_pairs {
            let type_ = match Type::from_tuple(type_) {
                Ok(t) => t,
                Err(e) => {
                    return Err(Error::TypeError(e));
                }
            };
            env.declare(name, self.mutable, type_)?;
        }
        Ok(Value::Bool(true))
    }

}