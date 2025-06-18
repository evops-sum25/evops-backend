impl From<evops_models::CreateEventError> for crate::error::Error {
    fn from(value: evops_models::CreateEventError) -> Self {
        use evops_models::CreateEventError as E;
        match value {
            E::AuthorNotFound(e) => Self::UnprocessableEntity(e.to_string().into()),
            E::TagNotFound(e) => Self::UnprocessableEntity(e.to_string().into()),
            E::Db(e) => Self::InternalServerError(e.to_string().into()),
        }
    }
}

impl From<evops_models::CreateTagError> for crate::error::Error {
    fn from(value: evops_models::CreateTagError) -> Self {
        use evops_models::CreateTagError as E;
        match value {
            E::AlreadyExists(e) => Self::Conflict(e.into()),
            E::Db(e) => Self::InternalServerError(e.to_string().into()),
        }
    }
}

impl From<evops_models::CreateUserError> for crate::error::Error {
    fn from(value: evops_models::CreateUserError) -> Self {
        use evops_models::CreateUserError as E;
        match value {
            E::Db(e) => Self::InternalServerError(e.to_string().into()),
        }
    }
}
