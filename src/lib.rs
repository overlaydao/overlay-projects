#![cfg_attr(not(feature = "std"), no_std)]
use concordium_std::*;
use core::fmt::Debug;

type ProjectId = String;

/*
 * TODO it might to be better to define project_uri and pub_key types.
type ProjectUri = String;
type PublicKey = String;
 */

#[derive(Serial, DeserialWithState, StateClone)]
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

#[derive(Serial, Deserial, SchemaType, Clone)]
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
    FailedInvokeUserContractView,
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
    // TODO if the project already exists, it will be initialized... is it OK?
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
        .invoke_contract_read_only(&user_contract_addr, &view_user_params, func, Amount::zero())
        .unwrap()
        .ok_or(Error::FailedInvokeUserContractView)?
        .get()?;

    ensure!(user_state.is_curator, Error::InvalidCaller);

    let state = host.state_mut();
    state
        .project
        .entry(params.project_id.clone())
        .or_insert_with(|| ProjectState {
            project_uri: Some(params.project_uri),
            owners: params.owners,
            pub_key: None,
            token_addr: None,
            seed_nft_addr: None,
            sale_addr: None,
            status: ProjectStatus::Candidate,
        });

    let func = EntrypointName::new("curate".into()).unwrap();
    let curate_param = CurateParam {
        addr: sender_account,
        project_id: params.project_id,
    };
    host.invoke_contract(&user_contract_addr, &curate_param, func, Amount::zero())
        .map(|(_, _)| ())
        .map_err(|_| Error::FailedInvokeUserContract)
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
        .invoke_contract_read_only(&user_contract_addr, &view_user_params, func, Amount::zero())
        .unwrap()
        .ok_or(Error::FailedInvokeUserContractView)?
        .get()?;

    ensure!(user_state.is_curator, Error::InvalidCaller);

    let state = host.state_mut();
    state
        .project
        .entry(params.project_id.clone())
        .or_insert_with(|| ProjectState {
            project_uri: Some(params.project_uri),
            owners: params.owners,
            pub_key: None,
            token_addr: None,
            seed_nft_addr: None,
            sale_addr: None,
            status: ProjectStatus::Candidate,
        });
    // TODO is it OK not to call `overlay-users.curate` here?
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

    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(Error::OnlyAccount),
        Address::Account(account_address) => account_address,
    };

    let view_user_params = AddrParam {
        addr: sender_account,
    };
    let user_state: UserStateResponse = host
        .invoke_contract_read_only(&user_contract_addr, &view_user_params, func, Amount::zero())
        .unwrap()
        .ok_or(Error::FailedInvokeUserContractView)?
        .get()?;

    ensure!(user_state.is_validator, Error::InvalidCaller);

    let state = host.state_mut();
    // TODO there may be no project inside "project" map. check the project map has project_id key
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(
        old_values.status == ProjectStatus::Candidate,
        Error::InvalidStatus
    );

    // TODO is it OK params.token_addr / params.owners is not used anywhere inside this function.
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
    host.invoke_contract(&user_contract_addr, &validate_param, func, Amount::zero())
        .map(|(_, _)| ())
        .map_err(|_| Error::FailedInvokeUserContract)
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
        .invoke_contract_read_only(&user_contract_addr, &view_user_params, func, Amount::zero())
        .unwrap()
        .ok_or(Error::FailedInvokeUserContractView)?
        .get()?;

    ensure!(user_state.is_validator, Error::InvalidCaller);

    let state = host.state_mut();
    // TODO there may be no project inside "project" map. check the project map has project_id key
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(
        old_values.status == ProjectStatus::Candidate,
        Error::InvalidStatus
    );

    // TODO is it OK params.token_addr / params.owners is not used anywhere inside this function.
    state
        .project
        .entry(params.project_id.clone())
        .and_modify(|project_state| {
            project_state.status = ProjectStatus::Whitelist;
        });

    // TODO is it OK not to call `overlay-users.validate` here?
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
    // TODO there may be no project inside "project" map. check the project map has project_id key
    let old_values = state.project.get(&params.project_id).unwrap();
    ensure!(
        old_values.owners.contains(&ctx.invoker()),
        Error::InvalidCaller
    );
    ensure!(
        (old_values.status == ProjectStatus::Whitelist && old_values.seed_nft_addr != None)
            || (old_values.status == ProjectStatus::Candidate && old_values.seed_nft_addr == None),
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
    // TODO there may be no project inside "project" map. check the project map has project_id key
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
    // TODO there may be no project inside "project" map. check the project map has project_id key
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
    // TODO there may be no project inside "project" map. check the project map has project_id key
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
        projects_state_response.push((project_id.clone(), project_state.clone()));
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

#[concordium_cfg_test]
/// implements Debug for State inside test functions.
/// this implementation will be build only when `concordium-std/wasm-test` feature is active.
/// (e.g. when launched by `cargo concordium test`)
impl<S: HasStateApi> Debug for State<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "admin: {:?}, staking_contract_addr: {:?}, user_contract_addr: {:?}, ",
            self.admin, self.staking_contract_addr, self.user_contract_addr,
        )?;
        for (project_id, project_state) in self.project.iter() {
            write!(
                f,
                "project_id: {:?}, project_state: {:?}, ",
                project_id, project_state
            )?;
        }
        Ok(())
    }
}

