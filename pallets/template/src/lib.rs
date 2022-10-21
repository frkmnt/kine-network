//** About **//
	// Information regarding the pallet
	
	
	#![cfg_attr(not(feature = "std"), no_std)]

	pub use pallet::*;
	
	#[cfg(test)]
	mod mock;
	
	#[cfg(test)]
	mod tests;
	
	#[cfg(feature = "runtime-benchmarks")]
	mod benchmarking;
	

#[frame_support::pallet]
pub mod pallet {
	
	//** Config **//

		//* Imports *//
			
			use frame_support::pallet_prelude::*;
			use frame_system::pallet_prelude::*;

		//* Config *//

			#[pallet::pallet]
			#[pallet::generate_store(pub(super) trait Store)]
			pub struct Pallet<T>(_);

			#[pallet::config]
			pub trait Config: frame_system::Config {
				type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
			}


			
	//** Types **//	
	
		//* Types *//
		//* Constants *//
		//* Enums *//
		//* Structs *//



	//** Storage **//
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	
	
	//** Events **//

		#[pallet::event]
		#[pallet::generate_deposit(pub(super) fn deposit_event)]
		pub enum Event<T: Config> {
			SomethingStored(u32, T::AccountId),
		}
	


	//** Errors **//

		#[pallet::error]
		pub enum Error<T> {
			NoneValue,
			StorageOverflow,
		}



	//** Hooks **//

		#[pallet::hooks]
		impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	

		
	//** Extrinsics **//
		
		#[pallet::call]
		impl<T:Config> Pallet<T> {

			//* Examples *//
			
				#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
				pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
					let who = ensure_signed(origin)?;
					<Something<T>>::put(something);
					Self::deposit_event(Event::SomethingStored(something, who));
					Ok(())
				}
		
				#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
				pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
					let _who = ensure_signed(origin)?;
		
					match <Something<T>>::get() {
						None => return Err(Error::<T>::NoneValue.into()),
						Some(old) => {
							let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
							<Something<T>>::put(new);
							Ok(())
						},
					}
				}

		}
	
	
	
	//** Helpers **//
	
		impl<T:Config> Pallet<T> {
					
		}
	
}