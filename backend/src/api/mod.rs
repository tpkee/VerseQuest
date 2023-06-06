use axum::{async_trait, extract::FromRequest, body::Bytes, http::Request, BoxError};
use axum::body::HttpBody;

pub struct SanitisedBody<T> (T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for SanitisedBody<T>
    where
        T: FromRequest<S, B>,
        B: HttpBody + Send + 'static,
        B::Data: Send,
        B::Error: Into<BoxError>,
        B: From<Bytes>,
        S: Send + Sync
{
    type Rejection = T::Rejection;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        let sanitised_request: Request<B> = {
            let temp_request = Request::builder().body(body).unwrap(); // So I can convert the Request<B> body to Bytes without consuming the parts of the original request
            let body_bytes = Bytes::from_request(temp_request, state).await.unwrap();
            let str = ammonia::clean(std::str::from_utf8(&body_bytes).expect("It's not possible to convert the body to UTF-8"));

            Request::from_parts(parts, Bytes::from(str).into())
        };

        Ok(SanitisedBody(T::from_request(sanitised_request, state).await?))
    }
}