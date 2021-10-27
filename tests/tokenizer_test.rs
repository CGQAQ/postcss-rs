use postcss::input::Input;
use postcss::tokenizer::*;

fn tokenize(css: &str, ignore_errors: bool) -> Vec<Token> {
  let input = Input::new(css.to_string(), None);
  let mut processor = Tokenizer::new(&input, ignore_errors);
  let mut tokens = vec![];
  while !processor.end_of_file() {
    tokens.push(processor.next_token(false))
  }
  return tokens;
}

fn run(css: &str, tokens: Vec<Token>) {
  assert_eq!(tokenize(css, false), tokens);
}

fn run_ignore_errors(css: &str, tokens: Vec<Token>) {
  assert_eq!(tokenize(css, true), tokens);
}

#[test]
fn tokenizes_empty_file() {
  run("", vec![]);
}

#[test]
fn tokenizes_space() {
  run(
    "\r\n \u{12}\t",
    vec![Token::Space {
      content: "\r\n \u{12}\t".into(),
    }],
  );
}

#[test]
fn tokenizes_word() {
  run(
    "ab",
    vec![Token::Word {
      content: "ab".into(),
      pos: 0,
      next: 1,
    }],
  );
}

#[test]
fn splits_word_by_exclamation_mark() {
  run(
    "aa!bb",
    vec![
      Token::Word {
        content: "aa".into(),
        pos: 0,
        next: 1,
      },
      Token::Word {
        content: "!bb".into(),
        pos: 2,
        next: 4,
      },
    ],
  );
}

#[test]
fn changes_lines_in_spaces() {
  run(
    "a \n b",
    vec![
      Token::Word {
        content: "a".into(),
        pos: 0,
        next: 0,
      },
      Token::Space {
        content: " \n ".into(),
      },
      Token::Word {
        content: "b".into(),
        pos: 4,
        next: 4,
      },
    ],
  );
}

#[test]
fn tokenizes_control_chars() {
  run(
    "{:;}",
    vec![
      Token::Control {
        kind: TokenControlKind::OpenCurly,
        content: TokenControlKind::OpenCurly.into(),
        pos: 0,
      },
      Token::Control {
        kind: TokenControlKind::Colon,
        content: TokenControlKind::Colon.into(),
        pos: 1,
      },
      Token::Control {
        kind: TokenControlKind::Semicolon,
        content: TokenControlKind::Semicolon.into(),
        pos: 2,
      },
      Token::Control {
        kind: TokenControlKind::CloseCurly,
        content: TokenControlKind::CloseCurly.into(),
        pos: 3,
      },
    ],
  );
}

#[test]
fn escapes_control_symbols() {
  run(
    "\\(\\{\\\"\\@\\\\\"\"",
    vec![
      Token::Word {
        content: "\\(".into(),
        pos: 0,
        next: 1,
      },
      Token::Word {
        content: "\\{".into(),
        pos: 2,
        next: 3,
      },
      Token::Word {
        content: "\\\"".into(),
        pos: 4,
        next: 5,
      },
      Token::Word {
        content: "\\@".into(),
        pos: 6,
        next: 7,
      },
      Token::Word {
        content: "\\\\".into(),
        pos: 8,
        next: 9,
      },
      Token::String {
        content: "\"\"".into(),
        pos: 10,
        next: 11,
      },
    ],
  );
}

#[test]
fn escapes_backslash() {
  run(
    "\\\\\\\\{",
    vec![
      Token::Word {
        content: "\\\\\\\\".into(),
        pos: 0,
        next: 3,
      },
      Token::Control {
        kind: TokenControlKind::OpenCurly,
        content: TokenControlKind::OpenCurly.into(),
        pos: 4,
      },
    ],
  );
}

#[test]
fn tokenizes_simple_brackets() {
  run(
    "(ab)",
    vec![Token::Brackets {
      content: "(ab)".into(),
      pos: 0,
      next: 3,
    }],
  );
}

#[test]
fn tokenizes_square_brackets() {
  run(
    "a[bc]",
    vec![
      Token::Word {
        content: "a".into(),
        pos: 0,
        next: 0,
      },
      Token::Control {
        kind: TokenControlKind::OpenSquare,
        content: TokenControlKind::OpenSquare.into(),
        pos: 1,
      },
      Token::Word {
        content: "bc".into(),
        pos: 2,
        next: 3,
      },
      Token::Control {
        kind: TokenControlKind::OpenSquare,
        content: TokenControlKind::OpenSquare.into(),
        pos: 4,
      },
    ],
  );
}