#[concordium_cfg_test]
/// implements PartialEq for `claim_eq` inside test functions.
/// this implementation will be build only when `concordium-std/wasm-test` feature is active.
/// (e.g. when launched by `cargo concordium test`)
impl<S: HasStateApi> PartialEq for State<S> {
    fn eq(&self, other: &Self) -> bool {
        if self.admin != other.admin {
            return false;
        }
        if self.staking_contract_addr != other.staking_contract_addr {
            return false;
        }
        if self.user_contract_addr != other.user_contract_addr {
            return false;
        }
        if self.project.iter().count() != other.project.iter().count() {
            return false;
        }
        for (my_project_id, my_project_state) in self.project.iter() {
            let other_project_state = other.project.get(&my_project_id);
            if other_project_state.is_none() {
                return false;
            }
            let other_project_state = other_project_state.unwrap();
            if my_project_state.clone() != other_project_state.clone() {
                return false;
            }
        }
        true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[concordium_cfg_test]
/// implements Debug for ProjectState inside test functions.
/// this implementation will be build only when `concordium-std/wasm-test` feature is active.
/// (e.g. when launched by `cargo concordium test`)
impl Debug for ProjectState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "project_uri: {:?}, owners: {:?}, pub_key: {:?}, token_addr: {:?}, seed_nft_addr: {:?}, sale_addr: {:?}, status: {:?}",
            self.project_uri, self.owners, self.pub_key, self.token_addr, self.seed_nft_addr, self.sale_addr, self.status
        )
    }
}

