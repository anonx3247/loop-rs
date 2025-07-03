use crate::ast::{ASTNode, EmptyASTNode, Value};
use crate::environment::environment::Environment;
use crate::Error;


#[derive(Debug, Clone)]
pub enum Tuple<T: Clonable> {
    Empty,
    Element(T),
    List(Vec<Tuple<T>>)
}

impl<T: std::fmt::Debug + Clonable> std::fmt::Display for Tuple<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tuple::Empty => write!(f, "()"),
            Tuple::Element(element) => write!(f, "{:?}", element),
            Tuple::List(elements) => write!(f, "({})", elements.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", ")),
        }
    }
}


impl<T: Clonable> Tuple<T> {
    pub fn single_like<K: Clonable>(element: &T, tuple: &Tuple<K>) -> Self {
        // makes a tuple with the same structure as the other tuple, but with all elements replaced with the given element
        match &tuple {
            Tuple::Empty => Tuple::Empty,
            Tuple::Element(_) => Tuple::Element(element.clone_element()),
            Tuple::List(elements) => {
                let mut result: Vec<Tuple<T>> = vec![];
                for i in 0..elements.len() {
                    match &elements[i] {
                        Tuple::Element(_) => result.push(Tuple::Element(element.clone_element())),
                        _ => result.push(Tuple::single_like(element, &elements[i])),
                    }
                }
                Tuple::List(result)
            },
        }
    }

    pub fn print_structure(&self) -> String {
        match self {
            Tuple::Empty => "()".to_string(),
            Tuple::Element(_) => "x".to_string(),
            Tuple::List(elements) => format!("({})", elements.iter().map(|e| e.print_structure()).collect::<Vec<String>>().join(", ")),
        }
    }

    pub fn map<K, F>(&self, f: &F) -> Tuple<K>
    where F: Fn(&T) -> K,
    K: Clonable,
    {
        match self {
            Tuple::Empty => Tuple::Empty,
            Tuple::Element(element) => Tuple::Element(f(&element)),
            Tuple::List(elements) => Tuple::List(elements.iter().map(|e| e.map(f)).collect()),
        }
    }
}


#[derive(Debug)]
pub enum TupleError {
    CannotPairUp,
}

pub trait Clonable {
    fn clone_element(&self) -> Self;
}


impl <T: Clonable> Clonable for Option<T> {
    fn clone_element(&self) -> Self {
        match self {
            Some(element) => Some(element.clone_element()),
            None => None,
        }
    }
}

impl<T: Clonable> Clonable for Tuple<T> {
    fn clone_element(&self) -> Tuple<T> {
        match self {
            Tuple::Empty => Tuple::Empty,
            Tuple::Element(element) => Tuple::Element(element.clone_element()),
            Tuple::List(elements) => Tuple::List(elements.iter().map(|e| e.clone_element()).collect()),
        }
    }
}

impl<T: Clonable> Tuple<T> {
    pub fn matches_structure<K: Clonable>(&self, other: &Tuple<K>) -> bool {
        match self {
            Tuple::Empty => matches!(other, Tuple::Empty),
            Tuple::Element(_) => matches!(other, Tuple::Element(_)),
            Tuple::List(elements) => match other {
                Tuple::Empty => false,
                Tuple::Element(_) => false,
                Tuple::List(other_elements) => {
                    if elements.len() != other_elements.len() {
                        return false;
                    }
                    for (i, element) in elements.iter().enumerate() {
                        if !element.matches_structure(&other_elements[i]) {
                            return false;
                        }
                    }
                    true
                }
            }
        }
    }

    pub fn matches_left_structure<K: Clonable>(&self, other: &Tuple<K>) -> bool {
        match self {
            Tuple::Empty => matches!(other, Tuple::Empty),
            Tuple::Element(_) => match other {
                Tuple::Empty => false,
                Tuple::Element(_) => true,
                Tuple::List(_) => true, // e.g. a := (1, 2, 3)
            },
            Tuple::List(elements) => match other {
                Tuple::Empty => false,
                Tuple::Element(_) => false,
                Tuple::List(other_elements) => {
                    if elements.len() != other_elements.len() {
                        return false;
                    }
                    for (i, element) in elements.iter().enumerate() {
                        if !element.matches_left_structure(&other_elements[i]) {
                            return false;
                        }
                    }
                    true
                }
            }
        }
    }

    pub fn loosely_matches_structure<K: Clonable>(&self, other: &Tuple<K>) -> bool {
        match (self, other) {
            (Tuple::Empty, Tuple::Empty) => true,
            (Tuple::Element(_), Tuple::Element(_)) => true,
            (Tuple::List(elements), Tuple::List(other_elements)) => {
                if elements.len() != other_elements.len() {
                    return false;
                }
                for (i, element) in elements.iter().enumerate() {
                    if !element.loosely_matches_structure(&other_elements[i]) {
                        return false;
                    }
                }
                true
            },
            (Tuple::Empty, _) | (_, Tuple::Empty) => false,
            (Tuple::Element(_), Tuple::List(_)) | (Tuple::List(_), Tuple::Element(_)) => true
        }
    }