#[test]
fn tokenizes_complicated_brackets() {
  run(
    "(())(\"\")(/**/)(\\\\)(\n)(",
    vec![
      Token::BadBracket { pos: 0 },
      Token::Brackets {
        content: "()".into(),
        pos: 1,
        next: 2,
      },
      Token::Control {
        kind: TokenControlKind::CloseParentheses,
        content: TokenControlKind::CloseParentheses.into(),
        pos: 3,
      },
      Token::BadBracket { pos: 4 },
      Token::String {
        content: "\"\"".into(),
        pos: 5,
        next: 6,
      },
      Token::Control {
        kind: TokenControlKind::CloseParentheses,
        content: TokenControlKind::CloseParentheses.into(),
        pos: 7,
      },
      Token::BadBracket { pos: 8 },
      Token::Comment {
        content: "/**/".into(),
        pos: 9,
        next: 12,
      },
      Token::Control {
        kind: TokenControlKind::CloseParentheses,
        content: TokenControlKind::CloseParentheses.into(),
        pos: 13,
      },
      Token::BadBracket { pos: 14 },
      Token::Word {
        content: "\\\\".into(),
        pos: 15,
        next: 16,
      },
      Token::Control {
        kind: TokenControlKind::CloseParentheses,
        content: TokenControlKind::CloseParentheses.into(),
        pos: 17,
      },
      Token::BadBracket { pos: 18 },
      Token::Space {
        content: "\n".into(),
      },
      Token::Control {
        kind: TokenControlKind::CloseParentheses,
        content: TokenControlKind::CloseParentheses.into(),
        pos: 20,
      },
      Token::BadBracket { pos: 21 },
    ],
  );
}

#[test]
fn tokenizes_string() {
  run(
    "'\"'\"\\\"\"",
    vec![
      Token::String {
        content: "'\"'".into(),
        pos: 0,
        next: 2,
      },
      Token::String {
        content: "\"\\\"\"".into(),
        pos: 3,
        next: 6,
      },
    ],
  );
}

#[test]
fn tokenizes_escaped_string() {
  run(
    "\"\\\\\"",
    vec![Token::String {
      content: "\"\\\\\"".into(),
      pos: 0,
      next: 3,
    }],
  );
}

#[test]
fn changes_lines_in_strings() {
  run(
    "\"\n\n\"\"\n\n\"",
    vec![
      Token::String {
        content: "\"\n\n\"".into(),
        pos: 0,
        next: 3,
      },
      Token::String {
        content: "\"\n\n\"".into(),
        pos: 4,
        next: 7,
      },
    ],
  );
}

#[test]
fn tokenizes_at_word() {
  run(
    "@word ",
    vec![
      Token::AtWord {
        content: "@word".into(),
        pos: 0,
        next: 4,
      },
      Token::Space {
        content: " ".into(),
      },
    ],
  );
}

#[test]
fn tokenizes_at_word_end() {
  run(
    "@one{@two()@three\"\"@four;",
    vec![
      Token::AtWord {
        content: "@one".into(),
        pos: 0,
        next: 3,
      },
      Token::Control {
        kind: TokenControlKind::OpenCurly,
        content: TokenControlKind::OpenCurly.into(),
        pos: 4,
      },
      Token::AtWord {
        content: "@two".into(),
        pos: 5,
        next: 8,
      },
      Token::Brackets {
        content: "()".into(),
        pos: 9,
        next: 10,
      },
      Token::AtWord {
        content: "@three".into(),
        pos: 11,
        next: 16,
      },
      Token::String {
        content: "\"\"".into(),
        pos: 17,
        next: 18,
      },
      Token::AtWord {
        content: "@four".into(),
        pos: 19,
        next: 23,
      },
      Token::Control {
        kind: TokenControlKind::Semicolon,
        content: TokenControlKind::Semicolon.into(),
        pos: 24,
      },
    ],
  );
}

