pub mod ir;
pub mod parser;

use parser::parse;


pub fn transpile(raw_md: &str, target: &str) -> Result<String, String> {
    let ir = parse(raw_md);

    return match target {
        "html" => Ok(ir.to_html()),
        "pdf" => Ok(ir.to_pdf()),
        _ => Err(format!("Unsupported transpilation target '{target}'")),
    };
}
