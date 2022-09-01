use crate::{AccountId, Assets, Authorship, Balances, NegativeImbalance, Runtime};
use frame_support::traits::{
	fungibles::{Balanced, CreditOf},
	Currency, OnUnbalanced,
};
use pallet_asset_tx_payment::HandleCredit;

pub struct Author;
impl OnUnbalanced<NegativeImbalance> for Author {
	fn on_nonzero_unbalanced(amount: NegativeImbalance) {
		if let Some(author) = Authorship::author() {
			Balances::resolve_creating(&author, amount);
		}
	}
}

/// A `HandleCredit` implementation that naively transfers the fees to the block author.
/// Will drop and burn the assets in case the transfer fails.
pub struct CreditToBlockAuthor;
impl HandleCredit<AccountId, Assets> for CreditToBlockAuthor {
	fn handle_credit(credit: CreditOf<AccountId, Assets>) {
		if let Some(author) = pallet_authorship::Pallet::<Runtime>::author() {
			// Drop the result which will trigger the `OnDrop` of the imbalance in case of error.
			let _ = Assets::resolve(&author, credit);
		}
	}
}

use crate::*;
use sp_runtime::SaturatedConversion;
use codec::Encode;

pub mod ocw_signer {
    use super::*;
    // ---------------------- offchain Pallet Configurations ----------------------

    /// Payload data to be signed when making signed transaction from off-chain workers,
    ///   inside `create_transaction` function.
    //pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;

    impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
    where
        Call: From<LocalCall>,
    {
        fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
            call: Call,
            public: <Signature as sp_runtime::traits::Verify>::Signer,
            account: AccountId,
            index: Index,
        ) -> Option<(
            Call,
            <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
        )> {
            let period = BlockHashCount::get() as u64;
            let current_block = System::block_number()
                .saturated_into::<u64>()
                .saturating_sub(1);
            let tip = 0;
			
            let extra: SignedExtra = (
                frame_system::CheckNonZeroSender::<Runtime>::new(),
                frame_system::CheckSpecVersion::<Runtime>::new(),
                frame_system::CheckTxVersion::<Runtime>::new(),
                frame_system::CheckGenesis::<Runtime>::new(),
                frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
                frame_system::CheckNonce::<Runtime>::from(index),
                frame_system::CheckWeight::<Runtime>::new(),
                //pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
				pallet_asset_tx_payment::ChargeAssetTxPayment::<Runtime>::from(tip, None),

            );

            #[cfg_attr(not(feature = "std"), allow(unused_variables))]
            let raw_payload = SignedPayload::new(call, extra)
                .map_err(|_e| {
                })
                .ok()?;

            let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;

            let address = account;
            let (call, extra, _) = raw_payload.deconstruct();
            Some((call, (sp_runtime::MultiAddress::Id(address), signature, extra)))
        }
    }

    impl frame_system::offchain::SigningTypes for Runtime {
        type Public = <Signature as sp_runtime::traits::Verify>::Signer;
        type Signature = Signature;
    }

    impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
    where
        Call: From<C>,
    {
        type OverarchingCall = Call;
        type Extrinsic = UncheckedExtrinsic;
    }

    //------------End of Offchain pallet configuration--------------------------------

}