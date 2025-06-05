
use bevy::prelude::*;
use crate::gadgets::components::GadgetType;

#[derive(Event, Reflect, Debug)]
pub struct OnGadgetCardSelected {
    pub gadget_card: GadgetType,
}

impl OnGadgetCardSelected {
    pub fn new(gadget_card: GadgetType) -> Self {
        Self { gadget_card }
    }
}