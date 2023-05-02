#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	hc.do_get("/hello2/Mike").await?.print().await?;

	hc.do_get("/src/main.rs").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);
	req_login.await?.print().await?;

	let req_create_ticket = hc.do_post(
		"/api/tickets",
		json!({
			"title": "Ticket AAA"
		}),
	);
	req_create_ticket.await?.print().await?;

	hc.do_get("/api/tickets").await?.print().await?;

	Ok(())
}
