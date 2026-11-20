// A registered `Accum` is refused as the store of an import.

include!("host_import_body.in");

host_main!(import Accum<Acc> = [Acc(1), Acc(2)]; dump Column<B>);
