use near_sdk::{ext_contract, AccountId};

#[ext_contract(contract_executable)]
trait ContractExecutable {
    fn execute(
        &mut self,
        command_id: String,
        source_chain: String,
        source_address: AccountId,
        payload: String,
    );
}
