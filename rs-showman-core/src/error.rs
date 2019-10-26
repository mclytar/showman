use actix_web::HttpResponse;
use serde::Serialize;
use actix_web::dev::HttpResponseBuilder;

#[allow(non_snake_case)]
macro_rules! STATIC_RESP {
    ($name:ident, $status:expr) => {
        #[allow(non_snake_case, missing_docs)]
        pub fn $name() -> ErrorBuilder {
            ErrorBuilder {
                description: $status.to_owned(),
                http_response_builder: HttpResponse::$name()
            }
        }
    };
}

pub type Result<OK> = std::result::Result<OK, Error>;

#[derive(Debug)]
pub struct ErrorBuilder {
    description: String,
    http_response_builder: HttpResponseBuilder
}

impl ErrorBuilder {
    pub fn text(mut self, value: &str) -> Error {
        Error {
            description: self.description,
            http_response: self.http_response_builder.body(value.to_owned())
        }
    }

    pub fn json<T>(mut self, value: T) -> Error
        where
            T: Serialize
    {
        Error {
            description: self.description,
            http_response: self.http_response_builder.json(value)
        }
    }

    pub fn json_ref<T>(mut self, value: &T) -> Error
        where
            T: Serialize
    {
        Error {
            description: self.description,
            http_response: self.http_response_builder.json2(value)
        }
    }

    pub fn finish(mut self) -> Error {
        Error {
            description: self.description,
            http_response: self.http_response_builder.finish()
        }
    }
}

impl Into<HttpResponse> for ErrorBuilder {
    fn into(mut self) -> HttpResponse {
        self.http_response_builder.finish()
    }
}

impl std::error::Error for ErrorBuilder {
    fn description(&self) -> &str {
        &self.description[..]
    }
}

impl std::fmt::Display for ErrorBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", &self.description)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Error {
    description: String,
    http_response: HttpResponse
}

impl Error {
    STATIC_RESP!(BadRequest, "bad request");
    STATIC_RESP!(Unauthorized, "unauthorized");
    STATIC_RESP!(PaymentRequired, "payment required");
    STATIC_RESP!(Forbidden, "forbidden");
    STATIC_RESP!(NotFound, "not found");
    STATIC_RESP!(MethodNotAllowed, "method not allowed");
    STATIC_RESP!(NotAcceptable, "not acceptable");
    STATIC_RESP!(ProxyAuthenticationRequired, "proxy authentication required");
    STATIC_RESP!(RequestTimeout, "request timeout");
    STATIC_RESP!(Conflict, "conflict");
    STATIC_RESP!(Gone, "gone");
    STATIC_RESP!(LengthRequired, "length required");
    STATIC_RESP!(PreconditionFailed, "precondition failed");
    STATIC_RESP!(PreconditionRequired, "precondition required");
    STATIC_RESP!(PayloadTooLarge, "payload too large");
    STATIC_RESP!(UriTooLong, "uri too long");
    STATIC_RESP!(UnsupportedMediaType, "unsupported media type");
    STATIC_RESP!(RangeNotSatisfiable, "range not satisfiable");
    STATIC_RESP!(ExpectationFailed, "expectation failed");
    STATIC_RESP!(UnprocessableEntity, "unprocessable entity");
    STATIC_RESP!(TooManyRequests, "too many requests");

    STATIC_RESP!(InternalServerError, "internal server error");
    STATIC_RESP!(NotImplemented, "not implemented");
    STATIC_RESP!(BadGateway, "bad gateway");
    STATIC_RESP!(ServiceUnavailable, "service unavailable");
    STATIC_RESP!(GatewayTimeout, "gateway timeout");
    STATIC_RESP!(VersionNotSupported, "http version not supported");
    STATIC_RESP!(VariantAlsoNegotiates, "variant also negotiates");
    STATIC_RESP!(InsufficientStorage, "insufficient storage");
    STATIC_RESP!(LoopDetected, "loop detected");
}

impl From<ErrorBuilder> for Error {
    fn from(mut e: ErrorBuilder) -> Self {
        Error {
            description: e.description,
            http_response: e.http_response_builder.finish()
        }
    }
}

impl Into<HttpResponse> for Error {
    fn into(self) -> HttpResponse {
        self.http_response
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.description[..]
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", &self.description)?;
        Ok(())
    }
}