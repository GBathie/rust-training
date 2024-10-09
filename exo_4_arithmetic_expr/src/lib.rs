use std::collections::HashMap;

pub enum ArithmeticExpr {
    // Define the structure of an arithmetic expression
}

impl ArithmeticExpr {
    pub fn from_rpn(rpn: &str) -> Self {
        todo!("Construct an arithmetic expression from a Reverse Polish Notation string")
    }

    pub fn size(&self) -> usize {
        todo!("Compute the size of the regex")
    }

    pub fn evaluate(&self, vars: &HashMap<usize, i32>) -> i32 {
        todo!("Compute the value of the expression using the given values for free variables")
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
        assert_eq!(expr.evaluate(&vars), 5);
    }
}
