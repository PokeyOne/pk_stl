use std::collections::HashMap;

use crate::StlModel;
use crate::error::{Error, Result};
use crate::geometry::Triangle;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Header(String),
    Keyword(String),
    Float(f32),
}

pub fn parse_ascii_stl(bytes: &[u8]) -> Result<StlModel> {
    let mut tokens = tokenize_ascii_stl(bytes)?.into_iter();

    let header = match tokens.next() {
        Some(Token::Header(header)) => header,
        _ => return Err(Error::ascii("Invalid header"))
    };

    let mut triangles = Vec::new();

    while let Some(t) = tokens.next() {
        if t != Token::Keyword("facet".to_string()) {
            if t == Token::Keyword("endsolid".to_string()) {
                break;
            } else {
                return Err(Error::ascii("Expected facet or endsolid"));
            }
        }

        let normal = parse_normal(&mut tokens)?;
        let vertices = parse_vertices(&mut tokens)?;

        if tokens.next() != Some(Token::Keyword("endfacet".to_string())) {
            return Err(Error::ascii("Expected endfacet keyword"));
        }

        triangles.push(Triangle::from([normal, vertices[0], vertices[1], vertices[2]]));
    }

    Ok(StlModel { header, triangles })
}

fn parse_normal<I>(tokens: &mut I) -> Result<[f32; 3]>
    where I: Iterator<Item = Token>
{
    let mut normal = [0.0; 3];

    if tokens.next() != Some(Token::Keyword("normal".to_string())) {
        return Err(Error::ascii("Expected normal keyword"));
    }

    for i in 0..3 {
        normal[i] = match tokens.next() {
            Some(Token::Float(f)) => f,
            _ => return Err(Error::ascii("Expected normal float"))
        }
    }

    Ok(normal)
}

fn parse_vertices<I>(tokens: &mut I) -> Result<[[f32; 3]; 3]>
    where I: Iterator<Item = Token>
{
    let mut vertices = [[0.0; 3]; 3];

    if tokens.next() != Some(Token::Keyword("outer".to_string())) {
        return Err(Error::ascii("Expected outer keyword"));
    }

    if tokens.next() != Some(Token::Keyword("loop".to_string())) {
        return Err(Error::ascii("Expected loop keyword"));
    }

    for i in 0..3 {
        if tokens.next() != Some(Token::Keyword("vertex".to_string())) {
            return Err(Error::ascii("Expected vertex keyword"));
        }

        for j in 0..3 {
            vertices[i][j] = match tokens.next() {
                Some(Token::Float(f)) => f,
                _ => return Err(Error::ascii("Expected vertex float"))
            }
        }
    }

    if tokens.next() != Some(Token::Keyword("endloop".to_string())) {
        return Err(Error::ascii("Expected endloop keyword"));
    }

    Ok(vertices)
}

fn tokenize_ascii_stl(bytes: &[u8]) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();

    let mut data = bytes.into_iter();

    let solid_keyword = data.by_ref().take(6).map(|val| { *val }).collect::<Vec<u8>>();
    if solid_keyword != b"solid " {
        return Err(Error::ascii("Model must start with 'solid ' keyword"));
    }

    let mut data = data.map(|val| { *val as char }).peekable();

    let mut header = String::new();

    while let Some(c) = data.next() {
        match c {
            '\0' | '\r' | '\n' => break,
            c => header.push(c)
        }
    }

    tokens.push(Token::Header(header));

    // Now parse the rest of the tokens dynamically
    let keyword_regex = KeywordRegex::compile(&[
        "facet",
        "outer",
        "loop",
        "vertex",
        "normal",
        "endloop",
        "endfacet",
        "endsolid"
    ]);

    loop {
        println!("Starting loop, next char is {:?}", data.peek());
        // Skip whitespace
        if let Some(c) = data.peek() {
            if c.is_whitespace() {
                println!("Skipping whitespace");
                data.next();
                continue;
            }
        }

        // Look for numbers in sign-mantissa-e-sign-exponent format
        if let Some(c) = data.peek() {
            if c.is_ascii_digit() || *c == '-' || *c == '+' || *c == '.' {
                let mut number = String::new();

                while let Some(c) = data.peek() {
                    if c.is_ascii_digit() || *c == '-' || *c == '+' || *c == '.' || *c == 'e' || *c == 'E' {
                        number.push(*c);
                        data.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Float(number.parse::<f32>().map_err(|_| { Error::ascii("Invalid float") })?));
                continue;
            }
        }

        // Look for keywords
        if let Some(keyword) = keyword_regex.find(&mut data) {
            let endsolid = keyword == "endsolid";

            tokens.push(Token::Keyword(keyword));

            if endsolid {
                break;
            }
            continue;
        }

        // If we get here, we've reached the end of the file
        if data.peek().is_none() {
            break;
        } else {
            return Err(Error::ascii(format!("Unexpected character: {:?}", data.next()).as_str()));
        }
    }

    Ok(tokens)
}

#[derive(Debug, Clone)]
struct KeywordRegex {
    root: KwNode
}

#[derive(Debug, Clone)]
enum KwNode {
    Branch(HashMap<char, KwNode>),
}

impl KeywordRegex {
    pub fn find<I>(&self, chars: &mut I) -> Option<String>
        where I: Iterator<Item = char>
    {
        self.root.find(chars)
    }

    pub fn compile(keywords: &[&str]) -> KeywordRegex {
        let mut root = KwNode::Branch(HashMap::new());

        for keyword in keywords {
            root.add(keyword);
        }

        KeywordRegex { root }
    }
}

impl KwNode {
    pub fn find<I>(&self, chars: &mut I) -> Option<String>
        where I: Iterator<Item = char>
    {
        match self {
            KwNode::Branch(map) if map.is_empty() => Some(String::new()),
            KwNode::Branch(map) => {
                if let Some(c) = chars.next() {
                    if let Some(node) = map.get(&c) {
                        node.find(chars).and_then(|mut s| {
                            s.insert(0, c);
                            Some(s)
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn add(&mut self, keyword: &str) {
        let mut chars = keyword.chars();

        if let Some(c) = chars.next() {
            match self {
                KwNode::Branch(map) => {
                    let node = map.entry(c).or_insert_with(|| {
                        KwNode::Branch(HashMap::new())
                    });

                    node.add(&chars.collect::<String>());
                }
            }
        }

        // If the string is empty we can do nothing because an empty branch
        // is treated as a leaf node.
    }
}