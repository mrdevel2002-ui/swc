use std::path::PathBuf;

use swc_common::Mark;
use swc_ecma_parser::{Syntax, TsSyntax};
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::exec_tr;
use swc_ecma_transforms_typescript::{typescript, Config};

#[testing::fixture("tests/fixture/namespace-bindings/**/exec.ts")]
fn runtime(input: PathBuf) {
    execute(input, true);
}

#[testing::fixture("tests/fixture/namespace-bindings/**/exec.ts")]
fn runtime_without_type_resolution(input: PathBuf) {
    execute(input, false);
}

fn execute(input: PathBuf, handle_types: bool) {
    let code = std::fs::read_to_string(input).expect("semantic execution fixture must be readable");
    exec_tr(
        "typescript_semantics",
        Syntax::Typescript(TsSyntax::default()),
        |_| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();
            (
                resolver(unresolved_mark, top_level_mark, handle_types),
                typescript(
                    Config {
                        no_empty_export: true,
                        ..Default::default()
                    },
                    unresolved_mark,
                    top_level_mark,
                ),
            )
        },
        &code,
    );
}
