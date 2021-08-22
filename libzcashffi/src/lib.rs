use libc::{c_char, c_uint, c_ulong};
use std::ffi::{CStr, CString};
use zcash_client_backend::encoding::{decode_payment_address, decode_transparent_address};
use zcash_primitives::{
    consensus::{BlockHeight, Network},
    constants::mainnet::{
        B58_PUBKEY_ADDRESS_PREFIX, B58_SCRIPT_ADDRESS_PREFIX, HRP_SAPLING_PAYMENT_ADDRESS,
    },
    legacy::{Script, TransparentAddress},
    transaction::{
        builder::Builder,
        components::{
            transparent::{OutPoint, TxOut},
            Amount,
        },
    },
};
use zcash_proofs::prover::LocalTxProver;

#[repr(C)]
pub struct UTXO {
    transaction_hash: *const c_char,
    index: c_uint,
    amount: c_ulong,
    private_key: *const c_char,
}

#[repr(C)]
pub struct Response {
    transaction_id: *const c_char,
    raw: *const c_char,
}

#[no_mangle]
pub extern "C" fn build_transaction(
    input_length: u32,
    inputs_ptr: *mut UTXO,
    to: *const c_char,
    amount: u64,
    change: *const c_char,
    height: u32,
    spend_params: *const u8,
    spend_params_len: u32,
    output_params: *const u8,
    output_params_len: u32,
) -> *mut Response {
    let height = BlockHeight::from_u32(height);
    let params = Network::MainNetwork;

    let items: &mut [UTXO] = unsafe {
        assert!(!inputs_ptr.is_null());
        std::slice::from_raw_parts_mut(inputs_ptr, input_length as usize)
    };

    let to_address = unsafe { CStr::from_ptr(to).to_str().unwrap() };
    let change_address = unsafe { CStr::from_ptr(change).to_str().unwrap() };

    let spend_params =
        unsafe { std::slice::from_raw_parts(spend_params, spend_params_len as usize) };
    let output_params =
        unsafe { std::slice::from_raw_parts(output_params, output_params_len as usize) };

    let mut total = 0u64;
    let mut builder = Builder::new(params.clone(), BlockHeight::from(height));
    for item in items {
        let transaction_hash = unsafe { CStr::from_ptr(item.transaction_hash) };
        let private_key = unsafe { CStr::from_ptr(item.private_key) };
        let mut hash = [0u8; 32];
        hex::decode_to_slice(transaction_hash.to_str().unwrap(), &mut hash as &mut [u8]).unwrap();
        let output = OutPoint::new(hash, item.index);
        let mut secret = [0u8; 32];
        hex::decode_to_slice(private_key.to_str().unwrap(), &mut secret as &mut [u8]).unwrap();
        let secret_key = secp256k1::SecretKey::from_slice(&secret).unwrap();

        //let pubkey = secp256k1::PublicKey::from_slice(&[0u8, 32]);

        total = total + amount;
        let coin = TxOut {
            value: Amount::from_u64(amount).unwrap(),
            script_pubkey: Script::default(), // TODO
        };

        builder
            .add_transparent_input(secret_key, output, coin)
            .unwrap();
    }

    let to = decode_payment_address(HRP_SAPLING_PAYMENT_ADDRESS, to_address)
        .unwrap()
        .unwrap();
    builder
        .add_sapling_output(None, to, Amount::from_u64(amount).unwrap(), None)
        .unwrap();

    // 1000 is DEFAULT_FEE
    let default_fee = 1000u64;
    if total > amount + default_fee {
        let to = decode_transparent_address(
            &B58_PUBKEY_ADDRESS_PREFIX,
            &B58_SCRIPT_ADDRESS_PREFIX,
            change_address,
        )
        .unwrap()
        .unwrap();
        builder
            .add_transparent_output(&to, Amount::from_u64(total - amount - default_fee).unwrap())
            .unwrap();
    }

    let tx_prover = LocalTxProver::from_bytes(spend_params, output_params);
    let (transaction, _) = builder.build(&tx_prover).unwrap();

    let mut raw = vec![];
    transaction.write(&mut raw).unwrap();

    let resp = Response {
        transaction_id: CString::new(hex::encode(transaction.txid().as_ref()))
            .unwrap()
            .into_raw(),
        raw: CString::new(hex::encode(raw)).unwrap().into_raw(),
    };

    Box::into_raw(Box::new(resp))
}
