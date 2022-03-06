use crate::actions::{MeasureTypes, SplitMeasures};
use crate::domains::measures::MeasureValue;

pub fn to_named_measures(measures: Vec<MeasureValue>, types: MeasureTypes) -> SplitMeasures {
    let mut result = SplitMeasures {
        particles: vec![],
        co2: vec![],
        temp: vec![],
        humidity: vec![],
    };

    for m_type in types {
        let filtered_measures: Vec<&MeasureValue> =
            measures.iter().filter(|m| m.name == m_type.name).collect();

        for m in filtered_measures {
            match m.name.as_str() {
                "pm2.5" => result.particles.push(m.measure),
                "co2" => result.co2.push(m.measure),
                "temperature" => result.temp.push(m.measure),
                "humidity" => result.humidity.push(m.measure),
                _ => (),
            }
        }
    }

    result
}
