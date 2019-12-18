// https://atcoder.jp/contests/APG4b/tasks/APG4b_bw

use itertools::Itertools as _;
use maplit::hashmap;
use matches::matches;

use std::collections::HashMap;
use std::io::{self, Read as _};
use std::str::FromStr;

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut env = hashmap!();
    for line in input.lines().skip(1) {
        line.parse::<Stmt>().unwrap().eval(&mut env);
    }
}

#[derive(Debug)]
enum Stmt {
    DeclInt(char, IntExpr),
    PrintInt(IntExpr),
    DeclVec(char, VecExpr),
    PrintVec(VecExpr),
}

impl Stmt {
    fn eval(&self, env: &mut HashMap<char, Val>) {
        match self {
            Self::DeclInt(name, expr) => {
                env.insert(*name, Val::Int(expr.eval(env)));
            }
            Self::PrintInt(expr) => println!("{}", expr.eval(env)),
            Self::DeclVec(name, expr) => {
                env.insert(*name, Val::Vec(expr.eval(env)));
            }
            Self::PrintVec(expr) => println!("[ {} ]", expr.eval(env).iter().format(" ")),
        }
    }
}

impl FromStr for Stmt {
    type Err = (String, usize, nom::error::ErrorKind);

    fn from_str(input: &str) -> Result<Self, (String, usize, nom::error::ErrorKind)> {
        use nom::branch::alt;
        use nom::bytes::complete::{tag, take_while_m_n};
        use nom::character::complete::{char, one_of, space0, space1};
        use nom::combinator::{map, map_res};
        use nom::multi::{fold_many0, separated_list};
        use nom::sequence::{pair, preceded, tuple};
        use nom::IResult;

        fn decl_int(input: &str) -> IResult<&str, Stmt> {
            let (input, _) = space0(input)?;
            let (input, _) = tag("int")(input)?;
            let (input, _) = space1(input)?;
            let (input, name) = var_name(input)?;
            let (input, _) = space0(input)?;
            let (input, _) = tag("=")(input)?;
            let (input, _) = space0(input)?;
            let (input, expr) = int_expr(input)?;
            let (input, _) = space0(input)?;
            let (input, _) = char(';')(input)?;
            Ok((input, Stmt::DeclInt(name, expr)))
        }

        fn print_int(input: &str) -> IResult<&str, Stmt> {
            let (input, _) = space0(input)?;
            let (input, _) = tag("print_int")(input)?;
            let (input, _) = space1(input)?;
            let (input, expr) = int_expr(input)?;
            let (input, _) = space0(input)?;
            let (input, _) = tag(";")(input)?;
            Ok((input, Stmt::PrintInt(expr)))
        }

        fn decl_vec(input: &str) -> IResult<&str, Stmt> {
            let (input, _) = space0(input)?;
            let (input, _) = tag("vec")(input)?;
            let (input, _) = space1(input)?;
            let (input, name) = var_name(input)?;
            let (input, _) = space0(input)?;
            let (input, _) = char('=')(input)?;
            let (input, _) = space0(input)?;
            let (input, val) = vec_expr(input)?;
            let (input, _) = space0(input)?;
            let (input, _) = char(';')(input)?;
            Ok((input, Stmt::DeclVec(name, val)))
        }

        fn print_vec(input: &str) -> IResult<&str, Stmt> {
            let (input, _) = space0(input)?;
            let (input, _) = tag("print_vec")(input)?;
            let (input, _) = space1(input)?;
            let (input, val) = vec_expr(input)?;
            let (input, _) = space0(input)?;
            let (input, _) = char(';')(input)?;
            Ok((input, Stmt::PrintVec(val)))
        }

        fn int_expr(input: &str) -> IResult<&str, IntExpr> {
            let (input, expr) = int_term(input)?;
            fold_many0(
                preceded(space0, pair(one_of("+-"), preceded(space0, int_term))),
                expr,
                |expr, (op, term)| match op {
                    '+' => IntExpr::Add(Box::new(expr), Box::new(term)),
                    '-' => IntExpr::Sub(Box::new(expr), Box::new(term)),
                    _ => unreachable!(),
                },
            )(input)
        }

        fn int_term(input: &str) -> IResult<&str, IntExpr> {
            let (input, _) = space0(input)?;
            alt((
                map(var_name, IntExpr::Var),
                map(
                    take_while_m_n::<_, &str, _>(1, 1, |c| matches!(c, '0'..='9')),
                    |s| IntExpr::Lit(s.parse().unwrap()),
                ),
            ))(input)
        }

        fn vec_expr(input: &str) -> IResult<&str, VecExpr> {
            let (input, expr) = vec_term(input)?;
            fold_many0(
                preceded(space0, pair(one_of("+-"), preceded(space0, vec_term))),
                expr,
                |expr, (op, term)| match op {
                    '+' => VecExpr::Add(Box::new(expr), Box::new(term)),
                    '-' => VecExpr::Sub(Box::new(expr), Box::new(term)),
                    _ => unreachable!(),
                },
            )(input)
        }

        fn vec_term(input: &str) -> IResult<&str, VecExpr> {
            let (input, _) = space0(input)?;
            alt((map(var_name, VecExpr::Var), |input| {
                let (input, _) = char('[')(input)?;
                let (input, _) = space0(input)?;
                let (input, vec) =
                    separated_list(tuple((space0, char(','), space0)), int_expr)(input)?;
                let (input, _) = space0(input)?;
                let (input, _) = char(']')(input)?;
                Ok((input, VecExpr::Lit(vec)))
            }))(input)
        }

        fn var_name(input: &str) -> IResult<&str, char> {
            map_res(take_while_m_n(1, 1, |c| matches!(c, 'a'..='z')), str::parse)(input)
        }

        decl_int(input)
            .or_else(|_| print_int(input))
            .or_else(|_| decl_vec(input))
            .or_else(|_| print_vec(input))
            .map(|(_, stmt)| stmt)
            .map_err(|err| match err {
                nom::Err::Incomplete(_) => unreachable!(),
                nom::Err::Error((s, k)) | nom::Err::Failure((s, k)) => {
                    (input.to_owned(), input.len() - s.len(), k)
                }
            })
    }
}

