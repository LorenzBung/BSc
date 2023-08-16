/// Dieser Datentyp beschreibt die in der Shell auftretenden Fehler.
#[derive(Debug, PartialEq)]
pub enum ErrorType {
    PathError,
    BrokenPipeError,
    ForkError,
}
