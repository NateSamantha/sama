use std::{cell::RefCell, sync::Arc};

use config::Config;
use estate::types::{service::DefinitionTarget, Catalog, Route};
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_os_rng());
}

#[derive(Clone)]
pub struct State(Arc<StateInner>);

impl State {
    pub async fn new(config: Config) -> eyre::Result<State> {
        let catalog = Catalog::from_json(&config.catalog).await?;

        Ok(State(Arc::new(StateInner { config, catalog })))
    }

    pub fn config(&self) -> &Config {
        &self.0.config
    }

    pub fn catalog(&self) -> &Catalog {
        &self.0.catalog
    }

    pub fn balance(&self, route: &Route) -> Option<DefinitionTarget> {
        let defs = self.catalog().def_by_key.get(route.def_key())?;
        Some(
            RNG.with(|rng| defs.iter().choose(&mut *rng.borrow_mut()))?
                .target
                .clone(),
        )
    }
}

struct StateInner {
    config: Config,
    catalog: Catalog,
}
