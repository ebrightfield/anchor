use anchor_localnet::localnet_account::LocalnetAccount;
use solana_sdk::pubkey::Pubkey;
use spl_token::solana_program::program_option::COption;
use anchor_localnet::SystemAccount;
use anchor_localnet::test_toml_generator::TestTomlGenerator;

pub fn suite_1() -> TestTomlGenerator {
    TestTomlGenerator {
        save_directory: "./tests/suite-1".to_string(),
        test_file_glob: Some("./tests/suite-1/test.ts".to_string()),
        accounts: accounts(),
        ..Default::default()
    }
}

pub fn accounts() -> Vec<LocalnetAccount> {
    let test_user = LocalnetAccount::new(
        Pubkey::new_unique(),
        "test_user.json".to_string(),
        SystemAccount,
    );
    let test_mint = LocalnetAccount::new(
        Pubkey::new_unique(),
        "mint.json".to_string(),
        anchor_spl::token::Mint::from(spl_token::state::Mint {
            mint_authority: COption::Some(test_user.address),
            supply: 0,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::Some(test_user.address),
        })
    );
    let test_token_account = LocalnetAccount::new(
        Pubkey::new_unique(),
        "test_user_token_act.json".to_string(),
        anchor_spl::token::TokenAccount::from(spl_token::state::Account {
            mint: test_mint.address,
            owner: test_user.address,
            amount: 0,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::Some(test_user.address)
        })
    );
    vec![
        test_user,
        test_mint,
        test_token_account,
    ]
}
