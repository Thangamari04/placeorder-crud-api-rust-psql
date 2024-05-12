use serde::Deserialize;
use validator::Validate;

// Define a struct for representing PersonalDetails
#[derive(Debug, Validate , Deserialize)]
pub struct PlaceOrder {
    #[validate(length(min = 3))]
    pub customer_name: String,

    #[validate(length(min = 10))]
    pub email:String,
    #[validate(length(min = 10))]
    pub address: String,
    #[validate(length(min = 6))]
    pub pincode: String,
    #[validate(length(min = 2))]
    pub state: String,
    #[validate(length(min = 2))]
    pub city: String,
    #[validate(length(min = 10))]
    pub phone_number: String,
    
}
