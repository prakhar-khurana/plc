use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Policy {
    /// Pairs for Rule 7 that must not be active simultaneously.
    pub pairs: Option<Vec<[String; 2]>>,
    /// Memory ranges and access policies for Rule 10.
    pub memory_areas: Option<Vec<MemoryArea>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MemoryArea {
    /// Address range, e.g. "%MW100-%MW200"
    pub address: String,
    /// Access policy: "ReadOnly" | "ReadWrite"
    pub access: String,
}

/// Example policy JSON embedded as a constant (not in comments).

pub const EXAMPLE_POLICY_JSON: &str = r#"{
  "pairs": [
    ["Motor_Fwd", "Motor_Rev"],
    ["Valve_Open", "Valve_Close"]
  ],
  "memory_areas": [
    { "address": "%MW100-%MW200", "access": "ReadOnly" },
    { "address": "%M50-%M80",     "access": "ReadWrite" }
  ]
}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_policy_json_parses() {
        let p: Policy = serde_json::from_str(EXAMPLE_POLICY_JSON).unwrap();
        assert!(p.pairs.as_ref().unwrap().len() >= 1);
        assert!(p.memory_areas.as_ref().unwrap().len() >= 1);
    }
}
