use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum BinaryOptionInstruction {
  Create(CreateBinaryOptionArgs)
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CreateBinaryOptionArgs {
  pub expiry_date_unix_timestamp: u64,
  pub pda_seed: u64,
  pub strike_price: u64
}

pub fn create(
  program_id: &Pubkey,
) -> Instruction {
  Instruction {
    program_id,
  }
}
