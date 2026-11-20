// `Written<Column<A>>` carries `ColMember` and is refused as the store of an import by `Kept` alone.

include!("host_import_body.in");

host_main!(import Written<Column<A>> = [A(1), A(2)]; dump Column<B>);
