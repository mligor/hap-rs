// THIS FILE IS AUTO-GENERATED

use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    event::EventEmitterPtr,
    service::{accessory_information::AccessoryInformation, heater_cooler, HapService},
    Result,
};

/// Heater Cooler Accessory.
pub type HeaterCooler = Accessory<HeaterCoolerInner>;

/// Inner type of the Heater Cooler Accessory.
#[derive(Default)]
pub struct HeaterCoolerInner {
    /// ID of the Heater Cooler Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Heater Cooler Service.
    pub heater_cooler: heater_cooler::HeaterCooler,
}

impl HapAccessory for HeaterCoolerInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> { vec![&self.accessory_information, &self.heater_cooler] }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![&mut self.accessory_information, &mut self.heater_cooler]
    }

    fn get_mut_information(&mut self) -> &mut AccessoryInformation { &mut self.accessory_information }

    fn init_iids(&mut self, accessory_id: u64, event_emitter: EventEmitterPtr) -> Result<()> {
        let mut next_iid = 1;
        for service in self.get_mut_services() {
            service.set_id(next_iid);
            next_iid += 1;
            for characteristic in service.get_mut_characteristics() {
                characteristic.set_id(next_iid)?;
                characteristic.set_accessory_id(accessory_id)?;
                characteristic.set_event_emitter(Some(event_emitter.clone()))?;
                next_iid += 1;
            }
        }
        Ok(())
    }
}

/// Creates a new Heater Cooler Accessory.
pub fn new(information: Information) -> Result<HeaterCooler> {
    let mut heater_cooler = heater_cooler::new();
    heater_cooler.set_primary(true);
    Ok(HeaterCooler::new(HeaterCoolerInner {
        accessory_information: information.to_service()?,
        heater_cooler,
        ..Default::default()
    }))
}
