pub mod proto {
    pub mod types {
        include!(concat!(env!("OUT_DIR"), "/proto_types.rs"));
        include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
    }
}

#[cfg(test)]
mod tests {
    use crate::proto::types::FloatValue;

    #[test]
    fn default_ok() {
        let v = FloatValue::default();
        let serialized_v = serde_json::to_string(&v).unwrap();
        assert_eq!(serialized_v, "0.0");
    }

    #[test]
    fn value_ok() {
        let v = FloatValue { value: 5.5 };
        let serialized_v = serde_json::to_string(&v).unwrap();
        assert_eq!(serialized_v, "5.5");
    }

    #[test]
    fn nan_should_fail() {
        let v = FloatValue { value: f32::NAN };
        assert!(serde_json::to_string(&v).is_err());
    }

    #[test]
    fn infinity_should_fail() {
        let v = FloatValue {
            value: f32::INFINITY,
        };
        assert!(serde_json::to_string(&v).is_err());
    }

    #[test]
    fn neg_infinity_should_fail() {
        let v = FloatValue {
            value: f32::NEG_INFINITY,
        };
        assert!(serde_json::to_string(&v).is_err());
    }
}
