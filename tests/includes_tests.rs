use crate::common::compile_run_check_logs;

mod common;

#[test]
fn math() {
    let code = r#"
method norm(x)
    //if x == "NaN" then
    if x == undefined then
        return "NaN"
    end if
    return Math.round(x * 1e11) / 1e11
end method

output norm(Math.log(8))
output norm(Math.expm1(8))
output norm(Math.exp(8))
output norm(Math.log1p(8))
output norm(Math.log10(8))
output norm(Math.log2(8))

output norm(Math.sin(8))
output norm(Math.cos(8))
output norm(Math.asin(8))
output norm(Math.acos(8))
output norm(Math.atan(8))
output norm(Math.atan2(8, 6))

output norm(Math.pow(8, 6))
output norm(Math.sqrt(8))
output norm(Math.cbrt(8))

output norm(Math.sinh(8))
output norm(Math.cosh(8))
output norm(Math.tanh(8))
output norm(Math.asinh(8))
output norm(Math.atanh(8))
output norm(Math.acosh(8))
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
2.07944154168
2979.95798704173
2980.95798704173
2.19722457734
0.90308998699
3
0.98935824662
-0.14550003381
NaN
NaN
1.44644133225
0.927295218
262144
2.82842712475
2
1490.47882578955
1490.47916125218
0.99999977493
2.77647228072
NaN
2.76865938331
"#,
    );
}