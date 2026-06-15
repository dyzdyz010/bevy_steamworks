use std::fmt::{self, Debug, Display};

/// Owned command failure returned by `Steamworks*Result::into_result`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksCommandError<C, E> {
    /// Command that failed.
    pub command: C,
    /// Failure reason.
    pub error: E,
}

impl<C, E> SteamworksCommandError<C, E> {
    /// Creates an owned command failure.
    #[must_use]
    pub fn new(command: C, error: E) -> Self {
        Self { command, error }
    }

    /// Returns the failed command.
    #[must_use]
    pub fn command(&self) -> &C {
        &self.command
    }

    /// Returns the failure reason.
    #[must_use]
    pub fn error(&self) -> &E {
        &self.error
    }

    /// Splits this failure into the failed command and error.
    #[must_use]
    pub fn into_parts(self) -> (C, E) {
        (self.command, self.error)
    }
}

impl<C, E> Display for SteamworksCommandError<C, E>
where
    C: Debug,
    E: Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Steamworks command {:?} failed: {}",
            self.command, self.error
        )
    }
}

impl<C, E> std::error::Error for SteamworksCommandError<C, E>
where
    C: Debug,
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

macro_rules! impl_steamworks_result_helpers {
    ($result:ident, $operation:ty, $command:ty, $error:ty) => {
        impl $result {
            /// Returns true when this result contains a successful operation.
            #[must_use]
            pub fn is_ok(&self) -> bool {
                matches!(self, Self::Ok(_))
            }

            /// Returns true when this result contains a failed command.
            #[must_use]
            pub fn is_err(&self) -> bool {
                matches!(self, Self::Err { .. })
            }

            /// Returns the successful operation, when this result is successful.
            #[must_use]
            pub fn operation(&self) -> Option<&$operation> {
                match self {
                    Self::Ok(operation) => Some(operation),
                    Self::Err { .. } => None,
                }
            }

            /// Returns the failed command, when this result is an error.
            #[must_use]
            pub fn command(&self) -> Option<&$command> {
                match self {
                    Self::Ok(_) => None,
                    Self::Err { command, .. } => Some(command),
                }
            }

            /// Returns the failure reason, when this result is an error.
            #[must_use]
            pub fn error(&self) -> Option<&$error> {
                match self {
                    Self::Ok(_) => None,
                    Self::Err { error, .. } => Some(error),
                }
            }

            /// Converts this message into a borrowed standard [`Result`].
            #[must_use = "inspect the borrowed result or handle the command failure"]
            pub fn as_result(&self) -> Result<&$operation, (&$command, &$error)> {
                match self {
                    Self::Ok(operation) => Ok(operation),
                    Self::Err { command, error } => Err((command, error)),
                }
            }

            /// Converts this message into a standard [`Result`] with a boxed command failure.
            #[must_use = "inspect the result or handle the boxed command failure"]
            pub fn into_result(
                self,
            ) -> Result<$operation, Box<crate::SteamworksCommandError<$command, $error>>> {
                match self {
                    Self::Ok(operation) => Ok(operation),
                    Self::Err { command, error } => {
                        Err(Box::new(crate::SteamworksCommandError::new(command, error)))
                    }
                }
            }
        }
    };
}

pub(crate) use impl_steamworks_result_helpers;
