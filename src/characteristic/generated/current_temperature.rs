// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Current Temperature Characteristic.
pub type CurrentTemperature = Characteristic<f32>;

/// Creates a new Current Temperature Characteristic.
pub fn new() -> CurrentTemperature {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::CurrentTemperature,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::Events],
        unit: Some(Unit::Celsius),
        max_value: Some(100 as f32),
        min_value: Some(0 as f32),
        step_value: Some(0.1 as f32),
        value: 10.0,
        ..Default::default()
    })
}
