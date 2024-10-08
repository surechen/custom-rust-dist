macro_rules! declare_instrcutions {
    ($($name:ident),+) => {
        $(pub(crate) mod $name;)*
        pub(crate) static SUPPORTED_TOOLS: &[&str] = &[$(stringify!($name)),+];

        pub(crate) fn install(tool: &str, path: &std::path::Path, config: &super::install::InstallConfiguration) -> anyhow::Result<()> {
            match tool.replace('-', "_").as_str() {
                $(
                    stringify!($name) => $name::install(path, config),
                )*
                _ => anyhow::bail!("no custom install instruction for '{tool}'")
            }
        }

        pub(crate) fn uninstall(tool: &str) -> anyhow::Result<()> {
            match tool.replace('-', "_").as_str() {
                $(
                    stringify!($name) => $name::uninstall(),
                )*
                _ => anyhow::bail!("no custom uninstall instruction for '{tool}'")
            }
        }

        pub(crate) fn already_installed(tool: &str) -> bool {
            match tool.replace('-', "_").as_str() {
                $(
                    stringify!($name) => $name::already_installed(),
                )*
                // Is not supported, assume not installed for now
                _ => false
            }
        }
    };
}

declare_instrcutions!(buildtools, vscode);

pub(crate) fn is_supported(name: &str) -> bool {
    SUPPORTED_TOOLS.contains(&name.replace('-', "_").as_str())
}
