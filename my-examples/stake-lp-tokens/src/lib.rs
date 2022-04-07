use scrypto::prelude::*;

//huge inspiration from time-lock



blueprint! {
    struct Stake {
      // Define what resources and data will be managed by Stake components

        // Authorization for the person making this thing to controll rewards vault.
        //(should deposit and witdraewl from rewards be possible?)
        minter_vault: Vault,
        minter_badge: ResourceDef,

        stake_vault: Vault,
        rewards_vault: Vault, //the owner will set the total rewards amount in here.

        
        stakers: HashMap<Address, (Decimal, u64)>, 
        //NB! hashmap. only unique keys, so a staker can not stake two times if not taken care of.
        //can not just add an extra element in the hashmap, because only one can exist.
        //better with other strcuts than hashmap, or just update in the stake function based on that?
    }

    impl Stake {

        pub fn new() -> (Component, Bucket) { //instansiate function

            let minter_bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
            .metadata("name", "Badge Mint Auth")
            .initial_supply_fungible(1);

            let minter_resource_def = minter_bucket.resource_def();
            let minter_return_bucket: Bucket = minter_bucket.take(1); // Return this badge to the caller


            let component = Self {
                minter_vault: Vault::with_bucket(minter_bucket),
                minter_badge: minter_resource_def,
                stakers: HashMap::new(),
                stake_vault: Vault::new(RADIX_TOKEN), //should be the lp token
                rewards_vault: Vault::new(RADIX_TOKEN)
            }
            .instantiate();

            (component, minter_return_bucket)
        }

        //Stake
        //send the staking amount to the staking vault.
        //send info about staking amount and current epoch to the stakers struct

        //add some logic that check if you have staked before and updates based on that?
        //an dont just overides, becuase it is a hashmap.
        pub fn stake(&mut self, stake_tokens: Bucket) -> Bucket{
        
            let amount = stake_tokens.amount(); 
            assert!(amount != Decimal::zero(), "You cannot stake zero amount");

            let start_time=Context::current_epoch();

        
            // Mint Badge with locked amount and end epoch as metadata
            let resource_def = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Time lock badge")
                .metadata("amount", amount.to_string())
                .metadata("start epoch", start_time.to_string())
                .flags(MINTABLE | BURNABLE )
                .badge(self.minter_vault.resource_def(), MAY_MINT | MAY_BURN)
                .no_initial_supply();
        

            // store new badge address in the stakers struct
            self.stakers.insert(resource_def.address(), (amount, start_time));

            //badge given to staker. use this for something?
            let badge = self.minter_vault.authorize(|badge| {resource_def.mint(1, badge)});
            badge
        }


        //unstake
        //witdraw the staked amount and the fees earned to the stakers wallet
        //calculation of rewards also need to be done here.

        pub fn unstake(&mut self, badge: Bucket) -> Bucket {
            let resource_def = badge.resource_def();
            let bucket = Bucket::new(RADIX_TOKEN);

            //do some math to calculate rewards based on total amount staked in pool,
            // amount user have staked and time he started staking compared to time now.

            // let total_staked = 0;
            // for stakers in self.stakers{
            //     total_staked = total_staked + self.stakers.values() //value 0
            // }
            //self.rewards_vault
            //
            

            //make so it goes through every element of the Hashmap. Does the match function do that?
            match self.stakers.get(&resource_def.address()) {
                Some(&value) => { 
                assert!(value.0 > Decimal::zero(), "Release amount is zero");

                // Burn the badge
                self.minter_vault.authorize(|badge| {
                    badge.burn_with_auth(badge);
                });
                // update stakers in the component
                self.stakers.remove(&resource_def.address());
                

                bucket.put(self.stake_vault.take(value.0));
                bucket.put(self.rewards_vault.take(1)); //put in rewards. do this based on the math

                },


                _ => {
                    info!("no mints found with provided badge");
                    std::process::abort();
                }
            }
            
            // Return the withdrawn tokens
            bucket
        }

        
        //make a function for only witdrawing the rewards

        //make a function for the creater of the new compnent to be able to witdraw and deposit from rewards vaults.
        //if he finds out that more tokens should be used for other rewards plattform.
        //of course he should not have acces to staking vault. just the people that has deposited and only for the amount.

    } 

}
