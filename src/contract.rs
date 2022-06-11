use cosmwasm_std::{
    entry_point, to_binary,   CosmosMsg, Deps, DepsMut,Binary,
    Env, MessageInfo, Response, StdResult, Uint128, WasmMsg,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg,JunoPunksMsg, InstantiateMsg, QueryMsg, Trait};
use crate::state::{
    CONFIG,USERINFO,State,METADATA
};

use cw721_base::{ExecuteMsg as Cw721BaseExecuteMsg, MintMsg};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
         total_nft:msg.total_nft,
         owner:msg.owner,
         max_nft:msg.max_nft,
         count:Uint128::new(0),
         check_mint:msg.check_mint,
         nft_address:"nft".to_string(),
         url : msg.url,
         image_url:msg.image_url
    };
    CONFIG.save(deps.storage, &state)?;
    let metadata:Vec<JunoPunksMsg> = vec![];
    METADATA.save(deps.storage,&metadata)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint{rand} => execute_mint(deps, env, info,rand),
        ExecuteMsg::SetNftAddress { address } => execute_set_nft_address(deps, info, address),
        ExecuteMsg::ChangeOwner { address } => execute_chage_owner(deps, info, address),
        ExecuteMsg::SetMaximumNft { amount } => execute_maxium_nft(deps, info, amount),
        ExecuteMsg::AddMetadata { metadata } => execute_add_metadata(deps,info,metadata)
    }
}

fn execute_mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    rand:Uint128
) -> Result<Response, ContractError> {
    let  state = CONFIG.load(deps.storage)?;

    if state.count >= state.total_nft {
        return Err(ContractError::MintEnded {});
    }

    if rand > state.total_nft{
        return Err(ContractError::WrongNumber {  });
    }

    let sender = info.sender.to_string();
    let token_id = ["JunoPunks".to_string(),rand.to_string()].join(".");

    

    let user_info = USERINFO.may_load(deps.storage, &sender)?;
    if user_info == None{
        let mut state = CONFIG.load(deps.storage)?;
        state.count = state.count+Uint128::new(1);
        state.check_mint[Uint128::u128(&rand) as usize -1] = false;
        CONFIG.save(deps.storage, &state)?;
        USERINFO.update(deps.storage,&sender,
            |mut user_info|->StdResult<_>{
            let new_user =Uint128::new(1);
            if user_info == None{
                Ok(new_user)
            }
            else{
                 Ok(new_user+Uint128::new(1))
                }
        })?;
        Ok(Response::new()
            .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: state.nft_address,
                msg: to_binary(&Cw721BaseExecuteMsg::Mint(MintMsg {
                    //::<Metadata>
                    token_id: token_id.clone(),
                    owner: sender,
                    token_uri: Some([[state.url,rand.to_string()].join(""),"json".to_string()].join(".")),
                    extension:  JunoPunksMsg{
                        name:Some("2".to_string()),
                        description:Some("desc".to_string()),
                        image:Some("image".to_string()),
                        dna:Some("dna".to_string()),
                        edition:Some(1),    
                        date:Some(123),
                        compiler:Some("compiler".to_string()),
                        attributes:vec![Trait{
                            trait_type:Some("123".to_string()),
                            value:Some("clause".to_string())
                        }]}
                }))?,
                funds: vec![],
            })))
    }
    else {
        return Err(ContractError::MintExceeded {  });
    }
}

fn execute_maxium_nft(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    CONFIG.update(deps.storage,
        |mut state|-> StdResult<_>{
            state.max_nft = amount;
            Ok(state)
        }
    )?;
    Ok(Response::default())
}

fn execute_chage_owner(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
   let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    CONFIG.update(deps.storage,
        |mut state|-> StdResult<_>{
            state.owner = address;
            Ok(state)
        }
    )?;
    Ok(Response::default())
}

fn execute_set_nft_address(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
   let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    CONFIG.update(deps.storage,
        |mut state|-> StdResult<_>{
            state.nft_address = address;
            Ok(state)
        }
    )?;
    Ok(Response::default())
}

pub fn execute_add_metadata(
    deps: DepsMut,
    // env:Env,
    info: MessageInfo,
    new_metadata: Vec<JunoPunksMsg>,
) -> Result<Response, ContractError> {
    let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    let mut metadata = METADATA.load(deps.storage)?;
    for new_data in new_metadata{
        metadata.push(new_data);
    }
    METADATA.save(deps.storage, &metadata)?;
    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStateInfo {} => to_binary(& query_get_info(deps)?),
        QueryMsg::GetUserInfo { address }=>to_binary(& query_user_info(deps,address)?),
    }
}


