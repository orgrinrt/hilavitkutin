// A registered `Resource` is refused as the store of an import.

include!("host_import_body.in");

host_main!(import Resource<Res> = [Res(1), Res(2)]; dump Column<B>);
