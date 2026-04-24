#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String};

#[contract]
pub struct DeliveryProof;

#[derive(Clone)]
#[contracttype]
pub struct Delivery {
    pub sender: Address,
    pub recipient: Address,
    pub courier: Address,
    pub package_desc: String,
    pub status: String,
}

#[contractimpl]
impl DeliveryProof {
    /// Create a new delivery
    pub fn create_delivery(env: Env, delivery_id: u64, sender: Address, recipient: Address, package_desc: String) {
        sender.require_auth();

        let mut deliveries: Map<u64, Delivery> = env
            .storage()
            .instance()
            .get(&"deliveries")
            .unwrap_or(Map::new(&env));

        if deliveries.contains_key(delivery_id) {
            panic!("Delivery already exists");
        }

        let delivery = Delivery {
            sender: sender.clone(),
            recipient,
            courier: sender, // Placeholder until courier accepts
            package_desc,
            status: String::from_str(&env, "created"),
        };

        deliveries.set(delivery_id, delivery);
        env.storage().instance().set(&"deliveries", &deliveries);
    }

    /// Courier accepts a delivery
    pub fn accept_delivery(env: Env, delivery_id: u64, courier: Address) {
        courier.require_auth();

        let mut deliveries: Map<u64, Delivery> = env
            .storage()
            .instance()
            .get(&"deliveries")
            .unwrap_or(Map::new(&env));

        let delivery = deliveries.get(delivery_id).unwrap_or_else(|| panic!("Delivery not found"));

        if delivery.status != String::from_str(&env, "created") {
            panic!("Delivery is not in created state");
        }

        let updated_delivery = Delivery {
            sender: delivery.sender,
            recipient: delivery.recipient,
            courier,
            package_desc: delivery.package_desc,
            status: String::from_str(&env, "accepted"),
        };

        deliveries.set(delivery_id, updated_delivery);
        env.storage().instance().set(&"deliveries", &deliveries);
    }

    /// Courier confirms delivery completion
    pub fn confirm_delivery(env: Env, delivery_id: u64) {
        let mut deliveries: Map<u64, Delivery> = env
            .storage()
            .instance()
            .get(&"deliveries")
            .unwrap_or(Map::new(&env));

        let delivery = deliveries.get(delivery_id).unwrap_or_else(|| panic!("Delivery not found"));

        if delivery.status != String::from_str(&env, "accepted") {
            panic!("Delivery is not in accepted state");
        }

        let updated_delivery = Delivery {
            sender: delivery.sender,
            recipient: delivery.recipient,
            courier: delivery.courier,
            package_desc: delivery.package_desc,
            status: String::from_str(&env, "confirmed"),
        };

        deliveries.set(delivery_id, updated_delivery);
        env.storage().instance().set(&"deliveries", &deliveries);
    }

    /// Recipient verifies delivery status
    pub fn verify_delivery(env: Env, delivery_id: u64) -> String {
        let deliveries: Map<u64, Delivery> = env
            .storage()
            .instance()
            .get(&"deliveries")
            .unwrap_or(Map::new(&env));

        let delivery = deliveries.get(delivery_id).unwrap_or_else(|| panic!("Delivery not found"));
        delivery.status
    }

    /// Get delivery details
    pub fn get_delivery(env: Env, delivery_id: u64) -> (Address, Address, Address, String, String) {
        let deliveries: Map<u64, Delivery> = env
            .storage()
            .instance()
            .get(&"deliveries")
            .unwrap_or(Map::new(&env));

        let delivery = deliveries.get(delivery_id).unwrap_or_else(|| panic!("Delivery not found"));
        (delivery.sender, delivery.recipient, delivery.courier, delivery.package_desc, delivery.status)
    }
}
