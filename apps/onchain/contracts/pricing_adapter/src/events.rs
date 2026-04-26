use soroban_sdk::{Address, Env, Symbol};

pub struct InitializedEvent {
    pub admin: Address,
}

impl InitializedEvent {
    pub fn publish(&self, env: &Env) {
        env.events()
            .publish((Symbol::new(env, "initialized"),), self.admin.clone());
    }
}

pub struct PriceUpdatedEvent {
    pub admin: Address,
    pub asset: Address,
    pub price: i128,
}

impl PriceUpdatedEvent {
    pub fn publish(&self, env: &Env) {
        env.events().publish(
            (Symbol::new(env, "price_updated"), self.asset.clone()),
            (self.admin.clone(), self.price),
        );
    }
}

pub struct OracleUpdatedEvent {
    pub admin: Address,
    pub asset: Address,
    pub oracle: Address,
}

impl OracleUpdatedEvent {
    pub fn publish(&self, env: &Env) {
        env.events().publish(
            (Symbol::new(env, "oracle_updated"), self.asset.clone()),
            (self.admin.clone(), self.oracle.clone()),
        );
    }
}
