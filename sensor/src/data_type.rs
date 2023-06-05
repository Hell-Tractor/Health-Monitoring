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

impl From<&config::Value> for Type {
    fn from(value: &config::Value) -> Self {
        match value.clone().into_string().unwrap().as_str() {
            "heartRate" => Type::HeartRate,
            "bloodPressureLow" => Type::BloodPressureLow,
            "bloodPressureHigh" => Type::BloodPressureHigh,
            "bloodOxygen" => Type::BloodOxygen,
            "floorClimbed" => Type::FloorClimbed,
            "stepCount" => Type::StepCount,
            "walkingDistance" => Type::WalkingDistance,
            "runningDistance" => Type::RunningDistance,
            _ => panic!("Invalid data type, must be one of: heartRate, bloodPressureLow, bloodPressureHigh, bloodOxygen, floorClimbed, stepCount, walkingDistance, runningDistance.")
        }
    }
}