use rocket_contrib::json::JsonValue;
use rocket_contrib::json;

#[catch(404)]
pub fn lookup() -> JsonValue {
  json!({
    "success": false,
    "code": 404,
    "data": "",
    "error": "Resource not found"
  })
}