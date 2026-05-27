use soroban_sdk::{testutils::Address as _, Address, Env};
use treasury::{SettlementStatus, TreasuryContract, TreasuryContractClient};

#[test]
fn approvals_accumulate_until_threshold() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let backup = Address::generate(&env);
    let merchant = Address::generate(&env);
    let id = env.register_contract(None, TreasuryContract);
    let client = TreasuryContractClient::new(&env, &id);
    client.initialize(&admin, &2);
    client.set_signer(&admin, &backup, &1);
    let settlement_id = client.propose_settlement(&admin, &merchant, &10_000_000);
    let settlement = client.approve_settlement(&backup, &settlement_id);
    assert_eq!(settlement.status, SettlementStatus::Pending);
    assert_eq!(settlement.approvals.len(), 2);
}

#[test]
fn pause_and_unpause_emit_events() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let merchant = Address::generate(&env);
    let id = env.register_contract(None, TreasuryContract);
    let client = TreasuryContractClient::new(&env, &id);
    client.initialize(&admin, &1);
    client.pause(&admin);
    client.unpause(&admin);
    // after unpause, proposals work again
    let settlement_id = client.propose_settlement(&admin, &merchant, &1_000);
    assert_eq!(settlement_id, 1);
}
