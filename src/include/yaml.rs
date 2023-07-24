use serde_yaml::Value;
use std::collections::HashMap;

pub fn parse_yaml_to_map(yaml_data: &str) -> Option<HashMap<String, Value>> {
    match serde_yaml::from_str::<Value>(yaml_data) {
        Ok(Value::Mapping(map)) => {
            let mut result = HashMap::new();
            for (key, value) in map {
                if let Value::String(key_str) = key {
                    result.insert(key_str, value);
                } else {
                    return None;
                }
            }
            Some(result)
        }
        _ => None,
    }
}
pub fn get_config_from_file(name: &str, file: &[u8]) -> Value {
    if let Some(map) = parse_yaml_to_map(super::to_ascii(file).as_str()) {
        return map.get(name).expect("Unwrap included config to hash map failed").clone();
    } else {
        panic!("Error parsing yaml icluded to build");
    }
}
pub fn get_config(name: &str, data: &Value) -> Value {
    return data[name].clone();
}

pub fn get_i32(data: &Value, name: &str) -> i32 {
    return data[name].as_i64().expect("Unwrap i32 from icluded yaml config failed") as i32;
}
pub fn get_u128(data: &Value, name: &str) -> u128 {
    return data[name].as_i64().expect("Unwrap u128 from icluded yaml config failed") as u128;
}
pub fn get_string(data: &Value, name: &str) -> String {
    return data[name].as_str().expect("Unwrap String from icluded yaml config failed").to_string();
}
pub fn get_bool(data: &Value, name: &str) -> bool {
    return data[name].as_bool().expect("Unwrap bool from icluded yaml config failed");
}