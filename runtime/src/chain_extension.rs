use codec::{Encode};
use frame_support::debug::{error, native};
use frame_support::traits::Randomness;
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use sp_runtime::DispatchError;
use pallet_contracts::Config;
use sp_std::vec::Vec;
// use sp_io::hashing::blake2_128;
/// contract extension for `FetchRandom`
pub struct FetchRandomExtension;

impl<C: Config> ChainExtension<C> for FetchRandomExtension {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
        where
            <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {

        match func_id {
            1101 => {
                let mut env = env.buf_in_buf_out();
                let random_seed: [u8; 32] = super::RandomnessCollectiveFlip::random_seed().0;
                let random_slice = random_seed.encode();
                native::trace!(
                    target: "runtime",
                    "[ChainExtension]|call|func_id:{:}",
                    func_id
                );
                env.write(&random_slice, false, None)
                    .map_err(|_| DispatchError::Other("ChainExtension failed to call random"))?;
            }

            1102 => {
                let mut env = env.buf_in_buf_out();
                let random_slice: Vec<u8> = env.read_as()?;

                let result=80;
                let resultu8=result.encode();
                native::trace!(
                    target: "runtime",
                    "[ChainExtension]|call|func_id:{:}",
                    func_id
                );
                env.write(&resultu8, false, None)
                    .map_err(|_| DispatchError::Other("ChainExtension failed to call price"))?;
            }

            1103 => {
                let mut env = env.buf_in_buf_out();
                let token_identifier: Vec<u8> = env.read_as()?;

                let price:u64 = 60000;

                env.write(&price.encode(), false, None)
                    .map_err(|_| DispatchError::Other("Get price fail"))?;
            }

            _ => {
                error!("call an unregistered `func_id`, func_id:{:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"));
            }
        }
        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}