#[derive(Debug)]
enum Val {
    Int(i32),
    Vec(Vec<i32>),
}

impl Val {
    fn unwrap_int(&self) -> i32 {
        match self {
            Self::Int(n) => *n,
            Self::Vec(_) => panic!(),
        }
    }

    fn unwrap_vec(&self) -> &[i32] {
        match self {
            Self::Int(_) => panic!(),
            Self::Vec(vec) => vec,
        }
    }
}

#[derive(Debug, Clone)]
enum IntExpr {
    Lit(i32),
    Var(char),
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
}

impl IntExpr {
    fn eval(&self, env: &HashMap<char, Val>) -> i32 {
        match self {
            Self::Lit(n) => *n,
            Self::Var(s) => env[s].unwrap_int(),
            Self::Add(l, r) => l.eval(env) + r.eval(env),
            Self::Sub(l, r) => l.eval(env) - r.eval(env),
        }
    }
}

#[derive(Debug, Clone)]
enum VecExpr {
    Lit(Vec<IntExpr>),
    Var(char),
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
}

impl VecExpr {
    fn eval(&self, env: &HashMap<char, Val>) -> Vec<i32> {
        match self {
            Self::Lit(v) => v.iter().map(|x| x.eval(env)).collect(),
            Self::Var(s) => env[s].unwrap_vec().to_owned(),
            Self::Add(l, r) => {
                let (l, r) = (l.eval(env), r.eval(env));
                l.into_iter().zip_eq(r).map(|(l, r)| l + r).collect()
            }
            Self::Sub(l, r) => {
                let (l, r) = (l.eval(env), r.eval(env));
                l.into_iter().zip_eq(r).map(|(l, r)| l - r).collect()
            }
        }
    }
}
