use crate::ast::tuple::{Clonable, Tuple, TupleError, TupleLike};
use crate::ast::{ASTNode};
use crate::ast::value::Value;
use crate::environment::environment::{Environment, RuntimeError};
use crate::ast::type_node::Type;
use crate::Error;

impl Clonable for String {
    fn clone_element(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug)]
pub enum AssignmentError {
    CannotAssignToEmptyName,
    CannotAssignToTupleName,
    CannotDeclareVariableWithEmptyName,
    CannotDeclareVariableWithTupleName,
    ValuesAndNamesDontMatch(Tuple<String>, Value),
}


#[derive(Debug)]
pub struct VariableAssignment {
    pub name: Tuple<String>,
    pub expr: Box<dyn ASTNode>,
}

#[derive(Debug)]
pub struct VariableDeclarationAssignment {
    pub mutable: bool,
    pub type_: Option<Type>,
    pub name: Tuple<String>,
    pub expr: Box<dyn ASTNode>,
}

impl ASTNode for VariableAssignment {
    fn element(&self) -> String {
        format!("{} :=", self.name)
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![self.expr.clone_to_node()]
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        let new_assignment = VariableAssignment {
            name: self.name.clone(),
            expr: self.expr.clone_to_node(),
        };
        Box::new(new_assignment)
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let value = self.expr.eval(env)?;
        match (&self.name, &value) {
            (Tuple::Empty, _) => {
                return Err(Error::RuntimeError(RuntimeError::AssignmentError(AssignmentError::CannotAssignToEmptyName)));
            }
            (Tuple::Element(name), _) => {
                env.assign(name, value.clone())?;
            }
            (Tuple::List(_), Value::Tuple(_)) => {
                let name_value_pairs = match self.name.pair_up(value.to_tuple()) {
                    Ok(pairs) => pairs,
                    Err(e) => {
                        return Err(Error::TupleError(e));
                    }
                };
                for (name, value) in name_value_pairs {
                    env.assign(&name, value.clone())?;
                }
            }
            (Tuple::List(_), _) => {
                return Err(Error::RuntimeError(RuntimeError::AssignmentError(AssignmentError::CannotAssignToTupleName)));
            }
        }
        Ok(Value::Bool(true))
    }

}

impl ASTNode for VariableDeclarationAssignment {
    fn element(&self) -> String {
        let type_ = match &self.type_ {
            Some(type_) => format!("{:?}", type_),
            _ => "[inferred]".to_string(),
        };
        format!("{} {} : {} =", if self.mutable { "mut" } else { "let" }, self.name, type_)
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![self.expr.clone_to_node()]
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        let new_assignment = VariableDeclarationAssignment {
            mutable: self.mutable,
            type_: self.type_.clone(),
            name: self.name.clone(),
            expr: self.expr.clone_to_node(),
        };
        Box::new(new_assignment)
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let value = self.expr.eval(env)?;
        match (&self.name, &value) {
            (Tuple::Empty, _) => {
                return Err(Error::RuntimeError(RuntimeError::AssignmentError(AssignmentError::CannotDeclareVariableWithEmptyName)));
            }
            (Tuple::Element(name), _) => {
                env.declare_assign(name.clone(), value.clone(), self.mutable, self.type_.clone())?;
            }
            (Tuple::List(_), Value::Tuple(_)) => {
                let name_value_pairs = match self.name.pair_up(value.to_tuple()) {
                    Ok(pairs) => pairs,
                    Err(e) => {
                        return Err(Error::TupleError(e));
                    }
                };
                let types = match &self.type_ {
                    Some(t) => {
                        let t = match self.name.apply_structure(t.to_tuple()) {
                            Ok(t) => t,
                            Err(e) => {
                                return Err(Error::TupleError(e));
                            }
                        };

                        t.map(&|e| Some(e.clone_element()))
                    }
                    None => match self.name.apply_structure(Tuple::Element(None)) {
                        Ok(t) => t,
                        Err(e) => {
                            return Err(Error::TupleError(e));
                        }
                    }
                };

                let name_type_pairs = match self.name.pair_up(types) {
                    Ok(pairs) => pairs,
                    Err(e) => {
                        return Err(Error::TupleError(e));
                    }
                };

                for ((name, value), (_, type_)) in name_value_pairs.iter().zip(name_type_pairs) {
                    env.declare_assign(name.clone(), value.clone(), self.mutable, type_)?;
                }
            }
            (Tuple::List(_), _) => {
                return Err(Error::TupleError(TupleError::CannotPairUp));
            }

        }
        Ok(Value::Bool(true))
    }
}