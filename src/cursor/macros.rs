macro_rules! Kind {
    [;]    => { $crate::cursor::Kind::Semi };
    [:]    => { $crate::cursor::Kind::Colon };
    [,]    => { $crate::cursor::Kind::Comma };
    [.]    => { $crate::cursor::Kind::Dot };
    [@]    => { $crate::cursor::Kind::At };
    [#]    => { $crate::cursor::Kind::Pound };
    [~]    => { $crate::cursor::Kind::Tilde };
    [?]    => { $crate::cursor::Kind::Question };
    [$]    => { $crate::cursor::Kind::Dollar };
    [=]    => { $crate::cursor::Kind::Eq };
    [!]    => { $crate::cursor::Kind::Bang };
    [<]    => { $crate::cursor::Kind::Lt };
    [>]    => { $crate::cursor::Kind::Gt };
    [&]    => { $crate::cursor::Kind::And };
    [|]    => { $crate::cursor::Kind::Or };
    [+]    => { $crate::cursor::Kind::Plus };
    [-]    => { $crate::cursor::Kind::Minus };
    [*]    => { $crate::cursor::Kind::Star };
    [/]    => { $crate::cursor::Kind::Slash };
    [^]    => { $crate::cursor::Kind::Caret };
    [%]    => { $crate::cursor::Kind::Percent };
    ['(']  => { $crate::cursor::Kind::OpenParen };
    [')']  => { $crate::cursor::Kind::CloseParen };
    ['{']  => { $crate::cursor::Kind::OpenBrace };
    ['}']  => { $crate::cursor::Kind::CloseBrace };
    ['[']  => { $crate::cursor::Kind::OpenBracket };
    [']']  => { $crate::cursor::Kind::CloseBracket };
}

pub(crate) use Kind;
