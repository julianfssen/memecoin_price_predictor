use {
  borsh::BorshDeserialize,
  crate::instruction::BinaryOptionInstruction,
  solana_program::{
      account_info::AccountInfo,
      entrypoint::ProgramResult,
      pubkey::Pubkey
  }
};

pub struct Processor;

impl Processor {
  pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction = BinaryOptionInstruction::try_from_slice(instruction_data)?;

    match instruction {
      BinaryOptionInstruction::Create(args) => {
        let expiry_date_unix_timestamp = args.expiry_date_unix_timestamp;
        let pda_seed = args.pda_seed;
        let strike_price = args.strike_price;

        let accounts_iter = &mut accounts.iter();

        Ok(())
      }
    }
  }
}
