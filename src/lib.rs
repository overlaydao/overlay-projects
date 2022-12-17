#![cfg_attr(not(feature = "std"), no_std)]
use concordium_std::*;
use core::fmt::Debug;

type ProjectId = String;

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
struct State<S> {
    admin: AccountAddress,
    staking_contract_addr: ContractAddress,
    user_contract_addr: ContractAddress,
    project: StateMap<ProjectId, ProjectState, S>,
}

#[derive(Serial, Deserial)]
struct ProjectState {
    project_uri: Option<String>,
    owners: Option<Vec<AccountAddress>>,
    pub_key: Option<String>,
    token_addr: Option<ContractAddress>,
    seed_nft_addr: Option<ContractAddress>,
    sale_addr: Option<ContractAddress>,
    status: ProjectStatus,
}

#[derive(Debug, PartialEq, Eq, Reject, Serial, Deserial, SchemaType)]
enum ProjectStatus {
    Candidate,
    Whitelist,
    OnSale,
    SaleClosed,
}

#[derive(Serial, Deserial, SchemaType)]
struct UserState {
    is_curator: bool,
    is_validator: bool,
}

#[derive(Serial, Deserial, SchemaType)]
struct UpdateContractStateParam {
    staking_contract_addr: ContractAddress,
    user_contract_addr: ContractAddress,
}

#[derive(Serial, Deserial, SchemaType)]
struct TransferAdminParam {
    admin: AccountAddress,
}

#[derive(Serial, Deserial, SchemaType)]
struct CureateProjectParam {
    project_id: ProjectId,
    project_uri: String,
}

#[derive(Serial, Deserial, SchemaType)]
struct ValidateProjectParam {
    project_id: ProjectId,
    owners: Vec<AccountAddress>,
    token_addr: Option<ContractAddress>,
}

#[derive(Serial, Deserial, SchemaType)]
struct AddPubKeyParam {
    project_id: ProjectId,
    pub_key: String,
}

#[derive(Serial, Deserial, SchemaType)]
struct UpdateOwnersParam {
    project_id: ProjectId,
    owners: Vec<AccountAddress>,
}

#[derive(Serial, Deserial, SchemaType)]
struct AddSeedSaleParam {
    project_id: ProjectId,
    seed_nft_addr: ContractAddress,
}

#[derive(Serial, Deserial, SchemaType)]
struct AddTokenAddrParam {
    project_id: ProjectId,
    token_addr: ContractAddress,
}

#[derive(Serial, Deserial, SchemaType)]
struct AddSaleParam {
    project_id: ProjectId,
    sale_addr: ContractAddress,
}

#[derive(Serial, Deserial, SchemaType)]
struct StartSaleParam {
    project_id: ProjectId,
}

#[derive(Serial, Deserial, SchemaType)]
struct CloseSaleParam {
    project_id: ProjectId,
}

#[derive(Serial, Deserial, SchemaType)]
struct ViewProjectParam {
    project_id: ProjectId,
}

#[derive(Serial, Deserial, SchemaType)]
struct ViewAdminRes {
    admin: AccountAddress,
    staking_contract_addr: ContractAddress,
    user_contract_addr: ContractAddress,
}

#[derive(Debug, PartialEq, Eq, Reject, Serialize, SchemaType)]
enum Error {
    #[from(ParseError)]
    ParseParamsError,
    InvalidCaller,
    InvalidStatus,
    ShouldBeSeedNFT,
    ShouldNotBeSeedNFT,
}

type ContractResult<A> = Result<A, Error>;

#[init(contract = "overlay-projects", parameter = "InitParams")]
fn contract_init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    let params: UpdateContractStateParam = ctx.parameter_cursor().get()?;
    let state = State {
        admin: ctx.init_origin(),
        staking_contract_addr: params.staking_contract_addr,
        user_contract_addr: params.user_contract_addr,
        project: state_builder.new_map(),
    };
    Ok(state)
}

