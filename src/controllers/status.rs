use rocket_contrib::json::JsonValue;
use rocket_contrib::json;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[get("/status")]
pub fn lookup() -> JsonValue {
  json!({
    "code": 200,
    "success": true,
    "data": {
      "status": "OK",
      "version": VERSION.to_string()
    },
    "error": ""
  })
}