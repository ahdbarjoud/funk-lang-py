pub mod structs {
    use std::ops::Range;

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

    // NOTE: Remove
    #[derive(Debug, Clone, PartialEq)]
    pub enum TokenType {
        // \n
        Newline,
        Whitespace,
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
        Identifier(String),
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
        Multiply,
        // /
        Divide,
        // **
        Exponent,
        // %
        Percent,
        // .
        Dot,
        // =
        Assign,
        // ==
        Equals,
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
        // Unexpected
        Unknown
    }
    impl TokenType {
        pub fn new(raw: String) -> TokenType {
            match raw {
                c if c == "+".to_string() => TokenType::Plus,
                c if c == "-".to_string() => TokenType::Minus,
                c if c == "/".to_string() => TokenType::Divide,
                c if c == "*".to_string() => TokenType::Multiply,

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

    #[derive(Debug, Clone, PartialEq)]
    pub enum Val {
        Number,
        String
    }





















}
//     #[derive(PartialEq, Debug, Clone)]
//     pub enum AST {
//         BiOpAST {
//             op: String,
//             left: Box<AST>,
//             right: Box<AST>,
//         },
//         UnOp {
//             op: String,
//             value: Box<AST>,
//         },
//         Integer {
//             value: i64,
//         },
//         Decminal {
//             value: f64,
//         },
//         Str {
//             value: String,
//         },
//         Assgignment {
//             name: String,
//             value: Box<AST>,
//             var_type: String,
//             scope: String,
//             line: usize,
//         },
//         FunktionParameter {
//             name: String,
//             typ: String,
//         },
//         Funktion {
//             name: String,
//             return_typ: String,
//             params: Vec<Box<AST>>,
//             body: Vec<Box<AST>>,
//         },
//         Conditional {
//             typ: String,
//             expr: Option<Box<AST>>,
//             body: Vec<Box<AST>>,
//             other: Option<Box<AST>>,
//         },
//         CallItem {
//             name: String,
//             call_type: String,
//             args: Option<Vec<Box<AST>>>,
//             scope: String,
//         },
//         Variable {
//             name: String,
//             value: Box<AST>,
//             var_type: String,
//             scope: String,
//             line: usize,
//         },
//     }

//     impl Add for AST {
//         type Output = Self;

//         fn add(self, other: Self) -> Self {
//             if let AST::Integer { value } = self {
//                 let val1 = value;

//                 if let AST::Integer { value } = other {
//                     let val2 = value;
//                     return Self::Integer { value: val1 + val2 };
//                 } else if let AST::Decminal { value } = other {
//                     let val2 = value;
//                     let val1 = val1 as f64;
//                     return Self::Decminal { value: val1 + val2 };
//                 } else {
//                     panic!("Add AST panic.")
//                 }
//             } else if let AST::Decminal { value } = self {
//                 let val1 = value;

//                 if let AST::Decminal { value } = other {
//                     let val2 = value;
//                     return Self::Decminal { value: val1 + val2 };
//                 } else if let AST::Integer { value } = other {
//                     let val2 = value as f64;
//                     return Self::Decminal { value: val1 + val2 };
//                 } else {
//                     panic!("Add AST panic.")
//                 }
//             } else {
//                 panic!("Add AST panic.")
//             }
//         }
//     }

//     impl Sub for AST {
//         type Output = Self;

//         fn sub(self, other: Self) -> Self {
//             if let AST::Integer { value } = self {
//                 let val1 = value;

//                 if let AST::Integer { value } = other {
//                     let val2 = value;
//                     Self::Integer { value: val1 - val2 }
//                 } else if let AST::Decminal { value } = other {
//                     let val2 = value;
//                     let val1 = val1 as f64;
//                     Self::Decminal { value: val1 - val2 }
//                 } else {
//                     panic!("Sub AST panic.")
//                 }
//             } else if let AST::Decminal { value } = self {
//                 let val1 = value;

//                 if let AST::Decminal { value } = other {
//                     let val2 = value;
//                     Self::Decminal { value: val1 - val2 }
//                 } else if let AST::Integer { value } = other {
//                     let val2 = value as f64;
//                     Self::Decminal { value: val1 - val2 }
//                 } else {
//                     panic!("Sub AST panic.")
//                 }
//             } else {
//                 panic!("Sub AST panic.")
//             }
//         }
//     }

//     impl Mul for AST {
//         type Output = Self;

//         fn mul(self, other: Self) -> Self {
//             if let AST::Integer { value } = self {
//                 let val1 = value;

//                 if let AST::Integer { value } = other {
//                     let val2 = value;
//                     Self::Integer { value: val1 * val2 }
//                 } else if let AST::Decminal { value } = other {
//                     let val1 = val1 as f64;
//                     let val2 = value;
//                     Self::Decminal { value: val1 * val2 }
//                 } else {
//                     panic!("Mul AST panic.");
//                 }
//             } else if let AST::Decminal { value } = self {
//                 let val1 = value;

//                 if let AST::Integer { value } = other {
//                     let val2 = value as f64;
//                     Self::Decminal { value: val1 * val2 }
//                 } else if let AST::Decminal { value } = other {
//                     let val2 = value;
//                     Self::Decminal { value: val1 * val2 }
//                 } else {
//                     panic!("Mul AST panic.")
//                 }
//             } else {
//                 panic!("Mul AST panic.");
//             }
//         }
//     }

//     impl Div for AST {
//         type Output = Self;

//         fn div(self, other: Self) -> Self {
//             if let AST::Integer { value } = self {
//                 let val1 = value;

//                 if let AST::Integer { value } = other {
//                     let val2 = value;
//                     Self::Integer { value: val1 / val2 }
//                 } else if let AST::Decminal { value } = other {
//                     let val1 = val1 as f64;
//                     let val2 = value;
//                     Self::Decminal { value: val1 / val2 }
//                 } else {
//                     panic!("Div AST panic.");
//                 }
//             } else if let AST::Decminal { value } = self {
//                 let val1 = value;

//                 if let AST::Integer { value } = other {
//                     let val2 = value as f64;
//                     Self::Decminal { value: val1 / val2 }
//                 } else if let AST::Decminal { value } = other {
//                     let val2 = value;
//                     Self::Decminal { value: val1 / val2 }
//                 } else {
//                     panic!("Div AST panic.")
//                 }
//             } else {
//                 panic!("Div AST panic.");
//             }
//         }
//     }
// }
