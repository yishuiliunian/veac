use veac_lang::error::ErrorKind;
use veac_lang::lexer::Lexer;
use veac_lang::token::TokenKind;

fn lex(input: &str) -> Vec<TokenKind> {
    let mut lexer = Lexer::new(input);
    lexer
        .tokenize()
        .unwrap()
        .into_iter()
        .map(|t| t.kind)
        .collect()
}

#[test]
fn test_keywords() {
    let tokens = lex("project asset timeline track clip text let");
    assert_eq!(
        tokens,
        vec![
            TokenKind::Project,
            TokenKind::Asset,
            TokenKind::Timeline,
            TokenKind::Track,
            TokenKind::Clip,
            TokenKind::Text,
            TokenKind::Let,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_asset_types() {
    let tokens = lex("video audio image");
    assert_eq!(
        tokens,
        vec![
            TokenKind::Video,
            TokenKind::Audio,
            TokenKind::Image,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_time_literals() {
    let tokens = lex("3.5s 500ms 84f");
    assert_eq!(
        tokens,
        vec![
            TokenKind::TimeSec(3.5),
            TokenKind::TimeMs(500.0),
            TokenKind::TimeFrames(84),
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_string_and_smpte() {
    let tokens = lex(r#""hello" "00:01:30:12""#);
    assert_eq!(
        tokens,
        vec![
            TokenKind::StringLit("hello".to_string()),
            TokenKind::Smpte("00:01:30:12".to_string()),
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_color_literal() {
    let tokens = lex("#FFFFFF #FF000080");
    assert_eq!(
        tokens,
        vec![
            TokenKind::ColorLit("FFFFFF".to_string()),
            TokenKind::ColorLit("FF000080".to_string()),
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_numbers() {
    let tokens = lex("42 3.14");
    assert_eq!(
        tokens,
        vec![
            TokenKind::IntLit(42),
            TokenKind::FloatLit(3.14),
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_punctuation() {
    let tokens = lex("{ } ( ) = ,");
    assert_eq!(
        tokens,
        vec![
            TokenKind::LBrace,
            TokenKind::RBrace,
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::Equals,
            TokenKind::Comma,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_comments_skipped() {
    let tokens = lex("project // this is a comment\nasset");
    assert_eq!(
        tokens,
        vec![TokenKind::Project, TokenKind::Asset, TokenKind::Eof]
    );
}

#[test]
fn test_full_asset_decl() {
    let tokens = lex(r#"asset intro = video("./assets/intro.mp4")"#);
    assert_eq!(
        tokens,
        vec![
            TokenKind::Asset,
            TokenKind::Ident("intro".to_string()),
            TokenKind::Equals,
            TokenKind::Video,
            TokenKind::LParen,
            TokenKind::StringLit("./assets/intro.mp4".to_string()),
            TokenKind::RParen,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_unterminated_string() {
    let mut lexer = Lexer::new(r#""hello"#);
    let result = lexer.tokenize();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::UnterminatedString);
}
