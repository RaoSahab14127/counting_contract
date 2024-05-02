// use serde::{Deserialize, Serialize};
 
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]


// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {
//     Value {},
// }
 
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]


// #[serde(rename_all = "snake_case")]
// pub struct ValueResp {
//     pub value: u64,
// }

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]


#[serde(rename_all= "snake_case")]
pub enum QueryMsg{
    Value{},
    ValueInc{ value: u64}
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all="snake_case")]

pub struct ValueResp{
    pub value: u64,
}

