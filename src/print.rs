use crate::types::Provider;
use windows::core::GUID;

pub fn summary(all: &[Provider]) {
    println!("Providers ({}):", all.len());
    for p in all {
        println!("# {:?}: {}", p.id, p.name);
        println!("  Countersets ({}):", p.countersets.len());
        for cs in &p.countersets {
            println!("  @ {:?}: {}; {}", cs.id, cs.name, cs.help);
            println!("    Counters ({}):", cs.counters.len());
            for c in &cs.counters {
                println!("    - {:?}: {}; {}", c.id, c.name, c.help);
            }
            match &cs.instances {
                Some(instances) => {
                    println!("    Instances ({}):", instances.len());
                    for i in instances {
                        println!("    > {:?}: {}", i.id, i.name);
                    }
                }
                None => {
                    println!("    Instances: none");
                }
            }
        }
        println!();
    }
}

pub fn counterset(all: &[Provider], counterset_id: &GUID) {
    let counterset = all
        .iter()
        .flat_map(|p| &p.countersets)
        .find(|cs| cs.id == *counterset_id);

    match counterset {
        Some(counterset) => {
            println!("{:#?}", counterset);
        }
        None => {
            println!("Counterset {:?} not found", counterset_id);
        }
    }
}
