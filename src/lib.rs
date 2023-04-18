//! OVERLAY projects smart contract.
//!
//! This is the repository that stores OVERLAY project's data.

#![cfg_attr(not(feature = "std"), no_std)]
use concordium_std::*;
use core::fmt::Debug;

type ProjectId = String;
type ProjectUri = String;
type PublicKey = String;

/// The state of the OVERLAY projects.
#[derive(Serial, DeserialWithState, StateClone)]
#[concordium(state_parameter = "S")]
struct State<S> {
    /// Owner/Admin address of this contract module.
    admin: AccountAddress,
    staking_contract_addr: ContractAddress,
    /// overlay-users contract address that handles user's data.
    user_contract_addr: ContractAddress,
    /// OVERLAY project data map.
    project: StateMap<ProjectId, ProjectState, S>,
}

/// The state of a single OVERLAY project.
#[derive(Serial, Deserial, SchemaType, Clone)]
struct ProjectState {
    project_uri: Option<ProjectUri>,
    owners: Vec<AccountAddress>,
    pub_key: Option<PublicKey>,
    token_addr: Option<ContractAddress>,
    seed_nft_addr: Option<ContractAddress>,
    sale_addr: Option<ContractAddress>,
    status: ProjectStatus,
}

/// Listing status of the project.
#[derive(Debug, PartialEq, Eq, Reject, Serial, Deserial, SchemaType, Clone)]
enum ProjectStatus {
    /// Candidate for token sale.
    Candidate,
    /// Period for lottery of users participating in the token sale.
    /// OnSale status will come after this status.
    Whitelist,
    /// Token sale is currently held.
    OnSale,
    /// Token sale is closed.
    SaleClosed,
}

/// The response schema for `overlay-users.view_user` function.
/// For more information see https://github.com/overlaydao/overlay-users.
#[derive(Serial, Deserial, SchemaType, Clone)]
struct UserStateResponse {
    is_curator: bool,
    is_validator: bool,
    curated_projects: Vec<ProjectId>,
    validated_projects: Vec<ProjectId>,
}

/// The parameter schema for `overlay-users.view_user` function.
/// For more information see https://github.com/overlaydao/overlay-users.
#[derive(Serial, Deserial, SchemaType)]
struct ViewUserParam {
    addr: AccountAddress,
}

/// The parameter schema for `overlay-users.curate` function.
/// For more information see https://github.com/overlaydao/overlay-users.
#[derive(Serial, Deserial, SchemaType)]
struct CurateParam {
    addr: AccountAddress,
    project_id: ProjectId,
}

/// The parameter schema for `overlay-users.validate` function.
/// For more information see https://github.com/overlaydao/overlay-users.
#[derive(Serial, Deserial, SchemaType)]
struct ValidateParam {
    addr: AccountAddress,
    project_id: ProjectId,
}

/// The parameter schema for `update_contract_state` function.
#[derive(Serial, Deserial, SchemaType)]
struct UpdateContractStateParam {
    staking_contract_addr: ContractAddress,
    user_contract_addr: ContractAddress,
}
/// The parameter schema for `init` function.
type InitParam = UpdateContractStateParam;

/// The parameter schema for `transfer_admin` function.
#[derive(Serial, Deserial, SchemaType)]
struct TransferAdminParam {
    admin: AccountAddress,
}

/// The parameter schema for `curate_project` function.
#[derive(Serial, Deserial, SchemaType)]
struct CurateProjectParam {
    project_id: ProjectId,
    project_uri: ProjectUri,
    owners: Vec<AccountAddress>,
}
/// The parameter schema for `apply_curate_project` function.
type ApplyCurateProjectParam = CurateProjectParam;

/// The parameter schema for `curate_project_admin` function.
#[derive(Serial, Deserial, SchemaType)]
struct CurateProjectAdminParam {
    curator: AccountAddress,
    project_id: ProjectId,
    project_uri: ProjectUri,
    owners: Vec<AccountAddress>,
}

