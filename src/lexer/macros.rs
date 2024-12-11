macro_rules! Kind {
    // Punctuation
    [;]     => { $crate::lexer::Kind::Semi };
    [,]     => { $crate::lexer::Kind::Comma };
    [@]     => { $crate::lexer::Kind::At };
    [#]     => { $crate::lexer::Kind::Pound };
    [~]     => { $crate::lexer::Kind::Tilde };
    [?]     => { $crate::lexer::Kind::Question };
    [$]     => { $crate::lexer::Kind::Dollar };
    [:]     => { $crate::lexer::Kind::Colon };
    [.]     => { $crate::lexer::Kind::Dot };
    [..]    => { $crate::lexer::Kind::DotDot };
    [...]   => { $crate::lexer::Kind::DotDotDot };
    [..=]   => { $crate::lexer::Kind::DotDotEq };
    // Custom punctuation
    [::]    => { $crate::lexer::Kind::PathSep };
    [->]    => { $crate::lexer::Kind::RArrow };
    [=>]    => { $crate::lexer::Kind::FatArrow };
    // Operators
    [!]     => { $crate::lexer::Kind::Not };
    // Binary
    [+]     => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![+])};
    [-]     => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![-])};
    [*]     => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![*])};
    [/]     => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![/])};
    [%]     => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![%])};
    [^]     => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![^])};
    [|]     => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![|])};
    [&]     => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![&])};
    [<<]    => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![<<])};
    [>>]    => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![>>])};
    // Logical
    [==]    => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![==]) };
    [!=]    => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![!=]) };
    [<]     => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![<]) };
    [>]     => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![>]) };
    [<=]    => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![<=]) };
    [>=]    => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![>=]) };
    [&&]    => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![&&]) };
    [||]    => { $crate::lexer::Kind::Binary($crate::lexer::BinaryKind![||]) };
    // Assign
    [=]     => { $crate::lexer::Kind::Assign($crate::lexer::AssignKind![=]) };
    [+=]    => { $crate::lexer::Kind::Assign($crate::lexer::AssignKind![+]) };
    [-=]    => { $crate::lexer::Kind::Assign($crate::lexer::AssignKind![-]) };
    [*=]    => { $crate::lexer::Kind::Assign($crate::lexer::AssignKind![*]) };
    [/=]    => { $crate::lexer::Kind::Assign($crate::lexer::AssignKind![/]) };
    [%=]    => { $crate::lexer::Kind::Assign($crate::lexer::AssignKind![%]) };
    [^=]    => { $crate::lexer::Kind::Assign($crate::lexer::AssignKind![^]) };
    [|=]    => { $crate::lexer::Kind::Assign($crate::lexer::AssignKind![|]) };
    [&=]    => { $crate::lexer::Kind::Assign($crate::lexer::AssignKind![&]) };
    [<<=]   => { $crate::lexer::Kind::Assign($crate::lexer::AssignKind![<<]) };
    [>>=]   => { $crate::lexer::Kind::Assign($crate::lexer::AssignKind![>>]) };
    // Delimiters
    ['(']   => { $crate::lexer::Kind::OpenDelim($crate::lexer::Delimiter::Paren) };
    ['{']   => { $crate::lexer::Kind::OpenDelim($crate::lexer::Delimiter::Brace) };
    ['[']   => { $crate::lexer::Kind::OpenDelim($crate::lexer::Delimiter::Bracket) };
    [')']   => { $crate::lexer::Kind::CloseDelim($crate::lexer::Delimiter::Paren) };
    ['}']   => { $crate::lexer::Kind::CloseDelim($crate::lexer::Delimiter::Brace) };
    [']']   => { $crate::lexer::Kind::CloseDelim($crate::lexer::Delimiter::Bracket) };
    // Keywords
    [pub]   => { $crate::lexer::Kind::Keyword($crate::lexer::Keyword::Pub) };
    [let]   => { $crate::lexer::Kind::Keyword($crate::lexer::Keyword::Let) };
    [const] => { $crate::lexer::Kind::Keyword($crate::lexer::Keyword::Const) };
    [mut]   => { $crate::lexer::Kind::Keyword($crate::lexer::Keyword::Mut) };
    [if]    => { $crate::lexer::Kind::Keyword($crate::lexer::Keyword::If) };
    [else]  => { $crate::lexer::Kind::Keyword($crate::lexer::Keyword::Else) };
    [while] => { $crate::lexer::Kind::Keyword($crate::lexer::Keyword::While) };
    [for]   => { $crate::lexer::Kind::Keyword($crate::lexer::Keyword::For) };
    [in]    => { $crate::lexer::Kind::Keyword($crate::lexer::Keyword::In) };
    [fn]    => { $crate::lexer::Kind::Keyword($crate::lexer::Keyword::Fn) };
}

macro_rules! BinaryKind {
    [+]     => { $crate::lexer::BinaryKind::Plus };
    [-]     => { $crate::lexer::BinaryKind::Minus };
    [*]     => { $crate::lexer::BinaryKind::Star };
    [/]     => { $crate::lexer::BinaryKind::Slash };
    [%]     => { $crate::lexer::BinaryKind::Percent };
    [^]     => { $crate::lexer::BinaryKind::Caret };
    [|]     => { $crate::lexer::BinaryKind::Or };
    [&]     => { $crate::lexer::BinaryKind::And };
    [<<]    => { $crate::lexer::BinaryKind::Shl };
    [>>]    => { $crate::lexer::BinaryKind::Shr };
    [==]    => { $crate::lexer::BinaryKind::Eq };
    [!=]    => { $crate::lexer::BinaryKind::Ne };
    [<]     => { $crate::lexer::BinaryKind::Lt };
    [>]     => { $crate::lexer::BinaryKind::Gt };
    [<=]    => { $crate::lexer::BinaryKind::Le };
    [>=]    => { $crate::lexer::BinaryKind::Ge };
    [&&]    => { $crate::lexer::BinaryKind::And };
    [||]    => { $crate::lexer::BinaryKind::Or };
}

macro_rules! AssignKind {
    [=]     => { $crate::lexer::AssignKind::Eq };
    [+]     => { $crate::lexer::AssignKind::Plus };
    [-]     => { $crate::lexer::AssignKind::Minus };
    [*]     => { $crate::lexer::AssignKind::Star };
    [/]     => { $crate::lexer::AssignKind::Slash };
    [%]     => { $crate::lexer::AssignKind::Percent };
    [^]     => { $crate::lexer::AssignKind::Caret };
    [|]     => { $crate::lexer::AssignKind::Or };
    [&]     => { $crate::lexer::AssignKind::And };
    [<<]    => { $crate::lexer::AssignKind::Shl };
    [>>]    => { $crate::lexer::AssignKind::Shr };
}

pub(crate) use AssignKind;
pub(crate) use BinaryKind;
pub(crate) use Kind;
