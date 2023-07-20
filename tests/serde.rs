#[cfg(test)]
#[cfg(feature = "serde")]
mod serde {

    use serde_json::{self, Value};
    use taffy::style::Style;

    #[test]
    fn serde_can_serialize() {
        let style = Style::DEFAULT;
        let _ = serde_json::to_string(&style).unwrap();
    }

    #[test]
    fn serde_can_deserialize_partial_values() {
        use serde_json;
        let json = r#"{
            "inset": {
                "left": { "Length": 22 },
                "right": "Auto"
            },
            "size":
            {
                "width": { "Percent": 50 }
            },
            "min_size":
            {
                "height": { "Length": 10 }
            },
            "max_size":
            {
                "width": "Auto"
            },
            "margin":
            {
                "right": { "Length": 99.0 },
                "bottom": { "Length": 99.0 }
            },
            "padding":
            {
                "left": { "Length": 99.0 }
            },
            "border":
            {
                "bottom": { "Length": 99.0 }
            },
            "gap": {
                "width": { "Length": 99.0 }
            },
            "grid_row": { "start": "Auto" },
            "grid_column": { "end": "Auto" }
        }"#;
        serde_json::from_str::<Value>(json).unwrap();
    }
}
