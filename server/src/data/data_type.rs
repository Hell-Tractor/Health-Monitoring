use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    HeartRate, // 0
    BloodPressureLow, // 1
    BloodPressureHigh, // 2
    BloodOxygen, // 3
    FloorClimbed, // 4
    StepCount,  // 5
    WalkingDistance, // 6
    RunningDistance, // 7
    ReadOnly = -1
}