#[concordium_cfg_test]
/// implements PartialEq for `claim_eq` inside test functions.
/// this implementation will be build only when `concordium-std/wasm-test` feature is active.
/// (e.g. when launched by `cargo concordium test`)
impl PartialEq for ProjectState {
    fn eq(&self, other: &Self) -> bool {
        self.project_uri == other.project_uri
            && self.owners == other.owners
            && self.pub_key == other.pub_key
            && self.token_addr == other.token_addr
            && self.seed_nft_addr == other.seed_nft_addr
            && self.sale_addr == other.sale_addr
            && self.status == other.status
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    #[concordium_test]
    /// Test that init succeeds.
    fn test_init() {
        // invoker will be an admin
        let invoker = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);

        let mut ctx = TestInitContext::empty();
        let mut state_builder = TestStateBuilder::new();
        ctx.set_init_origin(invoker);

        // prepare for expected state after init
        let expected_state = State {
            admin: invoker,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };

        // create params
        let params = UpdateContractStateParam {
            staking_contract_addr,
            user_contract_addr,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);

        // execute init
        let result = contract_init(&ctx, &mut state_builder);
        // check init result
        claim!(result.is_ok());
        let actual_state = result.unwrap();
        claim_eq!(
            actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking
    /// overlay-projects.update_contract_state.
    fn test_contract_update_contract_state_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let invoker = AccountAddress([7; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let next_staking_contract_addr = ContractAddress::new(2000, 0);
        let next_user_contract_addr = ContractAddress::new(2001, 0);
        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(invoker);

        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = UpdateContractStateParam {
            staking_contract_addr: next_staking_contract_addr,
            user_contract_addr: next_user_contract_addr,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_update_contract_state(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.update_contract_state successfully update
    /// staking_contract_addr and user_contract_addr.
    fn test_contract_update_contract_state() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let next_staking_contract_addr = ContractAddress::new(2000, 0);
        let next_user_contract_addr = ContractAddress::new(2001, 0);
        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);

        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let expected_state = State {
            admin,
            staking_contract_addr: next_staking_contract_addr,
            user_contract_addr: next_user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = UpdateContractStateParam {
            staking_contract_addr: next_staking_contract_addr,
            user_contract_addr: next_user_contract_addr,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let result = contract_update_contract_state(&ctx, &mut host);
        claim!(
            result.is_ok(),
            "test_contract_update_contract_state: Results in rejection"
        );
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking
    /// overlay-projects.transfer_admin.
    fn test_contract_transfer_admin_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let invoker = AccountAddress([8; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(invoker);
        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = TransferAdminParam { admin: invoker };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_transfer_admin(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.transfer_admin successfully update admin.
    fn test_contract_transfer_admin() {
        let admin = AccountAddress([1; 32]);
        let admin_to_be_set = AccountAddress([2u8; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let expected_state = State {
            admin: admin_to_be_set,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = TransferAdminParam {
            admin: admin_to_be_set,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let result = contract_transfer_admin(&ctx, &mut host);
        claim!(
            result.is_ok(),
            "test_contract_transfer_admin: Results in rejection"
        );
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking
    /// overlay-projects.apply_curate_project.
    fn test_contract_apply_curate_project_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let non_admin = AccountAddress([7u8; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: String = "somethingdangerous".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(non_admin);
        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = CurateProjectParam {
            project_id,
            project_uri,
            owners: vec![project_owner1, project_owner2],
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_apply_curate_project(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.apply_curate_project successfully update project.
    fn test_contract_apply_curate_project() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJK".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([5; 32]);
        let project_owner2 = AccountAddress([6; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = CurateProjectParam {
            project_id,
            project_uri,
            owners: vec![project_owner1, project_owner2],
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let result = contract_apply_curate_project(&ctx, &mut host);
        claim!(
            result.is_ok(),
            "test_contract_apply_curate_project: Results in rejection."
        );
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking overlay-projects.curate_project.
    fn test_contract_curate_project_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: String = "somethingdangerous".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);

        let mut ctx = TestReceiveContext::empty();
        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut host = TestHost::new(initial_state, state_builder);

        // set up overlay-users.view_user mock to return non-curator user.
        // this leads to InvalidCaller error.
        host.setup_mock_entrypoint(
            user_contract_addr,
            OwnedEntrypointName::new_unchecked("view_user".to_string()),
            MockFn::returning_ok(UserStateResponse {
                is_curator: false,
                is_validator: false,
                curated_projects: Vec::new(),
                validated_projects: Vec::new(),
            }),
        );

        let params = CurateProjectParam {
            project_id,
            project_uri,
            owners: vec![project_owner1, project_owner2],
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        ctx.set_sender(Address::Account(project_owner1));
        let _ = host.with_rollback(|host| contract_curate_project(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.curate_project successfully invoke overlay-users.curate function.
    fn test_contract_curate_project() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJK".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([5; 32]);
        let project_owner2 = AccountAddress([6; 32]);
        let curator_address = AccountAddress([3; 32]);

        let mut ctx = TestReceiveContext::empty();
        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        // set up overlay-users.view_user mock to return curator user so that the curate func would
        // be called by this func.
        host.setup_mock_entrypoint(
            user_contract_addr,
            OwnedEntrypointName::new_unchecked("view_user".to_string()),
            MockFn::returning_ok(UserStateResponse {
                is_curator: true,
                is_validator: false,
                curated_projects: Vec::new(),
                validated_projects: Vec::new(),
            }),
        );
        // set up overlay-users.curate.
        host.setup_mock_entrypoint(
            user_contract_addr,
            OwnedEntrypointName::new_unchecked("curate".to_string()),
            MockFn::returning_ok(()),
        );

        let params = CurateProjectParam {
            project_id,
            project_uri,
            owners: vec![project_owner1, project_owner2],
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        ctx.set_sender(Address::Account(curator_address));
        let result: ContractResult<()> = contract_curate_project(&ctx, &mut host);
        claim!(
            result.is_ok(),
            "test_contract_curate_project: Results in rejection."
        );
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking
    /// overlay-projects.curate_project_admin.
    fn test_contract_curate_project_admin_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: String = "somethingdangerous".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut host = TestHost::new(initial_state, state_builder);

        // set up overlay-users.view_user mock to return non-curator user.
        // this leads to InvalidCaller error.
        host.setup_mock_entrypoint(
            user_contract_addr,
            OwnedEntrypointName::new_unchecked("view_user".to_string()),
            MockFn::returning_ok(UserStateResponse {
                is_curator: false,
                is_validator: false,
                curated_projects: Vec::new(),
                validated_projects: Vec::new(),
            }),
        );

        let params = CurateProjectAdminParam {
            curator: project_owner1,
            project_id,
            project_uri,
            owners: vec![project_owner1, project_owner2],
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_curate_project_admin(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.curate_project_admin successfully finish its process.
    fn test_contract_curate_project_admin() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJK".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([5; 32]);
        let project_owner2 = AccountAddress([6; 32]);
        let curator = AccountAddress([3; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        // set up overlay-users.view_user mock to return curator user so that the curate func would
        // be called by this func.
        host.setup_mock_entrypoint(
            user_contract_addr,
            OwnedEntrypointName::new_unchecked("view_user".to_string()),
            MockFn::returning_ok(UserStateResponse {
                is_curator: true,
                is_validator: false,
                curated_projects: Vec::new(),
                validated_projects: Vec::new(),
            }),
        );

        let params = CurateProjectAdminParam {
            curator,
            project_id,
            project_uri,
            owners: vec![project_owner1, project_owner2],
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let result: ContractResult<()> = contract_curate_project_admin(&ctx, &mut host);
        claim!(
            result.is_ok(),
            "test_contract_curate_project: Results in rejection."
        );
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking
    /// overlay-projects.contract_validate_project.
    fn test_contract_validate_project_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);
        let non_validator = AccountAddress([9; 32]);

        let mut ctx = TestReceiveContext::empty();
        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut host = TestHost::new(initial_state, state_builder);

        // set up overlay-users.view_user mock to return non-curator user.
        // this leads to InvalidCaller error.
        host.setup_mock_entrypoint(
            user_contract_addr,
            OwnedEntrypointName::new_unchecked("view_user".to_string()),
            MockFn::returning_ok(UserStateResponse {
                is_curator: false,
                is_validator: false,
                curated_projects: Vec::new(),
                validated_projects: Vec::new(),
            }),
        );

        let params = ValidateProjectParam {
            project_id,
            owners: vec![project_owner1, project_owner2],
            token_addr: None,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        ctx.set_sender(Address::Account(non_validator));
        let _ = host.with_rollback(|host| contract_validate_project(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.validate_project successfully invoke overlay-users.validate
    /// function.
    fn test_contract_validate_project() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJK".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([5; 32]);
        let project_owner2 = AccountAddress([6; 32]);
        let validator = AccountAddress([3; 32]);

        let mut ctx = TestReceiveContext::empty();
        let mut state_builder = TestStateBuilder::new();
        let mut initial_project = state_builder.new_map();
        initial_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: initial_project,
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        // set up overlay-users.view_user mock to return curator user so that the curate func would
        // be called by this func.
        host.setup_mock_entrypoint(
            user_contract_addr,
            OwnedEntrypointName::new_unchecked("view_user".to_string()),
            MockFn::returning_ok(UserStateResponse {
                is_curator: false,
                is_validator: true,
                curated_projects: Vec::new(),
                validated_projects: Vec::new(),
            }),
        );
        // set up overlay-users.curate.
        host.setup_mock_entrypoint(
            user_contract_addr,
            OwnedEntrypointName::new_unchecked("validate".to_string()),
            MockFn::returning_ok(()),
        );

        let params = ValidateProjectParam {
            project_id,
            owners: vec![project_owner1, project_owner2],
            token_addr: None,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        ctx.set_sender(Address::Account(validator));
        let result: ContractResult<()> = contract_validate_project(&ctx, &mut host);
        claim!(
            result.is_ok(),
            "test_contract_curate_project: Results in rejection."
        );
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking
    /// overlay-projects.contract_validate_project_admin.
    fn test_contract_validate_project_admin_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);
        let non_validator = AccountAddress([9; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: state_builder.new_map(),
        };
        let mut host = TestHost::new(initial_state, state_builder);

        // set up overlay-users.view_user mock to return non-curator user.
        // this leads to InvalidCaller error.
        host.setup_mock_entrypoint(
            user_contract_addr,
            OwnedEntrypointName::new_unchecked("view_user".to_string()),
            MockFn::returning_ok(UserStateResponse {
                is_curator: false,
                is_validator: false,
                curated_projects: Vec::new(),
                validated_projects: Vec::new(),
            }),
        );

        let params = ValidateProjectAdminParam {
            validator: non_validator,
            project_id,
            owners: vec![project_owner1, project_owner2],
            token_addr: None,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_validate_project_admin(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.validate_project_admin successfully finish its process.
    fn test_contract_validate_project_admin() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJK".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([5; 32]);
        let project_owner2 = AccountAddress([6; 32]);
        let validator = AccountAddress([3; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let mut initial_project = state_builder.new_map();
        initial_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: initial_project,
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        // set up overlay-users.view_user mock to return curator user so that the curate func would
        // be called by this func.
        host.setup_mock_entrypoint(
            user_contract_addr,
            OwnedEntrypointName::new_unchecked("view_user".to_string()),
            MockFn::returning_ok(UserStateResponse {
                is_curator: false,
                is_validator: true,
                curated_projects: Vec::new(),
                validated_projects: Vec::new(),
            }),
        );

        let params = ValidateProjectAdminParam {
            validator,
            project_id,
            owners: vec![project_owner1, project_owner2],
            token_addr: None,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let result: ContractResult<()> = contract_validate_project_admin(&ctx, &mut host);
        claim!(
            result.is_ok(),
            "test_contract_curate_project: Results in rejection."
        );
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking overlay-projects.add_token_addr.
    fn test_contract_add_token_addr_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let token_addr = ContractAddress::new(1002, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(project_owner1);
        let mut state_builder = TestStateBuilder::new();
        let mut initial_project = state_builder.new_map();
        initial_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: initial_project,
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = AddTokenAddrParam {
            project_id,
            token_addr,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_add_token_addr(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.add_token_addr successfully update project's token address.
    fn test_contract_add_token_addr() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let token_addr = ContractAddress::new(1002, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(project_owner1);
        let mut state_builder = TestStateBuilder::new();
        let mut initial_project = state_builder.new_map();
        initial_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: initial_project,
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: Some(token_addr),
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = AddTokenAddrParam {
            project_id,
            token_addr,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_add_token_addr(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking overlay-projects.add_pub_key.
    fn test_contract_add_pub_key_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let pub_key: String = "test-pub-key".into();
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let mut initial_project = state_builder.new_map();
        initial_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: initial_project,
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = AddPubKeyParam {
            project_id,
            pub_key,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_add_pub_key(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.add_pub_key successfully update project's pub_key
    fn test_contract_add_pub_key() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let pub_key: String = "test-pub-key".into();
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let mut initial_project = state_builder.new_map();
        initial_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: initial_project,
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: Some(pub_key.clone()),
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = AddPubKeyParam {
            project_id,
            pub_key,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_add_pub_key(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking overlay-projects.update_owners.
    fn test_contract_update_owners_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);
        let new_project_owner1 = AccountAddress([9; 32]);
        let new_project_owner2 = AccountAddress([10; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let mut initial_project = state_builder.new_map();
        initial_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: initial_project,
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = UpdateOwnersParam {
            project_id,
            owners: vec![new_project_owner1, new_project_owner2],
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_update_owners(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.update_owners successfully update project's pub_key
    fn test_contract_update_owners() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);
        let new_project_owner1 = AccountAddress([9; 32]);
        let new_project_owner2 = AccountAddress([10; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let mut initial_project = state_builder.new_map();
        initial_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: initial_project,
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![new_project_owner1, new_project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = UpdateOwnersParam {
            project_id,
            owners: vec![new_project_owner1, new_project_owner2],
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_update_owners(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking overlay-projects.add_seed_sale.
    fn test_contract_add_seed_sale_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let seed_nft_addr = ContractAddress::new(1002, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let mut initial_project = state_builder.new_map();
        initial_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: initial_project,
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Candidate,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = AddSeedSaleParam {
            project_id,
            seed_nft_addr,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_add_seed_sale(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.add_seed_sale successfully update project's seed NFT address.
    fn test_contract_add_seed_sale() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let seed_nft_addr = ContractAddress::new(1002, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: String = "https://overlay.global/".into();
        let project_owner1 = AccountAddress([7; 32]);
        let project_owner2 = AccountAddress([8; 32]);

        let mut ctx = TestReceiveContext::empty();
        ctx.set_invoker(admin);
        let mut state_builder = TestStateBuilder::new();
        let mut initial_project = state_builder.new_map();
        initial_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
        let initial_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: initial_project,
        };
        let mut expected_project = state_builder.new_map();
        expected_project.insert(
            project_id.clone(),
            ProjectState {
                project_uri: Some(project_uri.clone()),
                owners: vec![project_owner1, project_owner2],
                pub_key: None,
                token_addr: None,
                seed_nft_addr: Some(seed_nft_addr),
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = AddSeedSaleParam {
            project_id,
            seed_nft_addr,
        };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_add_seed_sale(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }
}
