use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    HeartRate, // 0
    BloodPressure, // 1
    BloodOxygen, // 2
    FloorClimbed, // 3
    StepCount,  // 4
    WalkingDistance, // 5
    RunningDistance, // 6
    ReadOnly = -1
}