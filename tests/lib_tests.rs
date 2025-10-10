use crate::common::compile_run_check_logs;

mod common;

#[test]
fn array() {
    let code = r#"
A = new Array()

output A.length
A[5] = 5
output A.length
output A
A[500] = 500
output A.length
output A
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
0
6
,,,,,5
501
,,,,,5,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,500
"#,
    );
}

#[test]
fn undefined() {
    let code = r#"
A = undefined

output 1 / 0
output -1 / 0
output undefined == A
output A == 1
output true == A
output A == "something"
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Infinity
-Infinity
true
false
false
false
"#,
    );
}
