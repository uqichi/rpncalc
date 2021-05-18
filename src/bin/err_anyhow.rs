use anyhow::{Context, Result};

fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path)
        // with_contextを使った場合、Closureで渡すことでエラーが発生しなかったときにその文字列を生成する処理を実行せずに済む
        .with_context(|| format!("failed to read string from {}", path))?;
    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        // contextを使った場合、必ずその文字列を生成してしまう
        .context("failed to parse string to a number")
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}
