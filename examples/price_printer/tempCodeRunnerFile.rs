#[derive(Debug, Deserialize)]
struct RawPool {
    component_id: String,
    attributes: HashMap<String, String>,
    balances: HashMap<String, String>,
}