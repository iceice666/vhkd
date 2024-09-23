
#[derive(thiserror::Error, Debug)]
pub(crate) enum MacOsRuntimeError {
    #[error("Unable to generate new CG event")]
    CGEventError,
}