use sccompiler::{
    lex_parser::LexParser,
    token::{ConstVar, KeyWord, Operator, Token},
};

#[test]
fn test_skip_white_space() {
    let s = " void \tint \n\n struct";
    let mut lex_parser = LexParser::new(s.chars());
    let mut tokens = Vec::new();
    while let Ok(Some(token)) = lex_parser.next_token() {
        tokens.push(token)
    }
    assert!(tokens[0] == Token::Kw(KeyWord::Void));
    assert!(tokens[1] == Token::Kw(KeyWord::Int));
    assert!(tokens[2] == Token::Kw(KeyWord::Struct));
    assert!(tokens.len() == 3);
}

#[test]
fn test_skip_comment() {
    let s = " void /* sdfdf */ int";
    let mut lex_parser = LexParser::new(s.chars());
    let mut tokens = Vec::new();
    while let Ok(Some(token)) = lex_parser.next_token() {
        tokens.push(token)
    }
    assert!(tokens[0] == Token::Kw(KeyWord::Void));
    assert!(tokens[1] == Token::Kw(KeyWord::Int));
    assert!(tokens.len() == 2);

    let s = " void /*** \n sdfdf \t****/ int";
    let mut lex_parser = LexParser::new(s.chars());
    let mut tokens = Vec::new();
    while let Ok(Some(token)) = lex_parser.next_token() {
        tokens.push(token)
    }
    assert!(tokens[0] == Token::Kw(KeyWord::Void));
    assert!(tokens[1] == Token::Kw(KeyWord::Int));
    assert!(tokens.len() == 2);

    let s = " void //*** \n sdfdf \t****/ int";
    let mut lex_parser = LexParser::new(s.chars());
    let mut tokens = Vec::new();
    while let Ok(Some(token)) = lex_parser.next_token() {
        tokens.push(token)
    }
    assert!(tokens[0] == Token::Kw(KeyWord::Void));
    assert!(tokens[1] == Token::Op(Operator::Divide),);
    assert!(tokens[2] == Token::Kw(KeyWord::Int));
    assert!(tokens.len() == 3);

    let s = " void /* sdfdf int";
    let mut lex_parser = LexParser::new(s.chars());
    let mut err = None;
    for _ in 0..10 {
        let tk = lex_parser.next_token();
        if tk.is_err() {
            err = Some(tk.err().unwrap());
        }
    }
    assert!(err.unwrap().error_message() == "一直到文件尾未看到配对的注释结束符");
}

#[test]
fn test_parse_identifier() {
    let s = " void \t fs23";
    let mut lex_parser = LexParser::new(s.chars());
    let mut tokens = Vec::new();
    while let Ok(Some(token)) = lex_parser.next_token() {
        tokens.push(token)
    }
    assert!(tokens[0] == Token::Kw(KeyWord::Void));
    assert!(tokens[1] == Token::Ident("fs23".to_string()));
    assert!(tokens.len() == 2);
}

#[test]
fn test_parse_num() {
    let s = " void \t 123";
    let mut lex_parser = LexParser::new(s.chars());
    let mut tokens = Vec::new();
    while let Ok(Some(token)) = lex_parser.next_token() {
        tokens.push(token)
    }
    assert!(tokens[0] == Token::Kw(KeyWord::Void));
    assert!(tokens[1] == Token::Cvar(ConstVar::Int(123)));
    assert!(tokens.len() == 2);

    let s = " void \t 123.";
    let mut lex_parser = LexParser::new(s.chars());
    let mut tokens = Vec::new();
    while let Ok(Some(token)) = lex_parser.next_token() {
        tokens.push(token)
    }

    assert!(tokens.len() == 2);
    assert!(tokens[0] == Token::Kw(KeyWord::Void));
    assert!(tokens[1] == Token::Cvar(ConstVar::Int(123)));

    let s = " void \t 123.123 dsf";
    let mut lex_parser = LexParser::new(s.chars());
    let mut tokens = Vec::new();
    while let Ok(Some(token)) = lex_parser.next_token() {
        tokens.push(token)
    }

    assert!(tokens.len() == 3);
    assert!(tokens[0] == Token::Kw(KeyWord::Void));
    assert!(tokens[1] == Token::Cvar(ConstVar::Int(123)));
    assert!(tokens[2] == Token::Ident("dsf".to_string()));
}

#[test]
fn test_parse_string() {
    let s = " void \t \'x\' \"yzx\"";
    let mut lex_parser = LexParser::new(s.chars());
    let mut tokens = Vec::new();
    while let Ok(Some(token)) = lex_parser.next_token() {
        println!("token = {token:?}");
        tokens.push(token)
    }
    assert!(tokens.len() == 3);
    assert!(tokens[0] == Token::Kw(KeyWord::Void));
    assert!(tokens[1] == Token::Cvar(ConstVar::Char('x')));
    assert!(tokens[2] == Token::Cvar(ConstVar::String("yzx".to_string())));
}
