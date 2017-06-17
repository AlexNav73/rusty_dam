
pub trait Configuration {
    fn id(&self) -> String;
    fn es_index_name(&self) -> String;
    fn es_url(&self) -> String;
    fn working_dir(&self) -> String;
}
