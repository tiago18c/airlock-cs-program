//! A program that says hello and goodbye, and nothing else. It exists to be
//! upgraded: Airlock's showcase proposes its next build through a Squads
//! multisig, with the buffer's hash checked on chain before a member's vote
//! counts.
#![no_std]

use pinocchio::{
    no_allocator, nostd_panic_handler, program_entrypoint, AccountView, Address, ProgramResult,
};
use solana_program_log::log;

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();

/// The first byte of the data picks the instruction: 1 says goodbye,
/// anything else (or nothing) says hello. Neither takes an account.
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.first() {
        Some(1) => log("Goodbye from the Airlock clear signing demo: build 5"),
        _ => log("Hello from the Airlock clear signing demo: build 5"),
    }
    Ok(())
}
