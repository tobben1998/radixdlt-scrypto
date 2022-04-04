use scrypto::prelude::*;

blueprint! {
    struct GumballMachine {
        gumballs: Vault,
        collected_xrd: Vault,
        price: Decimal,
        admin_badge: ResourceDef,
    }

    impl GumballMachine {
        pub fn new(price: Decimal) -> (Component, Bucket) {
            let gum_bucket: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Gumball")
                .metadata("symbol", "GUM")
                .initial_supply_fungible(100);

            let admin_badge = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Gumball admin")
                .initial_supply_fungible(1);

            let gumball_machine = Self {
                gumballs:Vault::with_bucket(gum_bucket),
                collected_xrd: Vault::new(RADIX_TOKEN),
                price: price,
                admin_badge: admin_badge.resource_def(),
            }
            .instantiate();
            (gumball_machine, admin_badge)
        }

        #[auth(admin_badge)]
        pub fn collected_xrd(&mut self) -> Bucket{
            self.collected_xrd.take_all()
        }

        pub fn buy_gumball(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            //assert!(payment.amount() == self.price, "Wrong amountof xrd sent");
            self.collected_xrd.put(payment.take(self.price));
            (self.gumballs.take(1), payment)
        }
    }
}
