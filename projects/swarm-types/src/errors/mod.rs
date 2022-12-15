use miette::{ByteOffset, Diagnostic, NamedSource, Report, Result, SourceOffset, SourceSpan};
use thiserror::Error;

pub type SwarmResult<T = (), E = Report> = Result<T, E>;

// You can derive a `Diagnostic` from any `std::error::Error` type.
//
// `thiserror` is a great way to define them, and plays nicely with `miette`!
#[derive(Error, Debug, Diagnostic)]
#[error("Swarm Syntax Error")]
#[diagnostic(code(error::LarvaError), url(docsrs))]
pub struct LarvaError {
    #[source_code]
    src: NamedSource,
    #[label]
    bad_bit: SourceSpan,
    #[help]
    help_message: String,
}

impl LarvaError {
    pub fn new<T, S>(message: S, source: &str, file_name: &str, fail_at: usize) -> Result<T, Self>
    where
        S: Into<String>,
    {
        let start = SourceOffset::from(ByteOffset::from(fail_at));
        Err(LarvaError {
            src: NamedSource::new(file_name, source.to_string()),
            bad_bit: SourceSpan::new(start, start),
            help_message: message.into(),
        })
    }
}
