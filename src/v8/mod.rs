use std::{
  cell::{RefCell, UnsafeCell},
  ffi::c_void,
  marker::PhantomData,
  pin::Pin,
  rc::Rc,
};

use matchit::Router;

#[allow(clippy::needless_pass_by_value)] // this function should follow the callback type
fn log_callback(
  scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  mut _retval: v8::ReturnValue,
) {
  let message = args
    .get(0)
    .to_string(scope)
    .unwrap()
    .to_rust_string_lossy(scope);

  println!("Logged: {}", message);
}

const ROUTER_GLOBAL: &'static str = "___WALKER_ROUTER___";

pub struct State<'s, 'i> {
  context: v8::Local<'s, v8::Context>,
  context_scope: v8::ContextScope<'i, v8::HandleScope<'s>>,
  router: Rc<RefCell<Router<v8::Global<v8::Function>>>>,
  source: v8::Local<'s, v8::String>,
  request_template: v8::Global<v8::ObjectTemplate>,
}

impl<'s, 'i> State<'s, 'i>
where
  's: 'i,
{
  pub fn new(
    isolate_scope: &'i mut v8::HandleScope<'s, ()>,
    source: v8::Local<'s, v8::String>,
  ) -> Self {
    let global = v8::ObjectTemplate::new(isolate_scope);
    global.set(
      v8::String::new(isolate_scope, "log").unwrap().into(),
      v8::FunctionTemplate::new(isolate_scope, log_callback).into(),
    );

    let context = v8::Context::new_from_template(isolate_scope, global);
    let mut context_scope = v8::ContextScope::new(isolate_scope, context);

    let request_template = v8::ObjectTemplate::new(&mut context_scope);
    request_template.set_internal_field_count(1);

    // make it global
    let request_template = v8::Global::new(&mut context_scope, request_template);
    let shared_router = Rc::new(RefCell::new(Router::new()));

    let mut self_ = Self {
      context,
      context_scope,
      router: Rc::clone(&shared_router),
      source,
      request_template,
    };

    {
      let boxed_router = WalkerBuilder::new(shared_router);
      let new_gobo = WalkerBuilder::create_js_obj(boxed_router, &mut self_.context_scope);
      let key = v8::String::new(&mut self_.context_scope, ROUTER_GLOBAL).unwrap();

      self_.context.global(&mut self_.context_scope).set(
        &mut self_.context_scope,
        key.into(),
        new_gobo.into(),
      );
    }

    self_
  }

  pub fn register_listeners(&mut self) {
    let global = self.context.global(&mut self.context_scope);
    let func = v8::Function::new(
      &mut self.context_scope,
      |scope: &mut v8::HandleScope,
       args: v8::FunctionCallbackArguments<'s>,
       mut rv: v8::ReturnValue| {
        let path = args.get(0).to_rust_string_lossy(scope);
        let func = args.get(1);

        let function = v8::Local::<v8::Function>::try_from(func).expect("function expected");

        println!(
          "Function has been called, router is {}, function is {}",
          path,
          func.to_rust_string_lossy(scope)
        );

        let key = v8::String::new(scope, ROUTER_GLOBAL).unwrap();
        let globo = scope.get_current_context().global(scope);

        let router_global = globo
          .get(scope, key.into())
          .unwrap()
          .to_object(scope)
          .unwrap();
        let inner_router = unsafe {
          Self::get_inner_router(scope, router_global)
            .as_mut()
            .unwrap()
        };

        let global_func = v8::Global::new(scope, function);

        inner_router.add(path, global_func);

        rv.set_undefined()
      },
    )
    .unwrap();

    let name = v8::String::new(&mut self.context_scope, "GET").unwrap();
    global
      .set(&mut self.context_scope, name.into(), func.into())
      .unwrap();
  }

  pub fn run_script(&mut self) {
    let scope = &mut v8::HandleScope::new(&mut self.context_scope);
    let try_catch = &mut v8::TryCatch::new(scope);

    let script =
      v8::Script::compile(try_catch, self.source, None).expect("failed to compile script");

    if script.run(try_catch).is_none() {
      let exception = try_catch.exception().unwrap();
      let exception_string = exception
        .to_string(try_catch)
        .unwrap()
        .to_rust_string_lossy(try_catch);

      panic!("{}", exception_string);
    } else {
      println!("Script ran successfully...");
    }
  }

  pub fn load_found_router(&mut self) {
    let key = v8::String::new(&mut self.context_scope, ROUTER_GLOBAL).unwrap();
    let globo = self
      .context_scope
      .get_current_context()
      .global(&mut self.context_scope);
    let router_global = globo
      .get(&mut self.context_scope, key.into())
      .unwrap()
      .to_object(&mut self.context_scope)
      .unwrap();

    let native_router = unsafe {
      Self::get_inner_router(&mut self.context_scope, router_global)
        .as_mut()
        .unwrap()
    };

    let reffing = native_router.router.borrow();
    
    let routes = vec!["/route", "/route2", "/route3"];

    let scope = &mut v8::HandleScope::new(&mut self.context_scope);
    let try_catch = &mut v8::TryCatch::new(scope);
    let global = self.context.global(try_catch).into();

    for route in routes {
        let found = reffing.at(route).unwrap();

        if found.value.open(try_catch)
            .call(try_catch, global, &[])
            .is_none()
        {
            let exception = try_catch.exception().unwrap();
            let exception_string = exception
                .to_string(try_catch)
                .unwrap()
                .to_rust_string_lossy(try_catch);

            panic!("{}", exception_string);
        }
    }

  }

  fn get_inner_router<'a>(
    scope: &mut v8::HandleScope,
    request: v8::Local<'a, v8::Object>,
  ) -> *mut Box<WalkerBuilder<'s, 'i>> {
    let external = request.get_internal_field(scope, 0).unwrap();
    let external = unsafe { v8::Local::<v8::External>::cast(external) };
    external.value() as *mut Box<WalkerBuilder>
  }
}

trait RouterBuilder<'s, 'i> {
  fn add(&mut self, path: String, new_route: v8::Global<v8::Function>);
}

struct WalkerBuilder<'s, 'i> {
  pub router: Rc<RefCell<Router<v8::Global<v8::Function>>>>,
  _phantom: PhantomData<&'i ()>,
  _phantom2: PhantomData<&'s ()>,
}

impl<'s, 'i> RouterBuilder<'s, 'i> for WalkerBuilder<'s, 'i> {
  fn add(&mut self, path: String, new_route: v8::Global<v8::Function>) {
    self.router.borrow_mut().insert(path, new_route).unwrap();
  }
}

impl<'s, 'i> WalkerBuilder<'s, 'i>
where
  's: 'i,
{
  pub fn new(router: Rc<RefCell<Router<v8::Global<v8::Function>>>>) -> Pin<Box<Self>> {
    Box::pin(Self {
      router,
      _phantom2: PhantomData,
      _phantom: PhantomData,
    })
  }

  pub fn create_js_obj(
    boxed_me: Pin<Box<WalkerBuilder<'s, 'i>>>,
    context_scope: &mut v8::ContextScope<'i, v8::HandleScope<'s>>,
  ) -> v8::Local<'i, v8::Object> {
    let re_boxed = Box::new(boxed_me);

    let request_template = v8::ObjectTemplate::new(context_scope);
    request_template.set_internal_field_count(1);
    let result = request_template.new_instance(context_scope).unwrap();

    let external = v8::External::new(
      context_scope,
      Box::leak(re_boxed) as *mut Pin<Box<WalkerBuilder<'s, 'i>>> as *mut c_void,
    );

    result.set_internal_field(0, external.into());

    result
  }
}
