use axum::{
    async_trait, body::Bytes, extract::{FromRequest, Request}, http::StatusCode, response::{IntoResponse, Response}, routing::post, Router
};
use serde::Serialize;
use shared::{BusinessValidationError, CreateHostParams, Host, JsonRejection};
use tower_http::cors::CorsLayer;

enum AppError {
    BusinessValidationError(BusinessValidationError),
    JsonRejection(JsonRejection),
    BytesRejection{message: String},
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
            path: String
        }

        let (status, message, path) = match self {
            AppError::BusinessValidationError(BusinessValidationError { message, path }) => {
                (StatusCode::UNPROCESSABLE_ENTITY, message, path)
            }
            AppError::JsonRejection(JsonRejection::JsonDataError { message, path }) => {
                (StatusCode::UNPROCESSABLE_ENTITY, message, path)
            },
            AppError::JsonRejection(JsonRejection::JsonSyntaxError { message, }) => {
                (StatusCode::UNPROCESSABLE_ENTITY, message, "root".to_owned())
            },
            AppError::BytesRejection { message } => {
                (StatusCode::UNPROCESSABLE_ENTITY, message, "root".to_owned())
            },
        };
        (status, AppJson(ErrorResponse { message, path })).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl From<BusinessValidationError> for AppError {
    fn from(error: BusinessValidationError) -> Self {
        Self::BusinessValidationError(error)
    }
}

struct AppJson<T>(T);

#[async_trait]
impl<S> FromRequest<S> for AppJson<CreateHostParams>
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_err| AppError::BytesRejection { message: "unable to extract bytes".to_owned() })?;
        
        let body = shared::from_bytes::<CreateHostParams>(bytes.as_ref())?;

        Ok(Self(body))
    }
}


impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/hosts", post(create_host))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn create_host(
    AppJson(payload): AppJson<CreateHostParams>
) -> Result<impl IntoResponse, AppError> {
    // Business validation
    if let Err(err) = payload.validate() {
        return Err(AppError::BusinessValidationError(err));
    }
    let host = Host {
        id: 1337,
        hostname: payload.hostname,
        ipv4: payload.ipv4,
    };

    Ok((StatusCode::CREATED, AppJson(host)))
}
