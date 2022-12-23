use std::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::Local;
use uuid::Uuid;

use std::collections::HashMap;

pub type Db = RwLock<HashMap<u128, Order>>;
type OrderType = String;

pub fn db_init() -> Db {
    RwLock::new(HashMap::new())
}


#[derive(Serialize, Deserialize)]
pub struct SubmitOrder {
    name: OrderType,
    sauce: OrderType,
    meat: OrderType,
    toppings: OrderType,
}

impl SubmitOrder {
    fn flat_details(&self) -> String {
        let deets = vec![
            String::from(&self.sauce),
            String::from(&self.meat),
            String::from(&self.toppings)
        ];
        let deets = deets.join(" ");
        deets
    }
}
        


#[derive(Debug)]
pub struct Order {
    status: OrderType,
    name: OrderType,
    server: OrderType,
    details: OrderType,
    submitted: OrderType,
    id: Uuid,
}

impl Order {
    pub fn new(submit: &SubmitOrder, server: &str) -> Order {
        let submitted: String = Local::now().time().format("%H:%M:%S").to_string();
        Order {
            status: "No Ready".to_string(),
            name: String::from(&submit.name),
            server: server.to_string(),
            details: submit.flat_details(),
            submitted,
            id: Uuid::new_v4(),
        }
    }

    pub fn ready(&mut self) {
        self.status = "Ready".to_string();
    }
    
    pub fn status(&self) -> String {
        String::from(&self.status)
    }

    pub fn name(&self) -> String {
        String::from(&self.name)
    }
   
    pub fn server(&self) -> String {
        String::from(&self.server)
    }
    
    pub fn details(&self) -> String {
        String::from(&self.details)
    }

    pub fn submitted(&self) -> String {
        String::from(&self.submitted)
    }

    pub fn id(&self) -> u128 {
        self.id.as_u128()
    }
}
