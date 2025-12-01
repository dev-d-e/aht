#[cfg(feature = "js")]
mod js;

use crate::markup::*;
use crate::page::*;
use crate::utils::*;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, RwLock};

enum CommandKind {
    AddGlobalObject(String, Arc<RwLock<Element>>),
    ExecuteScript(String),
    Shutdown,
}

#[derive(Debug)]
pub(crate) struct ScriptRuntime {
    sender: Option<Sender<CommandKind>>,
}

impl ScriptRuntime {
    pub(crate) fn new() -> Self {
        Self { sender: None }
    }

    #[allow(unreachable_patterns)]
    pub(crate) fn run(&mut self, s: &str, script_type: &ScriptType, context: &mut PageContext) {
        match script_type {
            #[cfg(feature = "js")]
            ScriptType::JS => self.run_js(s, context),
            _ => {
                error!("unsupported {:?}", script_type.to_string())
            }
        };
    }

    #[cfg(feature = "js")]
    fn run_js(&mut self, s: &str, context: &mut PageContext) {
        let (tx, rx) = channel();
        self.sender.replace(tx);

        std::thread::spawn(move || {
            js::run(rx);
        });

        if let Some(tx) = &self.sender {
            let e = context.body_element();
            let _ = tx.send(CommandKind::AddGlobalObject("body".to_string(), e.clone()));
            let _ = tx.send(CommandKind::ExecuteScript(s.to_string()));
        }
    }
}
