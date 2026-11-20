// A registered `Virtual` is refused as the store of a dump.

include!("host_import_body.in");

host_main!(import Column<A> = [A(1), A(2)]; dump Virtual<Ev>);
