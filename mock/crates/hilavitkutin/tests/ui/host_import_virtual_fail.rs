// A registered `Virtual` is refused as the store of an import.

include!("host_import_body.in");

host_main!(import Virtual<Ev> = [Ev(1), Ev(2)]; dump Column<B>);
