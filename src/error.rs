use mode::Mode;


error_chain! {
    foreign_links {
        AppDirs(::app_dirs::AppDirsError);
        Config(::config::ConfigError);
        Pom(::pom::Error);
    }

    errors {
        BindingModeNonRegisterable(mode: Mode) {
            description("no bindings can be registered for the mode")
            display("no bindings can be registered for this mode: {:?}", mode)
        }

        BindingNotFound(mode: Mode, binding: String) {
            description("no binding found for mode")
            display("no binding named {:?} found for mode {:?}", binding, mode)
        }

        UndefinedCommand(cmd: String) {
            description("undefined command")
            display("undefined command: {}", cmd)
        }
    }
}
