use v8;

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

fn collect_callback(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut m_return: v8::ReturnValue) {
    let array = v8::Array::new(scope, 0);

    for i in 0..args.length() {
        let arg = args.get(i);

        array.set_index(scope, i as u32, arg);
    }

    m_return.set(array.into());
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
    global_fn!("collect", collect_callback);

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

        let items = collect(1, 2, 3)

        println(items)
    ").unwrap();

    println!("code: {}", code.to_rust_string_lossy(scope));

    let script = v8::Script::compile(scope, code, None).unwrap();

    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();

    println!("result: {}", result.to_rust_string_lossy(scope));
}
