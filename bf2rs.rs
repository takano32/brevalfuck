use std::io::{self, Read};

const INDENT: &str = "\t";

fn main() {
    let mut src = String::new();
    io::stdin()
        .read_to_string(&mut src)
        .expect("failed to read stdin");

    header(src.contains(','));

    let mut nest = 1usize;
    for ch in src.chars() {
        print_code(ch, &mut nest);
    }

    footer();
}

fn header(uses_input: bool) {
    if uses_input {
        println!(
            r#"use std::io::{{self, Read}};

fn main() {{
	let mut data = vec![0i32];
	let mut index = 0usize;
	let mut input = Vec::new();
	io::stdin().read_to_end(&mut input).unwrap();
	let mut input_index = 0usize;
"#
        );
    } else {
        println!(
            r#"fn main() {{
	let mut data = vec![0i32];
	let mut index = 0usize;
"#
        );
    }
}

fn footer() {
    println!("}}");
}

fn print_code(ch: char, nest: &mut usize) {
    match ch {
        '>' => {
            line(*nest, "// >");
            line(*nest, "index += 1;");
            line(*nest, "if data.len() <= index {");
            *nest += 1;
            line(*nest, "data.push(0);");
            *nest -= 1;
            line(*nest, "}");
            blank();
        }
        '<' => {
            line(*nest, "// <");
            line(*nest, "index -= 1;");
            blank();
        }
        '+' => {
            line(*nest, "// +");
            line(*nest, "data[index] += 1;");
            blank();
        }
        '-' => {
            line(*nest, "// -");
            line(*nest, "data[index] -= 1;");
            blank();
        }
        '.' => {
            line(*nest, "// .");
            line(*nest, r#"print!("{}", data[index] as u8 as char);"#);
            blank();
        }
        ',' => {
            line(*nest, "// ,");
            line(
                *nest,
                "data[index] = if input_index < input.len() { input[input_index] as i32 } else { 0 };",
            );
            line(*nest, "input_index += 1;");
            blank();
        }
        '[' => {
            line(*nest, "// [");
            line(*nest, "while data[index] != 0 {");
            *nest += 1;
            blank();
        }
        ']' => {
            if *nest > 1 {
                *nest -= 1;
            }
            line(*nest, "// ]");
            line(*nest, "}");
            blank();
        }
        _ => {}
    }
}

fn line(nest: usize, text: &str) {
    println!("{}{}", INDENT.repeat(nest), text);
}

fn blank() {
    println!();
}
