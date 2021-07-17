pub type ObjectType = &'static str;

pub const BOOLEAN_OBJ: ObjectType = "BOOLEAN";
pub const INTEGER_OBJ: ObjectType = "INTEGER";
pub const NULL_OBJ: ObjectType = "NULL";


enum Object {
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
