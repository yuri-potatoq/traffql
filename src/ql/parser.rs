// Query language BNF Grammar
//
// letter = "A" | "B" | "C" | "D" | "E" | "F" | "G"
//        | "H" | "I" | "J" | "K" | "L" | "M" | "N"
//        | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
//        | "V" | "W" | "X" | "Y" | "Z" | "a" | "b"
//        | "c" | "d" | "e" | "f" | "g" | "h" | "i"
//        | "j" | "k" | "l" | "m" | "n" | "o" | "p"
//        | "q" | "r" | "s" | "t" | "u" | "v" | "w"
//        | "x" | "y" | "z" ;
//
// digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
//
// symbol = "[" | "]" | "{" | "}" | "(" | ")" | "<" | ">"
//        | "'" | '"' | "=" | "|" | "." | "," | ";" | "-"
//        | "+" | "*" | "?" | "\n" | "\t" | "\r" | "\f" | "\b" ;
//
// character = letter | digit | symbol | "_";
//
// Ident = letter | digit | "_";
//
// OP = '==' | 'AND' | '.'
//
// Expr ::=
//      | Expr + '==' + Expr
//      | Expr + 'AND' + Expr
//      | Ident + '.' + Ident  // access operator
//      | Value
//      | Ident
//

#[derive(Debug, PartialEq)]
enum QlASTOp {
    // Comparison
    EQ,  // ==
    NEQ, // !=
    LT,  // <
    GT,  // >

    // Logical
    AND,
    OR,
}

#[derive(Debug, PartialEq)]
enum QlASTLiteral {
    QlText(String),
    //TODO: also parse floats
    QlNumber(u32),
}

#[derive(Debug, PartialEq)]
enum QlExpr {
    // AND | OR
    BinaryInfix(QlASTOp, Box<QlASTNode>, Box<QlASTNode>),
    // NOT
    UnaryLogical(QlASTOp, Box<QlASTNode>),
    Access(Box<QlASTNode>, Vec<Box<QlASTNode>>),

    // Single value expression as literal(integer/string) or identifier.
    Term(Box<QlASTNode>),
}

#[derive(Debug, PartialEq)]
enum QlASTNode {
    QlLiteral(QlASTLiteral),
    QlIdent(String),
    QlNodeExpr(QlExpr),
    //TODO: support closure and functions
}

trait Parser<'s, V> {
    fn parse(&self, input: &'s str) -> Result<(V, &'s str), ()>;
}

impl<'s, F, V> Parser<'s, V> for F
where
    F: Fn(&'s str) -> Result<(V, &'s str), ()>,
{
    fn parse(&self, input: &'s str) -> Result<(V, &'s str), ()> {
        self(input)
    }
}

fn by_char_parser<'s>(target: &'static str) -> impl Parser<'s, char> {
    move |inp: &'s str| {
        let target_char = if let Some(c) = target.chars().next() {
            c
        } else {
            println!("can't parse target char {:?}", target);
            return Err(());
        };

        match inp.chars().next() {
            Some(ch) if target_char.eq(&ch) => {
                let rest = &inp[ch.len_utf8()..];
                Ok((ch, rest))
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
}

fn prefix_parser<'s, P, S>(
    prefix_parser: impl Parser<'s, P>,
    parser: impl Parser<'s, S>,
) -> impl Parser<'s, S> {
    move |inp: &'s str| match prefix_parser.parse(inp) {
        Ok((_, rest)) => parser.parse(rest),
        Err(()) => {
            println!("can't parse prefix");
            Err(())
        }
    }
}

macro_rules! parser_from_patterns {
    ($fn_name:ident::<$out_ty:ty>, $($patts:pat)|*) => {
        fn $fn_name<'s>() -> impl Parser<'s, char> {
            |inp: &'s str| match inp.chars().next() {
                Some(ch) => match ch {
                    $($patts)|* => Ok((ch, &inp[ch.len_utf8()..])),
                    _ => {
                        println!("[{}] char not bound by {:?}", stringify!($fn_name), ch);
                        Err(())
                    }
                },
                None => {
                    println!("[{}] end of input when use", stringify!($fn_name));
                    Err(())
                }
            }
        }
    };
}