/// The parameter schema for `validate_project` function.
#[derive(Serial, Deserial, SchemaType)]
struct ValidateProjectParam {
    project_id: ProjectId,
    owners: Vec<AccountAddress>,
    token_addr: Option<ContractAddress>,
}

/// The parameter schema for `validate_project_admin` function.
#[derive(Serial, Deserial, SchemaType)]
struct ValidateProjectAdminParam {
    validator: AccountAddress,
    project_id: ProjectId,
    owners: Vec<AccountAddress>,
    token_addr: Option<ContractAddress>,
}

/// The parameter schema for `add_pub_key` function.
#[derive(Serial, Deserial, SchemaType)]
struct AddPubKeyParam {
    project_id: ProjectId,
    pub_key: PublicKey,
}

/// The parameter schema for `update_owners` function.
#[derive(Serial, Deserial, SchemaType)]
struct UpdateOwnersParam {
    project_id: ProjectId,
    owners: Vec<AccountAddress>,
}

/// The parameter schema for `add_seed_sale` function.
#[derive(Serial, Deserial, SchemaType)]
struct AddSeedSaleParam {
    project_id: ProjectId,
    seed_nft_addr: ContractAddress,
}

/// The parameter schema for `add_token_addr` function.
#[derive(Serial, Deserial, SchemaType)]
struct AddTokenAddrParam {
    project_id: ProjectId,
    token_addr: ContractAddress,
}

/// The parameter schema for `add_sale` function.
#[derive(Serial, Deserial, SchemaType)]
struct AddSaleParam {
    project_id: ProjectId,
    sale_addr: ContractAddress,
}

/// The parameter schema for `start_sale` function.
#[derive(Serial, Deserial, SchemaType)]
struct StartSaleParam {
    project_id: ProjectId,
}

/// The parameter schema for `close_sale` function.
#[derive(Serial, Deserial, SchemaType)]
struct CloseSaleParam {
    project_id: ProjectId,
}

/// The parameter schema for `upgrade` function.
#[derive(Debug, Serialize, SchemaType)]
struct UpgradeParam {
    module: ModuleReference,
    migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
}

/// The parameter schema for `view_project` function.
#[derive(Serial, Deserial, SchemaType)]
struct ViewProjectParam {
    project_id: ProjectId,
}

/// The response schema for `view_admin` function.
#[derive(Serial, Deserial, SchemaType)]
struct ViewAdminRes {
    admin: AccountAddress,
    staking_contract_addr: ContractAddress,
    user_contract_addr: ContractAddress,
}

/// The response schema for `view_project` function.
type ViewProjectResponse = ProjectState;

/// The response schema for `view_projects` function.
type ViewProjectsResponse = Vec<(ProjectId, ProjectState)>;

/// The response schema for `view_project_ids` function.
type ViewProjectIdsResponse = Vec<ProjectId>;

/// Custom error definitions of OVERLAY projects smart contract.
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
    ProjectHasBeenInitializedAlready,
    ProjectNotFound,
}

type ContractResult<A> = Result<A, Error>;

/// The smart contract module init function.
/// Although anyone can init this module, this function is expected to be called by OVERLAY team.
#[init(contract = "overlay-projects", parameter = "InitParam")]
fn contract_init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    let params: InitParam = ctx.parameter_cursor().get()?;
    let state = State {
        admin: ctx.init_origin(),
        staking_contract_addr: params.staking_contract_addr,
        user_contract_addr: params.user_contract_addr,
        project: state_builder.new_map(),
    };
    Ok(state)
}

/// Update associated staking/user contract address.
///
/// Caller: current admin account.
/// Reject if:
/// * Caller is not the current admin account.
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
    let state = host.state_mut();
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    let params: UpdateContractStateParam = ctx.parameter_cursor().get()?;
    state.staking_contract_addr = params.staking_contract_addr;
    state.user_contract_addr = params.user_contract_addr;
    Ok(())
}

