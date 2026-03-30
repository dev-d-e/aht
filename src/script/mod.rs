#[cfg(feature = "js")]
mod js;

use crate::markup::*;
use crate::page::*;
use crate::utils::*;
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, RwLock};

enum CommandKind {
    ExecuteScript(String),
    Rebuild,
    Shutdown,
}

#[derive(Debug)]
pub(crate) struct ScriptRuntime {
    sender: Sender<CommandKind>,
}

impl Drop for ScriptRuntime {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl ScriptRuntime {
    #[allow(unreachable_patterns)]
    pub(crate) fn new(t: ScriptType, cx: Arc<RwLock<PageContext>>) -> Self {
        let (sender, receiver) = channel();

        std::thread::spawn(move || {
            match t {
                #[cfg(feature = "js")]
                ScriptType::JS => js::run(cx, receiver),
                _ => {
                    error!("unsupported {:?}", t)
                }
            };
        });
        Self { sender }
    }

    pub(crate) fn exec(&mut self, s: String) {
        let _ = self.sender.send(CommandKind::ExecuteScript(s));
    }

    pub(crate) fn rebuild(&mut self) {
        let _ = self.sender.send(CommandKind::Rebuild);
    }

    pub(crate) fn shutdown(&mut self) {
        let _ = self.sender.send(CommandKind::Shutdown);
    }
}
