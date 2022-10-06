// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use cfg_traits::{OrderManager, TrancheCurrency};
	use cfg_types::{FulfillmentWithPrice, PoolLocator, TotalOrder};
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		PalletId,
	};
	use frame_system::pallet_prelude::BlockNumberFor;
	use sp_runtime::{traits::AccountIdConversion, FixedPointNumber, FixedPointOperand};

	pub struct OrderManagerAccount;

	impl OrderManagerAccount {
		pub const LOCAL_ID: PalletId = PalletId(*b"OrdrMngr");

		pub fn get<T: frame_system::Config>() -> T::AccountId {
			OrderManagerAccount::LOCAL_ID.into_account_truncating()
		}
	}

	type BalanceOf<T> =
		<<T as Config>::Tokens as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
	type CurrencyOf<T> =
		<<T as Config>::Tokens as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;

	#[pallet::config]
	pub trait Config: frame_system::Config
	where
		<Self::Tokens as Inspect<Self::AccountId>>::Balance:
			From<u64> + FixedPointOperand + MaxEncodedLen + MaybeSerializeDeserialize,
		<Self::Tokens as Inspect<Self::AccountId>>::AssetId:
			MaxEncodedLen + MaybeSerializeDeserialize,
	{
		type FundsAccount: Get<PalletId>;

		type PoolId: Member + Parameter + Default + Copy + MaxEncodedLen;

		type TrancheId: Member + Parameter + Default + Copy + MaxEncodedLen;

		type InvestmentId: Member
			+ Parameter
			+ Copy
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ Into<CurrencyOf<Self>>
			+ TrancheCurrency<Self::PoolId, Self::TrancheId>;

		type Rate: FixedPointNumber<Inner = BalanceOf<Self>>;

		type Tokens: Inspect<Self::AccountId> + Mutate<Self::AccountId> + Transfer<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config>
	where
		<T::Tokens as Inspect<T::AccountId>>::Balance:
			From<u64> + FixedPointOperand + MaxEncodedLen + MaybeSerializeDeserialize,
		<T::Tokens as Inspect<T::AccountId>>::AssetId: MaxEncodedLen + MaybeSerializeDeserialize,
	{
		pub invest_orders: Vec<(T::InvestmentId, BalanceOf<T>, CurrencyOf<T>)>,
		pub redeem_orders: Vec<(T::InvestmentId, BalanceOf<T>, CurrencyOf<T>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T>
	where
		<T::Tokens as Inspect<T::AccountId>>::Balance:
			From<u64> + FixedPointOperand + MaxEncodedLen + MaybeSerializeDeserialize,
		<T::Tokens as Inspect<T::AccountId>>::AssetId: MaxEncodedLen + MaybeSerializeDeserialize,
	{
		fn default() -> Self {
			Self {
				invest_orders: Default::default(),
				redeem_orders: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T>
	where
		<T::Tokens as Inspect<T::AccountId>>::Balance:
			From<u64> + FixedPointOperand + MaxEncodedLen + MaybeSerializeDeserialize,
		<T::Tokens as Inspect<T::AccountId>>::AssetId: MaxEncodedLen + MaybeSerializeDeserialize,
	{
		fn build(&self) {
			for (id, amount, payment_currency) in &self.invest_orders {
				InvestOrders::<T>::insert(*id, TotalOrder { amount: *amount });
				if !PaymentCurrency::<T>::contains_key(id) {
					PaymentCurrency::<T>::insert(*id, *payment_currency);
				}
			}
			for (id, amount, payment_currency) in &self.redeem_orders {
				RedeemOrders::<T>::insert(*id, TotalOrder { amount: *amount });
				if !PaymentCurrency::<T>::contains_key(id) {
					PaymentCurrency::<T>::insert(*id, *payment_currency);
				}
			}
		}
	}

	#[pallet::storage]
	pub type PaymentCurrency<T: Config> =
		StorageMap<_, Blake2_128Concat, T::InvestmentId, CurrencyOf<T>>;

	#[pallet::storage]
	pub type InvestOrders<T: Config> =
		StorageMap<_, Blake2_128Concat, T::InvestmentId, TotalOrder<BalanceOf<T>>>;

	#[pallet::storage]
	pub type RedeemOrders<T: Config> =
		StorageMap<_, Blake2_128Concat, T::InvestmentId, TotalOrder<BalanceOf<T>>>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		<T::Tokens as Inspect<T::AccountId>>::Balance:
			From<u64> + FixedPointOperand + MaxEncodedLen + MaybeSerializeDeserialize,
		<T::Tokens as Inspect<T::AccountId>>::AssetId: MaxEncodedLen + MaybeSerializeDeserialize,
	{
		// TODO: Remove once we are on Substrate:polkadot-v0.9.29
	}
	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		<T::Tokens as Inspect<T::AccountId>>::Balance:
			From<u64> + FixedPointOperand + MaxEncodedLen + MaybeSerializeDeserialize,
		<T::Tokens as Inspect<T::AccountId>>::AssetId: MaxEncodedLen + MaybeSerializeDeserialize,
	{
		// TODO: Remove once we are on Substrate:polkadot-v0.9.29
	}

	impl<T: Config> Pallet<T>
	where
		<T::Tokens as Inspect<T::AccountId>>::Balance:
			From<u64> + FixedPointOperand + MaxEncodedLen + MaybeSerializeDeserialize,
		<T::Tokens as Inspect<T::AccountId>>::AssetId: MaxEncodedLen + MaybeSerializeDeserialize,
	{
		/// **Test Method**
		///
		/// Define an `InvestmentId` and this logic will mint the given `amount` into the
		/// `TEST_PALLET_ID` we intermediately store all tokens for investing.
		///
		/// **This mints `PaymentCurrency` tokens**
		pub fn update_invest_order(
			investment_id: T::InvestmentId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			Pallet::<T>::invest_orders(investment_id)?;
			let mut orders =
				InvestOrders::<T>::get(&investment_id).unwrap_or(TotalOrder::default());
			orders.amount += amount;
			InvestOrders::<T>::insert(&investment_id, orders);

			T::Tokens::transfer(
				PaymentCurrency::<T>::get(&investment_id)
					.expect("PaymentCurrency is provided in testing. Qed."),
				&T::FundsAccount::get().into_account_truncating(),
				&OrderManagerAccount::get::<T>(),
				amount,
				false,
			)
			.map(|_| ())
		}

		/// **Test Method**
		///
		/// Define an `InvestmentId` and this logic will mint the given `amount` into the
		/// `TEST_PALLET_ID` we intermediately store all tokens for investing.
		///
		/// **This mints `TrancheToken`s**
		pub fn update_redeem_order(
			investment_id: T::InvestmentId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let mut orders =
				RedeemOrders::<T>::get(&investment_id).unwrap_or(TotalOrder::default());
			orders.amount += amount;
			RedeemOrders::<T>::insert(&investment_id, orders);

			/*
			T::Tokens::transfer(
				investment_id.into(),
				&T::FundsAccount::get().into_account_truncating(),
				&OrderManagerAccount::get::<T>(),
				amount,
				false,
			)
			.map(|_| ())
			 */
			Ok(())
		}
	}

	impl<T: Config> OrderManager for Pallet<T>
	where
		<T::Tokens as Inspect<T::AccountId>>::Balance:
			From<u64> + FixedPointOperand + MaxEncodedLen + MaybeSerializeDeserialize,
		<T::Tokens as Inspect<T::AccountId>>::AssetId: MaxEncodedLen + MaybeSerializeDeserialize,
	{
		type Error = DispatchError;
		type Fulfillment = FulfillmentWithPrice<T::Rate>;
		type InvestmentId = T::InvestmentId;
		type Orders = TotalOrder<BalanceOf<T>>;

		/// When called the manager return the current
		/// invest orders for the given investment class.
		fn invest_orders(asset_id: Self::InvestmentId) -> Result<Self::Orders, Self::Error> {
			Ok(InvestOrders::<T>::get(asset_id).unwrap_or(TotalOrder::default()))
		}

		/// When called the manager return the current
		/// redeem orders for the given investment class.
		fn redeem_orders(asset_id: Self::InvestmentId) -> Result<Self::Orders, Self::Error> {
			Ok(RedeemOrders::<T>::get(asset_id).unwrap_or(TotalOrder::default()))
		}

		fn process_invest_orders(
			asset_id: Self::InvestmentId,
		) -> Result<Self::Orders, Self::Error> {
			Ok(InvestOrders::<T>::get(asset_id).unwrap_or(TotalOrder::default()))
		}

		fn process_redeem_orders(
			asset_id: Self::InvestmentId,
		) -> Result<Self::Orders, Self::Error> {
			Ok(RedeemOrders::<T>::get(asset_id).unwrap_or(TotalOrder::default()))
		}

		/// Signals the manager that the previously
		/// fetch invest orders for a given investment class
		/// will be fulfilled by fulfillment.
		fn invest_fulfillment(
			asset_id: Self::InvestmentId,
			fulfillment: Self::Fulfillment,
		) -> Result<(), Self::Error> {
			let orders = InvestOrders::<T>::get(asset_id).unwrap_or(TotalOrder::default());
			InvestOrders::<T>::insert(asset_id, TotalOrder::default());

			// Move tokens to pools
			let tokens_to_transfer_to_pool = fulfillment.of_amount.mul_floor(orders.amount);
			T::Tokens::mint_into(
				PaymentCurrency::<T>::get(asset_id)
					.expect("PaymentCurrency is provided in testing. Qed."),
				&PoolLocator {
					pool_id: asset_id.of_pool(),
				}
				.into_account_truncating(),
				tokens_to_transfer_to_pool,
			)
			.expect("Minting must work. Qed.");

			// Update local order
			InvestOrders::<T>::insert(
				asset_id,
				TotalOrder {
					amount: orders.amount - tokens_to_transfer_to_pool,
				},
			);

			// Mint tranche tokens into test pallet-id
			let tranche_tokens_to_mint = fulfillment
				.price
				.reciprocal()
				.unwrap()
				.checked_mul_int(tokens_to_transfer_to_pool)
				.unwrap();
			T::Tokens::mint_into(
				asset_id.into(),
				&OrderManagerAccount::get::<T>(),
				tranche_tokens_to_mint,
			)
			.expect("Minting must work. Qed.");

			Ok(())
		}

		/// Signals the manager that the previously
		/// fetch redeem orders for a given investment class
		/// will be fulfilled by fulfillment.
		fn redeem_fulfillment(
			asset_id: Self::InvestmentId,
			fulfillment: Self::Fulfillment,
		) -> Result<(), Self::Error> {
			let orders = RedeemOrders::<T>::get(asset_id).unwrap_or(TotalOrder::default());
			RedeemOrders::<T>::insert(asset_id, TotalOrder::default());

			let tokens_to_burn_from_test_pallet = fulfillment.of_amount.mul_floor(orders.amount);
			T::Tokens::burn_from(
				asset_id.into(),
				&OrderManagerAccount::get::<T>(),
				tokens_to_burn_from_test_pallet,
			)
			.expect("Burning must work. Qed.");

			// Update local order
			// Update local order
			RedeemOrders::<T>::insert(
				asset_id,
				TotalOrder {
					amount: orders.amount - tokens_to_burn_from_test_pallet,
				},
			);

			// Burn payment currency from pool
			let payment_currency_to_burn = fulfillment
				.price
				.checked_mul_int(tokens_to_burn_from_test_pallet)
				.unwrap();
			T::Tokens::burn_from(
				PaymentCurrency::<T>::get(asset_id)
					.expect("PaymentCurrency is provided in testing. Qed."),
				&PoolLocator {
					pool_id: asset_id.of_pool(),
				}
				.into_account_truncating(),
				payment_currency_to_burn,
			)
			.expect("Minting must work. Qed.");

			Ok(())
		}
	}
}