/// Transfer admin of this module to another account.
///
/// Caller: current admin account.
/// Reject if:
/// * Caller is not the current admin account.
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
    let state = host.state_mut();
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    let params: TransferAdminParam = ctx.parameter_cursor().get()?;
    state.admin = params.admin;
    Ok(())
}

/// Init project and add to project map.
///
/// Caller: current admin account.
/// Reject if:
/// * Caller is not the current admin account.
/// * The inputted project has already registered.
#[receive(
    contract = "overlay-projects",
    name = "apply_curate_project",
    parameter = "ApplyCurateProjectParam",
    mutable,
    error = "Error"
)]
fn contract_apply_curate_project<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    let state = host.state_mut();
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    let params: ApplyCurateProjectParam = ctx.parameter_cursor().get()?;
    let existed = state.project.insert(
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
    ensure!(existed.is_none(), Error::ProjectHasBeenInitializedAlready);
    Ok(())
}

/// Add inputted project to curated project list of caller's overlay-user state.
///
/// Caller: Anyone who is a curator user.
/// Reject if:
/// * Caller is not overlay user marked as curator.
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

    // let's check the caller is the curator.
    let func = EntrypointName::new_unchecked("view_user");
    let user_contract_addr = host.state_mut().user_contract_addr;
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(Error::OnlyAccount),
        Address::Account(account_address) => account_address,
    };
    let view_user_params = ViewUserParam {
        addr: sender_account,
    };
    let user_state: UserStateResponse = host
        .invoke_contract_read_only(&user_contract_addr, &view_user_params, func, Amount::zero())
        .unwrap()
        .ok_or(Error::FailedInvokeUserContractView)?
        .get()?;
    ensure!(user_state.is_curator, Error::InvalidCaller);

    // TODO need to check the project id is proper??
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

    // let's add the project to curated project list of this overlay-user's state.
    let func = EntrypointName::new("curate".into()).unwrap();
    let curate_param = CurateParam {
        addr: sender_account,
        project_id: params.project_id,
    };
    host.invoke_contract(&user_contract_addr, &curate_param, func, Amount::zero())
        .map(|(_, _)| ())
        .map_err(|_| Error::FailedInvokeUserContract)
}

