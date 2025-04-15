// Import necessary modules from the Solana program library
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg, // To log messages
};

// This defines the entrypoint for the Solana program
entrypoint!(process_instruction);

// This function processes the incoming instruction
fn process_instruction(
    program_id: &Pubkey,      // The program ID (address)
    accounts: &[AccountInfo], // The accounts involved in the transaction
    instruction_data: &[u8],  // The data passed to the program
) -> ProgramResult {
    // Log a message to indicate that the contract has been called
    msg!("Hello World");


    // Log the wallet addresses (public keys) of the accounts passed to the program
    for account in accounts {
        msg!("Wallet Address: {:?}", account.key);
    }
    
    // Log the program ID (the address of the program itself)
    msg!("Program ID: {:?}", program_id);

    // Returning Ok(()) signifies that the program has executed successfully
    Ok(())
}
