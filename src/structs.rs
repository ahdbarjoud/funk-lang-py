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
                c if c == "%".to_string() => TokenType::Percent,
                c if c == "==".to_string() => TokenType::Equals,
                c if c == "=".to_string() => TokenType::Assign,
                c if c == "<".to_string() => TokenType::LessThan,
                c if c == ">".to_string() => TokenType::GreaterThan,
                c if c == ">=".to_string() => TokenType::GreaterOrEqual,
                c if c == "<=".to_string() => TokenType::LessThanOrEqual,
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
