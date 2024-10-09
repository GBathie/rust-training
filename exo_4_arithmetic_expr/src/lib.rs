use std::collections::HashMap;

pub enum ArithmeticExpr {
    Const(i32),
    Operation {
        op: Op,
        left: Box<ArithmeticExpr>,
        right: Box<ArithmeticExpr>,
    },
    Var(usize),
}

pub enum Op {
    Add,
    Mul,
    Sub,
}

impl ArithmeticExpr {
    pub fn from_rpn(rpn: &str) -> Self {
        let mut stack = vec![];

        for s in rpn.split_whitespace() {
            match s {
                op_s if op_s == "+" || op_s == "*" || op_s == "-" => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let op = match op_s {
                        "+" => Op::Add,
                        "*" => Op::Mul,
                        "-" => Op::Sub,
                        _ => unreachable!(),
                    };
                    let op = ArithmeticExpr::Operation {
                        op: op,
                        left: Box::from(left),
                        right: Box::from(right),
                    };
                    stack.push(op);
                }

                x if x.starts_with("x_") => {
                    let id: usize = x.split("_").skip(1).next().unwrap().parse().unwrap();
                    let var = ArithmeticExpr::Var(id);
                    stack.push(var);
                }
                y => {
                    let value = y.parse().unwrap();
                    let cst = ArithmeticExpr::Const(value);
                    stack.push(cst);
                }
            }
        }

        stack.pop().unwrap()
        // todo!("Construct an arithmetic expression from a Reverse Polish Notation string")
    }

    pub fn size(&self) -> usize {
        match self {
            ArithmeticExpr::Const(_) => 1,
            ArithmeticExpr::Operation { left, right, .. } => 1 + left.size() + right.size(),
            ArithmeticExpr::Var(_) => 1,
        }
    }

    pub fn evaluate(&self, vars: &HashMap<usize, i32>) -> i32 {
        match self {
            ArithmeticExpr::Const(x) => *x,
            ArithmeticExpr::Operation { op, left, right } => {
                let l = left.evaluate(vars);
                let r = right.evaluate(vars);
                match op {
                    Op::Add => l + r,
                    Op::Mul => l * r,
                    Op::Sub => l - r,
                }
            }
            ArithmeticExpr::Var(i) => vars.get(i).copied().unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_add() {
        let rpn = "3 4 +";
        let expr = ArithmeticExpr::from_rpn(rpn);

        assert_eq!(expr.size(), 3);
        assert_eq!(expr.evaluate(&HashMap::default()), 7);
    }

    #[test]
    fn mul_var() {
        let rpn = "3 x_1 *";
        let expr = ArithmeticExpr::from_rpn(rpn);

        assert_eq!(expr.size(), 3);
        let mut vars = HashMap::default();
        vars.insert(1, 3);
        assert_eq!(expr.evaluate(&vars), 9);

        vars.insert(1, 5);
        assert_eq!(expr.evaluate(&vars), 15);
    }

    #[test]
    fn sub() {
        let rpn = "7 9 -";
        let expr = ArithmeticExpr::from_rpn(rpn);

        assert_eq!(expr.evaluate(&HashMap::default()), -2);
    }

    #[test]
    fn depth_two() {
        let rpn = "7 9 - 4 +";
        let expr = ArithmeticExpr::from_rpn(rpn);

        assert_eq!(expr.evaluate(&HashMap::default()), 2);
    }

    #[test]
    fn odd_sum() {
        let rpn = "1 3 + 5 + 7 9 + 11 + +";
        let expr = ArithmeticExpr::from_rpn(rpn);

        assert_eq!(expr.size(), 11);
        assert_eq!(expr.evaluate(&HashMap::default()), 36);
    }

    #[test]
    fn many_variables() {
        let rpn = "x_1 1 + x_2 + x_3 2 * *";
        let expr = ArithmeticExpr::from_rpn(rpn);

        let mut vars = HashMap::default();
        vars.insert(1, 8);
        vars.insert(2, 2);
        vars.insert(3, 5);
        assert_eq!(expr.evaluate(&vars), 110);

        vars.insert(1, 984);
        vars.insert(2, 17);
        vars.insert(3, 0);
        assert_eq!(expr.evaluate(&vars), 0);
    }
}