/// TODO TBD
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
    ensure!(ctx.invoker() == host.state().admin, Error::InvalidCaller);
    let params: CurateProjectAdminParam = ctx.parameter_cursor().get()?;
    let func = EntrypointName::new_unchecked("view_user");
    let user_contract_addr = host.state_mut().user_contract_addr;
    let view_user_params = ViewUserParam {
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

/// Add inputted project to validated project list of caller's overlay-user state.
///
/// Caller: Anyone who is a validator user.
/// Reject if:
/// * Caller is not overlay user marked as validator.
/// * The inputted project id has not been registered or its status is not Candidate.
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

    // let's check the caller is the curator.
    let func = EntrypointName::new("view_user".into()).unwrap();
    let user_contract_addr = host.state_mut().user_contract_addr;
    let sender_account = match ctx.sender() {
        Address::Contract(_) => bail!(Error::OnlyAccount),
        Address::Account(account_address) => account_address,
    };
    let view_user_params = ViewUserParam {
        addr: sender_account,
    };
    let user_state: UserStateResponse = host
        .invoke_contract_read_only(&user_contract_addr, &view_user_params, func, Amount::zero())
        .unwrap()
        .ok_or(Error::FailedInvokeUserContractView)?
        .get()?;
    ensure!(user_state.is_validator, Error::InvalidCaller);

    let state = host.state_mut();
    let project = state.project.get(&params.project_id);
    ensure!(project.is_some(), Error::ProjectNotFound);
    let project = project.unwrap();
    ensure!(
        project.status == ProjectStatus::Candidate,
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

/// TODO TBD
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
    ensure!(ctx.invoker() == host.state().admin, Error::InvalidCaller);
    let params: ValidateProjectAdminParam = ctx.parameter_cursor().get()?;

    // let's call the inputted validator address is actually a validator.
    let func = EntrypointName::new("view_user".into()).unwrap();
    let user_contract_addr = host.state_mut().user_contract_addr;
    let view_user_params = ViewUserParam {
        addr: params.validator,
    };
    let user_state: UserStateResponse = host
        .invoke_contract_read_only(&user_contract_addr, &view_user_params, func, Amount::zero())
        .unwrap()
        .ok_or(Error::FailedInvokeUserContractView)?
        .get()?;
    ensure!(user_state.is_validator, Error::InvalidCaller);

    let state = host.state_mut();
    let project = state.project.get(&params.project_id);
    ensure!(project.is_some(), Error::ProjectNotFound);
    let project = project.unwrap();
    ensure!(
        project.status == ProjectStatus::Candidate,
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

/// Update token address of the inputted project.
///
/// Caller: Owner of the project.
/// Reject if:
/// * Caller is not the owner of the project.
/// * The inputted project id has not been registered.
/// * The inputted project state dose not match with any conditions below.
///   * status == Whitelist AND seed_nft_addr != None
///   * status == Candidate AND seed_nft_addr == None
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
    let project = state.project.get_mut(&params.project_id);
    ensure!(project.is_some(), Error::ProjectNotFound);
    let mut project = project.unwrap();
    ensure!(
        project.owners.contains(&ctx.invoker()),
        Error::InvalidCaller
    );
    ensure!(
        (project.status == ProjectStatus::Whitelist && project.seed_nft_addr != None)
            || (project.status == ProjectStatus::Candidate && project.seed_nft_addr == None),
        Error::InvalidStatus
    );
    project.token_addr = Some(params.token_addr);
    Ok(())
}

/// Update public key of the inputted project.
///
/// Caller: current admin account.
/// Reject if:
/// * Caller is not the current admin account.
/// * The inputted project id has not been registered.
/// * The inputted project state is Candidate
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
    let state = host.state_mut();
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    let params: AddPubKeyParam = ctx.parameter_cursor().get()?;
    let project = state.project.get_mut(&params.project_id);
    ensure!(project.is_some(), Error::ProjectNotFound);
    let mut project = project.unwrap();
    ensure!(
        project.status != ProjectStatus::Candidate,
        Error::InvalidStatus
    );
    project.pub_key = Some(params.pub_key);
    Ok(())
}

/// Update owners of the inputted project.
///
/// Caller: current admin account.
/// Reject if:
/// * Caller is not the current admin account.
/// * The inputted project id has not been registered.
/// * The inputted project state is Candidate
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
    let state = host.state_mut();
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    let params: UpdateOwnersParam = ctx.parameter_cursor().get()?;
    let project = state.project.get_mut(&params.project_id);
    ensure!(project.is_some(), Error::ProjectNotFound);
    let mut project = project.unwrap();
    ensure!(
        project.status != ProjectStatus::Candidate,
        Error::InvalidStatus
    );
    project.owners = params.owners;
    Ok(())
}

/// Update seed NFT address of the inputted project.
///
/// Caller: current admin account.
/// Reject if:
/// * Caller is not the current admin account.
/// * The inputted project id has not been registered.
/// * The inputted project state is not Whitelist
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
    let state = host.state_mut();
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    let params: AddSeedSaleParam = ctx.parameter_cursor().get()?;
    let project = state.project.get_mut(&params.project_id);
    ensure!(project.is_some(), Error::ProjectNotFound);
    let mut project = project.unwrap();
    ensure!(
        project.status == ProjectStatus::Whitelist,
        Error::InvalidStatus
    );
    project.seed_nft_addr = Some(params.seed_nft_addr);
    Ok(())
}

