impl From<evops_types::CreateEventError> for crate::error::Error {
    fn from(value: evops_types::CreateEventError) -> Self {
        use evops_types::CreateEventError as E;
        match value {
            E::Db(e) => Self::InternalServerError(e.into()),
        }
    }
}

impl From<evops_types::CreateTagError> for crate::error::Error {
    fn from(value: evops_types::CreateTagError) -> Self {
        use evops_types::CreateTagError as E;
        match value {
            E::Duplicate(e) => Self::Conflict(e.into()),
            E::Db(e) => Self::InternalServerError(e.into()),
        }
    }
}

impl From<evops_types::CreateUserError> for crate::error::Error {
    fn from(value: evops_types::CreateUserError) -> Self {
        use evops_types::CreateUserError as E;
        match value {
            E::Db(e) => Self::InternalServerError(e.into()),
        }
    }
}
