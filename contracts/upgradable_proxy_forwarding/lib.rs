#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod upgradable_proxy_forwarding {
    use ink::env::call::Call;

	#[ink(storage)]
	pub struct UpgradableProxyForwarding {
		admin: AccountId,
		forward_to: AccountId,
		value: u64,
	}

	impl UpgradableProxyForwarding {
		#[ink(constructor)]
		pub fn new(init_value: u64) -> Self {
			Self {
				admin: Self::env().caller(),
				forward_to: AccountId::from([0u8; 32]),
				value: init_value,
			}
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
		pub fn get_more(&self) -> u32 {
			ink::env::call::build_call::<ink::env::DefaultEnvironment>()
				.call_type(
					Call::new(self.forward_to)
						.transferred_value(self.env().transferred_value())
						.gas_limit(0),
				)
				.call_flags(
					ink::env::CallFlags::default().set_forward_input(true).set_tail_call(true),
				)
                .invoke();
			unreachable!("the forwarded call will never return since `tail_call` was set");
		}

		#[ink(message)]
		pub fn change_forward_address(&mut self, new_forward_to: AccountId) {
			assert_eq!(
				self.env().caller(),
				self.admin,
				"caller {:?} does not have sufficient permissions, only {:?} does",
				self.env().caller(),
				self.admin,
			);
			self.forward_to = new_forward_to;
		}
	}
}
