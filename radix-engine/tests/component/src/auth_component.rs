use scrypto::prelude::*;

blueprint! {
    struct AuthComponent {
        some_non_fungible: NonFungibleAddress,
    }

    impl AuthComponent {
        pub fn create_component(some_non_fungible: NonFungibleAddress) -> ComponentAddress {
            Self { some_non_fungible }
                .instantiate()
                .add_access_check(
                    AccessRules::new()
                        .method("get_secret", rule!(require("some_non_fungible")))
                        .default(rule!(allow_all)),
                )
                .globalize()
        }

        pub fn get_secret(&self) -> String {
            "Secret".to_owned()
        }

        pub fn update_auth(&mut self, some_non_fungible: NonFungibleAddress) {
            self.some_non_fungible = some_non_fungible;
        }
    }
}
