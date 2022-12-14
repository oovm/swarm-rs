use swarm_parser::{PegParser, VonParser};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let parsed = VonParser::parse(include_str!("basic.swarm")).unwrap();
    println!("{:#?}", parsed);
}
