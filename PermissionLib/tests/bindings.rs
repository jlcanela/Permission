use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    Generator::new(
        Config {
            class: "InteropClass".to_string(),
            dll_name: "permission_lib".to_string(),
            namespace_mappings: NamespaceMappings::new("My.Company"),
            ..Config::default()
        },
        permission_lib::my_inventory(),
    )
    .write_file("bindings/csharp/Interop.cs")?;

    Ok(())
}
