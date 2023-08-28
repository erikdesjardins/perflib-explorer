use crate::types::{CounterSet, Provider};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use windows::core::{Result, GUID};

mod counters;
mod countersets;
mod instances;
mod providers;

pub fn all_providers(buf: &mut Vec<u8>) -> Result<Vec<Provider>> {
    let mut providers = HashMap::<GUID, Provider>::new();

    for counterset_id in countersets::all_ids()? {
        let provider_id = providers::id_from_counterset(buf, &counterset_id)?;

        let name = countersets::name(buf, &counterset_id)?;
        let counters = counters::of_counterset(buf, &counterset_id)?;
        let instances = instances::of_counterset(buf, &counterset_id)?;

        let counterset = CounterSet {
            id: counterset_id,
            name,
            counters,
            instances,
        };

        match providers.entry(provider_id) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().countersets.push(counterset);
            }
            Entry::Vacant(entry) => {
                let provider_name = providers::name_from_counterset(buf, &counterset_id)?;
                entry.insert(Provider {
                    id: provider_id,
                    name: provider_name,
                    countersets: vec![counterset],
                });
            }
        }
    }

    // Sort providers by their ID (the API sorts them descending, but we use ascending here).
    let mut providers = providers.into_values().collect::<Vec<_>>();
    providers.sort_by_key(|p| p.id.to_u128());

    // Then sort countersets by their name.
    for provider in &mut providers {
        provider.countersets.sort_by(|a, b| a.name.cmp(&b.name));
    }

    Ok(providers)
}
