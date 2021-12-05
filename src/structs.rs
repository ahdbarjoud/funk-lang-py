pub mod structs {
    use std::ops::{Add, Range};
    use std::collections::HashMap;

    pub const OPEARTORS: [&'static str; 28] = [
        "+", "-", "*", "/", "%", "^", ">", "<", ">=", "<=", "==", "!=", "!", "&", ":", "?", "|",
        "::", "#", "&&", "||", "++", "--", "=", "###", ".", "..", "...",
    ];
    pub const KEYWORDS: [&'static str; 18] = [
        "funk", "while", "for", "when", "if", "elseif", "else", "Integer", "String", "Decimal",
        "klass", "in", "return", "Boolean", "Array", "Hash", "mut", "Void",
    ];

    #[derive(Debug, Clone, PartialEq)]
    pub struct Token {
        pub ty: TokenType,
        pub line: usize,
        pub range: Range<usize>
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum TokenType {
        // \n
        Newline,
        // (
        LPar,
        // )
        RPar,
        // {
        LCurl,
        // }
        RCurl,
        // [
        LBrac,
        // ]
        RBrac,
        // ,
        Comma,
        // ;
        Semi,
        Literal(Val),
        Keyword(Keyword),
        Identifier,
        // / Single Line - # Multi Line #
        Comment,
        // ++
        PlusPlus,
        // --
        MinusMinus,
        // +
        Plus,
        // -
        Minus,
        // *
        Star,
        // /
        Slash,
        // **
        StarStar,
        // %
        Percent,
        // .
        Dot,
        // =
        Equals,
        // ==
        EqualsEquals,
        // <
        LessThan,
        // >
        GreaterThan,
        // <=
        LessThanOrEqual,
        // >=
        GreaterOrEqual,
        // !=
        NotEqual,
        // &&
        And,
        // ||
        Or,
        // +=
        PlusEqual,
        // -=
        MinusEqual,
        // !
        Not,
        // Unexpected
        Unknown
    }

    impl TokenType {
        pub fn new(raw: String) -> TokenType {
            match raw {
                c if c == "+".to_string() => TokenType::Plus,
                c if c == "-".to_string() => TokenType::Minus,
                c if c == "/".to_string() => TokenType::Slash,
                c if c == "*".to_string() => TokenType::Star,
                c if c == "**".to_string() => TokenType::StarStar,
                c if c == "%".to_string() => TokenType::Percent,
                c if c == "==".to_string() => TokenType::EqualsEquals,
                c if c == "=".to_string() => TokenType::Equals,
                c if c == "<".to_string() => TokenType::LessThan,
                c if c == ">".to_string() => TokenType::GreaterThan,
                c if c == ">=".to_string() => TokenType::GreaterOrEqual,
                c if c == "<=".to_string() => TokenType::LessThanOrEqual,
                c if c == "!".to_string() => TokenType::Not,
                c if c == "!=".to_string() => TokenType::NotEqual,
                c if c == "&&".to_string() => TokenType::And,
                c if c == "||".to_string() => TokenType::Or,
                c if c == "+=".to_string() => TokenType::PlusEqual,
                c if c == "-=".to_string() => TokenType::MinusEqual,
                c if c == "(".to_string() => TokenType::LPar,
                c if c == ")".to_string() => TokenType::RPar,
                c if c == "[".to_string() => TokenType::LBrac,
                c if c == "]".to_string() => TokenType::RBrac,
                c if c == "{".to_string() => TokenType::LCurl,
                c if c == "}".to_string() => TokenType::RCurl,
                c if c == ",".to_string() => TokenType::Comma,
                c if c == ";".to_string() => TokenType::Semi,
                _ => TokenType::Unknown
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Keyword {
        Integer,
        Decimal,
        Boolean,
        If,
        Else,
        Elseif,
        Return,
        While,
        For,
        Break,
        Funk,
        Unknown
    }

    impl Keyword {
        pub fn new(raw: String) -> Keyword {
            match raw {
                c if c == "Integer" => Keyword::Integer,
                c if c == "Decimal" => Keyword::Decimal,
                c if c == "Boolean" => Keyword::Boolean,
                c if c == "if" => Keyword::If,
                c if c == "else" => Keyword::Else,
                c if c == "elseif" => Keyword::Elseif,
                c if c == "return" => Keyword::Return,
                c if c == "while" => Keyword::While,
                c if c == "for" => Keyword::For,
                c if c == "break" => Keyword::Break,
                c if c == "funk" => Keyword::Funk,
                _  => Keyword::Unknown,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Val {
        Number,
        String
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        Integer,
        Decimal
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum AST {
        Expression(Expr),
        Statement(Statement)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Statement {
        Assignment(Assign)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Assign {
        pub name: String,
        pub ty: Type,
        pub value: Box<AST>
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Expr {
        Integer(i64),
        Decimal(f64),
        String(String),
        Boolean(bool),
        Binary(BinaryExpr),
        Argument{
            name: String,
            ty: Keyword
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum BinOp {
        Add,
        Subtract,
        Multiply,
        Divide,
        Exponent,
        Unknown
    }
    impl BinOp {
        pub fn new(tkty: &TokenType) -> BinOp {
            match  tkty {
                TokenType::Plus => BinOp::Add,
                TokenType::Minus => BinOp::Subtract,
                TokenType::Star => BinOp::Multiply,
                TokenType::Slash => BinOp::Divide,
                TokenType::StarStar => BinOp::Exponent,
                _ => BinOp::Unknown
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BinaryExpr {
        pub left: Box<AST>,
        pub op: BinOp,
        pub right: Box<AST>
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Object {
        Integer(Integer),
        Decimal(Decimal),
        Module(Module),
        Function(Function),
        Class(Class),
        Variable(Variable)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Variable {
        pub name: String,
        pub ty: Type,
        pub value: Box<Object>
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Integer {
        pub value: i64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Decimal {
        pub value: f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Module {
        path: String,
        env: Env
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Function {
        params: Vec<Object>,
        body: Vec<Object>,
        env: Env
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Class {
        pub env: Env
    }

    impl Add for Object {
        type Output = Object;

        fn add(self, other: Self) -> Self::Output {
            match self {
                Object::Integer(int1) => {
                    match other {
                        Object::Integer(int2) => {
                            Object::Integer(Integer{ value: int1.value + int2.value })
                        },
                        _ => {
                            panic!("");
                        }
                    }
                },
                _ => {
                    panic!("");
                }
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Env {
        pub vars: HashMap<String, Object>,
        pub funcs: HashMap<String, Object>,
        pub mods: HashMap<String, Object>
    }

    impl Env {
        pub fn new() -> Env {
            Env{ vars: HashMap::new(), funcs: HashMap::new(), mods: HashMap::new() }
        }

        pub fn get_var(&mut self, name: String) -> Option<&mut Object> {
            self.vars.get_mut(&name)
        }
        pub fn set_var(&mut self, name: String, val: Object) -> Option<Object> {
            self.vars.insert(name, val)
        }
        pub fn remove_var(&mut self, name: String) -> Option<Object> {
            self.vars.remove(&name)
        }

        pub fn get_mod(&mut self, name: String) -> Option<&mut Object> {
            self.mods.get_mut(&name)
        }
        pub fn set_mod(&mut self, name: String, val: Object) -> Option<Object> {
            self.mods.insert(name, val)
        }
        pub fn remove_mod(&mut self, name: String) -> Option<Object> {
            self.mods.remove(&name)
        }
    }
}