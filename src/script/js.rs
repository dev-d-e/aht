use super::*;
use std::cell::OnceCell;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};
use v8::V8::{initialize, initialize_platform};
use v8::{
    Array, Context, ContextScope, External, Function, FunctionCallbackArguments, Global,
    HandleScope, Isolate, Local, Object, ObjectTemplate, OwnedIsolate, PinScope,
    PropertyDescriptor, ReturnValue, Script, String, new_default_platform,
};

const ROOT: &str = "page";

#[derive(Default)]
#[repr(transparent)]
struct KeyAndExternal(Vec<ElementKey>);

impl KeyAndExternal {
    fn get<'a>(
        &mut self,
        scope: &mut PinScope<'a, '_>,
        k: ElementKey,
    ) -> Option<Local<'a, External>> {
        self.0.push(k);
        let k = self.0.last()? as *const ElementKey;
        let k = k as *mut core::ffi::c_void;
        Some(External::new(scope, k))
    }
}

struct JSRuntime {
    isolate: OwnedIsolate,
    context: OnceCell<Global<Context>>,
    page: Arc<RwLock<PageContext>>,
    v: KeyAndExternal,
}

impl JSRuntime {
    fn new(page: Arc<RwLock<PageContext>>) -> Self {
        initialize_platform(new_default_platform(0, false).make_shared());
        initialize();
        let isolate = Isolate::new(Default::default());
        let mut o = Self {
            isolate,
            context: OnceCell::new(),
            page,
            v: Default::default(),
        };
        o.context.get_or_init(|| {
            let scope = std::pin::pin!(HandleScope::new(&mut o.isolate));
            let scope = &mut scope.init();
            let context = Context::new(scope, Default::default());
            let global = Global::new(scope, context);
            global
        });
        o.build_root();
        o
    }

    fn with_scope(&mut self, f: impl FnOnce(&mut PinScope) -> Option<bool>) {
        if let Some(context) = self.context.get() {
            let scope = std::pin::pin!(HandleScope::new(&mut self.isolate));
            let scope = &mut scope.init();
            let context = Local::new(scope, context);
            let scope = &mut ContextScope::new(scope, context);
            f(scope);
        }
    }

    fn build_root(&mut self) {
        let p = Arc::as_ptr(&self.page) as *mut core::ffi::c_void;
        let ks = result_return!(self.page.read().map(|p| vec!(
            p.head_key(),
            p.body_key(),
            p.style_key(),
        )));
        let page = self.page.clone();
        let v = &mut self.v as *mut KeyAndExternal;
        self.with_scope(|scope| {
            let t = ObjectTemplate::new(scope);
            t.set_internal_field_count(1);
            let o = t.new_instance(scope)?;
            o.set_internal_field(0, External::new(scope, p).into());

            for k in ks {
                let value = element_to_object(scope, k, &page, v)?;
                o.set(scope, to_key(scope, k, &page)?.into(), value.into());
            }

            let global = scope.get_current_context().global(scope);
            global.set(scope, String::new(scope, ROOT)?.into(), o.into())
        });
    }

    fn execute_script(&mut self, s: &str) {
        self.with_scope(|scope| {
            let code = String::new(scope, s)?;
            let script = Script::compile(scope, code, None)?;
            let _ = script.run(scope)?;
            None
        });
    }
}

#[inline]
fn to_key<'a>(
    scope: &PinScope<'a, '_>,
    k: ElementKey,
    page: &Arc<RwLock<PageContext>>,
) -> Option<Local<'a, String>> {
    let o = page.read().ok()?;
    String::new(scope, o.get(k)?.mark_type().as_str())
}

