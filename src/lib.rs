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

#[derive(Serial, Deserial, SchemaType, Clone)]
struct ProjectState {
    project_uri: Option<String>,
    owners: Vec<AccountAddress>,
    pub_key: Option<String>,
    token_addr: Option<ContractAddress>,
    seed_nft_addr: Option<ContractAddress>,
    sale_addr: Option<ContractAddress>,
    status: ProjectStatus,
}

#[derive(Debug, PartialEq, Eq, Reject, Serial, Deserial, SchemaType, Clone)]
enum ProjectStatus {
    Candidate,
    Whitelist,
    OnSale,
    SaleClosed,
}

#[derive(Serial, Deserial, SchemaType)]
struct UserStateResponse {
    is_curator: bool,
    is_validator: bool,
    curated_projects: Vec<ProjectId>,
    validated_projects: Vec<ProjectId>,
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
struct CurateParam {
    addr: AccountAddress,
    project_id: ProjectId,
}

#[derive(Serial, Deserial, SchemaType)]
struct ValidateParam {
    addr: AccountAddress,
    project_id: ProjectId,
}

#[derive(Serial, Deserial, SchemaType)]
struct CurateProjectParam {
    project_id: ProjectId,
    project_uri: String,
    owners: Vec<AccountAddress>,
}

#[derive(Serial, Deserial, SchemaType)]
struct CurateProjectAdminParam {
    curator: AccountAddress,
    project_id: ProjectId,
    project_uri: String,
    owners: Vec<AccountAddress>,
}

#[derive(Serial, Deserial, SchemaType)]
struct ValidateProjectParam {
    project_id: ProjectId,
    owners: Vec<AccountAddress>,
    token_addr: Option<ContractAddress>,
}

#[derive(Serial, Deserial, SchemaType)]
struct ValidateProjectAdminParam {
    validator: AccountAddress,
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

#[derive(Debug, Serialize, SchemaType)]
struct UpgradeParam {
    module: ModuleReference,
    migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
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

#[derive(Serial, Deserial, SchemaType)]
struct AddrParam {
    addr: AccountAddress,
}

#[derive(Debug, PartialEq, Eq, Reject, Serialize, SchemaType)]
enum Error {
    #[from(ParseError)]
    ParseParamsError,
    InvalidCaller,
    InvalidStatus,
    ShouldNotBeTON,
    OnlyAccount,
    FailedInvokeUserContract,
    FailedInvokeUserContractView
}

type ContractResult<A> = Result<A, Error>;

#[init(contract = "overlay-projects", parameter = "UpdateContractStateParam")]
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
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);

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
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);

    state.admin = params.admin;
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "apply_curate_project",
    parameter = "CurateProjectParam",
    mutable,
    error = "Error"
)]
fn contract_apply_curate_project<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: CurateProjectParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    state.project.insert(
        params.project_id,
        ProjectState {
            project_uri: Some(params.project_uri),
            owners: params.owners,
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
    name = "curate_project",
    parameter = "CurateProjectParam",
    mutable,
    error = "Error"
)]
fn contract_curate_project<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: CurateProjectParam = ctx.parameter_cursor().get()?;
    let func = EntrypointName::new_unchecked("view_user");
    let user_contract_addr = host.state_mut().user_contract_addr;

    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(Error::OnlyAccount),
        Address::Account(account_address) => account_address,
    };

    let view_user_params = AddrParam {
        addr: sender_account,
    };
    let user_state: UserStateResponse = host
        .invoke_contract_read_only(
            &user_contract_addr,
            &view_user_params,
            func,
            Amount::zero(),
        ).unwrap().ok_or(Error::FailedInvokeUserContractView)?.get()?;

    ensure!(user_state.is_curator, Error::InvalidCaller);

    let state = host.state_mut();
    state.project.entry(params.project_id.clone()).or_insert_with(|| ProjectState {
            project_uri: Some(params.project_uri),
            owners: params.owners,
            pub_key: None,
            token_addr: None,
            seed_nft_addr: None,
            sale_addr: None,
            status: ProjectStatus::Candidate,
        },
    );

    let func = EntrypointName::new("curate".into()).unwrap();
    let curate_param = CurateParam {
        addr: sender_account,
        project_id: params.project_id,
    };
    let result = host.invoke_contract(
        &user_contract_addr,
        &curate_param,
        func,
        Amount::zero(),
    );

    match result {
        Ok((_, _)) => Ok(()),
        Err(_) => Err(Error::FailedInvokeUserContract),
    }
}

