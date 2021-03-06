use crate::ast;
use std::collections::HashMap;
use std::fmt;

pub type ObjectType = &'static str;

pub const BOOLEAN_OBJ: ObjectType = "BOOLEAN";
pub const INTEGER_OBJ: ObjectType = "INTEGER";
pub const NULL_OBJ: ObjectType = "NULL";
pub const RETURN_VALUE_OBJ: ObjectType = "RETURN_VALUE";
pub const ERROR_OBJ: ObjectType = "ERROR";
pub const FUNCTION_OBJ: ObjectType = "FUNCTION";

#[derive(PartialEq, Eq, Clone)]
pub enum Object {
    Integer {
        Value: i64,
    },
    Boolean {
        Value: bool,
    },
    ReturnValue {
        Value: Box<Object>,
    },
    Error {
        Message: String,
    },
    Function {
        Parameters: Vec<ast::Expression>,
        Body: Box<ast::Statement>,
        Env: Environment,
    },
    Null,
}

impl Object {
    pub fn Type(&self) -> ObjectType {
        match self {
            Object::Integer { .. } => INTEGER_OBJ,
            Object::Boolean { .. } => BOOLEAN_OBJ,
            Object::ReturnValue { .. } => RETURN_VALUE_OBJ,
            Object::Error { .. } => ERROR_OBJ,
            Object::Function { .. } => FUNCTION_OBJ,
            Object::Null => NULL_OBJ,
        }
    }

    pub fn Inspect(&self) -> String {
        match self {
            Object::Integer { Value } => format!("{}", Value),
            Object::Boolean { Value } => format!("{}", Value),
            Object::ReturnValue { Value } => Value.Inspect(),
            Object::Error { Message } => format! {"ERROR: {}", Message},
            Object::Function {
                Parameters,
                Body,
                Env,
            } => {
                let mut params = vec![];
                for p in Parameters.iter() {
                    params.push(p.into_string());
                }
                format! {"fn({}) {{
                    {}
                }}", params.join(" "), Body.into_string()}
            }
            Object::Null => String::from("null"),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer { Value } => write!(f, "Object::Integer{{Value: {}}}", Value),
            Object::Boolean { Value } => write! {
                f,
                "Object::Boolean{{Value: {}}}",
                Value
            },
            Object::ReturnValue { Value } => write! {
                f,
                "Object::ReturnValue{{Value: {}}}",
                Value
            },
            Object::Error { Message } => write! {
                f,
                "Object::Error{{Message: {}}}",
                Message
            },
            Object::Function {
                Parameters,
                Body,
                Env,
            } => write! {
                f,
                "Object::Function"
            },
            Object::Null => write! {
                f,
                "Object::Null"
            },
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

pub fn NewEnvironment() -> Environment {
    let s = HashMap::new();
    Environment {
        store: s,
        outer: None,
    }
}

pub fn NewEnclosedEnvironment(outer: Environment) -> Environment {
    Environment {
        store: HashMap::new(),
        outer: Some(Box::new(outer)),
    }
}

impl Environment {
    pub fn Get(&self, name: &String) -> Option<&Object> {
        let obj = self.store.get(name);
        match obj {
            Some(x) => Some(x),
            None => match &self.outer {
                Some(x) => x.Get(name),
                None => None,
            },
        }
    }

    pub fn Set(&mut self, name: &String, val: Object) -> Object {
        self.store.insert(name.clone(), val.clone());
        val
    }
}
