#[derive(thiserror::Error, Debug)]
pub enum MacOsRuntimeError {
    #[error("Unable to generate new CG event")]
    CGEventError,
}