#[receive(
    contract = "overlay-projects",
    name = "curate_project_admin",
    parameter = "CurateProjectAdminParam",
    mutable,
    error = "Error"
)]
fn contract_curate_project_admin<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: CurateProjectAdminParam = ctx.parameter_cursor().get()?;
    let func = EntrypointName::new_unchecked("view_user");
    let user_contract_addr = host.state_mut().user_contract_addr;

    ensure!(ctx.invoker() == host.state().admin, Error::InvalidCaller);

    let view_user_params = AddrParam {
        addr: params.curator,
    };
    let user_state: UserStateResponse = host
        .invoke_contract_read_only(
            &user_contract_addr,
            &view_user_params,
            func,
            Amount::zero(),
        ).unwrap().ok_or(Error::FailedInvokeUserContractView)?.get()?;

    ensure!(user_state.is_curator, Error::InvalidCaller);

    let state = host.state_mut();
    state.project.entry(params.project_id.clone()).or_insert_with(|| ProjectState {
            project_uri: Some(params.project_uri),
            owners: params.owners,
            pub_key: None,
            token_addr: None,
            seed_nft_addr: None,
            sale_addr: None,
            status: ProjectStatus::Candidate,
        },
    );

    let func = EntrypointName::new("curate".into()).unwrap();
    let curate_param = CurateParam {
        addr: params.curator,
        project_id: params.project_id,
    };
    let result = host.invoke_contract(
        &user_contract_addr,
        &curate_param,
        func,
        Amount::zero(),
    );

    match result {
        Ok((_, _)) => Ok(()),
        Err(_) => Err(Error::FailedInvokeUserContract),
    }
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

    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(Error::OnlyAccount),
        Address::Account(account_address) => account_address,
    };

    let view_user_params = AddrParam {
        addr: sender_account,
    };
    let user_state: UserStateResponse = host
        .invoke_contract_read_only(
            &user_contract_addr,
            &view_user_params,
            func,
            Amount::zero(),
        ).unwrap().ok_or(Error::FailedInvokeUserContractView)?.get()?;

    ensure!(user_state.is_validator, Error::InvalidCaller);

    let state = host.state_mut();
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(
        old_values.status == ProjectStatus::Candidate,
        Error::InvalidStatus
    );

    state
        .project
        .entry(params.project_id.clone())
        .and_modify(|project_state| {
            project_state.status = ProjectStatus::Whitelist;
        });

    let func = EntrypointName::new("validate".into()).unwrap();
    let validate_param = ValidateParam {
        addr: sender_account,
        project_id: params.project_id,
    };
    let result = host.invoke_contract(
        &user_contract_addr,
        &validate_param,
        func,
        Amount::zero(),
    );

    match result {
        Ok((_, _)) => Ok(()),
        Err(_) => Err(Error::FailedInvokeUserContract),
    }
}

