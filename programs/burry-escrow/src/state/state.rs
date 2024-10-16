use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow{
    pub unlock_price:f64,
    pub excrow_amount:u64
}