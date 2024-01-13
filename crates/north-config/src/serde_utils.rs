#![allow(dead_code)]
use serde_json::{Map, Value};

/// Trait used to merge Json Values
pub trait Merge {
    /// Method use to merge two Json Values : ValueA <- ValueB.
    fn merge(&mut self, new_json_value: Value);
    /// Merge a new value in specific json pointer.
    fn merge_in(&mut self, json_pointer: &str, new_json_value: Value);
}

/// function merges two json values into one
///
/// # Example
/// ```
/// use serde_json::json;
/// use north_config::serde_utils::merge_json;
///
/// let mut a = json!({
///   "title": "This is a title",
///   "person" : {
///      "firstName" : "John",
///      "lastName" : "Doe"
///    },
///    "cities":[ "london", "paris" ]
///  });
///
///  let b = json!({
///    "title": "This is another title",
///    "person" : {
///       "firstName" : "Jane"
///    },
///    "cities":[ "colombo" ]
///  });
///  merge_json(&mut a, b)
/// ```
pub fn merge_json(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge_json(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a @ &mut Value::Array(_), Value::Array(b)) => {
            let a = a.as_array_mut().unwrap();
            a.extend(b);
            a.dedup();
        }
        (a @ &mut Value::Array(_), Value::Object(b)) => {
            let a = a.as_array_mut().unwrap();
            a.push(Value::Object(b));
            a.dedup();
        }
        (a, b) => *a = b,
    }
}

#[cfg(feature = "yaml")]
/// function merges a yaml string into a json value
///
/// usage:
/// ```rust
/// use serde_json::json;
/// use north_config::serde_utils::merge_json_and_yaml;
/// let mut a = json!({
///   "title": "This is a title",
///   "person" : {
///      "firstName" : "John",
///      "lastName" : "Doe"
///    },
///    "cities":[ "london", "paris" ]
///  });
///
///  let b = r"name: demo";
///  merge_json_and_yaml(&mut a, b.to_string())
/// ```
pub fn merge_json_and_yaml(a: &mut Value, raw_str: String) {
    let try_json = serde_yaml::from_str::<Value>(&raw_str).unwrap();
    merge_json(a, try_json)
}