parser_from_patterns!(letter_parser::<char>, 'a'..='z' | 'A'..='Z');
parser_from_patterns!(number_parser::<char>, '0'..='9');
parser_from_patterns!(
    symbol_parser::<char>,
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

fn either_parser<'s, V>(p1: impl Parser<'s, V>, p2: impl Parser<'s, V>) -> impl Parser<'s, V> {
    move |inp: &'s str| match p1.parse(inp) {
        r1 @ Ok(_) => r1,
        Err(_) => p2.parse(inp),
    }
}

fn seq_parser_chars<'s>(p: impl Parser<'s, char>) -> impl Parser<'s, String> {
    move |inp: &'s str| {
        let mut cursor = 0;
        let mut collected = String::new();

        for c in inp.chars() {
            let inp_slice = &inp[cursor..cursor + c.len_utf8()];

            match p.parse(inp_slice) {
                Ok((parsed, _)) => {
                    cursor += c.len_utf8();
                    collected.push(parsed);
                }
                _ => break,
            }
        }

        let n = collected.len();
        Ok((collected, &inp[n..]))
    }
}

fn identifier_parser<'s>() -> impl Parser<'s, QlASTNode> {
    parser_from_patterns!(ident_sym_parser::<String>, '_' | '@' | '$' | '=');
    let ident_chars_parser = seq_parser_chars(either_parser(ident_sym_parser(), letter_parser()));

    move |inp: &'s str| match ident_chars_parser.parse(inp) {
        Ok((parsed, rest)) => Ok((QlASTNode::QlIdent(parsed), rest)),
        Err(_) => {
            println!("can't parser identifier sequence {}", inp);
            Err(())
        }
    }
}

/// Return a parser which parse a literal string from input.
///
/// Given the input:
/// ```text
/// 'POST' == request.method
/// ```
///
/// Will return:
/// ```text
/// Ok(
///     QlASTNode::QlLiteral(
///         QlASTLiteral::QlText('POST'),
///     ),
///     " == request.method"
/// )
/// ```
fn string_literal_parser<'s>() -> impl Parser<'s, QlASTNode> {
    let single_quote_parser = by_char_parser("'");

    //TODO: strict identifiers characters to only allow especial symbols at the start.
    let literal_value_parser = seq_parser_chars(either_parser(
        symbol_parser(),
        either_parser(number_parser(), letter_parser()),
    ));

    move |inp: &'s str| {
        let (_, literal) = single_quote_parser.parse(inp)?;
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

fn number_literal_parser<'s>() -> impl Parser<'s, QlASTNode> {
    //TODO: parse float pointing numbers
    let literal_value_parser = seq_parser_chars(number_parser());

    move |inp: &'s str| match literal_value_parser.parse(inp) {
        Ok((parsed, rest)) => {
            let number = parsed.parse::<u32>().map_err(|err| {
                println!("fail to parse number {}: {}", parsed, err);
            })?;

            Ok((QlASTNode::QlLiteral(QlASTLiteral::QlNumber(number)), rest))
        }
        Err(_) => {
            println!("fail to parse literal {}", inp);
            Err(())
        }
    }
}

fn skip_space<'s>() -> impl Parser<'s, ()> {
    move |inp: &'s str| match inp.chars().next() {
        Some(ch) if ch == ' ' => Ok(((), &inp[ch.len_utf8()..])),
        _ => Ok(((), inp)),
    }
}

fn logic_op_parser<'s>() -> impl Parser<'s, QlASTOp> {
    |inp: &'s str| {
        let mut chars = inp.chars();
        match (chars.next(), chars.next(), chars.next()) {
            (Some('A'), Some('N'), Some('D')) => Ok((QlASTOp::AND, &inp[3..])),
            (Some('O'), Some('R'), None) => Ok((QlASTOp::OR, &inp[2..])),
            _ => {
                println!("[logic_operator_parser] can't parse as logic operator");
                Err(())
            }
        }
    }
}

fn boolean_op_parser<'s>() -> impl Parser<'s, QlASTOp> {
    |inp: &'s str| {
        if inp.len() >= 2 {
            match &inp[..2] {
                "==" => return Ok((QlASTOp::EQ, &inp[2..])),
                "!=" => return Ok((QlASTOp::NEQ, &inp[2..])),
                _ => {}
            };
        }

        if inp.len() > 0 {
            match &inp[..1] {
                "<" => return Ok((QlASTOp::LT, &inp[1..])),
                ">" => return Ok((QlASTOp::GT, &inp[1..])),
                _ => {}
            };
        }

        println!("[boolean_op_parser] can't parse as boolean operator");
        Err(())
    }
}

