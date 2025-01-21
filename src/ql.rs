
#[derive(Debug, PartialEq)]
enum QlASTOp {
    // Comparison
    EQ,
    NEQ,

    // Logical
    AND,
    OR,
}

#[derive(Debug, PartialEq)]
enum QlASTLiteral {
    QlText(String),
    QlInteger(u32),
}

#[derive(Debug, PartialEq)]
enum QlExpr {
    // AND | OR
    BinaryLogicalInfix(QlASTOp, Box<QlASTNode>, Box<QlASTNode>),
    // NOT
    UnaryLogical(Box<QlASTNode>),
    // ==
    Eq(Box<QlASTNode>, Box<QlASTNode>),
    //TODO: add arithmetic expressions/operators
}

#[derive(Debug, PartialEq)]
enum QlASTNode {
    QlLiteral(QlASTLiteral),
    QlIdent(String),
    QlOperator(QlASTOp),
    QlNodeExpr(QlExpr),
    //TODO: support closure and functions
}

trait Parser<V> {
    fn parse(&self, input: String) -> Result<(V, String), ()>;
}

impl<F, V> Parser<V> for F
where
    F: Fn(String) -> Result<(V, String), ()>,
{
    fn parse(&self, input: String) -> Result<(V, String), ()> {
        self(input)
    }
}

fn by_char_parser(target: &'static str) -> impl Parser<String> {
    move |inp: String| match inp.chars().nth(0) {
        Some(ch) if ch.to_string() == target.to_string() => {
            Ok((ch.to_string(), inp[1..].to_owned()))
        }
        Some(ch) => {
            println!(
                "invalid char at first position {:?} when parsing {:?}",
                ch, target
            );
            Err(())
        }
        None => {
            println!("end of input when parsing {:?}", target);
            Err(())
        }
    }
}

macro_rules! parser_from_patterns {
    ($fn_name:ident::<$out_ty:ty>, $($patts:pat)|*) => {
        fn $fn_name() -> impl Parser<$out_ty> {
            |inp: String| match inp.chars().nth(0) {
                Some(ch) => match ch {
                    $($patts)|* => Ok((ch.to_string(), inp[1..].to_string())),
                    _ => {
                        println!("char not bound by {} {:?}", stringify!($fn_name), ch);
                        Err(())
                    }
                },
                None => {
                    println!("end of input when use {}", stringify!($fn_name));
                    Err(())
                }
            }
        }
    };
}

parser_from_patterns!(letter_parser::<String>, 'a'..='z' | 'A'..='Z');
parser_from_patterns!(number_parser::<String>, '0'..='9');
parser_from_patterns!(
    symbol_parser::<String>,
    '[' | ']'
        | '{'
        | '}'
        | '('
        | ')'
        | '<'
        | '>'
        | '='
        | '|'
        | '.'
        | ','
        | ';'
        | '-'
        | '+'
        | '*'
        | '?'
        | '\n'
        | '\t'
        | '\r'
);
// | '\'' | '"' |

fn either_parser<V>(p1: impl Parser<V>, p2: impl Parser<V>) -> impl Parser<V> {
    move |inp: String| match p1.parse(inp.clone()) {
        r1 @ Ok(_) => r1,
        Err(_) => p2.parse(inp.clone()),
    }
}

fn seq_parser(p: impl Parser<String>) -> impl Parser<String> {
    move |inp: String| {
        let collected = inp
            .clone()
            .chars()
            .map_while(|next| match p.parse(next.to_string()) {
                Ok((parsed, _)) => Some(parsed),
                _ => None,
            })
            .collect::<Vec<String>>()
            .join("");

        let n = collected.len();
        Ok((collected, inp[n..].to_string()))
    }
}

fn identifier_parser() -> impl Parser<QlASTNode> {
    parser_from_patterns!(ident_sym_parser::<String>, '_' | '@' | '$' | '=');
    let ident_chars_parser = seq_parser(either_parser(ident_sym_parser(), letter_parser()));

    move |inp: String| match ident_chars_parser.parse(inp.clone()) {
        Ok((parsed, rest)) => Ok((QlASTNode::QlIdent(parsed), rest)),
        Err(_) => {
            println!("can't parser identifier sequence {}", inp);
            Err(())
        }
    }
}

fn literal_string_parser() -> impl Parser<QlASTNode> {
    let single_quote_parser = by_char_parser("'");

    //TODO: strict identifiers characters to only allow especial symbols at the start.
    let literal_value_parser = seq_parser(either_parser(
        symbol_parser(),
        either_parser(number_parser(), letter_parser()),
    ));

    move |inp: String| {
        let (_, literal) = single_quote_parser.parse(inp.clone())?;
        match literal_value_parser.parse(literal) {
            Ok((parsed, rest)) => Ok((
                QlASTNode::QlLiteral(QlASTLiteral::QlText(parsed)),
                single_quote_parser.parse(rest)?.1,
            )),
            Err(_) => {
                println!("fail to parse literal {}", inp);
                Err(())
            }
        }
    }
}

