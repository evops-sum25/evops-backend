use aide::transform::TransformOperation;

use crate::DEFAULT_SECURITY_REQUIREMENT;

pub trait AddResponse {
    /// 400
    fn response_bad_request(self) -> Self;

    /// 401
    fn response_unauthorized(self) -> Self;

    /// 403
    fn response_forbidden(self) -> Self;

    /// 404
    fn response_not_found(self) -> Self;

    /// 409
    fn response_conflict(self) -> Self;

    /// 422
    fn response_unprocessable_entity(self) -> Self;

    /// 500
    fn response_internal_server_error(self) -> Self;
}

impl AddResponse for TransformOperation<'_> {
    fn response_bad_request(self) -> Self {
        self.response_with::<400, String, _>(|r| {
            r.description("Bad Request (e.g. invalid JSON syntax)")
        })
    }

    fn response_unauthorized(self) -> Self {
        self.security_requirement(DEFAULT_SECURITY_REQUIREMENT)
            .response_with::<401, String, _>(|r| r.description("Unauthorized (e.g. invalid JWT)"))
    }

    fn response_forbidden(self) -> Self {
        self.response_with::<403, String, _>(|r| {
            r.description("Forbidden (e.g. modifying another user’s resource)")
        })
    }

    fn response_not_found(self) -> Self {
        self.response_with::<404, String, _>(|r| r.description("Not Found (e.g. nonexistent ID)"))
    }

    fn response_conflict(self) -> Self {
        self.response_with::<409, String, _>(|r| {
            r.description("Conflict (e.g. the entity exists in the database)")
        })
    }

    fn response_unprocessable_entity(self) -> Self {
        self.response_with::<422, String, _>(|r| {
            r.description("Unprocessable Entity (e.g. wrong data types)")
        })
    }

    fn response_internal_server_error(self) -> Self {
        self.response_with::<500, String, _>(|r| r.description("Internal Server Error"))
    }
}