#[test]
fn tokenizes_urls() {
  run(
    "url(/*\\))",
    vec![
      Token::Word {
        content: "url".into(),
        pos: 0,
        next: 2,
      },
      Token::Brackets {
        content: "(/*\\))".into(),
        pos: 3,
        next: 8,
      },
    ],
  );
}

#[test]
fn tokenizes_quoted_urls() {
  run(
    "url(\")\")",
    vec![
      Token::Word {
        content: "url".into(),
        pos: 0,
        next: 2,
      },
      Token::BadBracket { pos: 3 },
      Token::String {
        content: "\")\"".into(),
        pos: 4,
        next: 6,
      },
      Token::Control {
        kind: TokenControlKind::CloseParentheses,
        content: TokenControlKind::CloseParentheses.into(),
        pos: 7,
      },
    ],
  );
}

#[test]
fn tokenizes_at_symbol() {
  run(
    "@",
    vec![Token::AtWord {
      content: "@".into(),
      pos: 0,
      next: 0,
    }],
  );
}

#[test]
fn tokenizes_comment() {
  run(
    "/* a\nb */",
    vec![Token::Comment {
      content: "/* a\nb */".into(),
      pos: 0,
      next: 8,
    }],
  );
}

#[test]
fn changes_lines_in_comments() {
  run(
    "a/* \n */b",
    vec![
      Token::Word {
        content: "a".into(),
        pos: 0,
        next: 0,
      },
      Token::Comment {
        content: "/* \n */".into(),
        pos: 1,
        next: 7,
      },
      Token::Word {
        content: "b".into(),
        pos: 8,
        next: 8,
      },
    ],
  );
}

#[test]
fn supports_line_feed() {
  run(
    "a\u{12}b",
    vec![
      Token::Word {
        content: "a".into(),
        pos: 0,
        next: 0,
      },
      Token::Space {
        content: "\u{12}".into(),
      },
      Token::Word {
        content: "b".into(),
        pos: 2,
        next: 2,
      },
    ],
  );
}

#[test]
fn supports_carriage_return() {
  run(
    "a\rb\r\nc",
    vec![
      Token::Word {
        content: "a".into(),
        pos: 0,
        next: 0,
      },
      Token::Space {
        content: "\r".into(),
      },
      Token::Word {
        content: "b".into(),
        pos: 2,
        next: 2,
      },
      Token::Space {
        content: "\r\n".into(),
      },
      Token::Word {
        content: "c".into(),
        pos: 5,
        next: 5,
      },
    ],
  );
}

#[test]
fn tokenizes_css() {
  run(
    "a {\n  content: \"a\";\n  width: calc(1px;)\n  }\n/* small screen */\n@media screen {}",
    vec![
      Token::Word {
        content: "a".into(),
        pos: 0,
        next: 0,
      },
      Token::Space {
        content: " ".into(),
      },
      Token::Control {
        kind: TokenControlKind::OpenCurly,
        content: TokenControlKind::OpenCurly.into(),
        pos: 2,
      },
      Token::Space {
        content: "\n  ".into(),
      },
      Token::Word {
        content: "content".into(),
        pos: 6,
        next: 12,
      },
      Token::Control {
        kind: TokenControlKind::Colon,
        content: TokenControlKind::Colon.into(),
        pos: 13,
      },
      Token::Space {
        content: " ".into(),
      },
      Token::String {
        content: "\"a\"".into(),
        pos: 15,
        next: 17,
      },
      Token::Control {
        kind: TokenControlKind::Semicolon,
        content: TokenControlKind::Semicolon.into(),
        pos: 18,
      },
      Token::Space {
        content: "\n  ".into(),
      },
      Token::Word {
        content: "width".into(),
        pos: 22,
        next: 26,
      },
      Token::Control {
        kind: TokenControlKind::Colon,
        content: TokenControlKind::Colon.into(),
        pos: 27,
      },
      Token::Space {
        content: " ".into(),
      },
      Token::Word {
        content: "calc".into(),
        pos: 29,
        next: 32,
      },
      Token::Brackets {
        content: "(1px;)".into(),
        pos: 33,
        next: 38,
      },
      Token::Space {
        content: "\n  ".into(),
      },
      Token::Control {
        kind: TokenControlKind::CloseCurly,
        content: TokenControlKind::CloseCurly.into(),
        pos: 42,
      },
      Token::Space {
        content: "\n".into(),
      },
      Token::Comment {
        content: "/* small screen */".into(),
        pos: 44,
        next: 61,
      },
      Token::Space {
        content: "\n".into(),
      },
      Token::AtWord {
        content: "@media".into(),
        pos: 63,
        next: 68,
      },
      Token::Space {
        content: " ".into(),
      },
      Token::Word {
        content: "screen".into(),
        pos: 70,
        next: 75,
      },
      Token::Space {
        content: " ".into(),
      },
      Token::Control {
        kind: TokenControlKind::OpenCurly,
        content: TokenControlKind::OpenCurly.into(),
        pos: 77,
      },
      Token::Control {
        kind: TokenControlKind::CloseCurly,
        content: TokenControlKind::CloseCurly.into(),
        pos: 78,
      },
    ],
  );
}

