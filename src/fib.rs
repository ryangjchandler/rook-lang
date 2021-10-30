use v8::{String, HandleScope};

pub fn fib(scope: &mut HandleScope) {
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

    println!("output:");
    let result = script.run(scope).unwrap();
    
    let result = result.to_string(scope).unwrap();

    println!("result: {}", result.to_rust_string_lossy(scope));
}