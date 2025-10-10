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
fn special_cases_eq() {
    let code = r#"
A = undefined
B = new Array()
C = new Array()

output B == B
output B != C

output 1 / 0
output -1 / 0
output undefined == A
output A == 1
output true == A
output A == "something"
output A == 44
output 1 == true
output 0 == false
output 1 == "1"
output false == "false"
output false >= "z"
output 5 <= "z"
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
true
true
Infinity
-Infinity
true
false
false
false
false
true
true
true
true
false
true
"#,
    );
}
