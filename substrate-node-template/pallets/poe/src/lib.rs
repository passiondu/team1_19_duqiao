#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, ensure, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtimse's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		 Proofs get(fn proofs):map hasher(blake2_128_concat) Vec<u8> => (T::AccountId,T::BlockNumber);
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		//Something get(fn something): Option<u32>;
	}
} 

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		ClaimCreated(AccountId,Vec<u8>),
		ClaimRevoked(AccountId,Vec<u8>),
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		//SomethingStored(u32, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		/// Error names should be descriptive.
		//NoneValue,
		/// Errors should have helpful documentation associated with them.
		//StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

        #[weight = 0]
        pub fn create_claim(origin,claim: Vec<u8>) -> dispatch::DispatchResult {
                let  sender = ensure_signed(origin)?;
                ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);
                Proofs::<T>::insert(&claim,(sender.clone(),frame_system::Module::<T>::block_number));
                Self::deposit_event(RawEvent::ClaimCreated(sender,claim));
                Ok(())
        }

        #[weight = 0]
        pub fn revoke_claim(origin,claim: Vec<u8>) -> dispatch::DispatchResult {
                let  sender = ensure_signed(origin)?;
                ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);
                let(owner,_block_number)=Proofs::<T>::get(&claim);
                ensure!(owner==sender,Error::<T>::NotClaimOwner);
                Proofs::<T>::remove(&claim);
                Self::deposit_event(RawEvent::ClaimRevoked(sender,claim));
                Ok(())
               
		}
		
		#[weight = 10_000]
        fn transfer_claim(origin, proof: Vec<u8>,sender: T::AccountId)
        {
            let owner = ensure_signed(origin)?;
            let (get,_) = <Proofs<T>>::get(&proof);
            ensure!(get == owner, Error::<T>::NotProofOwner);
            ensure!(<Proofs<T>>::contains_key(&proof), Error::<T>::NoSuchProof);
            ensure!(sender != owner, Error::<T>::SameOwner);

            <Proofs<T>>::remove(&proof);
            <Proofs<T>>::insert(&proof, (&sender, <frame_system::Module<T>>::block_number()) );

            Self::deposit_event(RawEvent::ClaimTransfer(owner,sender, proof));
        }

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}
