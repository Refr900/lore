macro_rules! Token {
    [=]     => { $crate::lexer::TokenKind::Eq };
    // Logical
    [==]    => { $crate::lexer::TokenKind::EqEq };
    [!=]    => { $crate::lexer::TokenKind::Ne };
    [<]     => { $crate::lexer::TokenKind::Lt };
    [>]     => { $crate::lexer::TokenKind::Gt };
    [<=]    => { $crate::lexer::TokenKind::Le };
    [>=]    => { $crate::lexer::TokenKind::Ge };
    [&&]    => { $crate::lexer::TokenKind::AndAnd };
    [||]    => { $crate::lexer::TokenKind::OrOr };
    [!]     => { $crate::lexer::TokenKind::Not };
    // Punctuation
    [.]     => { $crate::lexer::TokenKind::Dot };
    [..]    => { $crate::lexer::TokenKind::DotDot };
    [...]   => { $crate::lexer::TokenKind::DotDotDot };
    [..=]   => { $crate::lexer::TokenKind::DotDotEq };
    [,]     => { $crate::lexer::TokenKind::Comma };
    [;]     => { $crate::lexer::TokenKind::Semi };
    [:]     => { $crate::lexer::TokenKind::Colon };
    // Custom punctuation
    [::]    => { $crate::lexer::TokenKind::PathSep };
    [->]    => { $crate::lexer::TokenKind::RArrow };
    [=>]    => { $crate::lexer::TokenKind::FatArrow };
    // Binary operators
    [+]     => { $crate::lexer::TokenKind::Binary($crate::lexer::BinaryToken::Plus) };
    [-]     => { $crate::lexer::TokenKind::Binary($crate::lexer::BinaryToken::Minus) };
    [*]     => { $crate::lexer::TokenKind::Binary($crate::lexer::BinaryToken::Star) };
    [/]     => { $crate::lexer::TokenKind::Binary($crate::lexer::BinaryToken::Slash) };
    [%]     => { $crate::lexer::TokenKind::Binary($crate::lexer::BinaryToken::Percent) };
    [^]     => { $crate::lexer::TokenKind::Binary($crate::lexer::BinaryToken::Caret) };
    [&]     => { $crate::lexer::TokenKind::Binary($crate::lexer::BinaryToken::And) };
    [|]     => { $crate::lexer::TokenKind::Binary($crate::lexer::BinaryToken::Or) };
    [<<]    => { $crate::lexer::TokenKind::Binary($crate::lexer::BinaryToken::Shl) };
    [>>]    => { $crate::lexer::TokenKind::Binary($crate::lexer::BinaryToken::Shr) };
    // Delimiters
    ['(']   => { $crate::lexer::TokenKind::OpenDelim($crate::lexer::Delimiter::Paren) };
    [')']   => { $crate::lexer::TokenKind::CloseDelim($crate::lexer::Delimiter::Paren) };
    ['{']   => { $crate::lexer::TokenKind::OpenDelim($crate::lexer::Delimiter::Brace) };
    ['}']   => { $crate::lexer::TokenKind::CloseDelim($crate::lexer::Delimiter::Brace) };
    ['[']   => { $crate::lexer::TokenKind::OpenDelim($crate::lexer::Delimiter::Bracket) };
    [']']   => { $crate::lexer::TokenKind::CloseDelim($crate::lexer::Delimiter::Bracket) };
    // Keywords
    [pub]   => { $crate::lexer::TokenKind::Keyword(Keyword::Pub) };
    [let]   => { $crate::lexer::TokenKind::Keyword(Keyword::Let) };
    [const] => { $crate::lexer::TokenKind::Keyword(Keyword::Const) };
    [mut]   => { $crate::lexer::TokenKind::Keyword(Keyword::Mut) };
    [if]    => { $crate::lexer::TokenKind::Keyword(Keyword::If) };
    [else]  => { $crate::lexer::TokenKind::Keyword(Keyword::Else) };
    [while] => { $crate::lexer::TokenKind::Keyword(Keyword::While) };
    [for]   => { $crate::lexer::TokenKind::Keyword(Keyword::For) };
    [in]    => { $crate::lexer::TokenKind::Keyword(Keyword::In) };
    [fn]    => { $crate::lexer::TokenKind::Keyword(Keyword::Fn) };
}

pub(crate) use Token;
