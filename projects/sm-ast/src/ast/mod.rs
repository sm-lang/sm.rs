use crate::{Identifier, Symbol};
use sm_types::BigInt;

/// The core expression language for symbolic manipulation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SmTree {
    /// `null`, a null constant.
    Null,
    /// `true`, a boolean constant.
    Bool(bool),
    /// `0`, A constant BigInt value.
    Integer(BigInt),
    /// `x`, a symbol, potentially with a namespace.
    Symbol(Symbol),
    /// `x = 1`
    DefineConstant(Box<DefineConstant>),
    /// `f(x) = 1`
    DefineFunction(Box<DefineFunction>),
    /// `+x`
    UnaryExpression(Box<UnaryExpression>),
    /// `x + y`
    BinaryExpression(Box<BinaryExpression>),
}

/// ```sm
/// x = 1
/// let x = 1
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefineConstant {
    pub lhs: Identifier,
    pub rhs: Vec<SmTree>,
}

/// ```sm
/// f(x)(y)(z) = x + y + z if x > 0
/// fn f(x, y)(z) if x > 0
/// {
///     x + y + z
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefineFunction {
    pub name: Identifier,
    pub patterns: Vec<FunctionArguments>,
    pub conditions: Vec<SmTree>,
}

/// (x, y, z)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionArguments {
    args: Vec<FunctionArgument>,
}

/// x: Type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionArgument {
    id: Identifier,
    ty: Option<SmTree>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnaryExpression {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinaryExpression {}