pub fn query_get_info(deps:Deps) -> StdResult<State>{
    let state = CONFIG.load(deps.storage)?;
    Ok(state)
}

pub fn query_user_info(deps:Deps,address:String) -> StdResult<Uint128>{
    let user_info = USERINFO.may_load(deps.storage,&address)?;
    if user_info == None{
        Ok(Uint128::new(0))
    }
    else{
     Ok(Uint128::new(1))
    }
}


pub fn query_metadata(deps:Deps) -> StdResult<Vec<JunoPunksMsg>>{
    let metadata = METADATA.load(deps.storage)?;
    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use crate::msg::Trait;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{CosmosMsg};

    #[test]
    fn buy_token() {
        let mut deps = mock_dependencies();
        let instantiate_msg = InstantiateMsg {
            total_nft:Uint128::new(5),
            max_nft:Uint128::new(1),
            owner:"creator".to_string(),
            check_mint:vec![true,true,true,true,true],
            url :"url".to_string(),
            image_url:"image_url".to_string()
        };
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());
        
        

        let info = mock_info("creator", &[]);
        let message = ExecuteMsg::SetNftAddress { address:"nft_address1".to_string() };
        execute(deps.as_mut(), mock_env(), info, message).unwrap();

        let state = query_get_info(deps.as_ref()).unwrap();
        assert_eq!(state.image_url,"image_url".to_string());
        
        let state= query_get_info(deps.as_ref()).unwrap();
        assert_eq!(state.nft_address,"nft_address1".to_string());

        let info = mock_info("creator", &[]);
        let message = ExecuteMsg::AddMetadata { metadata: vec![JunoPunksMsg{
                        name:None,
                        description:None,
                        image:None,
                        dna:None,
                        edition:None,    
                        date:None,
                        compiler:None,
                        attributes:vec![Trait{
                            trait_type:None,
                            value:None
                        }]}
            ,
            JunoPunksMsg{
            name:Some("2".to_string()),
            description:Some("desc".to_string()),
            image:Some("image".to_string()),
            dna:Some("dna".to_string()),
            edition:Some(1),    
            date:Some(123),
            compiler:Some("compiler".to_string()),
            attributes:vec![Trait{
                trait_type:Some("123".to_string()),
                value:Some("clause".to_string())
            }]},
            JunoPunksMsg{
            name:Some("3".to_string()),
            description:Some("desc".to_string()),
            image:Some("image".to_string()),
            dna:Some("dna".to_string()),
            edition:Some(1),    
            date:Some(123),
            compiler:Some("compiler".to_string()),
            attributes:vec![Trait{
                trait_type:Some("123".to_string()),
                value:Some("clause".to_string())
            }]},
            JunoPunksMsg{
            name:Some("4".to_string()),
            description:Some("desc".to_string()),
            image:Some("image".to_string()),
            dna:Some("dna".to_string()),
            edition:Some(1),    
            date:Some(123),
            compiler:Some("compiler".to_string()),
            attributes:vec![Trait{
                trait_type:Some("123".to_string()),
                value:Some("clause".to_string())
            }]},
            JunoPunksMsg{
            name:Some("5".to_string()),
            description:Some("desc".to_string()),
            image:Some("image".to_string()),
            dna:Some("dna".to_string()),
            edition:Some(1),    
            date:Some(123),
            compiler:Some("compiler".to_string()),
            attributes:vec![Trait{
                trait_type:Some("123".to_string()),
                value:Some("clause".to_string())
            }]
        }] };
        execute(deps.as_mut(), mock_env(), info, message).unwrap();

        let metadata = query_metadata(deps.as_ref()).unwrap();


        let info = mock_info("creator", &[]);
        let message = ExecuteMsg::Mint {rand:Uint128::new(1)};
        let res = execute(deps.as_mut(), mock_env(), info, message).unwrap();
   
        assert_eq!(1,res.messages.len());
        assert_eq!(res.messages[0].msg,CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: state.nft_address,
                msg: to_binary(&Cw721BaseExecuteMsg::Mint(MintMsg {
                    //::<Metadata>
                    token_id: "JunoPunks.1".to_string(),
                    owner: "creator".to_string(),
                    token_uri: Some("url1.json".to_string()),
                    extension:metadata[0].clone() ,
                })).unwrap(),
                funds: vec![],
            }));
        let user_info = query_user_info(deps.as_ref(), "creator".to_string()).unwrap();
        assert_eq!(user_info,Uint128::new(1));

        // let info = mock_info("creator", &[]);
        // let message = ExecuteMsg::Mint {};
        // let res = execute(deps.as_mut(), mock_env(), info, message).unwrap();
   
    }

}
