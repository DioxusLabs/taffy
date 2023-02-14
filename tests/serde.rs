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
        let json = r###"{
            "inset": {
                "left": { "Points": 22 },
                "right": "Auto"
            },
            "size":
            {
                "width": { "Percent": 50 }
            },
            "min_size":
            {
                "height": { "Points": 10 }
            },
            "max_size":
            {
                "width": "Auto"
            },
            "margin":
            {
                "right": { "Points": 99.0 },
                "bottom": { "Points": 99.0 }
            },
            "padding":
            {
                "left": { "Points": 99.0 }
            },
            "border":
            {
                "bottom": { "Points": 99.0 }
            },
            "gap": {
                "width": { "Points": 99.0 }
            },
            "grid_row": { "start": "Auto" },
            "grid_column": { "end": "Auto" }
        }"###;
        let _: Value = serde_json::from_str(&json).unwrap();
    }
}
