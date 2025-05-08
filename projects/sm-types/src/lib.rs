use num_bigint::BigInt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use egg::{
    Id,
    Language,
    Symbol,
};

mod errors;
pub use crate::errors::{Result, SmError, SmErrorKind};

/// Represents a namespace, which is a sequence of identifiers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Namespace(Vec<String>);

impl Namespace {
    pub fn new(parts: Vec<String>) -> Self {
        Namespace(parts)
    }

    pub fn root() -> Self {
        Namespace(Vec::new())
    }

    pub fn join(&self, part: String) -> Self {
        let mut new_parts = self.0.clone();
        new_parts.push(part);
        Namespace(new_parts)
    }
}

impl std::fmt::Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join("::"))
    }
}

/// Represents a symbol with a namespace and a name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamespacedSymbol {
    pub namespace: Namespace,
    pub name: String,
}

impl NamespacedSymbol {
    pub fn new(namespace: Namespace, name: String) -> Self {
        NamespacedSymbol { namespace, name }
    }
}

impl std::fmt::Display for NamespacedSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.namespace.0.is_empty() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}::{}", self.namespace, self.name)
        }
    }
}

/// The core expression language for symbolic manipulation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr {
    /// A constant BigInt value.
    Constant(BigInt),
    /// A boolean constant.
    Bool(bool),
    /// A null constant.
    Null,
    /// A symbol, potentially with a namespace.
    Symbol(NamespacedSymbol),
    /// An application of an operator/function to arguments.
    /// The first child is the operator/function, the rest are arguments.
    Apply(Box<Expr>, Vec<Expr>),
    /// A function definition: name, parameters, body.
    /// For `f(x) = body` or `def f(x) { body }`
    FuncDef {
        name: NamespacedSymbol,
        params: Vec<NamespacedSymbol>, // For curried functions, this might be the first set of params
        body: Box<Expr>,
    },
    /// Represents a curried function application or definition part.
    /// `f(x)(y)` could be `Curried(f, [x])(y)` or `FuncDef { name: f, params: [x], body: FuncDef { name: <anon>, params: [y], body: ...}}`
    /// For simplicity in egg, we might represent `f(x)(y)` as `Apply(Apply(f, [x]), [y])`
    /// Or, a specific node for lambda-like abstractions if needed for rewriting rules.
    /// Let's start with Apply for currying and see if a dedicated node is needed.

    // Placeholder for custom operators if Apply isn't sufficient
    // CustomOp(String, Vec<Expr>),
}

// Implement egg::Language for Expr
// This requires Display, FromStr, and Iterators for children

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Constant(c) => write!(f, "{}", c),
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Null => write!(f, "null"),
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Apply(func, args) => {
                write!(f, "({}", func)?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                write!(f, ")")
            }
            Expr::FuncDef { name, params, body } => {
                write!(f, "def {}({})
 = {}", 
                    name, 
                    params.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "), 
                    body
                )
            }
        }
    }
}

// For egg, we need to define how to parse expressions from strings (for rewrite rules)
// and how to iterate over children.
// This is a simplified version; a real parser would be more complex.
impl std::str::FromStr for Expr {
    type Err = SmError;

