use syntax::Loc;
use std::fmt;
use super::ext;

#[derive(Clone, Debug)]
pub enum Term {
    Ident(Ident),
    Universe,
    AppChain{f: TermWithLoc, args: Vec<(TermWithLoc, bool)>},
    Pi{x: Ident, A: TermWithLoc, B: TermWithLoc, implicit: bool},
    Arrow{A: TermWithLoc, B: TermWithLoc},
    // Infix(Infix),
    Typing(Typing),
    // ArgTyping(TermWithLoc),
    Let{env: Env, body: TermWithLoc},
    Lam{x: Ident, A: TermWithLoc, t: TermWithLoc},
    Case{t: TermWithLoc, arms: Vec<(Arm, Loc)>},
    If{p: TermWithLoc, tv: TermWithLoc, fv: TermWithLoc},
    Ext(ext::ExtTerm),
    Hole(Option<Ident>),
}
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Ident(i) => write!(f, "{}", i),
            Term::Universe => write!(f, "type"),
            Term::AppChain{f:_f, args} => {
                write!(f, "{}", _f)?;
                for (arg, implicit) in args {
                    if *implicit {
                        write!(f, " {{{}}}", arg)?;
                    }
                    else {
                        write!(f, " {}", arg)?;
                    }
                }
                Ok(())
            },
            Term::Pi{x,A,B, implicit} => if *implicit {
                write!(f, "({{{}:{}}} -> {})", x, A, B)
            }
            else {
                write!(f, "(({}:{}) -> {})", x, A, B)
            },
            Term::Arrow{A,B} => write!(f, "({} -> {})", A, B),
            // Term::Infix(i) => write!(f, "({})", i),
            Term::Typing(ty) => write!(f, "({})", ty),
            // Term::ArgTyping(t) => write!(f, "'{}", t),
            Term::Let{env, body} => write!(f, "(let {} in {})", env, body),
            Term::Lam{x,A,t} => write!(f, "(\\{}:{}->{})", x, A, t),
            Term::Case{t, arms} => {
                write!(f, "(case {} of ", t)?;
                for i in 0..arms.len() {
                    write!(f, "{}", arms[i].0)?;
                    if i < arms.len()-1 { write!(f, "; ")?; }
                }
                write!(f, ")")
            },
            Term::If{p, tv, fv} => write!(f, "(if {} then {} else {})", p, tv, fv),
            Term::Ext(et) => write!(f, "{}", et),
            Term::Hole(i) => write!(f, "?{}", if let Some(i) = i { format!("{}", i) } else { "".into() }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TermWithLoc {
    pub term: Box<Term>,
    pub start: usize,
    pub end: usize,
}
impl fmt::Display for TermWithLoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", self.term)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ident {
    pub domain: Vec<String>,
    pub name: String,
    pub loc: Loc,
}
impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for domain_name in &self.domain {
            write!(f, "{}::", domain_name)?;
        }
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug)]
pub struct Op(pub String);

#[derive(Clone, Debug)]
pub struct Infix {
    pub head: TermWithLoc,
    pub tail: Vec<(Op, TermWithLoc)>,
}
impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.head)?;
        for (op, t) in &self.tail {
            write!(f, " {} {}", op.0, t)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Arm {
    pub patn: TermWithLoc,
    pub t: TermWithLoc,
}
impl fmt::Display for Arm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", self.patn, self.t)
    }
}

#[derive(Clone, Debug)]
pub struct Typing {
    pub x: TermWithLoc,
    pub T: TermWithLoc,
}
impl fmt::Display for Typing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.x, self.T)
    }
}

#[derive(Clone, Debug)]
pub struct Env(pub Vec<(Statement, Loc)>);
impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Env(stats) = self;
        for i in 0..stats.len() {
            write!(f, "{}", stats[i].0)?;
            if i < stats.len()-1 { write!(f, "; ")?; }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Module {
    pub env: Env,
    pub name: String,
    pub children: Vec<Box<Module>>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Datatype{header: TermWithLoc, ctors: Vec<TermWithLoc>},
    // DefInfix{op: Op, name: Ident},
    // InfixPrio{head: Ident, tail: Vec<Ident>},
    Def(TermWithLoc, TermWithLoc),
    Typing(Typing),
    Module{header: TermWithLoc, env: Env},
}
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Datatype{header, ctors} => {
                write!(f, "datatype {} {{", header)?;
                for i in 0..ctors.len() {
                    let ref ctor = ctors[i];
                    write!(f, " {} ", ctor)?;
                    if i < ctors.len()-1 { write!(f, "|")?; }
                }
                write!(f, "}}")
            },
            // Statement::DefInfix{op: Op(op), name} => write!(f, "infix {} {}", op, name),
            /* Statement::InfixPrio{head, tail} => {
                write!(f, "infix_prio {}", head)?;
                for x in tail { write!(f, " {}", x)?; }
                Ok(())
            }, */
            Statement::Def(l,r) => write!(f, "{} := {}", l, r),
            Statement::Typing(Typing{x,T}) => write!(f, "{} : {}", x, T),
            Statement::Module{header, env} => write!(f, "module {} {{\n{}\n}}", header, env),
        }
    }
}