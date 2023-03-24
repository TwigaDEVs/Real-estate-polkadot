#![cfg_attr(not(feature = "std"), no_std)]
// use ink_storage::traits::{PackedLayout, SpreadLayout};



#[ink::contract]
mod nyumba {


    use ink::storage::{Mapping};
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;


 
    #[derive(Debug, scale::Encode, scale::Decode, Clone,PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct UserData {
        name: String,
        email: String,
        account_id: AccountId,
    }



    #[derive(Debug, scale::Encode, scale::Decode, Clone,PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct AssetData {
        property_owner: AccountId,
        property_id:String,
        property_name: String,
        property_location: String,
        property_value: u128,
        token_supply: u128,
        tokens_owned: BTreeMap<AccountId, u128>,
        token_price: u128,
        token_symbol: String,
        token_name: String,
    }
    
    #[ink(storage)]
    pub struct Nyumba {
        users: Mapping<AccountId, UserData>,
        all_users: Vec<UserData>,
        assets: Mapping<String, AssetData>,
        all_assets:Vec<AssetData>,
    }
    
    impl Nyumba {


        // nyumba user registration
        
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                users: Mapping::new(),
                all_users: Vec::new(),
                assets: Mapping::new(),
                all_assets: Vec::new(),
            }}

            #[ink(message)]
            pub fn register_user(&mut self, name: String, email: String) -> bool {
                let caller = self.env().caller();
                if self.users.get(&caller).is_some() {
                    return false;
                }
                let user_data = UserData { name, email, account_id: caller };
                self.users.insert(caller, &user_data);
                self.all_users.push(user_data);
                true
            }
    
            #[ink(message)]
            pub fn get_user(&self, user_id: AccountId) -> Option<UserData> {
                self.users.get(&user_id)
            }
    
            #[ink(message)]
            pub fn update_user(&mut self, name: String, email: String) -> bool {
                let caller = self.env().caller();
                if !self.users.get(&caller).is_some() {
                    return false;
                }
                let mut user_data = self.users.get(&caller).unwrap().clone();
                user_data.name = name;
                user_data.email = email;
                self.users.insert(caller, &user_data);
                true
            }
            
            #[ink(message)]
            pub fn delete_user(&mut self) -> bool {
                let caller = self.env().caller();
                if self.users.get(&caller).is_none() {
                    return false;
                }
                let user_data = self.users.take(&caller).unwrap();
                self.all_users.retain(|u| u.account_id != caller);
                true
            }
            
            
    
            #[ink(message)]
            pub fn get_all_users(&self) -> Vec<UserData> {
                self.all_users.clone()
            }

            //  nyumba asset registration


            #[ink(message)]
            pub fn register_asset(&mut self, property_owner: AccountId, 
                                property_name: String, property_location: String,
                                property_value: u128,
                                token_price: u128, token_symbol: String,property_id:String, 
                                token_name: String) -> bool {
                let caller = self.env().caller();
                if self.assets.get(&property_id).is_some() {
                    return false;
                }
                let token_supply = property_value / token_price;
                let new_property_id = property_id.clone();

                let asset_data = AssetData {
                    property_owner,
                    property_name,
                    property_location,
                    property_value,
                    token_supply,
                    tokens_owned: BTreeMap::new(),
                    token_price,
                    token_symbol,
                    property_id: new_property_id,
                    token_name,
                };
                self.assets.insert(&property_id, &asset_data);
                self.all_assets.push(asset_data);
                true
            }


            #[ink(message)]
            pub fn get_asset(&self, asset_id: String) -> Option<AssetData> {
                self.assets.get(&asset_id)
            }

        
            #[ink(message)]
            pub fn update_asset(&mut self,  property_owner: AccountId, 
                        property_name: String, property_location: String,
                        property_value: u128,
                        token_price: u128, token_symbol: String,property_id:String, 
                        token_name: String) -> bool {

                let caller = self.env().caller();
                if !self.assets.get(&property_id).is_some() {
                    return false;
                }


                let mut asset_data = self.assets.get(&property_id).unwrap().clone();

                let token_supply = property_value / token_price;
                let new_property_id = property_id.clone();

                let tokens_owned = asset_data.tokens_owned.clone();


                asset_data.property_owner = property_owner;
                asset_data.property_name = property_name;
                asset_data.property_location = property_location;
                asset_data.property_value = property_value;
                asset_data.token_price = token_price;
                asset_data.token_symbol = token_symbol;
                asset_data.property_id = new_property_id;
                asset_data.token_name = token_name;
                asset_data.token_supply = token_supply;
                asset_data.tokens_owned = tokens_owned;

                self.assets.insert(property_id, &asset_data);
                true
            }

            #[ink(message)]
            pub fn delete_asset(&mut self, property_id:String,) -> bool {
                let caller = self.env().caller();
                if self.assets.get(&property_id).is_none() {
                    return false;
                }
                let asset_data = self.assets.take(&property_id).unwrap();
                self.all_assets.retain(|u| u.property_id != property_id);
                true
            }

    
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use super::*;
    

        #[ink::test]
        fn add_owner_works() {
            // Setup
            let mut owner_register = Nyumba::new();
            let account_id = AccountId::from([1; 32]);
            let name = String::from("Alice");
            let email = String::from("123 Main St.");
          

            // Action
            owner_register.register_user(name.clone(), email.clone());

            // Assert
            assert_eq!(owner_register.get_user(account_id), Some(UserData{name, email,account_id}));
        }

        #[ink::test]
        fn get_all_users_test(){
            let mut user_register = Nyumba::new();
            let account_id = AccountId::from([1; 32]);
            let name = String::from("Alice");
            let email = String::from("123 Main St.");
          

            // Action
            user_register.register_user(name.clone(), email.clone());

            let users = user_register.get_all_users();
            println!("All users: {:?}", users);
            assert_eq!(users[0].name, "Alice");

            // Assert

        }

        #[ink::test]
        fn update_user_test(){

            let mut contract = Nyumba::new();
            let account_id = AccountId::from([1; 32]);
            let name = String::from("Alice");
            let email = String::from("123 Main St.");

            // Action
            contract.register_user(name.clone(), email.clone());

            // Assert
            assert_eq!(contract.get_user(account_id), Some(UserData{name, email,account_id}));

            let name_updated = String::from("Alice Sus");
            let email_updated = String::from("alice@gmail.com");

            let result = contract.update_user(name_updated.clone(), email_updated.clone());

            assert_eq!(result, true);
            let retrieved_user_data = contract.get_user(account_id).unwrap();
            assert_eq!(retrieved_user_data.name, "Alice Sus");
            assert_eq!(retrieved_user_data.email, "alice@gmail.com");

        }


        #[ink::test]
        fn register_asset_test(){

            let mut contract = Nyumba::new();
            let property_owner = AccountId::from([1; 32]);
            let property_id = String::from("prop1");
            let property_name = String::from("Twiga Devs Prop");
            let property_location = String::from("Remote");
            let property_value:u128 = 1000000;
            let token_price:u128 = 30000;
            let token_symbol = String::from("PRE");
            let token_name = String::from("Polkadot Reak Estate");
            let token_supply:u128 = property_value / token_price;
            let new_property_id = property_id.clone();


            // property_owner: AccountId, 
            // property_name: String, property_location: String,
            // property_value: u128,
            // token_price: u128, token_symbol: String,property_id:String, 
            //token_name: String



            let register = contract.register_asset(
                property_owner.clone(),
                property_name.clone(),
                property_location.clone(),
                property_value.clone(),
                token_price.clone(),
                token_symbol.clone(),
                property_id.clone(), 
                token_name.clone()
            );


                        // Assert
         assert_eq!(contract.get_asset(property_id), Some(AssetData{property_owner,
                                                                    property_id: new_property_id,
                                                                    property_name,
                                                                    property_location,
                                                                    property_value,
                                                                    token_supply,
                                                                    tokens_owned: BTreeMap::new(),
                                                                    token_price,
                                                                    token_symbol,
                                                                    token_name,}));

        }


       #[ink::test]
        fn update_asset_test(){

            let mut contract = Nyumba::new();
            let property_owner = AccountId::from([1; 32]);
            let property_id = String::from("prop1");
            let property_name = String::from("Twiga Devs Props");
            let property_location = String::from("Remote");
            let property_value:u128 = 1000000;
            let token_price:u128 = 30000;
            let token_symbol = String::from("PRE");
            let token_name = String::from("Polkadot Reak Estate");
            let token_supply:u128 = property_value / token_price;
            let new_property_id = property_id.clone();

            // Action
            contract.register_asset(
                property_owner.clone(),
                property_name.clone(),
                property_location.clone(),
                property_value.clone(),
                token_price.clone(),
                token_symbol.clone(),
                property_id.clone(), 
                token_name.clone()
            );


            let property_value_updated:u128  = 3000000;
            let property_name_updated = String::from("Twiga Devs");

            let result = contract.update_asset(
                                                property_owner.clone(),
                                                property_name_updated.clone(),
                                                property_location.clone(),
                                                property_value_updated.clone(),
                                                token_price.clone(),
                                                token_symbol.clone(),
                                                property_id.clone(), 
                                                token_name.clone()
                                            );

            assert_eq!(result, true);
            let retrieved_asset_data = contract.get_asset(property_id).unwrap();

            assert_eq!(retrieved_asset_data.property_name, "Twiga Devs");
            assert_eq!(retrieved_asset_data.property_value, 3000000);

        }





    }

}
