use anyhow::{bail, ensure, Context, Result};

use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;
        while let Some(token) = tokens.pop() {
            pos += 1;
            if let Ok(v) = token.parse::<i32>() {
                stack.push(v);
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let ans = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(ans);
            }
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        ensure!(stack.len() == 1, "invalid syntax");
        Ok(stack[0])
    }
}

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
    formula_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);
    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("answer: {}", answer),
            Err(e) => println!("{:#?}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);

        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("-5").unwrap(), -5);

        assert_eq!(calc.eval("10 2 +").unwrap(), 12);
        assert_eq!(calc.eval("-10 2 +").unwrap(), -8);
        assert_eq!(calc.eval("-10 -2 +").unwrap(), -12);

        assert_eq!(calc.eval("10 2 -").unwrap(), 8);
        assert_eq!(calc.eval("-10 2 -").unwrap(), -12);
        assert_eq!(calc.eval("-10 -2 -").unwrap(), -8);

        assert_eq!(calc.eval("10 2 *").unwrap(), 20);
        assert_eq!(calc.eval("-10 2 *").unwrap(), -20);
        assert_eq!(calc.eval("-10 -2 *").unwrap(), 20);

        assert_eq!(calc.eval("10 2 /").unwrap(), 5);
        assert_eq!(calc.eval("-10 2 /").unwrap(), -5);
        assert_eq!(calc.eval("-10 -2 /").unwrap(), 5);
        assert_eq!(calc.eval("10 3 /").unwrap(), 3);
        assert_eq!(calc.eval("0 3 /").unwrap(), 0);

        assert_eq!(calc.eval("10 2 %").unwrap(), 0);
        assert_eq!(calc.eval("-10 2 %").unwrap(), 0);
        assert_eq!(calc.eval("-10 -2 %").unwrap(), 0);
        assert_eq!(calc.eval("10 3 %").unwrap(), 1);
        assert_eq!(calc.eval("0 3 %").unwrap(), 0);
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);

        assert!(calc.eval("1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("1 1 ^").is_err());
        // assert!(calc.eval("3 0 /").is_err());
        // assert!(calc.eval("3 0 %").is_err());
    }
}
