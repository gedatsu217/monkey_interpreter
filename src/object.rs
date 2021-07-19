use std::fmt;

pub type ObjectType = &'static str;

pub const BOOLEAN_OBJ: ObjectType = "BOOLEAN";
pub const INTEGER_OBJ: ObjectType = "INTEGER";
pub const NULL_OBJ: ObjectType = "NULL";


pub enum Object {
    Integer{Value: i64},
    Boolean{Value: bool},
    Null,
}

impl Object {
    fn Type(&self) -> ObjectType {
        match self {
            Object::Integer{..} => INTEGER_OBJ,
            Object::Boolean{..} => BOOLEAN_OBJ,
            Object::Null => NULL_OBJ,
        }
    }

    fn Inspect(&self) -> String {
        match self {
            Object::Integer{Value} => format!("{}", Value),
            Object::Boolean{Value} => format!("{}", Value),
            Object::Null => String::from("null"),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer { Value } => write!(
                f,
                "Object::Integer{{Value: {}}}",
                Value
            ),
            Object::Boolean {Value} => write! {
                f,
                "Object::Boolean{{Value: {}}}",
                Value
            },
            Object::Null => write! {
                f,
                "Object::Null"
            },
        }
    }
}
