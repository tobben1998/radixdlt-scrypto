use scrypto::prelude::*;
use sbor::*;

//inspiration from time-lock and chrimstams caledner coal staking day 10?

#[derive(Debug, TypeId, Encode, Decode, Describe, PartialEq, Eq)]
pub struct StakerData{
    started_at: u64,
    amount: Decimal,
}

#[derive(Debug, TypeId, Encode, Decode, Describe, PartialEq, Eq)]
pub struct StakedEpoch{
    epoch: u64,
    staked: Decimal,
}

blueprint! {
    struct Stake {

      // Define what resources and data will be managed by Stake components

        // Authorization for the person making this thing to controll rewards vault.
        //(should deposit and witdraewl from rewards be possible?)
        minter_vault: Vault,
        minter_badge: ResourceDef,

        stake_vault: Vault,
        rewards_vault: Vault, //the owner will set the total rewards amount in here.

        
        stakers: HashMap<Address, StakerData>,
        
        //NB! hashmap. only unique keys, so a staker can not stake two times if not taken care of.
        //can not just add an extra element in the hashmap, because only one can exist.
        //better with other strcuts than hashmap, or just update in the stake function based on that?
        //maybe call the witdraw function and then stake function, if they have already staked.


        staked_vec: Vec<StakedEpoch> 
        //update this for every deposit and withdrawel (if no problem with race condition)
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

        pub fn stake(&mut self, stake_tokens: Bucket) -> Bucket{

                        
            let curr_epoch=Context::current_epoch();      
            let amount = stake_tokens.amount(); 
            assert!(amount > Decimal::zero(), "You need to stake more than zero tokens");


            //match it with your own adress. how to you get your own adress. 
            //self refer to the struct stake. want to to caller.Address or somerhing
            match self.stakers.get(){
                Some(staker) => {
                    std::process::abort();
                    //call the witdraw function instead of a aborting. I think will be possible then?
                    //regarding only one address per hashmap.
                }
            }


            // Mint Badge with locked amount and start epoch as metadata
            let badge = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Staker Badge")
                .metadata("amount", amount.to_string())
                .metadata("start epoch", curr_epoch.to_string())
                .flags(MINTABLE | BURNABLE )
                .badge(self.minter_vault.resource_def(), MAY_MINT | MAY_BURN)
                .initial_supply_fungible(1);
        

            //Updates how much is staked in stakedVec
            if curr_epoch == Self.staked_vec.last().epoch {
                let last=self.staked_vec.pop;
                self.staked_vec.push([curr_epoch,amount+last.staked]);
            }
            else if curr_epoch > Self.staked_vec.last().epoch {
                self.staked_vec.push([curr_epoch,amount+Self.staked_vec[Self.staked_vec.len()-2].staked]); // add amount of penultimate element
            }

            // store new badge address in the stakers struct
            self.stakers.insert(badge.address(), StakerData {started_at: curr_epoch, amount: amount});
            self.stake_vault.put(stake_tokens);

            badge
        }


        //unstake
        //witdraw the staked amount and the fees earned to the stakers wallet
        //calculation of rewards also need to be done here.

        pub fn unstake(&mut self, badge: Bucket) -> Bucket {
            let bucket = Bucket::new(RADIX_TOKEN);

            //match: so it goes through every element of the Hashmap.
            let staker_data = match self.stakers.get(&badge.address()) { //what is the difference between resoruce adress and adress? maybe chanhe to resoruce adress?
                Some(&staker) => { 
                assert!(value.0 > Decimal::zero(), "Release amount is zero");

                // Burn the badge
                self.minter_vault.authorize(|badge| {
                    badge.burn_with_auth(badge);
                });

                // update stakers in the component
                self.stakers.remove(&badge.address());

                //Updates how much is staked in stakedVec
                if curr_epoch == Self.staked_vec.last().epoch {
                    let last=self.staked_vec.pop;
                    self.staked_vec.push([curr_epoch,amount-last.staked]);
                }
                else if curr_epoch > Self.staked_vec.last().epoch {
                    self.staked_vec.push([curr_epoch,amount-Self.staked_vec[Self.staked_vec.len()-2].staked]);// subtract amount of penultimate element
                }

                
                

                bucket.put(self.stake_vault.take(staker_data.amount));

                let rewards=0;
                let total_rewards_epoch= 100; //this should be made decide in new or something


                //do some math to calculate rewards based on total amount staked in pool,
                // amount user have staked and time he started staking compared to time now and 
                //how manye total tokens have been staked in the different epochs.

                //loop thourgh staked and constisusly calcuate total share of the pool and and and that to your rewards. (if tha hashmap is possible to make regarding race conditions.)
            



                bucket.put(self.rewards_vault.take(1)); //put in rewards. do this based on the math
                
                
                },
                None => {
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
