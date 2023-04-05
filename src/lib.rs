#[cfg(feature = "derive")]
pub use deco_derive::deco;

#[macro_export]
macro_rules! make_decorator {
    ($pub:vis fn $fun:ident <$arg_ty:ident>($function:ident : $typ:ty, $args:ident: $arg_ty2:ident) $body:block) => {
        $pub fn $fun<F, $arg_ty, R>($function: F) -> impl Fn($arg_ty2) -> R
        where
            F: Fn($arg_ty2) -> R,
        {
            move |$args| $body
        }
    };

    ($pub:vis fn $fun:ident ($function:ident: $typ:ty, $args:ident: $arg_ty:ty) $body:block) => {
        $pub fn $fun<F, R>($function: F) -> impl Fn($arg_ty) -> R
        where
            F: Fn($arg_ty) -> R,
        {
            move |$args| $body
        }
    };

    ($pub:vis fn $fun:ident ($function:ident: $typ:ty) $body:block) => {
        $pub fn $fun<F, R>($function: F) -> impl Fn() -> R
        where
            F: Fn() -> R,
        {
            move || $body
        }
    };
}

#[macro_export]
macro_rules! decorator {
    (@ $deco:ident $mangled:ident $pub:vis fn $fun:ident ( $($name:ident : $typ:ty),* ) $( -> $ret:ty )? $body:block) => {
        #[allow(unused_parens)]
        $pub fn $mangled (($($name),*) : ($($typ),*)) $( -> $ret )? $body

        $pub fn $fun ($($name : $typ),*) $( -> $ret )? {
            $deco($mangled)(($($name),*))
        }
    };
}

#[cfg(test)]
mod tests {
    use core::time::Duration;
    use std::thread::sleep;
    use std::time::Instant;

    use crate as decorators;
    use deco_derive::deco;

    //Non-generic decorator, can inspect/modify args
    #[deco]
    fn timer(fun: Function, args: (u64, &str)) {
        let start_time = Instant::now();
        let r = fun(args);
        let run_time = Instant::now() - start_time;
        println!("Finished in {:?} secs", run_time);
        r
    }

    #[deco(timer)]
    fn do_work(work_time: u64, payload: &str) -> bool {
        sleep(Duration::new(work_time, 0));
        println!("Work done: {}", payload);
        true
    }

    #[test]
    fn test() {
        if do_work(1, ":3") {
            do_work(2, "yay!");
        }

        //Generic decorator, cannot inspect/modify args
        #[deco]
        fn do_stuff<A>(f: F, args: A) {
            println!("doing stuff!");
            f(args)
        }

        #[deco(do_stuff)]
        fn noop(_a: i32) {}

        noop(69);

        //Decorator without any arguments to pass
        #[deco]
        fn argless_decorator(f: F) {
            println!("Doing argless work!");
            f()
        }

        fn example_function() {}

        let decorated_manually = argless_decorator(example_function);
        decorated_manually();
    }
}
