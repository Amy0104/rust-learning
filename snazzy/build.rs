use std::env;
use std::io::Result;

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("The out dir is {}", out_dir);
    prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
    Ok(())
}
