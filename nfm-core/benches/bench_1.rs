use {
    nfm_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

const MARKDOWN: &'static str = "
# <anchor-1>Header 1

This is a link back to [Header 1](#anchor-1).

- Here
    0. is
    0. a
        - mixed
- list.

- - -

Here is some content with a footnote^[1](#foot-1)^.

<foot-1>1: Some more info on that footnote.
";

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench 1", |b| b.iter(|| Parser::parse_str(MARKDOWN)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
