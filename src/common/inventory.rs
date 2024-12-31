pub struct InventoryItem {
    translation_key: &str,
    item_type: &str,
    amount: u64,
    key: &str,
    auto_consolidate: bool,
    base_cost_rate: u32,
}
