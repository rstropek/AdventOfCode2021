use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");

    let input = fs::read_to_string("./input.txt").unwrap();

    let mut program = String::new();

    program.push_str("pub type Calculator = fn(z: i64, w: i64) -> i64;

    pub const CALCULATORS: [Calculator; 14] = [\n");

    let mut ix = 14usize;
    for line in input.split('\n') {
        match &line[..3] {
            "inp" => {
                if ix < 14 {
                    program.push_str("z\n},\n");

                }
                program.push_str("
                    |mut z: i64, w: i64| {
                        let mut x;
                        let mut y;\n");
                ix -= 1;
            },
            "add" => program.push_str(&format!("{} += {};\n", line.as_bytes()[4] as char, &line[6..])),
            "mul" => {
                let operand = &line[6..];
                match operand {
                    "0" => program.push_str(&format!("{} = 0;\n", line.as_bytes()[4] as char)),
                    _ => program.push_str(&format!("{} *= {};\n", line.as_bytes()[4] as char, operand)),
                }
            },
            "div" => {
                let operand = &line[6..];
                if operand != "1" {
                    program.push_str(&format!("{} /= {};\n", line.as_bytes()[4] as char, &line[6..]));
                }
            },
            "mod" => program.push_str(&format!("{} %= {};\n", line.as_bytes()[4] as char, &line[6..])),
            "eql" => program.push_str(&format!("{0} = if {0} == {1} {{ 1 }} else {{ 0 }};\n", line.as_bytes()[4] as char, &line[6..])),
            _ => panic!()
        }
    }

    program.push_str("z\n}\n];\n");

    fs::write(
        &dest_path,
        program
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