#[receive(
    contract = "overlay-projects",
    name = "update_contract_state",
    parameter = "UpdateContractStateParam",
    mutable,
    error = "Error"
)]
fn contract_update_contract_state<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: UpdateContractStateParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(ctx.sender() == Address::Account(state.admin), Error::InvalidCaller);

    state.staking_contract_addr = params.staking_contract_addr;
    state.user_contract_addr = params.user_contract_addr;
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "transfer_admin",
    parameter = "TransferAdminParam",
    mutable,
    error = "Error"
)]
fn contract_transfer_admin<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: TransferAdminParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(ctx.sender() == Address::Account(state.admin), Error::InvalidCaller);

    state.admin = params.admin;
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "apply_curate_project",
    parameter = "CureateProjectParam",
    mutable,
    error = "Error"
)]
fn contract_apply_curate_project<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: CureateProjectParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(ctx.sender() == Address::Account(state.admin), Error::InvalidCaller);
    state.project.insert(
        params.project_id,
        ProjectState {
            project_uri: Some(params.project_uri),
            owners: None,
            pub_key: None,
            token_addr: None,
            seed_nft_addr: None,
            sale_addr: None,
            status: ProjectStatus::Candidate,
        }
    );
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "curate_project",
    parameter = "CureateProjectParam",
    mutable,
    error = "Error"
)]
fn contract_curate_project<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: CureateProjectParam = ctx.parameter_cursor().get()?;
    let func = EntrypointName::new("view_user".into()).unwrap();
    let user_contract_addr = host.state_mut().user_contract_addr;
    let user_state: UserState = host.invoke_contract_raw(
        &user_contract_addr,
        Parameter(&to_bytes(&ctx.sender())),
        func,
        Amount::zero(),
    ).unwrap_abort().1.unwrap_abort().get().unwrap_abort();
    ensure!(user_state.is_curator, Error::InvalidCaller);
    
    let state = host.state_mut();
    state.project.insert(
        params.project_id,
        ProjectState {
            project_uri: Some(params.project_uri),
            owners: None,
            pub_key: None,
            token_addr: None,
            seed_nft_addr: None,
            sale_addr: None,
            status: ProjectStatus::Candidate,
        },
    );
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "validate_project",
    parameter = "ValidateProjectParam",
    mutable,
    error = "Error"
)]
fn contract_validate_project<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: ValidateProjectParam = ctx.parameter_cursor().get()?;
    let func = EntrypointName::new("view_user".into()).unwrap();
    let user_contract_addr = host.state_mut().user_contract_addr;
    let user_state: UserState = host.invoke_contract_raw(
        &user_contract_addr,
        Parameter(&to_bytes(&ctx.sender())),
        func,
        Amount::zero(),
    ).unwrap_abort().1.unwrap_abort().get().unwrap_abort();
    ensure!(user_state.is_validator, Error::InvalidCaller);
    let state = host.state_mut();
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(old_values.status == ProjectStatus::Candidate, Error::InvalidStatus);

    state.project.entry(params.project_id).and_modify(|project_state| {
        project_state.owners = Some(params.owners);
        project_state.token_addr = params.token_addr;
        project_state.status = ProjectStatus::Whitelist;
    });
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "add_pub_key",
    parameter = "AddPubKeyParam",
    mutable,
    error = "Error"
)]
fn contract_add_pub_key<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: AddPubKeyParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(ctx.sender() == Address::Account(state.admin), Error::InvalidCaller);
    ensure!(old_values.status != ProjectStatus::Candidate, Error::InvalidStatus);

    state.project.entry(params.project_id).and_modify(|project_state| {
        project_state.pub_key = Some(params.pub_key);
    });
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "update_owners",
    parameter = "UpdateOwnersParam",
    mutable,
    error = "Error"
)]
fn contract_update_owners<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: UpdateOwnersParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(ctx.sender() == Address::Account(state.admin), Error::InvalidCaller);
    ensure!(old_values.status != ProjectStatus::Candidate, Error::InvalidStatus);
    state.project.entry(params.project_id).and_modify(|project_state| {
        project_state.owners = Some(params.owners);
    });
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "add_seed_sale",
    parameter = "AddSeedSaleParam",
    mutable,
    error = "Error"
)]
fn contract_add_seed_sale<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: AddSeedSaleParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(ctx.sender() == Address::Account(state.admin), Error::InvalidCaller);
    ensure!(old_values.status == ProjectStatus::Whitelist, Error::InvalidStatus);

    state.project.entry(params.project_id).and_modify(|project_state| {
        project_state.seed_nft_addr = Some(params.seed_nft_addr);
    });
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "add_token_addr",
    parameter = "AddTokenAddrParam",
    mutable,
    error = "Error"
)]
fn contract_add_token_addr<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: AddTokenAddrParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(ctx.sender() == Address::Account(state.admin), Error::InvalidCaller);
    ensure!(old_values.status == ProjectStatus::Whitelist, Error::InvalidStatus);
    ensure!(old_values.seed_nft_addr != None, Error::ShouldBeSeedNFT);

    state.project.entry(params.project_id).and_modify(|project_state| {
        project_state.token_addr = Some(params.token_addr);
    });
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "add_sale",
    parameter = "AddSaleParam",
    mutable,
    error = "Error"
)]
fn contract_add_sale<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: AddSaleParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(ctx.sender() == Address::Account(state.admin), Error::InvalidCaller);
    ensure!(old_values.status == ProjectStatus::Whitelist, Error::InvalidStatus);
    ensure!(old_values.seed_nft_addr == None, Error::ShouldNotBeSeedNFT);

    state.project.entry(params.project_id).and_modify(|project_state| {
        project_state.sale_addr = Some(params.sale_addr);
    });
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "start_sale",
    parameter = "StartSaleParam",
    mutable
)]
fn contract_start_sale<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: StartSaleParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(ctx.sender() == Address::Account(state.admin), Error::InvalidCaller);
    ensure!(old_values.status == ProjectStatus::Whitelist, Error::InvalidStatus);

    state.project.entry(params.project_id).and_modify(|project_state| {
        project_state.status = ProjectStatus::OnSale;
    });
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "close_sale",
    parameter = "CloseSaleParam",
    mutable
)]
fn contract_close_sale<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: CloseSaleParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(ctx.sender() == Address::Account(state.admin), Error::InvalidCaller);
    ensure!(old_values.status == ProjectStatus::OnSale, Error::InvalidStatus);

    state.project.entry(params.project_id).and_modify(|project_state| {
        project_state.status = ProjectStatus::SaleClosed;
    });
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "view_admin",
    return_value = "ViewAdminRes"
)]
fn view_admin<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl  HasHost<State<S>, StateApiType = S>,
) -> ContractResult<ViewAdminRes> {
    let state = host.state();
    ensure!(ctx.sender() == Address::Account(host.state().admin), Error::InvalidCaller);
    Ok(ViewAdminRes {
        admin: state.admin,
        staking_contract_addr: state.staking_contract_addr,
        user_contract_addr: state.user_contract_addr,
    })
}

#[receive(
    contract = "overlay-projects",
    name = "view_project",
    parameter = "ViewProjectParam",
    return_value = "ProjectState"
)]
fn view_project<'a, S: HasStateApi + 'a>(
    ctx: &'a impl HasReceiveContext,
    host: &'a impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<&'a ProjectState> {
    let params: ViewProjectParam = ctx.parameter_cursor().get()?;
    let state = host.state();
    let binding = state.project.get(&params.project_id);
    let project_state = binding.as_deref().unwrap();
    Ok(project_state)
    // Ok(ProjectState {
    //     owners: project_state.owners,
    //     project_uri: project_state.project_uri,
    //     pub_key: project_state.pub_key,
    //     token_addr: project_state.token_addr,
    //     seed_nft_addr: project_state.seed_nft_addr,
    //     sale_addr: project_state.sale_addr,
    //     status: project_state.status,
    // })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