#[test]
#[should_panic(expected = "Unclosed string 1")]
fn throws_error_on_unclosed_string() {
  tokenize(" \"", false);
}

#[test]
#[should_panic(expected = "Unclosed comment 1")]
fn throws_error_on_unclosed_comment() {
  tokenize(" /*", false);
}

#[test]
#[should_panic(expected = "Unclosed bracket 3")]
fn throws_error_on_unclosed_url() {
  tokenize("url(", false);
}

#[test]
fn ignores_unclosing_string_on_request() {
  run_ignore_errors(
    " \"",
    vec![
      Token::Space {
        content: " ".into(),
      },
      Token::String {
        content: "\"".into(),
        pos: 1,
        next: 2,
      },
    ],
  );
}

#[test]
fn ignores_unclosing_comment_on_request() {
  run_ignore_errors(
    " /*",
    vec![
      Token::Space {
        content: " ".into(),
      },
      Token::Comment {
        content: "/*".into(),
        pos: 1,
        next: 3,
      },
    ],
  );
}

#[test]
fn ignores_unclosing_function_on_request() {
  run_ignore_errors(
    "url(",
    vec![
      Token::Word {
        content: "url".into(),
        pos: 0,
        next: 2,
      },
      Token::Brackets {
        content: "(".into(),
        pos: 3,
        next: 3,
      },
    ],
  );
}

#[test]
fn tokenizes_hexadecimal_escape() {
  run(
    "\\0a \\09 \\z ",
    vec![
      Token::Word {
        content: "\\0a".into(),
        pos: 0,
        next: 3,
      },
      Token::Word {
        content: "\\09".into(),
        pos: 4,
        next: 7,
      },
      Token::Word {
        content: "\\z".into(),
        pos: 8,
        next: 9,
      },
      Token::Space {
        content: " ".into(),
      },
    ],
  );
}

#[test]
fn ignore_unclosed_per_token_request() {
  fn tokn(css: &str) -> Vec<Token> {
    let input = Input::new(css.to_string(), None);
    let mut processor = Tokenizer::new(&input, false);
    let mut tokens = vec![];
    while !processor.end_of_file() {
      tokens.push(processor.next_token(true))
    }
    return tokens;
  }

  let tokens = tokn("How's it going (");
  let expected = vec![
    Token::Word {
      content: "How".into(),
      pos: 0,
      next: 2,
    },
    Token::String {
      content: "'s".into(),
      pos: 3,
      next: 4,
    },
    Token::Space {
      content: " ".into(),
    },
    Token::Word {
      content: "it".into(),
      pos: 6,
      next: 7,
    },
    Token::Space {
      content: " ".into(),
    },
    Token::Word {
      content: "going".into(),
      pos: 9,
      next: 13,
    },
    Token::Space {
      content: " ".into(),
    },
    Token::BadBracket { pos: 15 },
  ];
  assert_eq!(tokens, expected);
}

#[test]
fn provides_correct_position() {
  let css = "Three tokens";
  let input = Input::new(css.to_string(), None);
  let mut processor = Tokenizer::new(&input, false);
  assert_eq!(processor.position(), 0);
  processor.next_token(false);
  assert_eq!(processor.position(), 5);
  processor.next_token(false);
  assert_eq!(processor.position(), 6);
  processor.next_token(false);
  assert_eq!(processor.position(), 12);
  // processor.next_token(false);
  // assert_eq!(processor.position(), 12);
}
