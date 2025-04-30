use Borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{ProgramResult, entrypoint},
    msg,
    oubkey::PubKey
};

#[derive(BorshSerialize, BorshDeserialize)]

enum InstructionType {
    Increment(u32), 
    Decrement(u32)
}

#[derive(BorshSerialize, BorshDeserialize)]

struct Counter {
    count: u32
}

entrypoint!(counter_contract);

pub fn counter_contract(
    //takes 3 arguments
    program_id: &PubKey,
    accounts: &[AccountInfo],    //what accounts is the user going to interact with
    instruction_data: &[u8]      //0 to 255. Can be converted to function name and args
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;
    
    match instruction_type {
        InstructionType::Increment(u32) => {
            msg!("Executing increment");
            counter_data.count += value;
        },
        InstructionType::Decrement(u32) => {
            msg!("Executing decrement");
            counter_data.count -= value;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut());        //writing the change back to the account address

    Ok(());
}