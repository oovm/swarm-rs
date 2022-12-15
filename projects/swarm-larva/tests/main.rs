use swarm_larva::LarvaParser;
use swarm_types::SwarmResult;

#[test]
fn ready() {
    println!("it works!")
}

#[test]

fn test() -> SwarmResult {
    let parser = LarvaParser::default();
    let parsed = parser.parse(include_str!("basic.swarm")).unwrap();
    println!("{:#?}", parsed);
    Ok(())
}
