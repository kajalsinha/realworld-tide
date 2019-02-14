use futures::future;
use http::StatusCode;
use tide::{configuration::Store, Extract, IntoResponse, Request, Response, RouteMatch};

use crate::auth::{extract_claims, Claims};

impl<S: 'static> Extract<S> for Claims {
    type Fut = future::Ready<Result<Self, Response>>;

    fn extract(
        data: &mut S,
        req: &mut Request,
        params: &Option<RouteMatch<'_>>,
        store: &Store,
    ) -> Self::Fut {
        let claims = extract_claims(req.headers()).ok_or(StatusCode::UNAUTHORIZED.into_response());
        future::ready(claims)
    }
}