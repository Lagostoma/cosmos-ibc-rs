use alloc::collections::BTreeMap;
use alloc::sync::Arc;

use ibc::core::ics24_host::identifier::PortId;
use ibc::core::router::{Module, ModuleId};
use ibc::prelude::*;
use ibc_app_transfer::types::MODULE_ID_STR;

use crate::testapp::ibc::applications::transfer::types::DummyTransferModule;

#[derive(Default)]
pub struct MockRouter {
    pub router: BTreeMap<ModuleId, Arc<dyn Module>>,

    /// Maps ports to the the module that owns it
    pub port_to_module: BTreeMap<PortId, ModuleId>,
}

impl MockRouter {
    pub fn new_with_transfer() -> Self {
        let mut router = Self::default();

        let module_id = ModuleId::new(MODULE_ID_STR.to_string());

        router.scope_port_to_module(PortId::transfer(), module_id.clone());

        let transfer_mod = DummyTransferModule::new();

        router
            .add_route(module_id, transfer_mod)
            .expect("Never fails");

        router
    }

    pub fn add_route(
        &mut self,
        module_id: ModuleId,
        module: impl Module + 'static,
    ) -> Result<(), String> {
        match self.router.insert(module_id, Arc::new(module)) {
            None => Ok(()),
            Some(_) => Err("Duplicate module_id".to_owned()),
        }
    }

    pub fn scope_port_to_module(&mut self, port_id: PortId, module_id: ModuleId) {
        self.port_to_module.insert(port_id, module_id);
    }
}