fn skip_space() -> impl Parser<()> {
    move |inp: String| match inp.chars().nth(0) {
        Some(ch) if ch == ' ' => Ok(((), inp[1..].to_string())),
        _ => Ok(((), inp)),
    }
}

fn logic_expression_parser() -> impl Parser<QlASTNode> {
    
    let ident_or_literal_parser = either_parser(literal_string_parser(), identifier_parser());
    parser_from_patterns!(ident_sym_parser::<String>, '_' | '@' | '$' | '=');

    move |inp: String| {
        let (expr1, rest1) = ident_or_literal_parser.parse(inp.clone())?;
        if rest1.len() < 1 {
            return Ok((expr1, rest1.clone()));
        }

        let rest2 = skip_space().parse(rest1.clone())?.1;

        match seq_parser(either_parser(ident_sym_parser(), letter_parser())).parse(rest2.clone()) {
            Ok((parsed, rest3)) => {
                if parsed.eq("==") {
                    match logic_expression_parser().parse(rest3[1..].to_string()) {
                        Ok((next_expr, final_rest)) => Ok((
                            QlASTNode::QlNodeExpr(QlExpr::Eq(
                                Box::new(dbg!(expr1)),
                                Box::new(next_expr),
                            )),
                            final_rest,
                        )),
                        _ => {
                            println!(
                                "can't recognized next expression statement {:?}",
                                dbg!(rest3)
                            );
                            return Err(());
                        }
                    }
                } else if parsed.eq("and") {
                    todo!();
                } else {
                    println!(
                        "can't recognized next expression statement parsed: {:?}",
                        parsed
                    );
                    return Err(());
                }
            }
            _ => {
                println!("can't parse next expression {:?}", rest2);
                Err(())
            }
        }
    }
}


