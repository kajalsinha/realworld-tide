use crate::conduit::articles;
use crate::conduit::favorites;
use crate::middleware::ContextExt;
use crate::web::articles::responses::{Article, ArticleResponse};
use crate::web::diesel_error;
use crate::Repo;
use tide::{Request, Response, ResultExt};
use uuid::Uuid;

pub async fn get_article(cx: Request<Repo>) -> tide::Result<Response> {
    let slug: String = cx.param("slug").client_err()?;
    let repo = cx.state();
    let (article, author, n_favorites) = articles::find_one(repo, &slug).map_err(|e| match e {
        diesel::NotFound => Response::new(404),
        e => diesel_error(&e),
    })?;

    let user_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let favorited = match user_id {
        Some(user_id) => {
            favorites::is_favorite(&repo, user_id, article.id).map_err(|e| diesel_error(&e))?
        }
        None => false,
    };

    let response = ArticleResponse {
        article: Article::new(article, author, n_favorites, favorited),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
