use scrypto::prelude::*;

blueprint! {
    struct AuthListComponent {
        count: u8,
        auth: Vec<NonFungibleAddress>,
    }

    impl AuthListComponent {
        pub fn create_component(
            count: u8,
            auth: Vec<NonFungibleAddress>,
            access_rules: AccessRules,
        ) -> ComponentAddress {
            Self { count, auth }
                .instantiate()
                .add_access_check(access_rules)
                .globalize()
        }

        pub fn update_count(&mut self, count: u8) {
            self.count = count;
        }

        pub fn update_auth(&mut self, auth: Vec<NonFungibleAddress>) {
            self.auth = auth;
        }

        pub fn get_secret(&self) -> String {
            "Secret".to_owned()
        }
    }
}
