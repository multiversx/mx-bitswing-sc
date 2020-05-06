
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


#[elrond_wasm_derive::contract(BandBridgeMockImpl)]
pub trait BandBridgeMock {

    fn init(&self) {
    }

    
    fn verify(&self) -> Result<bool, &str> {

        Ok(true)
    }
}
