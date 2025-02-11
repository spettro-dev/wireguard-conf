pub trait ExportConfig {
    /// Export struct to Wireguard's config syntax.
    fn config(&self) -> String;
}
