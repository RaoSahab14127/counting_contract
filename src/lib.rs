
use cosmwasm_std::to_json_binary;
use cosmwasm_std::{
 Binary,DepsMut, Deps, Env, MessageInfo, Empty, StdResult, Response, entry_point
};
mod msg;
#[entry_point]
pub fn instantiate(
    _deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty
)->StdResult<Response> {
    Ok(Response::new())
}


#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: msg::QueryMsg)->StdResult<Binary>{
    use msg::QueryMsg::*;
    match msg{
        Value {} => to_json_binary(&queryy::value()),
        ValueInc{value}=>to_json_binary(&incmod::increment(value))
    }
}

pub mod queryy{
    use crate::msg::ValueResp;
    pub fn value()-> ValueResp{
        ValueResp{ value: 0 }
    }
}

pub mod  incmod{
    use crate::msg::ValueResp;
    pub fn increment(value: u64)-> ValueResp{
        ValueResp{ value: value+1 }
    }
}