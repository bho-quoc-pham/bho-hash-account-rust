use sp_core::{Hasher, H160, H256};
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::{AccountId32};

pub trait AddressMapping<A> {
	fn into_account_id(address: H160) -> A;
}

pub struct HashedAddressMapping<H>(sp_std::marker::PhantomData<H>);

impl<H: Hasher<Out = H256>> AddressMapping<AccountId32> for HashedAddressMapping<H> {
	fn into_account_id(address: H160) -> AccountId32 {
		let mut data = [0u8; 24];
		data[0..4].copy_from_slice(b"evm:");
		data[4..24].copy_from_slice(&address[..]);
		let hash = H::hash(&data);

        // println!("{}", hash);

		AccountId32::from(Into::<[u8; 32]>::into(hash))
	}
}

fn main() {
    let data = HashedAddressMapping::<BlakeTwo256>
                ::into_account_id(
                    H160::from_slice(
                        &hex_literal::hex!("3C5aaa434D8D47B37AD9aa055C5D3256C228A359")
                    )
                );
    print!("{}", data);
}
