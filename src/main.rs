use v8;

macro_rules! v8_bool {
    ($scope: expr, $b: expr) => {
        v8::Boolean::new($scope, $b).into()
    };
}

fn println_callback(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut m_return: v8::ReturnValue) {
    let message = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);

    m_return.set(v8_bool!(scope, true));

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

    global.set(
        v8::String::new(scope, "println").unwrap().into(),
        v8::FunctionTemplate::new(scope, println_callback).into(),
    );

    let context = v8::Context::new_from_template(scope, global);
    let scope = &mut v8::ContextScope::new(scope, context);

    let code = v8::String::new(scope, "
        function fib(n) {
            if (n < 2) {
                return n
            }

            return fib(n - 2) + fib(n - 1)
        }

        println(fib(30))
    ").unwrap();

    println!("code: {}", code.to_rust_string_lossy(scope));

    let script = v8::Script::compile(scope, code, None).unwrap();

    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();

    println!("result: {}", result.to_rust_string_lossy(scope));
}
