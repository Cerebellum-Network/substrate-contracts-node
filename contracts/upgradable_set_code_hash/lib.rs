#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod upgradable_set_code_hash {

	#[ink(storage)]
	pub struct UpgradableSetCodeHash {
		value: u64,
	}

	impl UpgradableSetCodeHash {
		#[ink(constructor)]
		pub fn new(init_value: u64) -> Self {
			Self { value: init_value }
		}

		#[ink(constructor)]
		pub fn default() -> Self {
			Self::new(Default::default())
		}

        #[ink(message)]
        pub fn set(&mut self, new_value: u64) {
            self.value = new_value;
        }

		#[ink(message)]
		pub fn get(&self) -> u64 {
			self.value
		}

		#[ink(message)]
		pub fn get_more(&self) -> u64 {
			self.value + 1
		}

		#[ink(message)]
		pub fn set_code(&mut self, code_hash: [u8; 32]) {
			ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
				panic!("Failed to `set_code_hash` to {:?} due to {:?}", code_hash, err);
			});
            ink::env::debug_println!("Switched to code hash {:?}", code_hash);
		}
	}
}
