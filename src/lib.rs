
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


pub struct Order<BigUint> {
    pub owner: Address,
    pub resolve_time: u64,
    pub is_call: bool,
    pub strike_price: BigUint,
    pub value: BigUint,
    pub status: u8,
}

#[elrond_wasm_derive::callable(BandBridgeProxy)]
pub trait BandBridge {
    #[payable]
    #[callback(validate_callback)]
    fn validate(&self);
}

#[elrond_wasm_derive::contract(BitSwingImpl)]
pub trait BitSwing {

    // INIT

    fn init(&self,) -> Result<(), &str> {

        Ok(())
    }

    #[callback]
    fn validate_callback(&self, _result: bool) {
       // ...
    }
}
