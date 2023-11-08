#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod cluster_node_candidate_authorization {
    use ink::prelude::vec::Vec;
    use scale::Encode;

    #[ink(storage)]
    pub struct ClusterNodeCandidateAuthorization {
        known_node: Option<Vec<u8>>,
    }

    impl ClusterNodeCandidateAuthorization {
        #[ink(constructor)]
        pub fn new(init_known_node: Option<Vec<u8>>) -> Self {
            Self { known_node: init_known_node }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn set_known_node(&mut self, new_known_node: Option<Vec<u8>>) {
            self.known_node = new_known_node 
        }

        #[ink(message)]
        pub fn encode_address(&self, address: AccountId) -> Vec<u8> {
            address.encode()
        }

        #[ink(message, selector=0x96b0453e)]
        pub fn auth_node(&self, node_provider: AccountId, node: Vec<u8>, node_variant: u8) -> bool {
            self.known_node == Some(node)
        }
    }
}
