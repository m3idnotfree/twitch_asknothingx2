use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DropEntitlementGrantEvent {
    pub id: String,
    pub data: DEGEData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DEGEData {
    pub organization_id: String,
    pub category_id: String,
    pub category_name: String,
    pub campaign_id: String,
    pub user_id: String,
    pub user_name: String,
    pub user_login: String,
    pub entitlement_id: String,
    pub benefit_id: String,
    pub created_at: DateTime<FixedOffset>,
}
