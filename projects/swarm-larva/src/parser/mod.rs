use swarm_parser::{PegParser, SwarmParser};
use swarm_types::{LarvaError, SwarmResult};

#[derive(Default)]
pub struct LarvaParser {}

impl LarvaParser {
    pub fn parse(&self, input: &str) -> SwarmResult<()> {
        let parsed = match SwarmParser::parse(input) {
            Ok(o) => o.statements,
            Err(e) => LarvaError::new(e.specifics.to_string(), input, "test.swarm", e.position)?,
        };
        for statement in parsed {
            println!("{:#?}", statement);
        }
        Ok(())
    }
}
