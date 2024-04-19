use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest::error::Error;

#[derive(pest_derive::Parser)]
#[grammar = "./equation/calculator.pest"]
pub struct CalculatorParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::infix(pow, Right))
            .op(Op::prefix(unary_minus))
    };
}

#[derive(Debug)]
pub enum Expr {
    Float(f32), // consider benchmarking i32 , f32
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

impl Expr {
    pub fn evaluate(&self) -> f32 {
        match self {
            Expr::Float(n) => *n,
            Expr::UnaryMinus(expr) => -expr.evaluate(),
            Expr::BinOp { lhs, op, rhs } => {
                let lhs_val = lhs.evaluate();
                let rhs_val = rhs.evaluate();
                match op {
                    Op::Add => lhs_val + rhs_val,
                    Op::Subtract => lhs_val - rhs_val,
                    Op::Multiply => lhs_val * rhs_val,
                    Op::Divide => lhs_val / rhs_val,
                    Op::Modulo => lhs_val % rhs_val,
                    Op::Pow => lhs_val.powf(rhs_val),
                }
            }
        }
    }
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::digit => Expr::Float(primary.as_str().parse::<f32>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                Rule::modulo => Op::Modulo,
                Rule::pow => Op::Pow,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Pow,
}

pub fn solve(equation : &str) -> Result<i32 , Error<Rule>> {
    let mut result: f32 = 0.0;

    match CalculatorParser::parse(Rule::equation, equation) {
        Ok(mut pairs) => {
            let expr = parse_expr(pairs.next().unwrap().into_inner());
            result = expr.evaluate(); 
        }
        Err(e) => return Err(e),
    }
    
    // println!("{} = {}" , equation , result);
    Ok(result as i32)
}

pub fn parse_equation(equation : &str , replace : &[i32]) -> Box<str> {
    let mut final_eq = String::new();
    let mut index : i32 = 1;
    let ec = equation.chars();

    // another small parser to make variable-included 
    // equations ready for next level parsing 
    // ( must be combined together with main parser later )
    for c in ec {

        if c == '=' {
            break; 
        }

        if c == ' ' {
            continue; 
        }

        if c.is_ascii_alphabetic() {
            if let Some(e) = replace.get(index.saturating_sub(1) as usize) {
                final_eq.push_str(&format!("*{}", e)); 
            }

        } else {
            final_eq.push_str(&format!("{}" , c));
            continue;
        }

        index += 1;
    }

    final_eq.into_boxed_str()
}