use crate::{
    compile_error::{CompilerError, CompilerErrorLevel, CompilerStage},
    token::{ConstVar, KeyWord, Operator, Seperator, Token},
};
use std::collections::{HashMap, VecDeque};
use strum::IntoEnumIterator;

const WHITE_SPACE_CHARS: [char; 4] = [' ', '\t', '\r', '\n'];

#[derive(Clone, Debug)]
pub struct LexParser<I: Iterator<Item = char>> {
    word_table: HashMap<String, Token>,
    char_stream: I,
    ch: Option<char>,
    line_num: usize,
    temp_queue: VecDeque<Option<char>>,
}

impl<I: Iterator<Item = char>> LexParser<I> {
    pub fn new(char_stream: I) -> Self {
        let mut word_table = HashMap::new();
        for op in Operator::iter() {
            word_table
                .entry(Token::Op(op).get_string())
                .or_insert(Token::Op(op));
        }
        for sep in Seperator::iter() {
            word_table
                .entry(Token::Sep(sep).get_string())
                .or_insert(Token::Sep(sep));
        }
        for kw in KeyWord::iter() {
            word_table
                .entry(Token::Kw(kw).get_string())
                .or_insert(Token::Kw(kw));
        }
        let mut char_stream = char_stream;
        let ch = char_stream.next();
        Self {
            word_table,
            char_stream,
            ch,
            line_num: 1,
            temp_queue: VecDeque::new(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.preprocess();

        let ch = self.current_char()?;

        match ch {
            'a'..='z' | 'A'..='Z' | '_' => self.parse_identifier(ch),
            '0'..='9' => self.parse_num(ch),
            '+' | '*' | '/' | '%' | '&' | ';' | ',' | '(' | ')' | '[' | ']' | '{' | '}' => {
                self.move_char();
                Some(self.word_table.get(&ch.to_string()).cloned().unwrap())
            }
            '-' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '>').is_some() {
                    self.move_char();
                    Some(Token::Op(Operator::PointsTo))
                } else {
                    Some(Token::Op(Operator::Minus))
                }
            }
            '=' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '=').is_some() {
                    self.move_char();
                    Some(Token::Op(Operator::Eq))
                } else {
                    Some(Token::Op(Operator::Assign))
                }
            }
            '!' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '=').is_some() {
                    self.move_char();
                    Some(Token::Op(Operator::Neq))
                } else {
                    CompilerError::new(
                        CompilerErrorLevel::Error,
                        CompilerStage::Compile,
                        &format!("暂不支持 '!' (非操作符): line {}", self.line_num),
                    )
                    .process();
                    None
                }
            }
            '>' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '=').is_some() {
                    self.move_char();
                    Some(Token::Op(Operator::Geq))
                } else {
                    Some(Token::Op(Operator::Gt))
                }
            }
            '<' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '=').is_some() {
                    self.move_char();
                    Some(Token::Op(Operator::Leq))
                } else {
                    Some(Token::Op(Operator::Lt))
                }
            }
            '.' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '.').is_some() {
                    self.move_char();
                    if self.current_char().filter(|c| *c == '.').is_some() {
                        self.move_char();
                        Some(Token::Sep(Seperator::Ellipsis))
                    } else {
                        CompilerError::new(
                            CompilerErrorLevel::Error,
                            CompilerStage::Compile,
                            &format!("省略号拼写错误: line {}", self.line_num),
                        )
                        .process();
                        None
                    }
                } else {
                    Some(Token::Op(Operator::Dot))
                }
            }
            '\'' | '\"' => self.parse_string(ch),
            _ => {
                self.move_char();
                CompilerError::new(
                    CompilerErrorLevel::Error,
                    CompilerStage::Compile,
                    &format!("不认识的字符: {ch}: line {}", self.line_num),
                )
                .process();
                None
            }
        }
    }

    #[inline]
    fn move_char(&mut self) {
        if let Some(ch) = self.temp_queue.pop_front() {
            self.ch = ch;
        } else {
            self.ch = self.char_stream.next()
        }
    }

    #[inline]
    fn current_char(&self) -> Option<char> {
        self.ch
    }

    fn parse_string(&mut self, start: char) -> Option<Token> {
        let mut s = start.to_string();
        self.move_char();

        loop {
            if let Some(ch) = self.current_char() {
                if ch == start {
                    break;
                } else if ch == '\\' {
                    s.push(ch);
                    self.move_char();
                    let c = match ch {
                        '0' => '\0',
                        't' => '\t',
                        'n' => '\n',
                        'r' => '\r',
                        '\'' => '\'',
                        '\"' => '\"',
                        '\\' => '\\',
                        _ => {
                            CompilerError::new(
                                CompilerErrorLevel::Error,
                                CompilerStage::Compile,
                                &format!("未识别的转义字符: {}: line {}", ch, self.line_num),
                            )
                            .process();
                            return None;
                        }
                    };
                    s.push(c);
                    self.move_char();
                } else {
                    s.push(ch);
                    self.move_char()
                }
            } else {
                CompilerError::new(
                    CompilerErrorLevel::Error,
                    CompilerStage::Compile,
                    &format!("引号不闭合: line {}", self.line_num),
                )
                .process();
                return None;
            }
        }

        s.push(start);
        self.move_char();

        if start == '\'' {
            Some(Token::Cvar(ConstVar::Cchar(s.chars().next().unwrap())))
        } else {
            Some(Token::Cvar(ConstVar::Cstring(s)))
        }
    }

    fn parse_num(&mut self, start: char) -> Option<Token> {
        let mut s = start.to_string();
        self.move_char();
        while let Some(ch) = self.current_char().filter(char::is_ascii_digit) {
            s.push(ch);
            self.move_char();
        }
        if self.current_char().unwrap() == '.' {
            s.push('.');
            while let Some(ch) = self.current_char().filter(char::is_ascii_digit) {
                s.push(ch);
                self.move_char();
            }
        }

        let int_var: i32 = s.parse().unwrap();

        Some(Token::Cvar(ConstVar::Cint(int_var)))
    }

    fn parse_identifier(&mut self, start: char) -> Option<Token> {
        fn ok(c: &char) -> bool {
            matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9')
        }

        let mut s = start.to_string();
        self.move_char();
        while let Some(ch) = self.current_char().filter(ok) {
            s.push(ch);
            self.move_char();
        }

        self.word_table
            .entry(s.clone())
            .or_insert(Token::Ident(s.clone()));

        self.word_table.get(&s).cloned()
    }

    fn preprocess(&mut self) {
        if self.current_char().is_some() {
            loop {
                if WHITE_SPACE_CHARS.contains(&self.current_char().unwrap()) {
                    self.skip_with_space();
                } else if self.current_char().filter(|c| *c == '/').is_some() {
                    self.move_char();
                    if self.current_char().filter(|c| *c == '*').is_some() {
                        self.parse_comment();
                    } else {
                        self.temp_queue.push_back(self.current_char());
                        self.ch = Some('/');
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    fn parse_comment(&mut self) {
        self.move_char();
        loop {
            while self
                .current_char()
                .filter(|c| !['\n', '*'].contains(c))
                .is_some()
            {
                self.move_char();
            }
            if self.current_char().filter(|c| *c == '\n').is_some() {
                self.line_num += 1;
                self.move_char();
            } else if self.current_char().filter(|c| *c == '*').is_some() {
                self.move_char();
                if self.current_char().filter(|c| *c == '/').is_some() {
                    self.move_char();
                }
                return;
            } else {
                CompilerError::new(
                    CompilerErrorLevel::Error,
                    CompilerStage::Compile,
                    "一直到文件尾未看到配对的注释结束符",
                )
                .process();
                return;
            }
        }
    }

    fn skip_with_space(&mut self) {
        while let Some(ch) = self
            .current_char()
            .filter(|c| WHITE_SPACE_CHARS.contains(c))
        {
            self.move_char();
            if ['\r', '\n'].contains(&ch) {
                self.line_num += 1;
            }
            print!("{ch}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    #[test]
    fn test_lex_parser() {
        let s = std::fs::read_to_string("x.c").unwrap();

        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let mut lex_parse = LexParser::new(s.chars());

        while let Some(token) = lex_parse.next_token() {
            let color = match token {
                Token::Op(_) => Color::Red,
                Token::Sep(_) => Color::Red,
                Token::Cvar(_) => Color::Yellow,
                Token::Kw(_) => Color::Green,
                Token::Ident(_) => Color::White,
            };
            stdout
                .set_color(ColorSpec::new().set_fg(Some(color)))
                .unwrap();
            write!(&mut stdout, "{}", token.get_string()).unwrap();
        }
        stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
        println!("\n");
    }
}
