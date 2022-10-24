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

            use frame_support::{pallet_prelude::*,BoundedVec};
            use frame_system::pallet_prelude::*;
            use codec::{Decode, Encode, MaxEncodedLen};
            use sp_runtime::{RuntimeDebug, traits::{AtLeast32BitUnsigned, CheckedAdd, One}};
            use scale_info::{TypeInfo};
            use scale_info::prelude::vec::Vec;
            use core::convert::TryInto;
	
        //* Config *//
        
            #[pallet::pallet]
            #[pallet::generate_store(pub(super) trait Store)]
            pub struct Pallet<T>(_);

            #[pallet::config]
            pub trait Config: frame_system::Config {
                type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
                type MovieId: Member  + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;
                /// The maximum length of base uri stored on-chain.
                #[pallet::constant]
                type StringLimit: Get<u32>;
            }

    

	//** Types **//	
	
		//* Types *//
		//* Constants *//
		//* Enums *//
		//* Structs *//

            #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen,TypeInfo)]
            pub struct Movie<AccountId,BoundedString> {
                pub	owner: AccountId,
                pub name:BoundedString,
                pub synopsis:BoundedString,
                pub movie_description:BoundedString,
                pub classification:u32,
                pub release:BoundedString,
                pub director:BoundedString,
                pub lang:BoundedString,
                pub country:BoundedString,
                pub rating:u32,
                pub aspect_ratio:BoundedString,
                pub tags:BoundedString,
                pub trailer:BoundedString,
                pub imdb:BoundedString,
                pub social:BoundedString,
                pub ipfs:BoundedString,
                pub link:BoundedString,
            }



    //** Storage **//

        #[pallet::storage]
        pub type Movies<T: Config> =
            StorageMap<_, Blake2_128Concat, T::MovieId, Movie<T::AccountId,BoundedVec<u8, T::StringLimit>>>;

        #[pallet::storage]
        #[pallet::getter(fn next_movie_id)]
        pub(super) type NextMovieId<T: Config> = StorageValue<_, T::MovieId, ValueQuery>;



	//** Events **//

        #[pallet::event]
        #[pallet::generate_deposit(pub(super) fn deposit_event)]
        pub enum Event<T: Config> {
            MovieCreated(T::MovieId, T::AccountId),
        }
   


    //** Errors **//

        #[pallet::error]
        pub enum Error<T> {
            NoAvailableMovieId,
            Overflow,
            Underflow,
            BadMetadata,
        }



    //** Hooks **//

        #[pallet::hooks]
        impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}


    //** Extrinsics **//

        #[pallet::call]
        impl<T: Config> Pallet<T> {
            
            #[pallet::weight(10_000)]
            pub fn create_movie(
                origin: OriginFor<T>,
                name:Vec<u8>,
                synopsis:Vec<u8>,
                movie_description:Vec<u8>,
                classification:u32,
                release:Vec<u8>,
                director:Vec<u8>,
                lang:Vec<u8>,
                country:Vec<u8>,
                rating:u32,
                aspect_ratio:Vec<u8>,
                tags:Vec<u8>,
                trailer:Vec<u8>,
                imdb:Vec<u8>,
                social:Vec<u8>,
                ipfs:Vec<u8>,
                link:Vec<u8>,
            ) -> DispatchResult {
                let who = ensure_signed(origin)?;
                Self::do_create_movie(&who, name,synopsis,movie_description,classification,release,director,lang,country,rating,aspect_ratio,tags,trailer,imdb,social,ipfs,link)?;
                Ok(())
            }

        }


    //** Helpers **//

        impl<T: Config> Pallet<T> {

            pub fn do_create_movie(
                who: &T::AccountId,
                name:Vec<u8>,
                synopsis:Vec<u8>,
                movie_description:Vec<u8>,
                classification:u32,
                release:Vec<u8>,
                director:Vec<u8>,
                lang:Vec<u8>,
                country:Vec<u8>,
                rating:u32,
                aspect_ratio:Vec<u8>,
                tags:Vec<u8>,
                trailer:Vec<u8>,
                imdb:Vec<u8>,
                social:Vec<u8>,
                ipfs:Vec<u8>,
                link:Vec<u8>,
            ) -> Result<T::MovieId, DispatchError> {
        
                let movie_id =
                    NextMovieId::<T>::try_mutate(|id| -> Result<T::MovieId, DispatchError> {
                        let current_id = *id;
                        *id = id
                            .checked_add(&One::one())
                            .ok_or(Error::<T>::Overflow)?;
                        Ok(current_id)
                    })?;
        
                
                let bounded_name: BoundedVec<u8, T::StringLimit> =TryInto::try_into(name).map_err(|_| Error::<T>::BadMetadata)?;
                
                
                let bounded_synopsis: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(synopsis).map_err(|_|Error::<T>::BadMetadata)?;
                
                let bounded_movie_description: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(movie_description).map_err(|_| Error::<T>::BadMetadata)?;
                
                let bounded_release: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(release).map_err(|_| Error::<T>::BadMetadata)?;

                let bounded_director: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(director).map_err(|_| Error::<T>::BadMetadata)?;
                
                let bounded_lang: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(lang).map_err(|_| Error::<T>::BadMetadata)?;

                let bounded_country: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(country).map_err(|_| Error::<T>::BadMetadata)?;
                let bounded_aspect_ratio: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(aspect_ratio).map_err(|_| Error::<T>::BadMetadata)?;

                let bounded_tags: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(tags).map_err(|_| Error::<T>::BadMetadata)?;

                let bounded_trailer: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(trailer).map_err(|_|Error::<T>::BadMetadata)?;
                
                let bounded_imdb: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(imdb).map_err(|_|Error::<T>::BadMetadata)?;

                let bounded_social: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(social).map_err(|_|Error::<T>::BadMetadata)?;

                let bounded_link: BoundedVec<u8, T::StringLimit> =
                    TryInto::try_into(link).map_err(|_|Error::<T>::BadMetadata)?;

                let bounded_ipfs: BoundedVec<u8, T::StringLimit> =
                TryInto::try_into(ipfs).map_err(|_|Error::<T>::BadMetadata)?;

                
                let movie = Movie {
                    owner:who.clone(),
                    name:bounded_name,
                    synopsis:bounded_synopsis,
                    movie_description:bounded_movie_description,
                    classification:classification,
                    release:bounded_release,
                    director:bounded_director,
                    lang:bounded_lang,
                    country:bounded_country,
                    rating:rating,
                    aspect_ratio:bounded_aspect_ratio,
                    tags:bounded_tags,
                    trailer:bounded_trailer,
                    imdb:bounded_imdb,
                    social:bounded_social,
                    ipfs:bounded_ipfs,
                    link:bounded_link,
                };
        
            
                Movies::<T>::insert(movie_id, movie.clone());
        
                Self::deposit_event(Event::MovieCreated(movie_id, who.clone()));
                Ok(movie_id)
            } 
        }

    }
