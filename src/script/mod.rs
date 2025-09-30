#[cfg(feature = "js")]
mod js;

use crate::markup::{Element, Page};
use crate::parts::ScriptType;
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
    pub fn run(&mut self, s: &str, script_type: &ScriptType, page: &mut Page) {
        match script_type {
            #[cfg(feature = "js")]
            ScriptType::JS => self.run_js(s, page),
            _ => {
                println!("unsupported {:?}", script_type.to_string())
            }
        };
    }

    #[cfg(feature = "js")]
    fn run_js(&mut self, s: &str, page: &mut Page) {
        let (tx, rx) = channel();
        self.sender.replace(tx);

        std::thread::spawn(move || {
            js::run(rx);
        });

        if let Some(tx) = &self.sender {
            let e = page.body_element();
            let _ = tx.send(CommandKind::AddGlobalObject("body".to_string(), e));
            let _ = tx.send(CommandKind::ExecuteScript(s.to_string()));
        }
    }
}
