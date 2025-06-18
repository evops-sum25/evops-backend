impl From<evops_models::FindEventError> for crate::error::Error {
    fn from(value: evops_models::FindEventError) -> Self {
        use evops_models::FindEventError as E;

        match value {
            E::NotFound(_) => Self::NotFound(value.to_string().into()),
            E::Db(_) => Self::InternalServerError(value.to_string().into()),
        }
    }
}

impl From<evops_models::ListEventsError> for crate::error::Error {
    fn from(value: evops_models::ListEventsError) -> Self {
        use evops_models::ListEventsError as E;

        match value {
            E::Db(_) => Self::InternalServerError(value.to_string().into()),
        }
    }
}

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

impl From<evops_models::FindTagError> for crate::error::Error {
    fn from(value: evops_models::FindTagError) -> Self {
        use evops_models::FindTagError as E;

        match value {
            E::NotFound(_) => Self::NotFound(value.to_string().into()),
            E::Db(_) => Self::InternalServerError(value.to_string().into()),
        }
    }
}

impl From<evops_models::ListTagsError> for crate::error::Error {
    fn from(value: evops_models::ListTagsError) -> Self {
        use evops_models::ListTagsError as E;

        match value {
            E::Db(_) => Self::InternalServerError(value.to_string().into()),
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

impl From<evops_models::FindUserError> for crate::error::Error {
    fn from(value: evops_models::FindUserError) -> Self {
        use evops_models::FindUserError as E;

        match value {
            E::NotFound(_) => Self::NotFound(value.to_string().into()),
            E::Db(_) => Self::InternalServerError(value.to_string().into()),
        }
    }
}

impl From<evops_models::ListUsersError> for crate::error::Error {
    fn from(value: evops_models::ListUsersError) -> Self {
        use evops_models::ListUsersError as E;

        match value {
            E::Db(_) => Self::InternalServerError(value.to_string().into()),
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
