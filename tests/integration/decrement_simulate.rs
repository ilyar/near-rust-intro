use crate::utils::init;
use near_sdk_sim::DEFAULT_GAS;

#[test]
fn default() {
    let (root, contract, _alice) = init();

    let result = root.call(
        contract.account_id(),
        "decrement",
        &[].to_vec(),
        DEFAULT_GAS,
        0, // deposit
    );

    println!(
        "burnt tokens: {:.04}Ⓝ gas: {:.01} TeraGas",
        (result.tokens_burnt()) as f64 / 1e24,
        (result.gas_burnt()) as f64 / 1e12,
    );

    let expected = 27 * u64::pow(10, 13); // 2.7 TeraGas
    assert!(result.gas_burnt() <= expected);

    let actual: i8 = root
        .view(contract.account_id(), "get_num", &[].to_vec())
        .unwrap_json();

    assert_eq!(-1, actual);
}
