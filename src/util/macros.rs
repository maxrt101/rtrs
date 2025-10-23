
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
macro_rules! ok {
    ($e:expr) => {
        match $e {
            Ok(v) => Ok(v),
            Err(_) => Err(()),
        }
    };
}

#[macro_export]
macro_rules! ignore {
    ($e:expr) => {
        let _ = $e;
    };
}

#[macro_export]
macro_rules! map_range {
    ($value:expr, $default:expr, $(($from:expr, $to:expr, $val:expr)),* ) => {
        match $value {
            $(_ if $value >= $from && $value <= $to => { $val })*
            _ => $default,
        }
    };
}

#[macro_export]
macro_rules! cap {
    ($value:expr, $low_bound:expr, $high_bound:expr) => {{
        let value = $value;
        let low_bound = $low_bound;
        let high_bound = $high_bound;

        if value < low_bound {
            low_bound
        } else if value > high_bound {
            high_bound
        } else {
            value
        }
    }};
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
