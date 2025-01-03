// SPDX-License-Identifier: GPL-3.0-only

use crate::state::{BackendData, State};
use smithay::{
    backend::{allocator::dmabuf::Dmabuf, renderer::ImportDma},
    delegate_dmabuf,
    wayland::dmabuf::{DmabufGlobal, DmabufHandler, DmabufState, ImportError},
};

impl DmabufHandler for State {
    fn dmabuf_state(&mut self) -> &mut DmabufState {
        &mut self.common.dmabuf_state
    }

    fn dmabuf_imported(
        &mut self,
        global: &DmabufGlobal,
        dmabuf: Dmabuf,
    ) -> Result<(), ImportError> {
        match &mut self.backend {
            BackendData::Kms(ref mut state) => state
                .dmabuf_imported(global, dmabuf)
                .map_err(|_| ImportError::Failed),
            BackendData::Winit(ref mut state) => state
                .backend
                .renderer()
                .import_dmabuf(&dmabuf, None)
                .map(|_| ())
                .map_err(|_| ImportError::Failed),
            BackendData::X11(ref mut state) => state
                .renderer
                .import_dmabuf(&dmabuf, None)
                .map(|_| ())
                .map_err(|_| ImportError::Failed),
            _ => unreachable!("No backend set when importing dmabuf"),
        }
    }
}

delegate_dmabuf!(State);