    fn from_str(s: &str) -> Result<Self> {
        // This is a placeholder. A proper parser is needed for robust FromStr.
        // For now, we'll only support simple symbols and numbers for rewrite rules.
        if let Ok(i) = s.parse::<BigInt>() {
            Ok(Expr::Constant(i))
        } else if s == "true" {
            Ok(Expr::Bool(true))
        } else if s == "false" {
            Ok(Expr::Bool(false))
        } else if s == "null" {
            Ok(Expr::Null)
        } else {
            // Assume it's a symbol (without namespace for simplicity in FromStr)
            Ok(Expr::Symbol(NamespacedSymbol::new(Namespace::root(), s.to_string())))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolicLang {
    pub op: String, // Stores the operator name or symbol name
    // If it's a constant, op can be its string representation
}

impl Language for SymbolicLang {
    fn matches(&self, other: &Self) -> bool {
        self.op == other.op
    }

    fn children(&self) -> &[Id] {
        // For now, our SymbolicLang nodes are leaves or have children managed by egg's RecExpr
        &[]
    }

    fn children_mut(&mut self) -> &mut [Id] {
        // Similarly, children are managed by egg's RecExpr
        &mut []
    }

    fn display_op(&self) -> &str {
        &self.op
    }

    fn from_op_str(op_str: &str, children: Vec<Id>) -> std::result::Result<Self, String> {
        // This is where we'd parse the operator string and associate children.
        // For now, we assume op_str is just the operator name.
        // The actual Expr structure will handle constants and symbols more richly.
        // The `egg` library uses this to construct nodes in the e-graph.
        // We need a way to map our `Expr` enum to `SymbolicLang` and back for `egg`.
        // This part needs careful design to integrate `Expr` with `egg`'s `Language` trait.

        // A simple approach: `SymbolicLang` represents the *kind* of operation or value.
        // The actual value (like a BigInt) would be stored in an analysis associated with the e-class.
        Ok(SymbolicLang { op: op_str.to_string() })
    }
}

/// Analysis data associated with e-classes.
#[derive(Debug)]
pub struct ConstantFolding {
    pub constant: Option<Expr>, // Stores the folded constant if applicable
}

// TODO: Implement egg::Analysis for ConstantFolding
// TODO: Implement egg::Applier for SymbolicLang

/// Represents the environment for evaluation, including scopes and namespaces.
#[derive(Debug, Clone, Default)]
pub struct Environment {
    // For simplicity, a single scope. Real implementation would have a stack of scopes.
    bindings: HashMap<NamespacedSymbol, Expr>,
    // Could also have a separate map for function definitions.
    functions: HashMap<NamespacedSymbol, Expr>, // Stores FuncDef Exprs
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn define_var(&mut self, symbol: NamespacedSymbol, value: Expr) {
        self.bindings.insert(symbol, value);
    }

    pub fn get_var(&self, symbol: &NamespacedSymbol) -> Option<&Expr> {
        self.bindings.get(symbol)
    }

    pub fn define_func(&mut self, func_def: Expr) {
        if let Expr::FuncDef { name, .. } = &func_def {
            self.functions.insert(name.clone(), func_def);
        } else {
            // Handle error: not a function definition
        }
    }

    pub fn get_func(&self, name: &NamespacedSymbol) -> Option<&Expr> {
        self.functions.get(name)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigInt;

    #[test]
    fn test_namespace_symbol() {
        let ns = Namespace::new(vec!["std".to_string(), "math".to_string()]);
        let sym = NamespacedSymbol::new(ns, "pi".to_string());
        assert_eq!(sym.to_string(), "std::math::pi");

        let root_sym = NamespacedSymbol::new(Namespace::root(), "x".to_string());
        assert_eq!(root_sym.to_string(), "x");
    }

    #[test]
    fn test_expr_display() {
        let expr_const = Expr::Constant(123.to_bigint().unwrap());
        assert_eq!(expr_const.to_string(), "123");

        let expr_sym = Expr::Symbol(NamespacedSymbol::new(Namespace::root(), "x".to_string()));
        assert_eq!(expr_sym.to_string(), "x");

        let expr_app = Expr::Apply(
            Box::new(Expr::Symbol(NamespacedSymbol::new(Namespace::root(), "f".to_string()))),
            vec![Expr::Symbol(NamespacedSymbol::new(Namespace::root(), "x".to_string()))],
        );
        assert_eq!(expr_app.to_string(), "(f x)");

        let expr_curried_app = Expr::Apply(
            Box::new(Expr::Apply(
                Box::new(Expr::Symbol(NamespacedSymbol::new(Namespace::root(), "add".to_string()))),
                vec![Expr::Constant(1.to_bigint().unwrap())]
            )),
            vec![Expr::Constant(2.to_bigint().unwrap())]
        );
        assert_eq!(expr_curried_app.to_string(), "((add 1) 2)");

        let func_def = Expr::FuncDef {
            name: NamespacedSymbol::new(Namespace::root(), "sq".to_string()),
            params: vec![NamespacedSymbol::new(Namespace::root(), "x".to_string())],
            body: Box::new(Expr::Apply(
                Box::new(Expr::Symbol(NamespacedSymbol::new(Namespace::root(), "mul".to_string()))),
                vec![
                    Expr::Symbol(NamespacedSymbol::new(Namespace::root(), "x".to_string())),
                    Expr::Symbol(NamespacedSymbol::new(Namespace::root(), "x".to_string())),
                ]
            ))
        };
        assert_eq!(func_def.to_string(), "def sq(x) = (mul x x)");
    }

    #[test]
    fn test_expr_from_str() {
        assert_eq!(Expr::from_str("123").unwrap(), Expr::Constant(123.to_bigint().unwrap()));
        assert_eq!(Expr::from_str("true").unwrap(), Expr::Bool(true));
        assert_eq!(Expr::from_str("null").unwrap(), Expr::Null);
        assert_eq!(Expr::from_str("x").unwrap(), Expr::Symbol(NamespacedSymbol::new(Namespace::root(), "x".to_string())));
    }
}
