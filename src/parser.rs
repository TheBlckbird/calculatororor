use crate::{
    ast::{BinOp, Expr, UnOp},
    token::Token,
};

pub fn parse(input: &str) -> Result<Expr, &'static str> {
    let tokens = match lexer(input) {
        Ok(tokens) => tokens,
        Err(_) => return Err("lexer"),
    };
    let mut index = 0usize;
    parse_expr(&tokens, &mut index)

    // let mut ast = Vec::new();

    // loop {
    //     if index >= tokens.len() - 1 {
    //         break Ok(ast);
    //     }

    //     index += 1;
    // }
}

fn parse_expr(tokens: &Vec<Token>, index: &mut usize) -> Result<Expr, &'static str> {
    let token = &tokens[tokens.len() - *index - 1];

    match token {
        Token::Number(num) => {
            let left = Expr::Number(*num);
            parse_bin_op(tokens, index, left)
        }
        Token::Sub => {
            *index += 1;
            let left = if let Token::Number(num) = tokens[tokens.len() - *index - 1] {
                Expr::UnOp(Box::new(Expr::Number(num)), UnOp::Neg)
            } else {
                return Err("not a number after sub");
            };

            parse_bin_op(tokens, index, left)
        }
        _ => Err("not the right token"),
    }
}

fn parse_bin_op(
    tokens: &Vec<Token>,
    index: &mut usize,
    mut right: Expr,
) -> Result<Expr, &'static str> {
    // if let Token::Number(num) = tokens[*index] {
    //     return Ok(Expr::Number(num));
    // }

    *index += 1;

    if *index >= tokens.len() {
        return Ok(right);
    }

    let op = match tokens[tokens.len() - *index - 1] {
        Token::Add => BinOp::Add,
        Token::Sub => {
            let maybe_next_token =
                tokens.get((tokens.len() as isize - *index as isize - 2) as usize);

            let next_token = match maybe_next_token {
                Some(next_token) => next_token,
                None => return Ok(Expr::UnOp(Box::new(right), UnOp::Neg)),
            };

            if let Token::Number(_) = next_token {
                BinOp::Sub
            } else {
                right = Expr::UnOp(Box::new(right), UnOp::Neg);

                *index += 1;
                match tokens[tokens.len() - *index - 1] {
                    Token::Add => BinOp::Add,
                    Token::Sub => BinOp::Sub,
                    Token::Div => BinOp::Div,
                    Token::Mul => BinOp::Mul,
                    Token::Mod => BinOp::Mod,
                    Token::Pow => BinOp::Pow,
                    Token::Number(_) => unreachable!(),
                }
            }
        }
        Token::Div => BinOp::Div,
        Token::Mul => BinOp::Mul,
        Token::Mod => BinOp::Mod,
        Token::Pow => BinOp::Pow,
        Token::Number(_) => return Err("why is there a number?"),
    };

    *index += 1;

    let left = parse_expr(tokens, index)?;

    Ok(Expr::BinOp {
        left: Box::new(left),
        right: Box::new(right),
        op,
    })
}

fn lexer(input: &str) -> Result<Vec<Token>, ()> {
    let mut index = 0;
    let mut tokens = Vec::new();

    loop {
        match input.chars().nth(index) {
            Some(char) => match char {
                '+' => tokens.push(Token::Add),
                '-' => tokens.push(Token::Sub),
                '*' => tokens.push(Token::Mul),
                '/' => tokens.push(Token::Div),
                '%' => tokens.push(Token::Mod),
                '^' => tokens.push(Token::Pow),
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | '.' => {
                    let token = match parse_num(input, &mut index) {
                        Some(token) => token,
                        None => break Err(()),
                    };

                    tokens.push(token);
                }

                ' ' | '\n' | '\r' => {}
                _ => break Err(()),
            },
            None => break Ok(tokens),
        }

        index += 1;
    }
}

fn parse_num(input: &str, index: &mut usize) -> Option<Token> {
    // *index -= 1;
    let token = Token::Number(parse_num_rec(input, index, String::new())?.parse().unwrap());

    *index -= 1;
    Some(token)
}

fn parse_num_rec(input: &str, index: &mut usize, mut num_for_now: String) -> Option<String> {
    let char = input.chars().nth(*index);
    match char {
        Some(char) => match char {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | '.' => {
                *index += 1;
                num_for_now.push(char);
                parse_num_rec(input, index, num_for_now)
            }
            _ => Some(num_for_now),
        },
        None => Some(num_for_now),
    }
}
