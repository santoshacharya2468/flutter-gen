use serde_json::Value;

pub fn serde_value_to_dart_class(serde_value: &serde_json::Map<String, Value>, cname: &str) -> String {
    let class_name = to_pascal_case(cname);
    let mut dart_class = format!("class {} {{\n", class_name);
    let mut  nested_classes:Vec<String>=vec![];
     let json_object = serde_value;
        for (key, value) in json_object {
            match value {
                Value::String(_) => {
                    dart_class.push_str(&format!("  final String {}; \n", key));
                }
                Value::Number(n) if n.is_i64() => {
                    dart_class.push_str(&format!("  final int {};\n", key,));
                }
                Value::Number(n) if n.is_f64() => {
                    dart_class.push_str(&format!("  final double {};\n", key));
                }
                Value::Bool(_) => {
                    dart_class.push_str(&format!("  final bool {};\n", key));
                }
                Value::Object(obj)=>{
                    let nested_class=serde_value_to_dart_class(obj, key);
                    dart_class.push_str(&format!("  final {} {};\n", to_pascal_case(key),key));
                    nested_classes.push(nested_class);
                }
                _ => {} // Ignore other types for simplicity
            }
        }
        dart_class.push_str(&format!("\n  {}(", class_name));
        let fields: Vec<_> = json_object.keys().map(|k| k.clone()).collect();
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                dart_class.push_str(", ");
            }
            dart_class.push_str(&format!("this.{}", field));
        }
        dart_class.push_str(");\n");

    dart_class.push_str("}\n");
    for cls in nested_classes{
        dart_class.push_str(cls.as_str());
    }
    dart_class
}


fn to_pascal_case(input: &str) -> String {
    let mut pascal_case = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            pascal_case.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            pascal_case.push(c);
        }
    }

    pascal_case
}