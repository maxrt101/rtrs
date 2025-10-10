
#[macro_export]
macro_rules! to_empty_result {
    ($e:expr) => {
        match $e {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    };
}

#[macro_export]
macro_rules! timeit {
    ($code:block) => {{
        let start = rtrs::time::global_tick();
        $code;
        let end = rtrs::time::global_tick();
        end - start
    }};
}
