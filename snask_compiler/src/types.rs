#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    List,
    Dict,
    Void,
    Any,
    Function(Vec<Type>, Box<Type>), // param_types, return_type
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float)
    }
}

#[derive(Debug)]
pub struct TypeParseError;

impl std::str::FromStr for Type {
    type Err = TypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(Type::Int),
            "float" => Ok(Type::Float),
            "str" => Ok(Type::String),
            "bool" => Ok(Type::Bool),
            "list" => Ok(Type::List),
            "dict" => Ok(Type::Dict),
            "void" => Ok(Type::Void),
            "any" => Ok(Type::Any),
            _ => Err(TypeParseError),
        }
    }
}