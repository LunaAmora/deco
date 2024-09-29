#[cfg(feature = "derive")]
pub use deco_derive::deco;

#[macro_export]
macro_rules! make_decorator {
    ($pub:vis fn $fun:ident $(<$($gen:tt $(: $bnd:tt)?),*>)? ($f:ident : $typ:ident $(, $arg:ident: $arg_ty:ty)?) -> $ret:ty $body:block) => {
        $pub fn $fun <$($($gen: $($bnd)?),*,)? $typ> ($f: $typ) -> impl Fn($($arg_ty)?) -> $ret
        where
            $typ: Fn($($arg_ty)?) -> $ret,
        {
            move |$($arg)?| $body
        }
    };

    ($pub:vis fn $fun:ident $(<$($gen:tt $(: $bnd:tt)?),*>)? ($f:ident : $typ:ident $(, ($($arg:pat),*) : $arg_ty:ty)?) -> $ret:ty $body:block) => {
        #[allow(unused_parens)]
        $pub fn $fun <$($($gen: $($bnd)?),*,)? $typ> ($f: $typ) -> impl Fn($($arg_ty)?) -> $ret
        where
            $typ: Fn($($arg_ty)?) -> $ret,
        {
            move |$(($($arg),*))?| $body
        }
    };
}

#[macro_export]
macro_rules! decorator {
    (@ $deco:ident $mangled:ident $pub:vis fn $fun:ident $(<$($gen:tt $(: $bnd:tt)?),*>)? ( $($name:ident : $typ:ty),* ) $( -> $ret:ident )? $body:block) => {
        #[allow(unused_parens)]
        $pub fn $mangled $(<$($gen: $($bnd)?),*,>)?(($($name),*) : ($($typ),*)) $( -> $ret )? $body

        $pub fn $fun $(<$($gen: $($bnd)?),*,>)? ($($name : $typ),*) $( -> $ret )? {
            $deco($mangled)(($($name),*))
        }
    };
}

#[cfg(test)]
mod tests {
    use core::time::Duration;
    use std::fmt::Display;
    use std::thread::sleep;
    use std::time::Instant;

    use crate as decorators;
    use deco_derive::deco;

    //Non-generic decorator, can inspect/modify args
    #[deco]
    fn timer<R, D: Display>(fun: Function, args: (u64, D)) -> R {
        let start_time = Instant::now();
        let r = fun(args);
        let run_time = Instant::now() - start_time;
        println!("Finished in {:?} secs", run_time);
        r
    }

    #[deco(timer)]
    fn do_work<D: Display>(work_time: u64, payload: D) -> bool {
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
        fn do_stuff<A, R>(f: F, args: A) -> R {
            println!("doing stuff!");
            f(args)
        }

        #[deco(do_stuff)]
        fn noop(_a: i32) {}

        noop(69);

        //Decorator without any arguments to pass
        #[deco]
        fn argless_decorator<R>(f: F) -> R {
            println!("Doing argless work!");
            f()
        }

        fn example_function() {}

        let decorated_manually = argless_decorator(example_function);
        decorated_manually();
    }
}
