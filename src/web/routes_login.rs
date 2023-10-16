use crate::{web, Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
	Router::new().route("/api/login", post(api_login))
}

async fn api_login(
	cookies: Cookies,
	payload: Json<LoginPayload>,
) -> Result<Json<Value>> {
	println!("->> {:<12} - api_login", "HANDLER");

	// TODO: Implement real db/auth logic.
	if payload.username != "demo1" || payload.pwd != "welcome" {
		return Err(Error::LoginFail);
	}

	// FIXME: Implement real auth-token generation/signature.
	let mut cookie = Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign");
	cookie.set_http_only(true);
	cookie.set_path("/");
	cookies.add(cookie);

	// Create the success body.
	let body = Json(json!({
		"result": {
			"success": true
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
	username: String,
	pwd: String,
}
