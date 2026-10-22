//! CLI provenance and exact warm-table equality distinguish stdio from leaks.

use super::State;
use std::collections::BTreeMap;
use wasmtime::StoreContextMut;
use wasmtime::component::{Linker, Resource, ResourceTable, ResourceType};
use wasmtime_wasi::cli::WasiCliView;
use wasmtime_wasi::p2::bindings::cli::{stderr, stdin, stdout};
use wasmtime_wasi::p2::bindings::io::streams;
use wasmtime_wasi::p2::{DynInputStream, DynOutputStream};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stream {
    Stdin,
    Stdout,
    Stderr,
}

pub type Snapshot = BTreeMap<u32, Stream>;

/// Delegate the official bindings and retain the provenance of their handles.
pub fn add(linker: &mut Linker<State>) -> wasmtime::Result<()> {
    linker.allow_shadowing(true);
    linker.instance("wasi:cli/stdin@0.2.12")?.func_wrap(
        "get-stdin",
        |mut store: StoreContextMut<'_, State>, (): ()| {
            let state = store.data_mut();
            let resource = stdin::Host::get_stdin(&mut state.cli())?;
            state.cli_streams.insert(resource.rep(), Stream::Stdin);
            Ok((resource,))
        },
    )?;
    linker.instance("wasi:cli/stdout@0.2.12")?.func_wrap(
        "get-stdout",
        |mut store: StoreContextMut<'_, State>, (): ()| {
            let state = store.data_mut();
            let resource = stdout::Host::get_stdout(&mut state.cli())?;
            state.cli_streams.insert(resource.rep(), Stream::Stdout);
            Ok((resource,))
        },
    )?;
    linker.instance("wasi:cli/stderr@0.2.12")?.func_wrap(
        "get-stderr",
        |mut store: StoreContextMut<'_, State>, (): ()| {
            let state = store.data_mut();
            let resource = stderr::Host::get_stderr(&mut state.cli())?;
            state.cli_streams.insert(resource.rep(), Stream::Stderr);
            Ok((resource,))
        },
    )?;
    let mut streams = linker.instance("wasi:io/streams@0.2.12")?;
    streams.resource_async(
        "input-stream",
        ResourceType::host::<DynInputStream>(),
        |mut store, id| Box::new(async move { drop_input(store.data_mut(), id).await }),
    )?;
    streams.resource_async(
        "output-stream",
        ResourceType::host::<DynOutputStream>(),
        |mut store, id| Box::new(async move { drop_output(store.data_mut(), id).await }),
    )?;
    Ok(())
}

/// Recycled table slots cannot inherit ownership from a dropped CLI stream.
async fn drop_input(state: &mut State, id: u32) -> wasmtime::Result<()> {
    state.cli_streams.remove(&id);
    streams::HostInputStream::drop(&mut state.table, Resource::new_own(id)).await
}

/// Preserve the official awaited cancellation when removing CLI provenance.
async fn drop_output(state: &mut State, id: u32) -> wasmtime::Result<()> {
    state.cli_streams.remove(&id);
    streams::HostOutputStream::drop(&mut state.table, Resource::new_own(id)).await
}

/// Every retained entry must be an actual CLI-created handle with its right type.
pub fn snapshot(
    table: &mut ResourceTable,
    owners: &Snapshot,
    expected: Option<&Snapshot>,
) -> Snapshot {
    let mut actual = Snapshot::new();

    for (id, entry) in table.iter_mut() {
        assert!(
            !entry.is::<wasmtime_wasi_http::RequestOptions>(),
            "owned request-options leaked from the host table"
        );
        let owner = owners.get(&id).expect("retained resource has no CLI owner");

        let correct_type = match owner {
            Stream::Stdin => entry.is::<DynInputStream>(),
            Stream::Stdout | Stream::Stderr => entry.is::<DynOutputStream>(),
        };

        assert!(correct_type, "retained CLI handle changed resource type");
        actual.insert(id, *owner);
    }

    assert_eq!(&actual, owners, "CLI handle missing from warm table");

    if let Some(expected) = expected {
        assert_eq!(&actual, expected, "table did not return to warm baseline");
    }

    actual
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use wasmtime_wasi::p2::pipe::SinkOutputStream;

    fn warm_table() -> (ResourceTable, Snapshot) {
        let mut table = ResourceTable::new();
        let output: DynOutputStream = Box::new(SinkOutputStream);
        let resource = table.push(output).expect("CLI stream");
        let owners = BTreeMap::from([(resource.rep(), Stream::Stdout)]);
        (table, owners)
    }

    #[test]
    fn accepts_exact_cli_table() {
        let (mut table, owners) = warm_table();
        assert_eq!(snapshot(&mut table, &owners, None), owners);
    }

    #[test]
    fn rejects_extra_stream_of_cli_type() {
        let (mut table, owners) = warm_table();
        let leaked: DynOutputStream = Box::new(SinkOutputStream);
        table.push(leaked).expect("leaked stream");

        assert!(catch_unwind(AssertUnwindSafe(|| snapshot(&mut table, &owners, None))).is_err());
    }

    #[test]
    fn rejects_extra_cli_owned_stream_against_warm_baseline() {
        let (mut table, mut owners) = warm_table();
        let baseline = snapshot(&mut table, &owners, None);
        let leaked: DynOutputStream = Box::new(SinkOutputStream);
        let resource = table.push(leaked).expect("extra CLI stream");
        owners.insert(resource.rep(), Stream::Stdout);

        assert!(
            catch_unwind(AssertUnwindSafe(|| snapshot(
                &mut table,
                &owners,
                Some(&baseline)
            )))
            .is_err()
        );
    }

    #[tokio::test]
    async fn rejects_sdk_stream_reusing_dropped_cli_slot() {
        let (table, owners) = warm_table();
        let id = *owners.keys().next().expect("CLI stream id");
        let mut state = State::new("baseline", 0, Default::default()).expect("fixture state");
        state.table = table;
        state.cli_streams = owners;
        drop_output(&mut state, id).await.expect("official drop");
        let leaked: DynOutputStream = Box::new(SinkOutputStream);
        let replacement = state.table.push(leaked).expect("recycled SDK stream");
        assert_eq!(replacement.rep(), id);

        assert!(
            catch_unwind(AssertUnwindSafe(|| snapshot(
                &mut state.table,
                &state.cli_streams,
                None
            )))
            .is_err()
        );
    }

    #[test]
    fn rejects_request_options() {
        let (mut table, owners) = warm_table();
        table
            .push(wasmtime_wasi_http::RequestOptions::default())
            .expect("leaked request-options");

        assert!(catch_unwind(AssertUnwindSafe(|| snapshot(&mut table, &owners, None))).is_err());
    }

    #[test]
    fn rejects_non_cli_resource() {
        let (mut table, owners) = warm_table();
        table.push(42_u32).expect("leaked non-CLI resource");

        assert!(catch_unwind(AssertUnwindSafe(|| snapshot(&mut table, &owners, None))).is_err());
    }
}
