use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::pubkey::Pubkey
};

pub enum BinaryOptionSeeds {
  Long,
  Pool,
  Short
}

impl BinaryOptionSeeds {
  pub fn to_str(&self) -> &'static str {
    match self {
      BinaryOptionSeeds::Long => "long",
      BinaryOptionSeeds::Pool => "pool",
      BinaryOptionSeeds::Short => "short"
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BinaryOption {
    // The strike price for the underlying token when settling the
    // binary option upon the expiry date
    pub strike_price: u64,

    // UNIX timestamp of the expiry date for the binary option
    pub expiry_date_unix_timestamp: u64,

    // Randomly generated `u64` value that is used when generating
    // the PDAs for the binary option
    pub pda_seed: u64,

    // Flag to determine whether the binary option is still active
    // or expired (i.e. settled)
    pub expired: bool,

    // The pubkey for the account that initiated the binary option
    pub creator_account_pubkey: Pubkey,

    // The token mint pubkey for the binary option's underlying token
    pub underlying_mint_account_pubkey: Pubkey,

    // The pubkey for the account that holds the SOL pool / deposits
    // for the binary option
    pub pool_account_pubkey: Pubkey,

    // The account pubkey for the token that represents the long side
    // of the binary option
    pub long_mint_account_pubkey: Pubkey,

    // The account pubkey for the token that represents the short side
    // of the binary option
    pub short_mint_account_pubkey: Pubkey,

    // The account pubkey for the winning side of the binary options
    // (i.e. `long_mint_account_pubkey` if the underlying token price is 
    // above the strike price, and vice-versa for `short_mint_account_pubkey)
    pub winning_side_account_pubkey: Option<Pubkey>,
}
