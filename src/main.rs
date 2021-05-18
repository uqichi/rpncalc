use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN Program",
    version = "1.0.0",
    author = "my name",
    about = "RPN Calculator"
)]

struct Opts {
    #[clap(short, long)]
    verbose: bool,
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);
    for line in reader.lines() {
        let line = line.unwrap();
        let ans = calc.eval(&line);
        println!("answer: {}", ans);
    }
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();
        while let Some(token) = tokens.pop() {
            if let Ok(v) = token.parse::<i32>() {
                stack.push(v);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let ans = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(ans);
            }
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        if stack.len() == 1 {
            return stack[0];
        }
        panic!("invalid syntax")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);

        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("-5"), -5);

        assert_eq!(calc.eval("10 2 +"), 12);
        assert_eq!(calc.eval("-10 2 +"), -8);
        assert_eq!(calc.eval("-10 -2 +"), -12);

        assert_eq!(calc.eval("10 2 -"), 8);
        assert_eq!(calc.eval("-10 2 -"), -12);
        assert_eq!(calc.eval("-10 -2 -"), -8);

        assert_eq!(calc.eval("10 2 *"), 20);
        assert_eq!(calc.eval("-10 2 *"), -20);
        assert_eq!(calc.eval("-10 -2 *"), 20);

        assert_eq!(calc.eval("10 2 /"), 5);
        assert_eq!(calc.eval("-10 2 /"), -5);
        assert_eq!(calc.eval("-10 -2 /"), 5);
        assert_eq!(calc.eval("10 3 /"), 3);
        assert_eq!(calc.eval("0 3 /"), 0);

        assert_eq!(calc.eval("10 2 %"), 0);
        assert_eq!(calc.eval("-10 2 %"), 0);
        assert_eq!(calc.eval("-10 -2 %"), 0);
        assert_eq!(calc.eval("10 3 %"), 1);
        assert_eq!(calc.eval("0 3 %"), 0);
    }

    #[test]
    #[should_panic]
    fn test_ng_invalid_syntax_1() {
        let calc = RpnCalculator::new(false);
        calc.eval("1 +");
    }

    #[test]
    #[should_panic]
    fn test_ng_invalid_syntax_2() {
        let calc = RpnCalculator::new(false);
        calc.eval("+ 1 1 +");
    }

    #[test]
    #[should_panic]
    fn test_ng_invalid_syntax_3() {
        let calc = RpnCalculator::new(false);
        calc.eval("1 1 1 +");
    }

    #[test]
    #[should_panic]
    fn test_ng_invalid_token() {
        let calc = RpnCalculator::new(false);
        calc.eval("1 1 ^");
    }

    #[test]
    #[should_panic]
    fn test_ng_divisor_of_zero_1() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("3 0 /"), 0);
    }

    #[test]
    #[should_panic]
    fn test_ng_divisor_of_zero_2() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("3 0 %"), 3);
    }
}
