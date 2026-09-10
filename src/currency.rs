pub const USD: &str = "USD";

pub struct Currency {
     pub name: String,   
}

pub fn get_currency(name: String) -> Currency {
    return Currency{name: name};
}