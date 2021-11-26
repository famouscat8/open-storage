use crate::guards::caller_is_service_principal;
use crate::model::bucket_sync_state::EventToSync;
use crate::{RuntimeState, RUNTIME_STATE};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::remove_accessor::*;

#[update(guard = "caller_is_service_principal")]
#[trace]
fn remove_accessor(args: Args) -> Response {
    RUNTIME_STATE.with(|state| remove_accessor_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn remove_accessor_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    runtime_state
        .data
        .buckets
        .sync_event(EventToSync::AccessorRemoved(args.accessor_id));
    Response::Success
}