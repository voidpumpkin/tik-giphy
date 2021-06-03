#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use yew::services::ConsoleService;

        ConsoleService::log(format!($($arg)*).as_str());
    }};
}
