use v8;

mod fib;
mod func;

macro_rules! v8_bool {
    ($scope: expr, $b: expr) => {
        v8::Boolean::new($scope, $b)
    };
}

macro_rules! v8_number {
    ($scope: expr, $n: expr) => {
        v8::Number::new($scope, $n)
    };
}

macro_rules! v8_string {
    ($scope: expr, $s: expr) => {
        v8::String::new($scope, $s)
    };
}

fn println_callback(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut m_return: v8::ReturnValue) {
    let message = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);

    m_return.set(v8_bool!(scope, true).into());

    println!("{}", message);
}

fn main() {
    println!("Playing around with some v8 APIs for now...");

    let platform = v8::new_default_platform(0, false).make_shared();

    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);

    let global = v8::ObjectTemplate::new(scope);

    macro_rules! global_fn {
        ($name: expr, $callback: expr) => {
            global.set(
                v8::String::new(scope, $name).unwrap().into(),
                v8::FunctionTemplate::new(scope, $callback).into(),
            )
        };
    }

    global_fn!("println", println_callback);

    let context = v8::Context::new_from_template(scope, global);
    let scope = &mut v8::ContextScope::new(scope, context);

    let proc = std::env::args().nth(1).unwrap();

    match proc.as_str() {
        "fib" => fib::fib(scope),
        "func" => func::func(scope),
        _ => println!("Undefined procedure: {}", proc),
    }
}
