use sc_manager_adapters::erkul::ErkulClient;
use sc_manager_adapters::erkul::SimpleErkulClient;
use sc_manager_app::in_memory_equipment_repo::InMemoryEquipmentRepo;
use sc_manager_core::repositories::EquipmentRepository;

#[test]
fn fetch_and_register_equipment() {
    let mut client = SimpleErkulClient::new();
    client.add_equipment_for_org(
        "orgA",
        vec![
            sc_manager_core::domain::Equipment::new("eq1", "Laser", true),
            sc_manager_core::domain::Equipment::new("eq2", "Shield", false),
        ],
    );

    let mut repo = InMemoryEquipmentRepo::new();

    let items = client.fetch_equipment(Some("orgA"), None).unwrap();
    for it in items.into_iter() {
        // ensure if equipment is owned by org, we could mark it as read-only or set other fields
        repo.register(it).unwrap();
    }

    let all = repo.list_all().unwrap();
    assert_eq!(all.len(), 2);
}
