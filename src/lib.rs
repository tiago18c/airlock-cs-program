//! A program that says hello, and nothing else. It exists to be upgraded:
//! Airlock's showcase proposes its next build through a Squads multisig,
//! with the buffer's hash checked on chain before a member's vote counts.
#![no_std]

use pinocchio::{
    no_allocator, nostd_panic_handler, program_entrypoint, AccountView, Address, ProgramResult,
};
use solana_program_log::log;

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();

/// The one instruction: it takes no accounts and reads no data.
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &mut [AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    log("Hello from the Airlock clear signing demo: build 1");
    Ok(())
}
