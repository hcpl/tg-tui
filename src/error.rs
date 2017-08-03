error_chain! {
    foreign_links {
        AppDirs(::app_dirs::AppDirsError);
        TimeParse(::time::ParseError);
    }
}
