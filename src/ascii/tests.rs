use super::*;

#[test]
fn test_single_keyword_regex() {
    let regex = KeywordRegex::compile(&["foo"]);
    println!("{:?}", regex);

    assert_eq!(
        regex.find(&mut "foo".chars()),
        Some("foo".to_string())
    );
}

#[test]
fn test_multiple_keyword_regex() {
    let regex = KeywordRegex::compile(&["foo", "bar", "baz"]);
    println!("{:?}", regex);

    assert_eq!(
        regex.find(&mut "foo".chars()),
        Some("foo".to_string())
    );
    assert_eq!(
        regex.find(&mut "bar".chars()),
        Some("bar".to_string())
    );
    assert_eq!(
        regex.find(&mut "baz".chars()),
        Some("baz".to_string())
    );
}

#[test]
fn test_keyword_regex_only_takes_what_it_needs() {
    let regex = KeywordRegex::compile(&["foo", "bar", "baz"]);
    println!("{:?}", regex);

    let mut foobar_iterator = "foobar".chars();
    assert_eq!(
        regex.find(&mut foobar_iterator),
        Some("foo".to_string())
    );
    assert_eq!(foobar_iterator.next(), Some('b'));

    assert_eq!(
        regex.find(&mut "barbaz".chars()),
        Some("bar".to_string())
    );
    assert_eq!(
        regex.find(&mut "bazfoo".chars()),
        Some("baz".to_string())
    );
}

#[test]
fn test_tokenize() {
    let src = b"solid foo
facet normal 0.0 0.0 1.0
  outer loop
    vertex 0.0 0.0 5.0
    vertex 1.0 0.0 5.0
    vertex 0.0 1.0 5.0
  endloop
endfacet
endsolid foo";

    let tokens = tokenize_ascii_stl(src).unwrap();

    let expected_tokens = vec![
        Token::Header("foo".to_string()),
        Token::Keyword("facet".to_string()),
        Token::Keyword("normal".to_string()),
        Token::Float(0.0),
        Token::Float(0.0),
        Token::Float(1.0),
        Token::Keyword("outer".to_string()),
        Token::Keyword("loop".to_string()),
        Token::Keyword("vertex".to_string()),
        Token::Float(0.0),
        Token::Float(0.0),
        Token::Float(5.0),
        Token::Keyword("vertex".to_string()),
        Token::Float(1.0),
        Token::Float(0.0),
        Token::Float(5.0),
        Token::Keyword("vertex".to_string()),
        Token::Float(0.0),
        Token::Float(1.0),
        Token::Float(5.0),
        Token::Keyword("endloop".to_string()),
        Token::Keyword("endfacet".to_string()),
        Token::Keyword("endsolid".to_string())
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_parse_stl() {
    let src = b"solid foo
facet normal 0.0 0.0 1.0
    outer loop
        vertex 0.0 0.0 5.0
        vertex 1.0 0.0 5.0
        vertex 0.0 1.0 5.0
    endloop
endfacet
endsolid foo";

    let stl = parse_ascii_stl(src).unwrap();

    let expected_stl = StlModel {
        header: "foo".to_string(),
        triangles: vec![
            Triangle::from(
                [
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, 5.0],
                    [1.0, 0.0, 5.0],
                    [0.0, 1.0, 5.0],
                ]
            )
        ],
    };

    assert_eq!(stl, expected_stl);
}