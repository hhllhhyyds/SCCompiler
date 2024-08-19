use crate::{
    compile_error::CompileError,
    token::{ConstVar, KeyWord, Operator, Seperator, Token},
};
use std::collections::{HashMap, VecDeque};
use strum::IntoEnumIterator;

const WHITE_SPACE_CHARS: [char; 3] = [' ', '\t', '\n'];
macro_rules! is_white_space {
    ($c: ident) => {
        WHITE_SPACE_CHARS.contains($c)
    };
}

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

    pub fn next_token(&mut self) -> Result<Option<Token>, CompileError> {
        self.preprocess()?;

        let Some(ch) = self.current_char() else {
            return Ok(None);
        };

        match ch {
            'a'..='z' | 'A'..='Z' | '_' => Ok(Some(self.parse_identifier(ch))),
            '0'..='9' => self.parse_num(ch).map(Some),
            '+' | '*' | '/' | '%' | '&' | ';' | ',' | '(' | ')' | '[' | ']' | '{' | '}' => {
                self.move_char();
                Ok(Some(self.word_table.get(&ch.to_string()).cloned().unwrap()))
            }
            '-' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '>').is_some() {
                    self.move_char();
                    Ok(Some(Token::Op(Operator::PointsTo)))
                } else {
                    Ok(Some(Token::Op(Operator::Minus)))
                }
            }
            '=' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '=').is_some() {
                    self.move_char();
                    Ok(Some(Token::Op(Operator::Eq)))
                } else {
                    Ok(Some(Token::Op(Operator::Assign)))
                }
            }
            '!' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '=').is_some() {
                    self.move_char();
                    Ok(Some(Token::Op(Operator::Neq)))
                } else {
                    Err(CompileError::compile_stage_error(&format!(
                        "暂不支持 '!' (非操作符): line {}",
                        self.line_num
                    )))
                }
            }
            '>' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '=').is_some() {
                    self.move_char();
                    Ok(Some(Token::Op(Operator::Geq)))
                } else {
                    Ok(Some(Token::Op(Operator::Gt)))
                }
            }
            '<' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '=').is_some() {
                    self.move_char();
                    Ok(Some(Token::Op(Operator::Leq)))
                } else {
                    Ok(Some(Token::Op(Operator::Lt)))
                }
            }
            '.' => {
                self.move_char();
                if self.current_char().filter(|c| *c == '.').is_some() {
                    self.move_char();
                    if self.current_char().filter(|c| *c == '.').is_some() {
                        self.move_char();
                        Ok(Some(Token::Sep(Seperator::Ellipsis)))
                    } else {
                        Err(CompileError::compile_stage_error(&format!(
                            "省略号拼写错误: line {}",
                            self.line_num
                        )))
                    }
                } else {
                    Ok(Some(Token::Op(Operator::Dot)))
                }
            }
            '\'' | '\"' => self.parse_string(ch).map(Some),
            _ => {
                self.move_char();
                Err(CompileError::compile_stage_error(&format!(
                    "不认识的字符: {ch}: line {}",
                    self.line_num
                )))
            }
        }
    }

    #[inline]
    fn move_char(&mut self) {
        self.ch = self
            .temp_queue
            .pop_front()
            .unwrap_or(self.char_stream.next())
    }

    #[inline]
    fn current_char(&self) -> Option<char> {
        self.ch
    }

    fn parse_string(&mut self, start: char) -> Result<Token, CompileError> {
        let mut s = start.to_string();
        self.move_char();

        loop {
            if let Some(ch) = self.current_char() {
                if ch == start {
                    break;
                } else if ch == '\\' {
                    self.move_char();

                    s.push(if let Some(ch) = self.current_char() {
                        match ch {
                            '0' => '\0',
                            't' => '\t',
                            'n' => '\n',
                            'r' => '\r',
                            '\'' | '\"' | '\\' => ch,
                            _ => {
                                return Err(CompileError::compile_stage_error(&format!(
                                    "未识别的转义字符: {}: line {}",
                                    ch, self.line_num
                                )))
                            }
                        }
                    } else {
                        return Err(CompileError::compile_stage_error(&format!(
                            "转义字符后为空: {}: line {}",
                            ch, self.line_num
                        )));
                    });

                    self.move_char();
                } else {
                    s.push(ch);
                    self.move_char()
                }
            } else {
                return Err(CompileError::compile_stage_error(&format!(
                    "引号不闭合: line {}",
                    self.line_num
                )));
            }
        }

        s.push(start);
        self.move_char();

        if start == '\'' {
            if s.chars().count() != 3 {
                Err(CompileError::compile_stage_error("常量字符长度大于1"))
            } else {
                Ok(Token::Cvar(ConstVar::Char(s.chars().nth(1).unwrap())))
            }
        } else {
            assert!(start == '\"');
            Ok(Token::Cvar(ConstVar::String(
                s[1..(s.len() - 1)].to_string(),
            )))
        }
    }

    fn parse_num(&mut self, start: char) -> Result<Token, CompileError> {
        let mut s = start.to_string();
        self.move_char();

        while let Some(ch) = self.current_char().filter(char::is_ascii_digit) {
            s.push(ch);
            self.move_char();
        }
        if self.current_char().filter(|c| *c == '.').is_some() {
            s.push('.');
            self.move_char();
            while let Some(ch) = self.current_char().filter(char::is_ascii_digit) {
                s.push(ch);
                self.move_char();
            }
        }

        let int_var = s
            .parse::<f64>()
            .map_err(|e| CompileError::compile_stage_error(&e.to_string()))?;

        Ok(Token::Cvar(ConstVar::Int(int_var as i32)))
    }

    fn parse_identifier(&mut self, start: char) -> Token {
        let mut s = start.to_string();
        self.move_char();

        while let Some(ch) = self
            .current_char()
            .filter(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'))
        {
            s.push(ch);
            self.move_char();
        }

        self.word_table
            .entry(s.clone())
            .or_insert(Token::Ident(s.clone()));

        self.word_table.get(&s).cloned().unwrap()
    }

    fn preprocess(&mut self) -> Result<(), CompileError> {
        let mut ret = Ok(());

        if self.current_char().is_some() {
            loop {
                if self.current_char().filter(|c| is_white_space!(c)).is_some() {
                    self.skip_white_space();
                } else if self.current_char().filter(|c| *c == '/').is_some() {
                    self.move_char();
                    if self.current_char().filter(|c| *c == '*').is_some() {
                        ret = self.parse_comment();
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

        ret
    }

    fn parse_comment(&mut self) -> Result<(), CompileError> {
        self.move_char();
        loop {
            while self
                .current_char()
                .filter(|c| !(['\n', '*'].contains(c)))
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
                    return Ok(());
                }
            } else {
                return Err(CompileError::compile_stage_error(
                    "一直到文件尾未看到配对的注释结束符",
                ));
            }
        }
    }

    fn skip_white_space(&mut self) {
        while let Some(ch) = self.current_char().filter(|c| is_white_space!(c)) {
            if ch == '\n' {
                self.line_num += 1;
            }
            self.move_char();
        }
    }
}
