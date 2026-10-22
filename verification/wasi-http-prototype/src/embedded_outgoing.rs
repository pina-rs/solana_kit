//! Wasmtime50 consumes request-options in WIT but omits its table deletion.

use super::State;
use wasmtime::StoreContextMut;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi_http::WasiHttpView;
use wasmtime_wasi_http::p2::bindings::http::{outgoing_handler, types};
use wasmtime_wasi_http::p2::types::HostOutgoingRequest;

type Options = Option<Resource<types::RequestOptions>>;

pub fn add(linker: &mut Linker<State>) -> wasmtime::Result<()> {
    linker.allow_shadowing(true);
    linker
        .instance("wasi:http/outgoing-handler@0.2.12")?
        .func_wrap(
            "handle",
            |mut store: StoreContextMut<'_, State>,
             (request, options): (Resource<HostOutgoingRequest>, Options)| {
                let consumed = options.as_ref().map(Resource::rep);
                let mut view = store.data_mut().http();
                let result = outgoing_handler::Host::handle(&mut view, request, options);

                if let Some(rep) = consumed {
                    view.table
                        .delete(Resource::<types::RequestOptions>::new_own(rep))?;
                }

                let result = match result {
                    Ok(response) => Ok(response),
                    Err(error) => Err(error.downcast()?),
                };

                Ok((result,))
            },
        )?;
    Ok(())
}
