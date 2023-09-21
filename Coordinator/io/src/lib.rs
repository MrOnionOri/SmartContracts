
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{InOut,Metadata};


#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum CustomStruct {

    Example

    // Add States
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Action {
    
    // Add Actions
    RegisterUser(ActorId),
    AddItemWeapon,
    AddItemArmor,
    BuyItemFromUser,
    BuyItemFromMarket,
    SellItemToMarket,
    SellItemToUser,
    AddItemToInventory,
    ExampleAction
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum  Event {
    
    // Add Events
    UserRegistered(ActorId),
    ExampleEvent
}


pub struct ContractMetadata;


impl Metadata for ContractMetadata{
     type Init = ();
     type Handle = InOut<Action,Event>;
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Vec<(ActorId, u128)>;

}