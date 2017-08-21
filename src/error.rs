error_chain! {
    foreign_links {
        AppDirs(::app_dirs::AppDirsError);
        Config(::config::ConfigError);
        Pom(::pom::Error);
    }
}
