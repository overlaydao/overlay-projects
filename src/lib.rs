use concordium_std::*;
use core::fmt::Debug;

type ProjectId = String;

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
struct State<S> {
    admin: AccountAddress,
    staking_contract_addr: ContractAddress,
    project: StateMap<ProjectId, ProjectState<S>, S>,
}

#[derive(Serial, Deserial)]
struct ProjectState {
    owners: Option<Vec<AccountAddress>>,
    pub_key: Option<String>,
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
struct UpdateContractStateParam {
    staking_contract_addr: ContractAddress,
}

#[derive(Serial, Deserial, SchemaType)]
struct TransferAdminParam {
    admin: AccountAddress,
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
struct AddPubKeyParam {
    project_id: ProjectId,
    pub_key: String,
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
    seed_addr: ContractAddress,
}

#[derive(Serial, Deserial, SchemaType)]
struct StartSaleParam {
    project_id: ProjectId,
}

#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    #[from(ParseError)]
    ParseParamsError,
    InvalidCaller,
    ShouldBeSeedNFT,
}

#[init(contract = "overlay-projects", parameter = "InitParams")]
fn contract_init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    let params: UpdateContractStateParam = ctx.parameter_cursor().get()?;
    let state = State {
        admin: ctx.center(),
        staking_contract_addr: params.staking_contract_addr,
        project: state_builder.new_map(),
    };
    Ok(state);
}

#[receive(
    contract = "overlay-projects",
    name = "update_contract_state",
    parameter = "UpdateContractStateParam",
    mutable
)]
fn contract_update_contract_state<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State>,
) -> ReceiveResult<()> {
    let params: UpdateContractStateParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(ctx.sender() == state.admin, Error::InvalidCaller);

    state.staking_contract_addr = params.staking_contract_addr;
    Ok(());
}

#[receive(
    contract = "overlay-projects",
    name = "transfer_admin",
    parameter = "TransferAdminParam",
    mutable
)]
fn contract_transfer_admin<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<()> {
    let params: TransferAdminParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(ctx.sender() == state.admin, Error::InvalidCaller);

    state.admin = params.admin;
    Ok(());
}

#[receive(
    contract = "overlay-projects",
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
            pub_key: None,
            token_addr: None,
            seed_nft_addr: None,
            sale_addr: None,
            status: ProjectStatus::Candidate,
        },
    );
    Ok(());
}

#[receive(
    contract = "overlay-projects",
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

    if params.token_addr == None {
        state.project.insert(
            params.project_id,
            ProjectState {
                owners: Some(params.owners),
                pub_key: None,
                token_addr: None,
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
    } else {
        state.project.insert(
            params.project_id,
            ProjectState {
                owners: Some(params.owners),
                pub_key: None,
                token_addr: Some(params.token_addr),
                seed_nft_addr: None,
                sale_addr: None,
                status: ProjectStatus::Whitelist,
            },
        );
    }
    Ok(());
}

#[receive(
    contract = "overlay-projects",
    name = "add_pub_key",
    parameter = "AddPubKeyParam",
    mutable
)]
fn contract_add_pub_key<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveContext<()> {
    let params: AddPubKeyParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(params.project_id);
    ensure!(ctx.sender() == state.admin, Error::InvalidCaller);

    state.project.insert(
        parrams.project_id,
        ProjectState {
            owners: Some(old_values.owners),
            pub_key: Some(params.pub_key),
            token_addr: Some(old_values.token_addr),
            seed_nft_addr: Some(old_values.seed_nft_addr),
            sale_addr: Some(old_values.sale_addr),
            status: Some(old_values.status),
        }
    );
    Ok(());
}

#[receive(
    contract = "overlay-projects",
    name = "add_seed_sale",
    parameter = "AddSeedSaleParam"
    mutable
)]
fn contract_add_seed_sale<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<()> {
    let params: AddSeedSaleParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(params.project_id);
    ensure!(ctx.sender() == state.admin, Error::InvalidCaller);

    state.project.insert(
        params.project_id,
        ProjectState {
            owners: Some(old_values.owners),
            pub_key: Some(old_values.pub_key),
            token_addr: Some(old_values.token_addr),
            seed_nft_addr: Some(params.seed_nft_addr),
            sale_addr: Some(old_values.sale_addr),
            status: old_values.status,
        },
    );
}

#[receive(
    contract = "overlay-projects",
    name = "add_token_addr",
    parameter = "AddTokenAddrParam"
)]
fn contract_add_token_addr<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveContext<()> {
    let params: AddTokenAddrParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(params.project_id);
    ensure!(ctx.sender() == state.admin, Error::InvalidCaller);
    ensure!(old_values.seed_nft_addr != None, Error::ShouldBeSeedNFT);

    state.project.insert(
        params.project_id,
        ProjectState {
            owners: Some(old_values.owners),
            pub_key: Some(old_values.pub_key),
            token_addr: Some(params.token_addr),
            seed_nft_addr: Some(old_values.seed_nft_addr),
            sale_addr: Some(old_values.sale_addr),
            status: old_values.status,
        },
    );
}

#[receive(
    contract = "overlay-projects",
    name = "add_sale",
    parameter = "AddSaleParam"
    mutable
)]
fn contract_add_sale<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<()> {
    let params: AddSaleParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(params.project_id);
    ensure!(ctx.sender() == state.admin, Error::InvalidCaller);

    state.project.insert(
        params.project_id,
        ProjectState {
            owners: Some(old_values.owners),
            pub_key: Some(old_values.pub_key),
            token_addr: Some(old_values.token_addr),
            seed_nft_addr: Some(old_values.seed_nft_addr),
            sale_addr: Some(params.sale_addr),
            status: old_values.status,
        },
    );
}

#[receive(
    contract = "overlay-projects",
    name = "start_sale",
    parameter = "StartSaleParam"
    mutable
)]
fn contract_start_sale<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveContext<()> {
    let params = StartSaleParam = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let old_values = state.project.get(params.project_id);
    ensure!(ctx.sender() === state.admin, Error::InvalidCaller);

    state.project.insert(
        params.project_id,
        ProjectState {
            owners: Some(old_values.owners),
            pub_key: Some(old_values.pub_key),
            token_addr: Some(old_values.token_addr),
            seed_nft_addr: Some(old_values.seed_nft_addr),
            sale_addr: Some(old_values.sale_addr),
            status: ProjectStatus::Onsale,
        },
    );
}

// view_project_state

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
