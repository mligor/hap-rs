use accessory::{HapAccessory, HapAccessoryService, Accessory, Information};
use service::accessory_information::AccessoryInformation;
use event::EmitterPtr;

use Error;

pub type Bridge = Accessory<BridgeInner>;

#[derive(Default)]
pub struct BridgeInner {
    id: u64,

    pub accessory_information: AccessoryInformation,
}

impl HapAccessory for BridgeInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&HapAccessoryService> {
        vec![
            &self.accessory_information,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService> {
        vec![
            &mut self.accessory_information,
        ]
    }

    fn get_mut_information(&mut self) -> &mut AccessoryInformation {
        &mut self.accessory_information
    }

    fn init_iids(&mut self, accessory_id: u64, event_emitter: EmitterPtr) -> Result<(), Error> {
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

pub fn new(information: Information) -> Result<Bridge, Error> {
    Ok(Bridge::new(BridgeInner {
        accessory_information: information.to_service()?,
        ..Default::default()
    }))
}
