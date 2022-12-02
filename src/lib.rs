use concordium_std::*;
use core::fmt::Debug;

type ProjectId = u64;

#[derive(Serial, Deserial)]
struct ContractState {
    setter: AccountAddress,
    staking_contract_addr: ContractAddress,
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
struct State<S> {
    project: StateMap<ProjectId, ProjectState<S>, S>,
}

#[derive(Serial, Deserial)]
struct ProjectState {
    owners: Option<Vec<AccountAddress>>,
    token_addr: Option<ContractAddress>,
    seed_nft_addr: Option<ContractAddress>,
    sale_addr: Option<ContractAddress>,
    status: ProjectStatus,
}

#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum ProjectStatus {
    Candidate,
    Whitelist,
    OnSaleNFT,
    OnSale,
    SaleSucceeded,
    SaleFailed,
}

#[derive(Serial, Deserial, SchemaType)]
struct UpdateContractStateParams {
    staking_contract_addr: ContractAddress,
}

#[derive(Serial, Deserial, SchemaType)]
struct CureateProjectParam {
    project_id: ProjectId,
}

#[derive(Serial, Deserial, SchemaType)]
struct ValidateProjectParam {
    project_id: ProjectId,
    owners: Vec<AccountAddress>,
    token_addr: ContractAddress,
}

#[derive(Serial, Deserial, SchemaType)]
struct AddSeedSaleParam {
    project_id: ProjectId,
    seed_nft_addr: ContractAddress,
}

#[derive(Serial, Deserial, SchemaType)]
struct AddSaleParam {
    project_id: ProjectId,
    seed_addr: ContractAddress,
}

#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    #[from(ParseError)]
    ParseParamsError,
    InvalidCaller,
}

impl<S: HasStateApi> ContractState {
    fn update(_staking_contract_addr: ContractAddress) -> Self {
        ContractState {
            setter: self.setter,
            staking_contract_addr: _staking_contract_addr,
        }
    }
}

#[init(contract = "overlay-projects", parameter = "InitParams")]
fn contract_init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<ContractState> {
    let params: UpdateContractStateParams = ctx.parameter_cursor().get()?;
    let contract_state = ContractState {
        setter: ctx.center(),
        staking_contract_addr: params.staking_contract_addr,
    };
    Ok(ContractState);
}

#[receive(
    contract = "overlay-projects",
    name = "update_contract_state",
    parameter = "UpdateContractStateParams",
    mutable
)]
fn contract_update_contract_state<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<ContractState>,
) -> ReceiveResult<()> {
    let params: UpdateContractStateParams = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(ctx.sender() == state.setter, Error::InvalidCaller);

    state.update(params.staking_contract_addr);
    Ok(());
}

#[receive(
    contrct = "overlay-projects",
    name = "curate_project",
    parameter = "CureateProjectParam"
    mutable
)]
fn contract_curate_project<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<()> {
    let params: CureateProjectParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let isCurator; // todo   invoke contract to user
    ensure!(isCurator, Error::InvalidCaller);

    state.project.insert(
        params.project_id,
        ProjectState {
            owners: None,
            token_addr: None,
            seed_nft_addr: None,
            sale_addr: None,
            status: ProjectStatus::Candidate,
        },
    );
    Ok(());
}

#[receive(
    contrct = "overlay-projects",
    name = "validate_project",
    parameter = "ValidateProjectParam"
    mutable
)]
fn contract_validate_project<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<()> {
    let params: ValidateProjectParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let isValidator; // todo    invoke contract to user
    ensure!(isValidator, Error::InvalidCaller);

    state.project.insert(
        params.project_id,
        ProjectState {
            owners: Some(params.owners),
            token_addr: Some(params.token_addr),
            seed_nft_addr: None,
            sale_addr: None,
            status: ProjectStatus::Whitelist,
        },
    );
    Ok(());
}

#[receive(
    contrct = "overlay-projects",
    name = "add_seed_sale",
    parameter = "AddSeedSaleParam"
    mutable
)]
fn contract_add_seed_sale<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<ContractState<S>, StateApiType = S>,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<()> {
    let params: AddSeedSaleParam = ctx.parameter_cursor().get()?;
    let contract_state = _host.state();
    let state = host.state_mut();
    let old_values = state.project.get(params.project_id);
    ensure!(ctx.sender() == contract_state.setter, Error::InvalidCaller);

    state.project.insert(
        params.project_id,
        ProjectState {
            owners: Some(old_values.owners),
            token_addr: Some(old_values.token_addr),
            seed_nft_addr: Some(params.seed_nft_addr),
            sale_addr: Some(old_values.sale_addr),
            status: Some(old_values.status),
        },
    )
}

#[receive(
    contrct = "overlay-projects",
    name = "add_sale",
    parameter = "AddSaleParam"
    mutable
)]
fn contract_add_sale<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<ContractState<S>, StateApiType = S>,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<()> {
    let params: AddSaleParam = ctx.parameter_cursor().get()?;
    let contract_state = _host.state();
    let state = host.state_mut();
    let old_values = state.project.get(params.project_id);
    ensure!(ctx.sender() == contract_state.setter, Error::InvalidCaller);

    state.project.insert(
        params.project_id,
        ProjectState {
            owners: Some(old_values.owners),
            token_addr: Some(old_values.token_addr),
            seed_nft_addr: Some(old_values.seed_nft_addr),
            sale_addr: Some(params.sale_addr),
            status: Some(old_values.status),
        },
    );
}

// start_sale

// view_project_state

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
