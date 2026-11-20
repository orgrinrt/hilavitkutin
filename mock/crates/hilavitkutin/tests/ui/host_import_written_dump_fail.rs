// `Written<Column<B>>` carries `ColMember` and `dump` locates it, so `Kept` is the one bound refusing it as the store of a dump.

include!("host_import_body.in");

host_main!(import Column<A> = [A(1), A(2)]; dump Written<Column<B>>);
