// HTTP response status codes


#[allow(dead_code)]
#[derive(Debug)]
pub enum Status {
    // 1xx
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,
    // 2xx
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,
    // 3xx
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    TemporaryRedirect,
    PermanentRedirect,
    // 4xx
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableContent,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    // 5xx
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired
}

impl Into<&str> for Status {
    fn into(self) -> &'static str {
        match self {
            // 1xx
            Self::Continue => "100 Continue",
            Self::SwitchingProtocols => "101 Switching Protocols",
            Self::Processing => "102 Processing",
            Self::EarlyHints => "103 Early Hints",

            // 2xx
            Self::Ok => "200 Ok",
            Self::Created => "201 Created",
            Self::Accepted => "202 Accepted",
            Self::NonAuthoritativeInformation => "203 Non Authoritative Information",
            Self::NoContent => "204 No Content",
            Self::ResetContent => "205 Reset Content",
            Self::PartialContent => "206 Partial Content",
            Self::MultiStatus => "207 Multi Status",
            Self::AlreadyReported => "208 Already Reported",
            Self::IMUsed => "226 IM Used",

            // 3xx
            Self::MultipleChoices => "300 Multiple Choices",
            Self::MovedPermanently => "301 Moved Permanently",
            Self::Found => "302 Found",
            Self::SeeOther => "303 See Other",
            Self::NotModified => "304 Not Modified",
            Self::TemporaryRedirect => "307 Temporary Redirect",
            Self::PermanentRedirect => "308 Permanent Redirect",

            // 4xx
            Self::BadRequest => "400 Bad Request",
            Self::Unauthorized => "401 Unauthorized",
            Self::PaymentRequired => "402 Payment Required",
            Self::Forbidden => "403 Forbidden",
            Self::NotFound => "404 Not Found",
            Self::MethodNotAllowed => "405 Method Not Allowed",
            Self::NotAcceptable => "406 Not Acceptable",
            Self::ProxyAuthenticationRequired => "407 Proxy Authentication Required",
            Self::RequestTimeout => "408 Request Timeout",
            Self::Conflict => "409 Conflict",
            Self::Gone => "410 Gone",
            Self::LengthRequired => "411 Length Required",
            Self::PreconditionFailed => "412 Precondition Failed",
            Self::PayloadTooLarge => "413 Payload Too Large",
            Self::URITooLong => "414 URI Too Long",
            Self::UnsupportedMediaType => "415 Unsupported Media Type",
            Self::RangeNotSatisfiable => "416 Range Not Satisfiable",
            Self::ExpectationFailed => "417 Expectation Failed",
            Self::ImATeapot => "418 I'm A Teapot",
            Self::MisdirectedRequest => "421 Misdirected Request",
            Self::UnprocessableContent => "422 Unprocessable Content",
            Self::Locked => "423 Locked",
            Self::FailedDependency => "424 Failed Dependency",
            Self::TooEarly => "425 Too Early",
            Self::UpgradeRequired => "426 Upgrade Required",
            Self::PreconditionRequired => "428 Precondition Required",
            Self::TooManyRequests => "429 Too Many Requests",
            Self::RequestHeaderFieldsTooLarge => "431 Request Header Fields Too Large",
            Self::UnavailableForLegalReasons => "451 Unavailable For Legal Reasons",
            // 5xx
            Self::InternalServerError => "500 Internal Server Error",
            Self::NotImplemented => "501 Not Implemented",
            Self::BadGateway => "502 Bad Gateway",
            Self::ServiceUnavailable => "503 Service Unavailable",
            Self::GatewayTimeout => "504 Gateway Timeout",
            Self::HTTPVersionNotSupported => "505 HTTP Version Not Supported",
            Self::VariantAlsoNegotiates => "506 Variant Also Negotiates",
            Self::InsufficientStorage => "507 Insufficient Storage",
            Self::LoopDetected => "508 Loop Detected",
            Self::NotExtended => "510 Not Extended",
            Self::NetworkAuthenticationRequired => "511 Network Authentication Required"
        }
    }
}
