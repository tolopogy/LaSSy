use super::ConstId;
use super::{Ctx, DataType, Env};

use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Term {
    Const(ConstId),
    DBI(u64),
    Universe(Option<u64>),
    App{s: Rc<Term>, t: Rc<Term>},
    Lam(Abs),
    Pi(Abs),
    Let(LetEnv, Rc<Term>),
    Case{t: Rc<Term>, cases: Vec<Abs>},
    Typing{x: Rc<Term>, T: Rc<Term>},
    Nat(::num::BigInt),
    Int(::num::BigInt),
    Str(String),
    Tuple(Vec<Rc<Term>>),
}

#[derive(Clone, Debug)]
pub struct LetEnv {
    consts: Vec<super::Const>,
}
impl From<Env> for LetEnv {
    fn from(env: Env) -> Self { LetEnv {consts: env.consts} }
}
impl<'a> From<&'a Env> for LetEnv {
    fn from(env: &Env) -> Self { LetEnv {consts: env.consts.clone()} }
}

#[derive(Clone, Debug)]
pub struct Abs {
    pub A: Rc<Term>,
    pub t: Rc<Term>,
}
impl Abs {
    pub fn subst(self, i: u64, t: Rc<Term>) -> Abs {
        Abs{A: subst(self.A, i, t.clone()), t: subst(self.t, i+1, t)}
    }

    pub fn norm(self, ctx: &Ctx) -> Abs {
        let mut ctx_t = ctx.clone();
        ctx_t.local.push(self.A.clone());
        Abs{A: norm(ctx, self.A), t: norm(&ctx_t, self.t)}
    }
}

pub fn subst(s: Rc<Term>, i: u64, t: Rc<Term>) -> Rc<Term> {
    match *s.clone() {
        Term::DBI(j) if i == j => t,
        Term::App{s: ref M, t: ref N} =>
            Rc::new(Term::App{s: subst(M.clone(), i, t.clone()), t: subst(N.clone(), i, t)}),
        Term::Lam(ref abs) => Rc::new(Term::Lam(abs.clone().subst(i, t))),
        Term::Pi(ref abs) => Rc::new(Term::Pi(abs.clone().subst(i, t))),
        Term::Let(ref env, ref M) => Rc::new(Term::Let(
            /*Env {
                consts:
                    env.consts.iter().map(|c| match c {
                        super::Const::Def(def) => super::Const::Def( subst(def.clone(), i, t.clone()) ),
                        super::Const::DataType{param_types} =>
                            super::Const::DataType {
                                param_types:
                                    param_types.iter()
                                        .map(|param_type| subst(param_type.clone(), i, t.clone())).collect(),
                            },
                        super::Const::Ctor{datatype, arg_types} =>
                            super::Const::Ctor {
                                datatype: *datatype,
                                arg_types:
                                    arg_types.iter()
                                        .map(|arg_type| subst(arg_type.clone(), i, t.clone())).collect(),
                            },
                    }).collect(),
                typings:
                    env.typings.iter().map( |(M, T)|
                        (subst(M.clone(), i, t.clone()), subst(T.clone(), i, t.clone()))
                    ).collect(),
                module: env.module.clone(),
            }*/ unimplemented!(),
            subst(M.clone(), i, t),
        )),
        Term::Case{t: ref M, ref cases} =>
            Rc::new(Term::Case{
                cases: cases.iter().map(|abs| abs.clone().subst(i, t.clone())).collect(),
                t: subst(M.clone(), i, t)
            }),
        Term::Typing{ref x, ref T} =>
            Rc::new(Term::Typing{x: subst(x.clone(), i, t.clone()), T: subst(T.clone(), i, t)}),
        Term::Tuple(ref ts) =>
            Rc::new(Term::Tuple( ts.iter().map(|M| subst(M.clone(), i, t.clone())).collect() )),
        _ => s,
    }
}

pub fn norm(ctx: &Ctx, t: Rc<Term>) -> Rc<Term> {
    match *t.clone() {
        Term::Const(id) =>
            if let super::Const::Def(ref u) = ctx.global.consts[id as usize] { u.clone() }
            else { t },
        Term::DBI(i) => ctx.local[i as usize].clone(),
        Term::App{s: ref M, t: ref N} => {
            let M = norm(ctx, M.clone());
            let N = norm(ctx, N.clone());
            if let Term::Lam(Abs{ref A, t: ref M}) = *M {
                let M = subst(M.clone(), 0, N);
                norm(ctx, M)
            }
            else { t }
        },
        Term::Lam(ref abs) => Rc::new(Term::Lam(abs.clone().norm(ctx))),
        Term::Pi(ref abs) => Rc::new(Term::Pi(abs.clone().norm(ctx))),
        Term::Let(ref env, ref M) => {
            let mut ctx = ctx.clone();
            ctx.global.extend(unimplemented!()/*env.clone()*/);
            norm(&ctx, M.clone())
        },
        Term::Case{t: ref M, ref cases} => {
            let M = norm(ctx, M.clone());
            unimplemented!()
        },
        Term::Tuple(ref ts) => Rc::new(Term::Tuple( ts.iter().map(|M| norm(ctx, M.clone())).collect() )),
        Term::Typing{..} => unreachable!(),
        _ => t,
    }
}