/// Creates a [`Version`](crate::messages::Version) using the package version
#[macro_export]
macro_rules! pkg_version {
    () => {
        $crate::messages::Version {
            major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        }
    };
}
