use scrypto::prelude::*;
use sbor::*;

//inspiration from time-lock and chrimstams caledner coal staking day 10?

#[derive(Debug, TypeId, Encode, Decode, Clone, Copy, Describe, PartialEq, Eq)]
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

        // Authorization for the person making this thing to controll rewards vault.
        //(should deposit and witdraewl from rewards be possible?)
        minter_vault: Vault,
        minter_badge: ResourceDef,

        stake_vault: Vault,
        rewards_vault: Vault, //the owner will set the total rewards amount in here.

        
        stakers: HashMap<Address, StakerData>,
        
        //NB! hashmap. only unique keys, so a staker can not stake two times if not taken care of.
        //can not just add an extra element in the hashmap, because only one per key can exist.
        //better with other strcuts than hashmap, or just update in the stake function based on that?
        //maybe call the witdraw function and then stake function, if they have already staked.


        staked_vec: Vec<StakedEpoch>, //total tokens staked each epoch

        

       
        
        

    }

    impl Stake {

        pub fn new() -> (Component, Bucket) { //instansiate function

            let mut minter_bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
            .metadata("name", "Badge Mint Auth")
            .initial_supply_fungible(1);

            let minter_resource_def = minter_bucket.resource_def();
            let minter_return_bucket: Bucket = minter_bucket.take(1); // Return this badge to the caller


            let component = Self {
                minter_vault: Vault::with_bucket(minter_bucket),
                minter_badge: minter_resource_def,
                stake_vault: Vault::new(RADIX_TOKEN), //should be the lp token
                rewards_vault: Vault::new(RADIX_TOKEN), 
                stakers: HashMap::new(),
                staked_vec: Vec::new()
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


            //trying to handle the problem with people adding stakie more than one time here
            //maybe call the witdraw function and than deposit another time
            
            
            /*
            match self.stakers.get(""){
                Some(staker) => {
                    std::process::abort();
                    //call the witdraw function instead of a aborting. I think will be possible then?
                    //regarding only one address per hashmap.
                }
            };
            */

            // Mint Badge with locked amount and start epoch as metadata
            let badge = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Staker Badge")
                .metadata("amount", amount.to_string())
                .metadata("start epoch", curr_epoch.to_string())
                .flags(MINTABLE | BURNABLE )
                .badge(self.minter_vault.resource_def(), MAY_MINT | MAY_BURN)
                .initial_supply_fungible(1);

            
            if self.staked_vec.len()==0{ //add this to the inizialixation of the vector or something like that. no point in checking every time here
                self.staked_vec.push(StakedEpoch{ epoch:curr_epoch, staked: amount});
            }
            else{

                //Updates how much is staked in stakedVec
                let last=self.staked_vec.len()-1;

                let last_epoch_staked=self.staked_vec[last].staked;
                if curr_epoch == self.staked_vec[last].epoch {
                    self.staked_vec[last].staked=amount+last_epoch_staked;
                }
                else if curr_epoch > self.staked_vec[last].epoch {
                    self.staked_vec.push(StakedEpoch{ epoch: curr_epoch, staked: amount+last_epoch_staked});// add amount of penultimate element
                }

            }




            // store new badge address in the stakers struct
            self.stakers.insert(badge.resource_address(), StakerData {started_at: curr_epoch, amount: amount});
            self.stake_vault.put(stake_tokens);

            badge
        }


        //unstake
        //witdraw the staked amount and the fees earned to the stakers wallet
        //calculation of rewards also need to be done here.

        pub fn unstake(&mut self, badge: Bucket) -> (Bucket, Bucket) {
            let curr_epoch = Context::current_epoch();

            let mut bucket_stake = Bucket::new(RADIX_TOKEN);
            let mut bucket_reward = Bucket::new(RADIX_TOKEN);

            //match: so it goes through every element of the Hashmap.
            match self.stakers.get(&badge.resource_address()) {
                Some(&staker_data) => {
                
                //loops through the stakedVec, and caculate your percentage of total staked coin for each epoch
                //and give you a reward based on that and how many tokens are distributed each epoch.
                let total_rewards_epoch=100; //this should be decide in new or something like that.
                let mut rewards: Decimal=Decimal::from(0);
                let mut number_of_epochs:u64; //number of epochs since last deposit/witdraw. often 1.
                let last=self.staked_vec.len()-1;


                if self.staked_vec.len()>1{
                    for i in 1..self.staked_vec.len(){
                        number_of_epochs=self.staked_vec[i].epoch-self.staked_vec[i-1].epoch;
                        rewards += Decimal::from(number_of_epochs*total_rewards_epoch)*(staker_data.amount/self.staked_vec[i-1].staked);
                    }
                }
                //add rewards for the last epochs (not the best readability haha)
                number_of_epochs=curr_epoch-self.staked_vec.last().unwrap().epoch;
                rewards += Decimal::from(number_of_epochs*total_rewards_epoch)*(staker_data.amount/self.staked_vec.last().unwrap().staked);
                
                //puts your staked tokens and the your reward in buckets
                info!("Bukcetstake: {}    Bucket_reward: {}", staker_data.amount, rewards);
                bucket_stake.put(self.stake_vault.take(staker_data.amount));
                bucket_reward.put(self.rewards_vault.take(rewards));
                
                //Updates how much is staked in stakedVec
                let staked_last_epoch=self.staked_vec.last().unwrap().staked;
                if curr_epoch == self.staked_vec[last].epoch {
                    self.staked_vec[last].staked=staker_data.amount-staked_last_epoch;
                }
                else if curr_epoch > self.staked_vec[last].epoch {
                    self.staked_vec.push(StakedEpoch{epoch: curr_epoch, staked: staker_data.amount-staked_last_epoch});// subtract amount of last element
                }
                // update stakers in the component
                self.stakers.remove(&badge.resource_address());
                info!("here1");
                // Burn the badge
                info!("here2");
                self.minter_vault.authorize(|auth| {
                    info!("here3");
                    badge.burn_with_auth(auth);
                });
                info!("here4");
                // Return the withdrawn tokens
                },
                None => {
                    info!("no mints found with provided badge");
                    std::process::abort();
                }
            }


            (bucket_stake,bucket_reward)
        }


        pub fn add_rewards(&mut self, rewards:Bucket) {
            self.rewards_vault.put(rewards);
        }

        pub fn test_add_to_pool(&mut self, stake:Bucket) {
            self.stake_vault.put(stake)
        }

        
        //make a function for only witdrawing the rewards?

        //make a function for the creater of the new compnent to be able to witdraw and deposit from rewards vaults.
        //if he finds out that more tokens should be used for other rewards plattform.
        //of course he should not have acces to staking vault. just the people that has deposited and only for the amount.

    } 

}
