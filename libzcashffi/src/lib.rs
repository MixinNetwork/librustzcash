use libc::c_char;
use std::ffi::{CStr, CString};
use zcash_client_backend::{address::RecipientAddress, encoding::decode_transparent_address};
use zcash_primitives::{
    consensus::{BlockHeight, Network},
    constants::mainnet::{B58_PUBKEY_ADDRESS_PREFIX, B58_SCRIPT_ADDRESS_PREFIX},
    legacy::Script,
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
    index: u32,
    amount: u64,
    private_key: *const c_char,
}

#[repr(C)]
pub struct Response {
    transaction_id: *const c_char,
    raw: *const c_char,
    output_index: u64,
    output_amount: u64,
    change_index: u64,
    change_amount: u64,
}

#[no_mangle]
pub extern "C" fn build_transaction(
    inputs_ptr: *mut UTXO,
    input_length: u32,
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

    let mut builder = Builder::new(params.clone(), BlockHeight::from(height));

    let mut total = 0u64;
    for item in items {
        let transaction_hash = unsafe { CStr::from_ptr(item.transaction_hash) };
        let private_key = unsafe { CStr::from_ptr(item.private_key) };
        let mut hash = [0u8; 32];
        hex::decode_to_slice(transaction_hash.to_str().unwrap(), &mut hash as &mut [u8]).unwrap();
        let output = OutPoint::new(hash, item.index);

        let secret = hex::decode(private_key.to_str().unwrap()).unwrap();
        let secret_key = secp256k1::SecretKey::from_slice(secret.as_slice()).unwrap();

        total = total + item.amount;
        let coin = TxOut {
            value: Amount::from_u64(amount).unwrap(),
            script_pubkey: Script::default(), // TODO for TransparentAddress::PublicKey there don't have script_pubkey?
        };

        builder
            .add_transparent_input(secret_key, output, coin)
            .unwrap();
    }

    let to = RecipientAddress::decode(&params.clone(), to_address).unwrap();
    let mut change_index = 0u64;
    match to {
        RecipientAddress::Shielded(addr) => {
            builder
                .add_sapling_output(None, addr, Amount::from_u64(amount).unwrap(), None)
                .unwrap();
        }
        RecipientAddress::Transparent(addr) => {
            change_index = 1u64;
            builder
                .add_transparent_output(&addr, Amount::from_u64(amount).unwrap())
                .unwrap();
        }
    };

    // 1000 is DEFAULT_FEE
    let default_fee = 1000u64;
    if total - default_fee > amount {
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
        output_index: 0,
        output_amount: amount,
        change_index: change_index,
        change_amount: total - amount - default_fee,
    };

    Box::into_raw(Box::new(resp))
}