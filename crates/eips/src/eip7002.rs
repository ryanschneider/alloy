//! Contains the system contract and [WithdrawalRequest] types, first introduced in the [Prague hardfork](https://github.com/ethereum/execution-apis/blob/main/src/engine/prague.md).
//!
//! See also [EIP-7002](https://eips.ethereum.org/EIPS/eip-7002): Execution layer triggerable withdrawals

use alloy_primitives::{address, bytes, Address, Bytes, FixedBytes};
use alloy_rlp::{RlpDecodable, RlpEncodable};
use serde_with::{serde_as, DisplayFromStr};

/// The caller to be used when calling the EIP-7002 withdrawal requests contract at the end of the
/// block.
pub const SYSTEM_ADDRESS: Address = address!("fffffffffffffffffffffffffffffffffffffffe");

/// The address for the EIP-7002 withdrawal requests contract.
pub const WITHDRAWAL_REQUEST_PREDEPLOY_ADDRESS: Address =
    address!("09Fc772D0857550724b07B850a4323f39112aAaA");

/// The code for the EIP-7002 withdrawal requests contract.
pub static WITHDRAWAL_REQUEST_PREDEPLOY_CODE: Bytes = bytes!("   3373fffffffffffffffffffffffffffffffffffffffe1460c7573615156028575f545f5260205ff35b36603814156101f05760115f54807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff146101f057600182026001905f5b5f821115608057810190830284830290049160010191906065565b9093900434106101f057600154600101600155600354806003026004013381556001015f35815560010160203590553360601b5f5260385f601437604c5fa0600101600355005b6003546002548082038060101160db575060105b5f5b81811461017f5780604c02838201600302600401805490600101805490600101549160601b83528260140152807fffffffffffffffffffffffffffffffff0000000000000000000000000000000016826034015260401c906044018160381c81600701538160301c81600601538160281c81600501538160201c81600401538160181c81600301538160101c81600201538160081c81600101535360010160dd565b9101809214610191579060025561019c565b90505f6002555f6003555b5f54807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff14156101c957505f5b6001546002828201116101de5750505f6101e4565b01600290035b5f555f600155604c025ff35b5f5ffd");

/// The [EIP-7685](https://eips.ethereum.org/EIPS/eip-7685) request type for withdrawal requests.
pub const WITHDRAWAL_REQUEST_TYPE: u8 = 0x01;

/// Represents an execution layer triggerable withdrawal request.
///
/// See [EIP-7002](https://eips.ethereum.org/EIPS/eip-7002).
#[cfg_attr(feature = "serde", serde_as)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, RlpEncodable, RlpDecodable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ssz", derive(ssz_derive::Encode, ssz_derive::Decode))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct WithdrawalRequest {
    /// Address of the source of the exit.
    pub source_address: Address,
    /// Validator public key.
    pub validator_pubkey: FixedBytes<48>,
    /// Amount of withdrawn ether in gwei.
    #[serde_as(as = "DisplayFromStr")]
    pub amount: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex;
    use alloy_rlp::{Decodable, Encodable};
    use core::str::FromStr;

    #[test]
    fn test_encode_decode_request_roundtrip() {
        // Define multiple test cases as tuples containing the test data.
        //
        // Examples are randomly generated using some random validators found on Etherscan.
        let test_cases = vec![
            (
                Address::from_str("0xaE0E8770147AaA6828a0D6f642504663F10F7d1E").unwrap(),
                FixedBytes::<48>::from(hex!("8e8d8749f6bc79b78be7cc6e49ff640e608454840c360b344c3a4d9b7428e280e7f40d2271bad65d8cbbfdd43cb8793b")),
                10
            ),
            (
                Address::from_str("0xf86f8D6A7d2AF439245c1145d88B04dAf2d7e509").unwrap(),
                FixedBytes::<48>::from(hex!("a85d7a6aa90eedebe103b8d4d3dc86003aea8b6c8159d9d50f7685828bc97d211b2c512b1dcbb8d63b60a56c91dda8ea")),
                354
            ),
            (
                Address::from_str("0xf86f8D6A7d2AF439245c1145d88B04dAf2d7e509").unwrap(),
                FixedBytes::<48>::from(hex!("a77eec36b046fbbf088e9253aa8c6800863d882c56fc6fa04800bbed742820f1bc7eb837601322840a18bbe0d24893b2")),
                19
            ),
            (
                Address::from_str("0xAFedF06777839D59eED3163cC3e0A5057b514399").unwrap(),
                FixedBytes::<48>::from(hex!("a3ecb9359401bb22d00cefddf6f6879d14a2ee74d3325cc8cdff0796bd0b3b47c5f5b4d02e5a865d7b639eb8124286a5")),
                9
            ),
        ];

        // Iterate over each test case
        for (source_address, validator_pubkey, amount) in test_cases {
            let original_request = WithdrawalRequest { source_address, validator_pubkey, amount };

            // Encode the request
            let mut buf = Vec::new();
            original_request.encode(&mut buf);

            // Decode the request
            let decoded_request =
                WithdrawalRequest::decode(&mut &buf[..]).expect("Failed to decode request");

            // Ensure the encoded and then decoded request matches the original
            assert_eq!(original_request, decoded_request);
        }
    }

    #[test]
    fn test_serde_withdrawal_request() {
        // Sample JSON input representing a withdrawal request
        let json_data = r#"{
            "sourceAddress":"0xAE0E8770147AaA6828a0D6f642504663F10F7d1E",
            "validatorPubkey":"0x8e8d8749f6bc79b78be7cc6e49ff640e608454840c360b344c3a4d9b7428e280e7f40d2271bad65d8cbbfdd43cb8793b",
            "amount":"0x1"
        }"#;

        // Deserialize the JSON into a WithdrawalRequest struct
        let withdrawal_request: WithdrawalRequest =
            serde_json::from_str(json_data).expect("Failed to deserialize");

        // Verify the deserialized content
        assert_eq!(
            withdrawal_request.source_address,
            Address::from_str("0xAE0E8770147AaA6828a0D6f642504663F10F7d1E").unwrap()
        );
        assert_eq!(
            withdrawal_request.validator_pubkey,
            FixedBytes::<48>::from(hex!("8e8d8749f6bc79b78be7cc6e49ff640e608454840c360b344c3a4d9b7428e280e7f40d2271bad65d8cbbfdd43cb8793b"))
        );
        assert_eq!(withdrawal_request.amount, 1);

        // Serialize the struct back into JSON
        let serialized_json =
            serde_json::to_string(&withdrawal_request).expect("Failed to serialize");

        // Check if the serialized JSON matches the expected JSON structure
        let expected_json = r#"{"sourceAddress":"0xae0e8770147aaa6828a0d6f642504663f10f7d1e","validatorPubkey":"0x8e8d8749f6bc79b78be7cc6e49ff640e608454840c360b344c3a4d9b7428e280e7f40d2271bad65d8cbbfdd43cb8793b","amount":"0x1"}"#;
        assert_eq!(serialized_json, expected_json);
    }
}
