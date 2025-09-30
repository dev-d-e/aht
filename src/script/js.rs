use super::*;
use crate::markup::{AttrName, Element};
use std::cell::OnceCell;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};
use v8::V8::{initialize, initialize_platform};
use v8::{
    new_default_platform, Array, Context, ContextScope, External, Function, FunctionCallback,
    FunctionCallbackArguments, Global, HandleScope, Isolate, Local, MapFnTo, Object,
    ObjectTemplate, OwnedIsolate, ReturnValue, Script, String,
};

pub(super) struct JSRuntime {
    isolate: OwnedIsolate,
    context: OnceCell<Global<Context>>,
}

impl JSRuntime {
    pub(super) fn new() -> Self {
        initialize_platform(new_default_platform(0, false).make_shared());
        initialize();
        let isolate = Isolate::new(Default::default());
        let mut o = Self {
            isolate,
            context: OnceCell::new(),
        };
        o.context.get_or_init(|| {
            let scope = &mut HandleScope::new(&mut o.isolate);
            let context = Context::new(scope, Default::default());
            let global = Global::new(scope, context);
            global
        });
        o
    }

    fn with_scope(&mut self, f: impl FnOnce(&mut HandleScope)) {
        if let Some(context) = self.context.get() {
            let scope = &mut HandleScope::new(&mut self.isolate);
            let context = Local::new(scope, context);
            let scope = &mut ContextScope::new(scope, context);
            f(scope);
        }
    }

    pub(super) fn add_global_object(&mut self, name: &str, e: Arc<RwLock<Element>>) {
        self.with_scope(|scope| {
            let global = scope.get_current_context().global(scope);
            if let Some(name) = String::new(scope, name) {
                if let Some(value) = element_to_object(scope, &e) {
                    global.set(scope, name.into(), value.into());
                }
            }
        });
    }

    pub(super) fn execute_script(&mut self, s: &str) {
        self.with_scope(|scope| {
            if let Some(code) = String::new(scope, s) {
                if let Some(script) = Script::compile(scope, code, None) {
                    let _ = script.run(scope);
                }
            }
        });
    }
}

fn element_to_object<'a>(
    scope: &mut HandleScope<'a>,
    e: &Arc<RwLock<Element>>,
) -> Option<Local<'a, Object>> {
    let t = ObjectTemplate::new(scope);
    t.set_internal_field_count(1);
    t.new_instance(scope).map(|obj| {
        let a = External::new(scope, Arc::as_ptr(e) as *mut core::ffi::c_void);
        obj.set_internal_field(0, a.into());

        build_function(scope, obj, "mark_type", func_mark_type);
        build_function(scope, obj, "text", func_text);
        build_function(scope, obj, "attribute", func_attribute);
        if let Ok(e) = e.read() {
            if let Some(subset_key) = String::new(scope, "subset") {
                let mut v = Vec::new();
                for o in e.subset.iter() {
                    if let Some(o) = element_to_object(scope, o) {
                        v.push(o.into());
                    }
                }
                let subset = Array::new_with_elements(scope, &v);
                obj.set(scope, subset_key.into(), subset.into());
            }
        }
        obj
    })
}

#[inline]
fn build_function<'a>(
    scope: &mut HandleScope,
    obj: Local<'a, Object>,
    key: &str,
    func: impl MapFnTo<FunctionCallback>,
) {
    if let Some(key) = String::new(scope, key) {
        if let Some(func) = Function::new(scope, func) {
            obj.set(scope, key.into(), func.into());
        }
    }
}

fn func_mark_type(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    element_function(scope, args, |scope, _, e| {
        if let Some(s) = String::new(scope, e.mark_type.as_str()) {
            rv.set(s.into());
        }
    })
}

fn func_text(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    element_function(scope, args, |scope, _, e| {
        if let Some(s) = String::new(scope, e.text.as_str()) {
            rv.set(s.into());
        }
    })
}

fn func_attribute(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    element_function(scope, args, |scope, args, e| {
        let k = args.get(0).to_rust_string_lossy(scope);
        if let Some(k) = AttrName::from(&k) {
            if let Some(v) = e.attribute.get(&k) {
                if let Some(s) = String::new(scope, &v.to_string()) {
                    rv.set(s.into());
                }
            }
        }
    })
}

#[inline]
fn element_function(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut f: impl FnMut(&mut HandleScope, FunctionCallbackArguments, &Element),
) {
    let t = args.this();
    if let Some(a) = t.get_internal_field(scope, 0) {
        if let Ok(a) = <Local<'_, External>>::try_from(a) {
            let e = unsafe { &*(a.value() as *const RwLock<Element>) };
            if let Ok(e) = e.read() {
                f(scope, args, &e);
            }
        }
    }
}

pub(super) fn run(rx: Receiver<CommandKind>) {
    let mut jsrt = JSRuntime::new();
    loop {
        if let Ok(o) = rx.recv() {
            match o {
                CommandKind::AddGlobalObject(s, e) => {
                    jsrt.add_global_object(&s, e);
                }
                CommandKind::ExecuteScript(s) => {
                    jsrt.execute_script(&s);
                }
                CommandKind::Shutdown => {
                    break;
                }
            }
        }
        jsrt.with_scope(|scope| {
            scope.perform_microtask_checkpoint();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js() {
        let mut jsrt = JSRuntime::new();
        jsrt.execute_script("function sum(a, b) {return a + b}");
        jsrt.execute_script("sum(1,2)");
    }
}