/// Update sale address of the inputted project.
///
/// Caller: current admin account.
/// Reject if:
/// * Caller is not the current admin account.
/// * The inputted project id has not been registered.
/// * The inputted project state is not Whitelist.
/// * The inputted project seed NFT address is not None.
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
    let state = host.state_mut();
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    let params: AddSaleParam = ctx.parameter_cursor().get()?;
    let project = state.project.get_mut(&params.project_id);
    ensure!(project.is_some(), Error::ProjectNotFound);
    let mut project = project.unwrap();
    ensure!(
        project.status == ProjectStatus::Whitelist,
        Error::InvalidStatus
    );
    ensure!(project.seed_nft_addr == None, Error::ShouldNotBeTON);
    project.sale_addr = Some(params.sale_addr);
    Ok(())
}

/// Update the inputted project status as OnSale.
///
/// Caller: current admin account.
/// Reject if:
/// * Caller is not the current admin account.
/// * The inputted project id has not been registered.
/// * The inputted project state is not Whitelist.
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
    let state = host.state_mut();
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    let params: StartSaleParam = ctx.parameter_cursor().get()?;
    let project = state.project.get_mut(&params.project_id);
    ensure!(project.is_some(), Error::ProjectNotFound);
    let mut project = project.unwrap();
    ensure!(
        project.status == ProjectStatus::Whitelist,
        Error::InvalidStatus
    );
    project.status = ProjectStatus::OnSale;
    Ok(())
}

/// Update the inputted project status as SaleClosed.
///
/// Caller: current admin account.
/// Reject if:
/// * Caller is not the current admin account.
/// * The inputted project id has not been registered.
/// * The inputted project state is not OnSale.
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
    let state = host.state_mut();
    ensure!(ctx.invoker() == state.admin, Error::InvalidCaller);
    let params: CloseSaleParam = ctx.parameter_cursor().get()?;
    let project = state.project.get_mut(&params.project_id);
    ensure!(project.is_some(), Error::ProjectNotFound);
    let mut project = project.unwrap();
    ensure!(
        project.status == ProjectStatus::OnSale,
        Error::InvalidStatus
    );
    project.status = ProjectStatus::SaleClosed;
    Ok(())
}

/// Smart contract module upgrade function.
/// For more information see https://developer.concordium.software/en/mainnet/smart-contracts/guides/upgradeable-contract.html#guide-upgradable-contract
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

/// View the admin state.
///
/// Caller: Admin account only.
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
        ctx.sender() == Address::Account(state.admin),
        Error::InvalidCaller
    );
    Ok(ViewAdminRes {
        admin: state.admin,
        staking_contract_addr: state.staking_contract_addr,
        user_contract_addr: state.user_contract_addr,
    })
}

/// View the project state.
///
/// Caller: Any accounts / Any contracts
/// Reject if:
/// * The inputted project id has not been registered.
#[receive(
    contract = "overlay-projects",
    name = "view_project",
    parameter = "ViewProjectParam",
    return_value = "ProjectState"
)]
fn contract_view_project<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<ViewProjectResponse> {
    let params: ViewProjectParam = ctx.parameter_cursor().get()?;
    let state = host.state();
    let project = state.project.get(&params.project_id);
    ensure!(project.is_some(), Error::ProjectNotFound);
    let project = project.unwrap();
    Ok(ViewProjectResponse {
        owners: project.owners.clone(),
        project_uri: project.project_uri.clone(),
        pub_key: project.pub_key.clone(),
        token_addr: project.token_addr.clone(),
        seed_nft_addr: project.seed_nft_addr.clone(),
        sale_addr: project.sale_addr.clone(),
        status: project.status.clone(),
    })
}

/// View all project states.
///
/// Caller: Any accounts / Any contracts
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
    let projects_state_response: ViewProjectsResponse = projects_state
        .iter()
        .map(|(project_id, project_state)| (project_id.clone(), project_state.clone()))
        .collect();
    Ok(projects_state_response)
}

