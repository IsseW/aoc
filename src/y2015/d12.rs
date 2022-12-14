use serde_json::{self, Value};

pub fn solution_1(input: &str) -> String {
    fn count(object: &Value) -> i64 {
        match object {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(num) => num.as_i64().unwrap_or(0),
            Value::String(_) => 0,
            Value::Array(array) => array.iter().map(count).sum::<i64>(),
            Value::Object(object) => object.values().map(count).sum::<i64>(),
        }
    }
    let input: Value = serde_json::from_str(input).unwrap();
    count(&input).to_string()
}

pub fn solution_2(input: &str) -> String {
    fn count(object: &Value) -> i64 {
        match object {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(num) => num.as_i64().unwrap_or(0),
            Value::String(_) => 0,
            Value::Array(array) => array.iter().map(count).sum::<i64>(),
            Value::Object(object) => {
                if object
                    .values()
                    .any(|value| value.as_str().map(|s| s == "red").unwrap_or(false))
                {
                    0
                } else {
                    object.values().map(count).sum::<i64>()
                }
            }
        }
    }
    let input: Value = serde_json::from_str(input).unwrap();
    count(&input).to_string()
}
