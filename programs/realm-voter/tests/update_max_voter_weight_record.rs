use crate::program_test::realm_voter_test::{ConfigureGovernanceProgramArgs, RealmVoterTest};
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_update_max_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    realm_voter_test
        .with_governance_program_config(
            &registrar_cookie,
            &governance_program_cookie,
            Some(ConfigureGovernanceProgramArgs { weight: 1 }),
        )
        .await?;

    let mut max_voter_weight_record_cookie = realm_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    realm_voter_test.bench.advance_clock().await;
    let clock = realm_voter_test.bench.get_clock().await;

    // Act
    realm_voter_test
        .update_max_voter_weight_record(&registrar_cookie, &mut max_voter_weight_record_cookie, 110)
        .await?;

    // Assert

    let max_voter_weight_record = realm_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(max_voter_weight_record.max_voter_weight, 110);
    assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
    assert_eq!(max_voter_weight_record.realm, realm_cookie.address);
    assert_eq!(
        max_voter_weight_record.governing_token_mint,
        realm_cookie.account.community_mint
    );

    Ok(())
}
