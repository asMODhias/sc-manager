use crate::registry::{verify_adapter_allowed, HubCategory};

#[test]
fn adapters_are_registered() {
    // Verify canonical adapters used in the code are registered in the hub registry
    let must_exist = ["fleetyards", "erkul", "rsi_verseguide"];
    for id in must_exist.iter() {
        let res = verify_adapter_allowed(id);
        assert!(res.is_ok(), "Adapter {} should be registered in adapters::registry", id);
        match res.unwrap() {
            HubCategory::Forbidden => panic!("Adapter {} is marked forbidden", id),
            _ => {}
        }
    }
}