fn element_to_object<'a>(
    scope: &mut PinScope<'a, '_>,
    k: ElementKey,
    page: &Arc<RwLock<PageContext>>,
    v: *mut KeyAndExternal,
) -> Option<Local<'a, Object>> {
    let t = ObjectTemplate::new(scope);
    t.set_internal_field_count(1);
    let obj = t.new_instance(scope)?;

    let a = unsafe { v.as_mut()?.get(scope, k)? };
    obj.set_internal_field(0, a.into());

    let d = PropertyDescriptor::new_from_get_set(
        Function::new(scope, func_mark_type)?.into(),
        Function::new(scope, func_empty)?.into(),
    );
    obj.define_property(scope, String::new(scope, "mark_type")?.into(), &d);

    let d = PropertyDescriptor::new_from_get_set(
        Function::new(scope, func_text)?.into(),
        Function::new(scope, func_empty)?.into(),
    );
    obj.define_property(scope, String::new(scope, "text")?.into(), &d);

    let func = Function::new(scope, func_attribute)?.into();
    obj.set(scope, String::new(scope, "attribute")?.into(), func);

    let subset = page.read().ok()?.get_subset(k);
    let mut r = Vec::new();
    for k in subset {
        r.push(element_to_object(scope, k, page, v)?.into());
    }
    let subset = Array::new_with_elements(scope, &r);
    obj.set(scope, String::new(scope, "subset")?.into(), subset.into());
    Some(obj)
}

fn func_mark_type(scope: &mut PinScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    if let Some(s) = get_element(scope, &args)
        .and_then(|e| unsafe { e.as_ref() })
        .and_then(|e| String::new(scope, e.mark_type().as_str()))
    {
        rv.set(s.into());
    }
}

fn func_text(scope: &mut PinScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    if let Some(s) = get_element(scope, &args)
        .and_then(|e| unsafe { e.as_ref() })
        .and_then(|e| String::new(scope, e.text().as_str()))
    {
        rv.set(s.into());
    }
}

fn func_attribute(scope: &mut PinScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let n = args.get(0).to_rust_string_lossy(scope);
    if let Ok(n) = AttrName::try_from(&n) {
        if let Some(s) = get_element(scope, &args)
            .and_then(|e| unsafe { e.as_ref() })
            .and_then(|e| e.attribute_get(&n))
            .and_then(|a| String::new(scope, &a.to_string()))
        {
            rv.set(s.into());
        }
    }
}

fn func_empty(_scope: &mut PinScope, _args: FunctionCallbackArguments, _rv: ReturnValue) {}

#[inline]
fn get_this_key(scope: &mut PinScope, args: &FunctionCallbackArguments) -> Option<ElementKey> {
    let a = args.this().get_internal_field(scope, 0)?;
    let a = <Local<'_, External>>::try_from(a).ok()?;
    return Some(unsafe { *(a.value() as *const ElementKey) });
}

#[inline]
fn get_page(scope: &mut PinScope) -> Option<*const RwLock<PageContext>> {
    let global = scope.get_current_context().global(scope);
    let p = global.get(scope, String::new(scope, ROOT)?.into())?;
    let p = p.to_object(scope)?;
    let a = p.get_internal_field(scope, 0)?;
    let a = <Local<'_, External>>::try_from(a).ok()?;
    return Some(a.value() as *const RwLock<PageContext>);
}

#[inline]
fn get_element(scope: &mut PinScope, args: &FunctionCallbackArguments) -> Option<*const Element> {
    let k = get_this_key(scope, &args)?;
    let p = get_page(scope)?;
    let p = unsafe { p.as_ref()? };
    let p = p.read().ok()?;
    p.get(k).map(|e| e as *const Element)
}

pub(super) fn run(page: Arc<RwLock<PageContext>>, receiver: Receiver<CommandKind>) {
    let mut jsrt = JSRuntime::new(page);
    loop {
        if let Ok(o) = receiver.recv() {
            match o {
                CommandKind::ExecuteScript(s) => {
                    jsrt.execute_script(&s);
                }
                CommandKind::Rebuild => {
                    jsrt.build_root();
                }
                CommandKind::Shutdown => {
                    break;
                }
            }
        } else {
            break;
        }
        jsrt.with_scope(|scope| {
            scope.perform_microtask_checkpoint();
            None
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js() {
        let s = "<aht ><head><body><style><script>";
        let (e, err) = Page::parse_s(&s);
        println!("{err}");
        if let Some(e) = e {
            let mut jsrt = JSRuntime::new((&*e).clone());
            jsrt.execute_script("function sum(a, b) {return a + b}");
            jsrt.execute_script("sum(1,2)");
        }
    }
}
