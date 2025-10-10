
#[macro_export]
macro_rules! tasks_run_with_ctx {
    ( $ctx:ident $(, $task:ident)+ ) => {
        while $ctx.should_run() && ($( $task.is_running() || )* false) {
            $( let _ = $task.poll(); )*
        }
    };
}

#[macro_export]
macro_rules! tasks_run {
    ( $($task:ident),+ ) => {
        while $( $task.is_running() || )* false {
            $( let _ = $task.poll(); )*
        }
    };
}

#[macro_export]
macro_rules! tasks_await {
    ( $($task:ident),+ ) => {
        while $( $task.is_running() || )* false {
            $( $task.cycle().await; )*
        }
    };
}
