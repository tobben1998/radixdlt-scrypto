use scrypto::prelude::*;

//smart to make a vault for every list function and offer function. I need to do this beacuse 
//vault can only hold one type of nfts? 
//Would it be smarter to have the new function as the list function, and just send the 1% fee to an adress?


blueprint! {
    struct Nft {
        fee_vault: Vault,
    }

    impl Nft{
    //new function
    //instattiat
    //vault for collecting fees.
    //give badge to fee collector for being abke to witdraw. 
        pub fn new() -> (Component, Bucket) {

        }


    //list function:
    //function for listing your nft for sale with only one adress being able to buy. take 1% vut from that sale
    //put that nft into and a vault (and give a badge to the adrees that want to buy?)
    //inputs: adress and amount 

    //buy fucntion
    //Require a badge, or just the adress and the corecct amount 
    //assert if to low amount.take the nft from the vault and send 99% of tokens to the owner, and 1% to the fee adress





    //offer function.
    // put tokens in the vault, and give badge to or other stuff. input shoudl be adress
    //input

    //accpet offer function
    //assert if not hold nft. check if badge or correct adress. send nft and take nft
    }
}

