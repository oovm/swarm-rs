use std::env::current_dir;

use diagnostic_quick::QResult;
use peginator_codegen::Compile;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn peg_codegen() -> QResult {
    let dir = current_dir()?.join("../swarm-parser/").canonicalize()?;
    Compile::file(dir.join("src/swarm.peg")).destination(dir.join("src/swarm.rs")).format().run().unwrap();
    Ok(())
}