    pub fn pair_up_left<K: Clonable>(&self, other: Tuple<K>) -> Result<Vec<(T, Tuple<K>)>, TupleError> {
        if !self.matches_left_structure(&other) {
            return Err(TupleError::CannotPairUp);
        }
        match self {
            Tuple::Empty => match other {
                Tuple::Empty => Ok(vec![]),
                _ => Err(TupleError::CannotPairUp),
            },
            Tuple::Element(element) => match other {
                Tuple::Empty => Err(TupleError::CannotPairUp),
                _ => Ok(vec![(element.clone_element(), other)])
            },
            Tuple::List(elements) => match other {
                Tuple::List(other_elements) => {
                    if other_elements.len() != elements.len() {
                        return Err(TupleError::CannotPairUp);
                    }
                    let mut result = Vec::new();
                    for (i, _) in other_elements.iter().enumerate() {
                        result.extend(elements[i].pair_up_left(other_elements[i].clone_element())?);
                    }
                    Ok(result)
                },
                _ => Err(TupleError::CannotPairUp),
            }
        }
    }

    pub fn pair_up<K: Clonable>(&self, other: Tuple<K>) -> Result<Vec<(T, K)>, TupleError> {
        if !self.matches_structure(&other) {
            return Err(TupleError::CannotPairUp);
        }
        match self {
            Tuple::Empty => match other {
                Tuple::Empty => Ok(vec![]),
                _ => Err(TupleError::CannotPairUp),
            },
            Tuple::Element(element) => match other {
                Tuple::Empty => Err(TupleError::CannotPairUp),
                Tuple::Element(other_element) => Ok(vec![(element.clone_element(), other_element.clone_element())]),
                Tuple::List(_) => Err(TupleError::CannotPairUp),
            },
            Tuple::List(elements) => match other {
                Tuple::Empty => Err(TupleError::CannotPairUp),
                Tuple::Element(_) => Err(TupleError::CannotPairUp),
                Tuple::List(other_elements) => {
                let mut result = Vec::new();
                for (i, element) in elements.iter().enumerate() {
                    result.extend(element.pair_up(other_elements[i].clone_element())?);
                }
                Ok(result)
                }
            }
        }
    }

    pub fn apply_structure<K: Clonable>(&self, other: Tuple<K>) -> Result<Tuple<K>, TupleError> {
        match (self, &other) {
            (Tuple::Empty, _) => Ok(Tuple::Empty),
            (Tuple::Element(_), Tuple::Element(_)) => Ok(other.clone_element()),
            (Tuple::List(elements), Tuple::List(other_elements)) => {
                if elements.len() != other_elements.len() {
                    return Err(TupleError::CannotPairUp);
                }
                let mut result = Vec::new();
                for (i, element) in elements.iter().enumerate() {
                    result.push(element.apply_structure(other_elements[i].clone_element())?);
                }
                Ok(Tuple::List(result))
            },
            (Tuple::List(elements), Tuple::Element(_)) => {
                let mut result = Vec::new();
                for element in elements {
                    result.push(element.apply_structure(other.clone_element())?);
                }
                Ok(Tuple::List(result))
            },
            (Tuple::Element(_), Tuple::List(_)) => {
                Ok(other.clone_element())
            },
            (_, _) => Err(TupleError::CannotPairUp)
        }
    }
}


#[derive(Debug)]
pub struct TupleASTNode {
    pub children: Vec<Box<dyn ASTNode>>,
}

impl TupleASTNode {
    pub fn new(children: Vec<Box<dyn ASTNode>>) -> Self {
        Self { children }
    }

    pub fn from_tuple(tuple: Tuple<Box<dyn ASTNode>>) -> Box<dyn ASTNode> {
        match tuple {
            Tuple::Empty => Box::new(EmptyASTNode::new()),
            Tuple::Element(element) => element.clone_to_node(),
            Tuple::List(elements) => {
                let mut children = Vec::new();
                for element in elements {
                    children.push(TupleASTNode::from_tuple(element));
                }
                Box::new(TupleASTNode::new(children))
            }
        }
    }
}

impl ASTNode for TupleASTNode {
    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.children.iter().map(|c| c.as_ref().clone_to_node()).collect()
    }

    fn element(&self) -> String {
        "Tuple".to_string()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        let children = self.children.iter().map(|c| c.as_ref().clone_to_node()).collect();
        Box::new(TupleASTNode { children })
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let mut values = Vec::new();
        for child in self.children.iter() {
            values.push(child.eval(env)?);
        }
        Ok(Value::Tuple(values))
    }
}


pub trait TupleLike<T: Clonable> {
    fn to_tuple(&self) -> Tuple<T>;
}