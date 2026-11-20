// A host imports into the registered `Column<A>`, runs a frame, and dumps `Column<B>`.

include!("host_import_body.in");

host_main!(import Column<A> = [A(1), A(2)]; dump Column<B>);