/// View all project ids.
///
/// Caller: Any accounts / Any contracts
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
    let project_ids_response: ViewProjectIdsResponse = projects_state
        .iter()
        .map(|(project_id, _project_state)| project_id.clone())
        .collect();
    Ok(project_ids_response)
}

/// implements Debug for State inside test functions.
/// this implementation will be build only when `concordium-std/wasm-test` feature is active.
/// (e.g. when launched by `cargo concordium test`)
#[concordium_cfg_test]
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

/// implements PartialEq for `claim_eq` inside test functions.
/// this implementation will be build only when `concordium-std/wasm-test` feature is active.
/// (e.g. when launched by `cargo concordium test`)
#[concordium_cfg_test]
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

/// implements Debug for ProjectState inside test functions.
/// this implementation will be build only when `concordium-std/wasm-test` feature is active.
/// (e.g. when launched by `cargo concordium test`)
#[concordium_cfg_test]
impl Debug for ProjectState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "project_uri: {:?}, owners: {:?}, pub_key: {:?}, token_addr: {:?}, seed_nft_addr: {:?}, sale_addr: {:?}, status: {:?}",
            self.project_uri, self.owners, self.pub_key, self.token_addr, self.seed_nft_addr, self.sale_addr, self.status
        )
    }
}

/// implements PartialEq for `claim_eq` inside test functions.
/// this implementation will be build only when `concordium-std/wasm-test` feature is active.
/// (e.g. when launched by `cargo concordium test`)
#[concordium_cfg_test]
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
        let params = InitParam {
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
        let project_uri: ProjectUri = "somethingdangerous".into();
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

        let params = ApplyCurateProjectParam {
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
        let project_uri: ProjectUri = "https://overlay.global/".into();
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

        let params = ApplyCurateProjectParam {
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
        let project_uri: ProjectUri = "somethingdangerous".into();
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
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
        let project_uri: ProjectUri = "somethingdangerous".into();
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
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
        let pub_key: PublicKey = "test-pub-key".into();
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
        let pub_key: PublicKey = "test-pub-key".into();
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
        let project_uri: ProjectUri = "https://overlay.global/".into();
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

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking overlay-projects.start_sale.
    fn test_contract_start_sale_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: ProjectUri = "https://overlay.global/".into();
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

        let params = StartSaleParam { project_id };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_start_sale(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.start_sale successfully update project's status as "on sale".
    fn test_contract_start_sale() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::OnSale,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = StartSaleParam { project_id };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_start_sale(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that with_rollback works for the state on invoking overlay-projects.close_sale.
    fn test_contract_close_sale_with_rollback() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: ProjectUri = "https://overlay.global/".into();
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

        let params = CloseSaleParam { project_id };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_close_sale(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }

    #[concordium_test]
    /// Test that overlay-projects.close_sale successfully update project's status as "sale closed".
    fn test_contract_close_sale() {
        let admin = AccountAddress([1; 32]);
        let staking_contract_addr = ContractAddress::new(1000, 0);
        let user_contract_addr = ContractAddress::new(1001, 0);
        let project_id: ProjectId = "DLSFJJ&&X87877XJJN".into();
        let project_uri: ProjectUri = "https://overlay.global/".into();
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
                status: ProjectStatus::OnSale,
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
                status: ProjectStatus::SaleClosed,
            },
        );
        let expected_state = State {
            admin,
            staking_contract_addr,
            user_contract_addr,
            project: expected_project,
        };
        let mut host = TestHost::new(initial_state, state_builder);

        let params = CloseSaleParam { project_id };
        let params_byte = to_bytes(&params);
        ctx.set_parameter(&params_byte);
        let _ = host.with_rollback(|host| contract_close_sale(&ctx, host));
        let actual_state = host.state();
        claim_eq!(
            *actual_state,
            expected_state,
            "state has been changed unexpectedly..."
        );
    }
}
