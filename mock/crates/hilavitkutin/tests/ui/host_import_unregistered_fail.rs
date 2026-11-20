// A column the builder never registered is refused as the store of an import.

include!("host_import_body.in");

host_main!(import Column<Other> = [Other(1), Other(2)]; dump Column<B>);