#[receive(
    contract = "overlay-projects",
    name = "validate_project_admin",
    parameter = "ValidateProjectAdminParam",
    mutable,
    error = "Error"
)]
fn contract_validate_project_admin<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: ValidateProjectAdminParam = ctx.parameter_cursor().get()?;
    let func = EntrypointName::new("view_user".into()).unwrap();
    let user_contract_addr = host.state_mut().user_contract_addr;

    ensure!(ctx.invoker() == host.state().admin, Error::InvalidCaller);

    let view_user_params = AddrParam {
        addr: params.validator,
    };
    let user_state: UserStateResponse = host
        .invoke_contract_read_only(
            &user_contract_addr,
            &view_user_params,
            func,
            Amount::zero(),
        ).unwrap().ok_or(Error::FailedInvokeUserContractView)?.get()?;

    ensure!(user_state.is_validator, Error::InvalidCaller);

    let state = host.state_mut();
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(
        old_values.status == ProjectStatus::Candidate,
        Error::InvalidStatus
    );

    state
        .project
        .entry(params.project_id.clone())
        .and_modify(|project_state| {
            project_state.status = ProjectStatus::Whitelist;
        });

    let func = EntrypointName::new("validate".into()).unwrap();
    let validate_param = ValidateParam {
        addr: params.validator,
        project_id: params.project_id,
    };
    let result = host.invoke_contract(
        &user_contract_addr,
        &validate_param,
        func,
        Amount::zero(),
    );

    match result {
        Ok((_, _)) => Ok(()),
        Err(_) => Err(Error::FailedInvokeUserContract),
    }
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
    ensure!(
        old_values.owners.contains(&ctx.invoker()),
        Error::InvalidCaller
    );
    ensure!(
        (old_values.status == ProjectStatus::Whitelist && old_values.seed_nft_addr != None) ||
        (old_values.status == ProjectStatus::Candidate && old_values.seed_nft_addr == None),
        Error::InvalidStatus
    );

    state
        .project
        .entry(params.project_id)
        .and_modify(|project_state| {
            project_state.token_addr = Some(params.token_addr);
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
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    ensure!(
        old_values.status != ProjectStatus::Candidate,
        Error::InvalidStatus
    );

    state
        .project
        .entry(params.project_id)
        .and_modify(|project_state| {
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
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    ensure!(
        old_values.status != ProjectStatus::Candidate,
        Error::InvalidStatus
    );
    state
        .project
        .entry(params.project_id)
        .and_modify(|project_state| {
            project_state.owners = params.owners;
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
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    ensure!(
        old_values.status == ProjectStatus::Whitelist,
        Error::InvalidStatus
    );

    state
        .project
        .entry(params.project_id)
        .and_modify(|project_state| {
            project_state.seed_nft_addr = Some(params.seed_nft_addr);
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
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    ensure!(
        old_values.status == ProjectStatus::Whitelist,
        Error::InvalidStatus
    );
    ensure!(old_values.seed_nft_addr == None, Error::ShouldNotBeTON);

    state
        .project
        .entry(params.project_id)
        .and_modify(|project_state| {
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
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    ensure!(
        old_values.status == ProjectStatus::Whitelist,
        Error::InvalidStatus
    );

    state
        .project
        .entry(params.project_id)
        .and_modify(|project_state| {
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
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    ensure!(
        old_values.status == ProjectStatus::OnSale,
        Error::InvalidStatus
    );

    state
        .project
        .entry(params.project_id)
        .and_modify(|project_state| {
            project_state.status = ProjectStatus::SaleClosed;
        });
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "upgrade",
    parameter = "UpgradeParam",
    mutable
)]
fn contract_upgrade<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<()> {
    ensure!(ctx.sender().matches_account(&ctx.owner()));
    let params: UpgradeParam = ctx.parameter_cursor().get()?;
    host.upgrade(params.module)?;
    if let Some((func, parameter)) = params.migrate {
        host.invoke_contract_raw(
            &ctx.self_address(),
            parameter.as_parameter(),
            func.as_entrypoint_name(),
            Amount::zero(),
        )?;
    }
    Ok(())
}

#[receive(
    contract = "overlay-projects",
    name = "view_admin",
    return_value = "ViewAdminRes"
)]
fn contract_view_admin<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<ViewAdminRes> {
    let state = host.state();
    ensure!(
        ctx.sender() == Address::Account(host.state().admin),
        Error::InvalidCaller
    );
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
fn contract_view_project<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<ProjectState> {
    let params: ViewProjectParam = ctx.parameter_cursor().get()?;
    let state = host.state();
    let project_state = state.project.get(&params.project_id).unwrap();
    Ok(ProjectState {
        owners: project_state.owners.clone(),
        project_uri: project_state.project_uri.clone(),
        pub_key: project_state.pub_key.clone(),
        token_addr: project_state.token_addr.clone(),
        seed_nft_addr: project_state.seed_nft_addr.clone(),
        sale_addr: project_state.sale_addr.clone(),
        status: project_state.status.clone(),
    })
}

type ViewProjectsResponse = Vec<(ProjectId, ProjectState)>;

#[receive(
    contract = "overlay-projects",
    name = "view_projects",
    return_value = "ViewProjectsResponse"
)]
fn contract_view_projects<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<ViewProjectsResponse> {
    let projects_state = &host.state().project;
    let mut projects_state_response: ViewProjectsResponse = Vec::new();
    for (project_id, project_state) in projects_state.iter() {
        projects_state_response.push((
            project_id.clone(),
            project_state.clone()
        ));
    }
    Ok(projects_state_response)
}

type ViewProjectIdsResponse = Vec<ProjectId>;

#[receive(
    contract = "overlay-projects",
    name = "view_project_ids",
    return_value = "ViewProjectIdsResponse"
)]
fn contract_view_project_ids<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<ViewProjectIdsResponse> {
    let projects_state = &host.state().project;
    let mut project_ids_response: ViewProjectIdsResponse = Vec::new();
    for (project_id, _project_state) in projects_state.iter() {
        project_ids_response.push(project_id.clone());
    }
    Ok(project_ids_response)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
