use std::collections::HashMap;

pub fn hashmap() -> HashMap<String, i32> {
    let mut result = HashMap::new();
    result.insert("Alice".to_string(), 25);
    result.insert("Robert".to_string(), 27);
    result
}