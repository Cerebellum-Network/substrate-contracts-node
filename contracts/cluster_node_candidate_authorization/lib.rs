#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod cluster_node_candidate_authorization {

    #[ink(storage)]
    pub struct ClusterNodeCandidateAuthorization {
        value: bool,
    }

    impl ClusterNodeCandidateAuthorization {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message, selector=0x96b0453e)]
        pub fn my_auth_message(&mut self) -> bool {
            let return_value = self.value;
            self.value = !self.value;
            return_value
        }
    }
}