impl Merge for Value {
    /// # Examples: Merge two array together.
    /// ```
    /// use serde_json::Value;
    /// use north_config::serde_utils::Merge;
    ///
    /// let mut array1: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
    /// let array2: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    /// array1.merge(array2);
    /// assert_eq!(r#"["a","b","c"]"#, array1.to_string());
    /// ```
    /// # Examples: Merge two objects together.
    /// ```
    /// use serde_json::Value;
    /// use north_config::serde_utils::Merge;
    ///
    /// let mut object1: Value = serde_json::from_str(r#"{"value1":"a","value2":"b"}"#).unwrap();
    /// let object2: Value = serde_json::from_str(r#"{"value1":"a","value2":"c","value3":"d"}"#).unwrap();
    /// object1.merge(object2);
    /// assert_eq!(r#"{"value1":"a","value2":"c","value3":"d"}"#,object1.to_string());
    /// ```
    /// # Examples: Merge an object into an array.
    /// ```
    /// use serde_json::Value;
    /// use north_config::serde_utils::Merge;
    ///
    /// let mut array: Value = serde_json::from_str(r#"[]"#).unwrap();
    /// let object: Value = serde_json::from_str(r#"{"field1":"value1"}"#).unwrap();
    /// array.merge(object);
    /// assert_eq!(r#"[{"field1":"value1"}]"#,array.to_string());
    /// ```
    /// # Examples: Merge an array into an object.
    /// ```
    /// use serde_json::Value;
    /// use north_config::serde_utils::Merge;
    ///
    /// let mut object: Value = serde_json::from_str(r#"{"field1":"value1"}"#).unwrap();
    /// let array: Value = serde_json::from_str(r#"["value2","value3"]"#).unwrap();
    /// object.merge(array);
    /// assert_eq!(r#"["value2","value3"]"#,object.to_string());
    /// ```
    fn merge(&mut self, new_json_value: Value) {
        merge(self, &new_json_value);
    }
    /// # Examples: Merge an array in an object in a specific position.
    /// ```
    /// use serde_json::Value;
    /// use north_config::serde_utils::Merge;
    ///
    /// let mut object: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    /// let array: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    /// object.merge_in("/my_array", array.clone());
    /// assert_eq!(r#"{"my_array":[{"a":"t"},"b","c"]}"#, object.to_string());
    /// ```
    /// # Examples: Merge two objects together in a specific position.
    /// ```
    /// use serde_json::Value;
    /// use north_config::serde_utils::Merge;
    ///
    /// let mut object1: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
    /// let object2: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
    /// object1.merge_in("/my_array/0/a", object2.clone());
    /// assert_eq!(r#"{"my_array":[{"a":{"b":"c"}}]}"#, object1.to_string());
    /// ```
    /// # Examples: Merge an object in an array in a specific position.
    /// ```
    /// use serde_json::Value;
    /// use north_config::serde_utils::Merge;
    ///
    /// let mut json_value: Value = serde_json::from_str(r#"[{"array1":[{"field":"value1"}]}]"#).unwrap();
    /// json_value.merge_in("/other_field", Value::String("value".to_string()));
    /// assert_eq!(r#"[{"array1":[{"field":"value1"}]},{"other_field":"value"}]"#,json_value.to_string());
    /// ```
    // # Examples: Build a new object.
    // ```
    // use serde_json::{Map,Value};
    // use north_config::serde_utils::Merge;
    //
    // let mut object: Value = Value::default();
    // object.merge_in("/field", Value::String("value".to_string()));
    // object.merge_in("/object", Value::Object(Map::default()));
    // object.merge_in("/array", Value::Array(Vec::default()));
    // assert_eq!(r#"{"array":[],"field":"value","object":{}}"#,object.to_string());
    // ```
    fn merge_in(&mut self, json_pointer: &str, new_json_value: Value) {
        let fields: Vec<&str> = json_pointer.split('/').skip(1).collect();

        merge_in(self, fields, new_json_value);
    }
}

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), Value::Object(b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (&mut Value::Array(ref mut a), Value::Array(b)) => {
            a.extend(b.clone());
            a.dedup();
        }
        (&mut Value::Array(ref mut a), Value::Object(b)) => {
            a.push(Value::Object(b.clone()));
            a.dedup();
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

fn merge_in(json_value: &mut Value, fields: Vec<&str>, new_json_value: Value) {
    if fields.is_empty() {
        return json_value.merge(new_json_value);
    }

    let mut fields = fields.clone();
    let field = fields.remove(0);

    if field.is_empty() {
        return json_value.merge(new_json_value);
    }

    match json_value.pointer_mut(format!("/{}", field).as_str()) {
        // Find the field and the json_value_targeted.
        Some(json_targeted) => {
            if !fields.is_empty() {
                merge_in(json_targeted, fields, new_json_value);
            } else {
                json_targeted.merge(new_json_value);
            }
        }
        // Not find the field and the json_value_targeted.
        // Add the new field and retry the merge_in with same parameters.
        None => {
            let new_value = match field.parse::<usize>().ok() {
                Some(position) => {
                    let mut vec = Vec::default();
                    match vec.get(position) {
                        Some(_) => vec.insert(position, Value::default()),
                        None => vec.push(Value::default()),
                    }
                    Value::Array(vec)
                }
                None => {
                    let mut map = Map::default();
                    map.insert(field.to_string(), Value::default());
                    Value::Object(map)
                }
            };

            match (json_value.clone(), new_value.clone()) {
                (Value::Array(vec), Value::Object(_)) => {
                    json_value.merge(new_value);
                    let size = vec.len().to_string();
                    let mut new_fields: Vec<&str> = vec![&size, field];
                    new_fields.append(&mut fields.clone());
                    merge_in(json_value, new_fields, new_json_value);
                }
                (_, _) => {
                    let mut new_fields: Vec<&str> = vec![field];
                    new_fields.append(&mut fields.clone());
                    json_value.merge(new_value);
                    merge_in(json_value, new_fields, new_json_value);
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn can_merge_two_valid_jsons() {
        let mut a = json!({
            "title": "This is a title",
            "person" : {
                "firstName" : "John",
                "lastName" : "Doe"
            },
            "cities":[ "london", "paris" ]
        });

        let b = json!({
            "title": "This is another title",
            "person" : {
                "firstName" : "Jane"
            },
            "cities":[ "colombo" ]
        });
        merge_json(&mut a, b);

        assert_eq!(
            a,
            json!({
                "title": "This is another title",
                "person" : {
                    "firstName" : "Jane",
                    "lastName" : "Doe"
                },
                "cities":[ "london", "paris", "colombo" ]
            })
        );
    }

    #[test]
    fn it_should_merge_array_string() {
        let mut first_json_value: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
        let secound_json_value: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
        first_json_value.merge(secound_json_value);
        assert_eq!(r#"["a","b","c"]"#, first_json_value.to_string());
    }
    #[test]
    fn it_should_merge_array_object() {
        let mut first_json_value: Value =
            serde_json::from_str(r#"[{"value":"a"},{"value":"b"}]"#).unwrap();
        let secound_json_value: Value =
            serde_json::from_str(r#"[{"value":"b"},{"value":"c"}]"#).unwrap();
        first_json_value.merge(secound_json_value);
        assert_eq!(
            r#"[{"value":"a"},{"value":"b"},{"value":"c"}]"#,
            first_json_value.to_string()
        );
    }
    #[test]
    fn it_should_merge_object() {
        let mut first_json_value: Value =
            serde_json::from_str(r#"{"value1":"a","value2":"b"}"#).unwrap();
        let secound_json_value: Value =
            serde_json::from_str(r#"{"value1":"a","value2":"c","value3":"d"}"#).unwrap();
        first_json_value.merge(secound_json_value);
        assert_eq!(
            r#"{"value1":"a","value2":"c","value3":"d"}"#,
            first_json_value.to_string()
        );
    }
    #[test]
    fn it_should_merge_string() {
        let mut value_a: Value = Value::String("a".to_string());
        let value_b: Value = Value::String("b".to_string());
        value_a.merge(value_b.clone());
        assert_eq!(value_b.to_string(), value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_array_in_a_specifique_field_path() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
        value_a.merge_in("/my_array", value_b);
        assert_eq!(r#"{"my_array":[{"a":"t"},"b","c"]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_object_in_a_specifique_field_path() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array", value_b);
        assert_eq!(r#"{"my_array":[{"a":"t"},{"b":"c"}]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_in_an_object_in_specifique_path_position() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array/0", value_b);
        assert_eq!(r#"{"my_array":[{"a":"t","b":"c"}]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_an_array_in_specifique_path_position() {
        let mut value_a: Value = serde_json::from_str(r#"{"my_array":[{"a":"t"}]}"#).unwrap();
        let value_b: Value = serde_json::from_str(r#"{"b":"c"}"#).unwrap();
        value_a.merge_in("/my_array/1", value_b);
        assert_eq!(r#"{"my_array":[{"a":"t"},{"b":"c"}]}"#, value_a.to_string());
    }
    #[test]
    fn it_should_merge_in_root_array() {
        let mut json_value: Value = serde_json::from_str(r#"["value"]"#).unwrap();
        let json_value_to_merge: Value = serde_json::from_str(r#"["new_value"]"#).unwrap();
        json_value.merge_in("/", json_value_to_merge);
        assert_eq!(r#"["value","new_value"]"#, json_value.to_string());
    }
    #[test]
    fn it_should_merge_in_root_object() {
        let mut json_value: Value = serde_json::from_str(r#"{"field":"value"}"#).unwrap();
        let json_value_to_merge: Value = serde_json::from_str(r#"{"field2":"value2"}"#).unwrap();
        json_value.merge_in("/", json_value_to_merge);
        assert_eq!(
            r#"{"field":"value","field2":"value2"}"#,
            json_value.to_string()
        );
    }
    #[test]
    fn it_should_merge_null_in_specifique_path() {
        let mut json_value: Value = serde_json::from_str(r#"{"field":{"child":"value"}}"#).unwrap();
        let json_value_null: Value = Value::Null;
        json_value.merge_in("/field", json_value_null);
        assert_eq!(r#"{"field":null}"#, json_value.to_string());
    }
}
