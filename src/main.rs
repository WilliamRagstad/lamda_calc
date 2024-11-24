use std::collections::HashMap;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod test;

const DARK_GRAY: &str = "\x1b[90m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";
/// Lambda calculus parser using pest
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LambdaCalcParser;

/// AST for lambda calculus
#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Variable(String),
    Assignment(String, Box<Expr>),
    Abstraction(String, Box<Expr>),
    Application(Box<Expr>, Box<Expr>),
}

/// Transform a Pest pair into our own AST Expr node format
fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::variable => Expr::Variable(pair.as_str().to_string()),
        Rule::assignment => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let expr = parse_expr(inner.next().unwrap());
            Expr::Assignment(name, Box::new(expr))
        }
        Rule::abstraction => {
            let mut inner = pair.into_inner();
            let param = inner.next().unwrap().as_str().to_string();
            let body = parse_expr(inner.next().unwrap());
            Expr::Abstraction(param, Box::new(body))
        }
        Rule::application => {
            let mut inner = pair.into_inner();
            let lhs = parse_expr(inner.next().unwrap());
            let rhs = parse_expr(inner.next().unwrap());
            Expr::Application(Box::new(lhs), Box::new(rhs))
        }
        r => unreachable!("Rule {:?} not expected", r),
    }
}

/// Parse a top-level program into a list of expressions
fn parse_prog(input: &str) -> Vec<Expr> {
    let pairs = LambdaCalcParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
    let mut exprs = Vec::new();
    for pair in pairs {
        if let Rule::EOI = pair.as_rule() {
            break;
        }
        exprs.push(parse_expr(pair));
    }
    exprs
}

fn substitute(expr: &Expr, var: &str, value: &Expr) -> Expr {
    match expr {
        Expr::Variable(v) if v == var => value.clone(),
        Expr::Variable(_) => expr.clone(),
        Expr::Assignment(name, val) => {
            Expr::Assignment(name.clone(), Box::new(substitute(val, var, value)))
        }
        Expr::Abstraction(param, body) if param != var => {
            Expr::Abstraction(param.clone(), Box::new(substitute(body, var, value)))
        }
        Expr::Abstraction(_, _) => expr.clone(),
        Expr::Application(f, x) => Expr::Application(
            Box::new(substitute(f, var, value)),
            Box::new(substitute(x, var, value)),
        ),
    }
}

fn inline_variables(expr: Expr, env: &HashMap<String, Expr>) -> Expr {
    match &expr {
        Expr::Variable(v) => env.get(v).cloned().unwrap_or(expr),
        Expr::Assignment(name, val) => {
            Expr::Assignment(name.clone(), Box::new(inline_variables(*val.clone(), env)))
        }
        Expr::Abstraction(param, body) => Expr::Abstraction(
            param.clone(),
            Box::new(inline_variables(*body.clone(), env)),
        ),
        Expr::Application(f, x) => Expr::Application(
            Box::new(inline_variables(*f.clone(), env)),
            Box::new(inline_variables(*x.clone(), env)),
        ),
    }
}

fn eval(expr: &Expr, env: &mut HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Variable(v) => env.get(v).unwrap_or(expr).clone(),
        Expr::Assignment(name, val) => {
            let val = eval(val, env);
            let val = inline_variables(val, env);
            env.insert(name.clone(), val.clone());
            val
        }
        Expr::Abstraction(p, b) => {
            Expr::Abstraction(p.clone(), Box::new(inline_variables(*b.clone(), env)))
        }
        Expr::Application(f, x) => match eval(f, env) {
            Expr::Abstraction(param, body) => eval(&substitute(&body, &param, x), env),
            e => panic!("Expected lambda, found {:?}", e),
        },
    }
}

fn pretty_print(expr: &Expr) -> String {
    fn print_expr(expr: &Expr, top: bool) -> String {
        match expr {
            Expr::Variable(v) => v.clone(),
            Expr::Assignment(name, val) => format!(
                "{}{DARK_GRAY} = {RESET}{}{DARK_GRAY};{RESET}",
                name,
                print_expr(val, false)
            ),
            Expr::Abstraction(param, body) => {
                let body = if matches!(**body, Expr::Application(_, _)) {
                    format!(
                        "{DARK_GRAY}({RESET}{}{DARK_GRAY}){RESET}",
                        print_expr(body, false)
                    )
                } else {
                    print_expr(body, false)
                };
                format!("{YELLOW}λ{RESET}{}{DARK_GRAY}.{RESET}{}", param, body)
            }
            Expr::Application(f, x) => {
                let lhs = if matches!(**f, Expr::Variable(_)) {
                    print_expr(f, false)
                } else {
                    format!(
                        "{DARK_GRAY}({RESET}{}{DARK_GRAY}){RESET}",
                        print_expr(f, false)
                    )
                };
                let rhs = if matches!(**x, Expr::Variable(_)) {
                    print_expr(x, false)
                } else {
                    format!(
                        "{DARK_GRAY}({RESET}{}{DARK_GRAY}){RESET}",
                        print_expr(x, false)
                    )
                };
                if top {
                    format!("{DARK_GRAY}({RESET}{} {}{DARK_GRAY}){RESET}", lhs, rhs)
                } else {
                    format!("{} {}", lhs, rhs)
                }
            }
        }
    }
    print_expr(expr, true)
}

fn run(input: String, env: &mut HashMap<String, Expr>) {
    let exprs = parse_prog(input.replace("\r", "").trim());
    let exprs = exprs
        .into_iter()
        .map(|e| inline_variables(e, env))
        .collect::<Vec<_>>();
    println!(
        "{}",
        exprs
            .iter()
            .map(pretty_print)
            .collect::<Vec<_>>()
            .join("\n")
    );
    let mut exprs = exprs.into_iter();
    let first = exprs.next().expect("No expression found");
    let result = exprs.fold(eval(&first, env), |_, expr| eval(&expr, env));
    println!(
        "{DARK_GRAY}------------------{RESET}\n{}\n",
        pretty_print(&result)
    );
}

fn main() {
    let mut env = HashMap::new();
    // If one argument is given, read that file, otherwise run REPL
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        run(std::fs::read_to_string(&args[1]).unwrap(), &mut env);
    } else {
        use std::io::Write;
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            run(input, &mut env);
        }
    }
}
