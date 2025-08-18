//! Data structures representing external policy configuration.
//! The JSON format is intentionally simple.
//
// Example policy.json:
//
// {
//   "pairs": [["Motor_Fwd","Motor_Rev"], ["Valve_Open","Valve_Close"]],
//   "memory_areas": [
//     {"address": "%MW100-%MW200", "access": "ReadOnly"},
//     {"address": "%M50-%M80",     "access": "ReadWrite"}
//   ]
// }

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
    pub address: String, // e.g., "%MW100-%MW200"
    pub access: String,  // "ReadOnly" | "ReadWrite"
    {
    "pairs": [
      ["Motor_Fwd", "Motor_Rev"]
    ],
    "memory_areas": [
      { "address": "%MW100-%MW200", "access": "ReadOnly" }
    ]
  }
}
