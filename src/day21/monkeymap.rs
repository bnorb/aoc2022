use std::collections::HashMap;

#[derive(Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn parse(input: &str) -> Self {
        match input {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        }
    }

    fn do_it(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
        }
    }

    fn inverse(&self) -> Self {
        match self {
            Self::Add => Self::Sub,
            Self::Sub => Self::Add,
            Self::Mul => Self::Div,
            Self::Div => Self::Mul,
        }
    }
}

impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Self::Add => String::from("+"),
            Self::Sub => String::from("-"),
            Self::Mul => String::from("*"),
            Self::Div => String::from("/"),
        }
    }
}

enum Monkey<'a> {
    Simple(&'a str, i64),
    Compound(&'a str, &'a str, &'a str, Op),
    Human(&'a str),
    Root(&'a str, &'a str, &'a str),
}

impl<'a> Monkey<'a> {
    fn sides(&self) -> (&'a str, &'a str) {
        match self {
            Self::Compound(_, lhs, rhs, _) => (lhs, rhs),
            Self::Root(_, lhs, rhs) => (lhs, rhs),
            _ => unimplemented!(),
        }
    }
}

impl<'a> ToString for Monkey<'a> {
    fn to_string(&self) -> String {
        match self {
            Monkey::Compound(name, lhs, rhs, op) => {
                format!("{name}: {lhs} {} {rhs}", op.to_string())
            }
            Monkey::Simple(name, val) => format!("{name}: {val}"),
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone)]
pub enum Equation {
    Num(i64),
    Var(String),
    Eq(Box<Equation>, Box<Equation>, Op),
    X,
}

impl Equation {
    fn left_eq(&self, res: &Equation) -> Self {
        match self {
            Self::Eq(_, rhs, op) => Equation::Eq(Box::new(res.clone()), rhs.clone(), op.inverse()),
            _ => unimplemented!(),
        }
    }

    fn right_eq(&self, res: &Equation) -> Self {
        match self {
            Self::Eq(lhs, _, op) => match op {
                Op::Add | Op::Mul => Equation::Eq(Box::new(res.clone()), lhs.clone(), op.inverse()),
                Op::Sub | Op::Div => Equation::Eq(lhs.clone(), Box::new(res.clone()), op.clone()),
            },
            _ => unimplemented!(),
        }
    }

    pub fn substitute_var(&self, var: &str, val: &Equation) -> Option<Equation> {
        if let Self::Eq(lhs, rhs, op) = self {
            let checked = *lhs.clone();
            match checked {
                Equation::Var(v) if v == var => {
                    return Some(Self::Eq(Box::new(val.clone()), rhs.clone(), op.clone()))
                }
                Equation::Eq(..) => {
                    if let Some(eq) = checked.substitute_var(var, val) {
                        return Some(Self::Eq(Box::new(eq), rhs.clone(), op.clone()));
                    }
                }
                _ => {}
            }

            let checked = *rhs.clone();
            match checked {
                Equation::Var(v) if v == var => {
                    return Some(Self::Eq(lhs.clone(), Box::new(val.clone()), op.clone()))
                }
                Equation::Eq(..) => {
                    if let Some(eq) = checked.substitute_var(var, val) {
                        return Some(Self::Eq(lhs.clone(), Box::new(eq), op.clone()));
                    }
                }
                _ => unreachable!(),
            }
        }

        unreachable!()
    }

    pub fn calc(&self) -> i64 {
        match self {
            Equation::Eq(lhs, rhs, op) => op.do_it(lhs.calc(), rhs.calc()),
            Equation::Num(val) => *val,
            _ => unimplemented!("can't calc that"),
        }
    }
}

impl ToString for Equation {
    fn to_string(&self) -> String {
        match self {
            Self::X => String::from("X"),
            Self::Num(val) => val.to_string(),
            Self::Var(var) => String::from(var),
            Self::Eq(lhs, rhs, op) => format!(
                "({} {} {})",
                lhs.to_string(),
                op.to_string(),
                rhs.to_string()
            ),
        }
    }
}

pub struct MonkeyMap<'a> {
    map: HashMap<&'a str, Monkey<'a>>,
}

impl<'a> MonkeyMap<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut map = HashMap::new();
        input.lines().for_each(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let (name, yell) = (parts[0], parts[1]);

            let monkey = if let Ok(num) = yell.parse() {
                Monkey::Simple(name, num)
            } else {
                let parts: Vec<&str> = yell.split(' ').collect();
                Monkey::Compound(name, parts[0], parts[2], Op::parse(parts[1]))
            };

            map.insert(name, monkey);
        });

        Self { map }
    }

    pub fn correct(&mut self) {
        let root = self.map.get_mut("root").unwrap();
        let (lhs, rhs) = root.sides();
        *root = Monkey::Root("root", lhs, rhs);

        let humn = self.map.get_mut("humn").unwrap();
        *humn = Monkey::Human("humn");
    }

    pub fn get_val(&self, name: &'a str) -> Result<i64, ()> {
        let monkey = self.map.get(name).unwrap();
        match monkey {
            Monkey::Simple(_, num) => Ok(*num),
            Monkey::Compound(_, m1, m2, op) => {
                let num1 = self.get_val(m1)?;
                let num2 = self.get_val(m2)?;

                Ok(op.do_it(num1, num2))
            }
            Monkey::Human(_) => Err(()),
            _ => unimplemented!(),
        }
    }

    pub fn calc_half(&mut self) -> (String, i64) {
        let root = self.map.get("root").unwrap();
        let (lhs, rhs) = root.sides();

        if let Ok(val) = self.get_val(lhs) {
            let monkey = self.map.get_mut(lhs).unwrap();
            *monkey = Monkey::Simple(lhs, val);
            return (String::from(rhs), val);
        }

        let val = self.get_val(rhs).unwrap();
        let monkey = self.map.get_mut(rhs).unwrap();
        *monkey = Monkey::Simple(rhs, val);

        (String::from(lhs), val)
    }

    pub fn build_humn_equation(&self, node: &str) -> Equation {
        let monkey = self.map.get(node).unwrap();
        match monkey {
            Monkey::Simple(_, val) => Equation::Num(*val),
            Monkey::Human(_) => Equation::X,
            Monkey::Compound(_, left_node, right_node, op) => {
                let left = self.build_humn_equation(left_node);
                let right = self.build_humn_equation(right_node);

                let node_eq = Equation::Var(String::from(node));
                let eq = Equation::Eq(Box::new(left.clone()), Box::new(right.clone()), op.clone());

                match (left, right) {
                    (Equation::Num(lhs), Equation::Num(rhs)) => Equation::Num(op.do_it(lhs, rhs)),
                    (Equation::X, _) => eq.left_eq(&node_eq),
                    (_, Equation::X) => eq.right_eq(&node_eq),
                    (Equation::Num(_), rhs) => {
                        let req = eq.right_eq(&node_eq);
                        rhs.substitute_var(right_node, &req).unwrap()
                    }
                    (lhs, Equation::Num(_)) => {
                        let leq = eq.left_eq(&node_eq);
                        lhs.substitute_var(left_node, &leq).unwrap()
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}
