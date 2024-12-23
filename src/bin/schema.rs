use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use std::env::current_dir;
use std::fs::create_dir_all;

use staking_contract::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, ConfigResponse, UserStakeResponse, TotalStakedResponse, PendingRewardsResponse, AprResponse};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(UserStakeResponse), &out_dir);
    export_schema(&schema_for!(TotalStakedResponse), &out_dir);
    export_schema(&schema_for!(PendingRewardsResponse), &out_dir);
    export_schema(&schema_for!(AprResponse), &out_dir);
}