/// Logic Expression BNF:
/// ```text
/// EXPR        ::= LOGIC_EXPR
///
/// LOGIC_EXPR  ::= LOGIC_EXPR "AND" COMP_EXPR
///               | LOGIC_EXPR "OR" COMP_EXPR
///               | COMP_EXPR
///
/// COMP_EXPR   ::= COMP_EXPR "==" TERM
///               | COMP_EXPR "!=" TERM
///               | COMP_EXPR "<" TERM
///               | COMP_EXPR ">" TERM
///               | TERM
///
/// TERM        ::= PATH
///               | LITERAL
///               | IDENT
///               | "(" EXPR ")"
///
/// PATH        ::= IDENT (("." IDENT) | ("[" NUMBER "]"))*
/// ```
fn expression_parser<'s>() -> impl Parser<'s, QlASTNode> {
    let ident_or_literal_parser = either_parser(
        number_literal_parser(),
        either_parser(string_literal_parser(), identifier_parser()),
    );

    move |inp: &'s str| {
        let (left_expr, rest) = ident_or_literal_parser.parse(inp)?;

        let rest = skip_space().parse(rest)?.1;
        let infix_operator_parser = either_parser(logic_op_parser(), boolean_op_parser());

        match infix_operator_parser.parse(rest) {
            Ok((op, rest)) => {
                let (rigth_expr, rest) = expression_parser().parse(skip_space().parse(rest)?.1)?;
                return Ok((
                    QlASTNode::QlNodeExpr(QlExpr::BinaryInfix(
                        op,
                        Box::new(left_expr),
                        Box::new(rigth_expr),
                    )),
                    skip_space().parse(rest)?.1,
                ));
            }
            _ => Ok((
                QlASTNode::QlNodeExpr(QlExpr::Term(Box::new(dbg!(left_expr)))),
                rest,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_literals() {
        use QlASTNode::*;

        assert_eq!(
            number_literal_parser().parse("32"),
            Ok((QlLiteral(QlASTLiteral::QlNumber(32)), "")),
        );

        assert_eq!(
            number_literal_parser().parse("1"),
            Ok((QlLiteral(QlASTLiteral::QlNumber(1)), "")),
        );

        assert_eq!(
            number_literal_parser().parse("1 22"),
            Ok((QlLiteral(QlASTLiteral::QlNumber(1)), " 22")),
        );
    }

    #[test]
    fn simple_binary_expr_parser_test() {
        use QlASTNode::*;
        use QlExpr::*;

        assert_eq!(
            expression_parser().parse("'POST' == method"),
            Ok((
                QlNodeExpr(BinaryInfix(
                    QlASTOp::EQ,
                    Box::new(QlLiteral(QlASTLiteral::QlText("POST".to_string()))),
                    Box::new(QlNodeExpr(Term(Box::new(QlIdent("method".to_string())))))
                )),
                ""
            ))
        );

        assert_eq!(
            expression_parser().parse("method == 'POST' == response_method"),
            Ok((
                QlNodeExpr(BinaryInfix(
                    QlASTOp::EQ,
                    Box::new(QlIdent("method".to_string())),
                    Box::new(QlNodeExpr(BinaryInfix(
                        QlASTOp::EQ,
                        Box::new(QlLiteral(QlASTLiteral::QlText("POST".to_string()))),
                        Box::new(QlNodeExpr(Term(Box::new(QlIdent(
                            "response_method".to_string()
                        )))))
                    )),)
                )),
                ""
            ))
        );

        assert_eq!(
            expression_parser().parse("method == 'POST' AND status_code == 200"),
            Ok((
                QlNodeExpr(BinaryInfix(
                    QlASTOp::EQ,
                    Box::new(QlIdent("method".to_string())),
                    Box::new(QlNodeExpr(BinaryInfix(
                        QlASTOp::AND,
                        Box::new(QlLiteral(QlASTLiteral::QlText("POST".to_string()))),
                        Box::new(QlNodeExpr(BinaryInfix(
                            QlASTOp::EQ,
                            Box::new(QlIdent("status_code".to_string())),
                            Box::new(QlNodeExpr(Term(Box::new(QlLiteral(
                                QlASTLiteral::QlNumber(200)
                            )))))
                        )))
                    )))
                )),
                ""
            ))
        );
    }

    #[test]
    fn logic_op_parser_test() {
        assert_eq!(logic_op_parser().parse("AND"), Ok((QlASTOp::AND, "")));

        assert_eq!(logic_op_parser().parse("OR"), Ok((QlASTOp::OR, "")));

        assert_eq!(logic_op_parser().parse(""), Err(()));
    }

    #[test]
    fn boolean_op_parser_test() {
        assert_eq!(boolean_op_parser().parse(""), Err(()));

        assert_eq!(boolean_op_parser().parse("=="), Ok((QlASTOp::EQ, "")));

        assert_eq!(boolean_op_parser().parse("!="), Ok((QlASTOp::NEQ, "")));

        assert_eq!(boolean_op_parser().parse("<"), Ok((QlASTOp::LT, "")));

        assert_eq!(boolean_op_parser().parse(">"), Ok((QlASTOp::GT, "")));
    }

    #[test]
    fn string_literal_parser_test() {
        let inp = "'POST' == method";

        assert_eq!(
            string_literal_parser().parse(inp),
            Ok((
                QlASTNode::QlLiteral(QlASTLiteral::QlText("POST".to_string())),
                " == method"
            ))
        );
    }

    #[test]
    fn identifier_parser_test() {
        let inp = "@method == 'POST' and status_code == 200";

        assert_eq!(
            identifier_parser().parse(inp),
            Ok((
                QlASTNode::QlIdent("@method".to_string()),
                " == 'POST' and status_code == 200"
            ))
        );
    }

    #[test]
    fn any_char_parser() {
        let inp = "= 1";
        let parser = by_char_parser("=");

        assert_eq!(parser.parse(inp), Ok(("=".chars().next().unwrap(), " 1")))
    }

    #[test]
    fn letter_parser_test() {
        let inp = "abc";
        let parser = letter_parser();

        assert_eq!(parser.parse(inp), Ok(("a".chars().next().unwrap(), "bc")));

        let inp = "ABC";
        let parser = letter_parser();

        assert_eq!(parser.parse(inp), Ok(("A".chars().next().unwrap(), "BC")));
    }

    #[test]
    fn number_parser_test() {
        let inp = "99";
        let parser = number_parser();

        assert_eq!(parser.parse(inp), Ok(("9".chars().next().unwrap(), "9")));
    }

    #[test]
    fn syms_local_parser_test() {
        parser_from_patterns!(parse_syms::<String>, '_' | '@' | '!');

        let inp = "_!@";
        let parser = seq_parser_chars(parse_syms());

        assert_eq!(parser.parse(inp), Ok(("_!@".to_string(), "")));
    }

    #[test]
    fn sequence_parser() {
        let inp = "abc dd";
        let parser = seq_parser_chars(letter_parser());

        assert_eq!(Ok(("abc".to_string(), " dd")), parser.parse(inp))
    }

    #[test]
    fn combinator_either() {
        let either_comb = either_parser(letter_parser(), number_parser());

        assert_eq!(
            either_comb.parse("abc 232"),
            Ok(("a".chars().next().unwrap(), "bc 232"))
        );
        assert_eq!(
            either_comb.parse("232 abc"),
            Ok(("2".chars().next().unwrap(), "32 abc"))
        );
    }

    #[test]
    fn parse_access_operator() {
        let either_comb = either_parser(letter_parser(), number_parser());

        assert_eq!(
            either_comb.parse("abc 232"),
            Ok(("a".chars().next().unwrap(), "bc 232"))
        );
        assert_eq!(
            either_comb.parse("232 abc"),
            Ok(("2".chars().next().unwrap(), "32 abc"))
        );
    }

    #[test]
    fn parse_with_prefix() {
        let signed_num_parser =
            prefix_parser(by_char_parser("+"), seq_parser_chars(number_parser()));
        let input = "+1111";

        assert_eq!(signed_num_parser.parse(input), Ok(("1111".to_string(), "")),)
    }
}