fn eval_logic_expr(n: QlASTNode) -> Result<bool, ()> {
    use QlASTNode::*;
    match n {
        QlNodeExpr(eq_node @ QlExpr::Eq(_, _)) => match eq_node {
            // QlExpr::Eq(QlIdent(ident_a), QlIdent(ident_b)) => {
            //     // EVAL identifier into a valide comparable
            //     Ok(ident_a == ident_b)
            // }
            // QlExpr::Eq(QlIdent(ident_a), QlLiteral(lit_a)) => Ok(ident_a == lit_a),
            // QlExpr::Eq(QlLiteral(lit_a), QlLiteral(lit_b)) => Ok(lit_a == lit_b),
            // QlExpr::Eq(node_a, node_b) => Ok(eval(node_a) == eval(node_b)),
            _ => todo!()
        },
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
    letter = "A" | "B" | "C" | "D" | "E" | "F" | "G"
           | "H" | "I" | "J" | "K" | "L" | "M" | "N"
           | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
           | "V" | "W" | "X" | "Y" | "Z" | "a" | "b"
           | "c" | "d" | "e" | "f" | "g" | "h" | "i"
           | "j" | "k" | "l" | "m" | "n" | "o" | "p"
           | "q" | "r" | "s" | "t" | "u" | "v" | "w"
           | "x" | "y" | "z" ;

    digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;

    symbol = "[" | "]" | "{" | "}" | "(" | ")" | "<" | ">"
           | "'" | '"' | "=" | "|" | "." | "," | ";" | "-"
           | "+" | "*" | "?" | "\n" | "\t" | "\r" | "\f" | "\b" ;

    character = letter | digit | symbol | "_";

    Ident = letter | digit | "_";

    Expr ::=
         | Expr + '==' + Expr
         | Expr + 'and' + Expr
         | Value
         | Ident

    "request.method == 'POST' and response.status_code == 200"
                                            OP("AND") 
                              /                                  \
                           /                                        \
                        /                                              \
                    OP("==")                                        OP("==") 
                    /      \                                       /        \
Name("request.method")     Value("POST")  Name("response.status_code")     Value("200")

    */
    #[test]
    fn execution_result() {
        // // let qr = "request.status_code == 200 and json_query(request.body, 'user.email == test@mainator')";
        // let qr = "request.method == 'POST' and response.status_code == 200";
        // let vm = Vm {};
        // let result = vm.execute(String::from(qr));

        // assert_eq!(execute(), Ok(()))
    }

    // #[test]
    // fn eval_logic_expr_test() {
    //     let (expr_node) = logic_expression_parser().parse("'POST' == method".to_string());

    //     assert_eq!(eval_logic_expr(expr_node), Ok(true));
    // }

    #[test]
    fn double_expr_parser_test() {
        let expr1 = "'POST' == method and 'POST' == method".to_string();

        assert_eq!(
            logic_expression_parser().parse(expr1),
            Ok((
                QlASTNode::QlNodeExpr(QlExpr::Eq(
                    Box::new(QlASTNode::QlLiteral(QlASTLiteral::QlText(
                        "POST".to_string()
                    ))),
                    Box::new(QlASTNode::QlIdent("method".to_string()))
                ),),
                "".to_string()
            ))
        );
    }
    #[test]
    fn simple_expr_parser_test() {
        assert_eq!(
            logic_expression_parser().parse("'POST' == method".to_string()),
            Ok((
                QlASTNode::QlNodeExpr(QlExpr::Eq(
                    Box::new(QlASTNode::QlLiteral(QlASTLiteral::QlText(
                        "POST".to_string()
                    ))),
                    Box::new(QlASTNode::QlIdent("method".to_string()))
                ),),
                "".to_string()
            ))
        );

        assert_eq!(
            logic_expression_parser().parse("method == 'POST'".to_string()),
            Ok((
                QlASTNode::QlNodeExpr(QlExpr::Eq(
                    Box::new(QlASTNode::QlIdent("method".to_string())),
                    Box::new(QlASTNode::QlLiteral(QlASTLiteral::QlText(
                        "POST".to_string()
                    ))),
                ),),
                "".to_string()
            ))
        );

        assert_eq!(
            logic_expression_parser().parse("method == 'POST' == response_method".to_string()),
            Ok((
                QlASTNode::QlNodeExpr(QlExpr::Eq(
                    Box::new(QlASTNode::QlIdent("method".to_string())),
                    Box::new(QlASTNode::QlNodeExpr(QlExpr::Eq(
                        Box::new(QlASTNode::QlLiteral(QlASTLiteral::QlText(
                            "POST".to_string()
                        ))),
                        Box::new(QlASTNode::QlIdent("response_method".to_string())),
                    ))),
                ),),
                "".to_string()
            ))
        );
    }

    #[test]
    fn literal_parser_test() {
        let inp = "'POST' == method".to_string();

        assert_eq!(
            literal_string_parser().parse(inp),
            Ok((
                QlASTNode::QlLiteral(QlASTLiteral::QlText("POST".to_string())),
                " == method".to_string()
            ))
        );
    }

    #[test]
    fn identifier_parser_test() {
        let inp = "@method == 'POST' and status_code == 200".to_string();

        assert_eq!(
            identifier_parser().parse(inp),
            Ok((
                QlASTNode::QlIdent("@method".to_string()),
                " == 'POST' and status_code == 200".to_string()
            ))
        );
    }

    #[test]
    fn any_char_parser() {
        let inp = "= 1".to_string();
        let parser = by_char_parser("=");

        assert_eq!(parser.parse(inp), Ok(("=".to_string(), " 1".to_string())))
    }

    #[test]
    fn letter_parser_test() {
        let inp = "abc".to_string();
        let parser = letter_parser();

        assert_eq!(parser.parse(inp), Ok(("a".to_string(), "bc".to_string())));

        let inp = "ABC".to_string();
        let parser = letter_parser();

        assert_eq!(parser.parse(inp), Ok(("A".to_string(), "BC".to_string())));
    }

    #[test]
    fn number_parser_test() {
        let inp = "99".to_string();
        let parser = number_parser();

        assert_eq!(parser.parse(inp), Ok(("9".to_string(), "9".to_string())));
    }

    #[test]
    fn syms_local_parser_test() {
        parser_from_patterns!(parse_syms::<String>, '_' | '@' | '!');

        let inp = "_!@".to_string();
        let parser = seq_parser(parse_syms());

        assert_eq!(parser.parse(inp), Ok(("_!@".to_string(), "".to_string())));
    }

    #[test]
    fn sequence_parser() {
        let inp = "abc dd".to_string();
        let parser = seq_parser(letter_parser());

        assert_eq!(
            parser.parse(inp),
            Ok(("abc".to_string(), " dd".to_string()))
        )
    }

    #[test]
    fn combinator_either() {
        let either_comb = either_parser(letter_parser(), number_parser());

        assert_eq!(
            either_comb.parse("abc 232".to_string()),
            Ok(("a".to_string(), "bc 232".to_string()))
        );
        assert_eq!(
            either_comb.parse("232 abc".to_string()),
            Ok(("2".to_string(), "32 abc".to_string()))
        );
    }
}
