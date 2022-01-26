use scrypto::prelude::*;

// Introduction to badges and how to switch users in resim
blueprint! {
    struct House {
        santa_badge: ResourceDef,
        owner_badge: ResourceDef
    }

    impl House {
        pub fn new() -> (Component, Vec<Bucket>) {
            // Create a new santa badge
            // new_badge_fixed returns a bucket containing the
            // generated badge.
            let santa_badge = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Santa's Badge")
                .initial_supply_fungible(1);
            
            // Create a new owner badge
            let owner_badge = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Owner's Badge")
                .initial_supply_fungible(1);

            // Store both badge's resource_def in the component's state.
            // We will need them for authentification
            let component = Self {
                santa_badge: santa_badge.resource_def(),
                owner_badge: owner_badge.resource_def()
            }
            .instantiate();

            // Return back the component and both badges
            (component, vec![santa_badge, owner_badge])
        }

        pub fn enter(&self, badge: BucketRef) {
            assert!(badge.amount() > Decimal::zero(), "Missing badge");

            // Now we have access to a variable named 'auth' which is a BucketRef.
            // === Note on BucketRef
            // BucketRefs are Buckets whose ownership are not passed to the component.
            // This component can't store the provided badge in its vaults or send it to someone.
            if badge.resource_def() == self.owner_badge {
                info!("Welcome home !");
            } else if badge.resource_def() == self.santa_badge {
                info!("Hello ! Please take some cookies and milk !");
            } else {
                error!("You have committed crimes against Scrypto and its community. What say you in your defense ?");
            }

            // Don't forget to drop BucketRefs or you will get an error !
            badge.drop();
        }
    }
}
