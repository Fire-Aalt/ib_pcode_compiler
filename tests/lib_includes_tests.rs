use crate::common::compile_run_check_logs;

mod common;

#[test]
fn math_all() {
    let code = r#"
method norm(x)
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

#[test]
fn math_edge_cases() {
    let code = r#"
method norm(x)
    return Math.round(x * 1e11) / 1e11
end method

output norm(Math.pow(-5, 5))
output norm(Math.pow(-5, -5))
output norm(Math.pow(-5, -0.2))
output "---"
output norm(Math.log1p(0.2))
output norm(Math.log1p(0))
output norm(Math.log1p(-0.2))
output norm(Math.log1p(-1))
output norm(Math.log1p(-2))
output "---"
output norm(Math.log(0.2))
output norm(Math.log(0))
output norm(Math.log(-0.2))
output norm(Math.log(-1))
output norm(Math.log(-2))
output "---"
output norm(Math.log10(0.2))
output norm(Math.log10(0))
output norm(Math.log10(-0.2))
output norm(Math.log10(-1))
output norm(Math.log10(-2))
output "---"
output norm(Math.log2(0.2))
output norm(Math.log2(0))
output norm(Math.log2(-0.2))
output norm(Math.log2(-1))
output norm(Math.log2(-2))
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
-3125
-0.00032
NaN
---
0.18232155679
0
-0.22314355131
-Infinity
NaN
---
-1.60943791243
-Infinity
NaN
NaN
NaN
---
-0.69897000434
-Infinity
NaN
NaN
NaN
---
-2.32192809489
-Infinity
NaN
NaN
NaN 
"#,
    );
}

#[test]
fn collection() {
    let code = r#"
C = new Collection()

C.addItem(1)
C.addItem(2)
C.addItem(3)
output C.isEmpty()
C.addItem(4)
C.remove(3)

loop while C.hasNext()
    output C.getNext()
end loop

C.remove(1)
C.remove(4)
C.remove(2)

output C.isEmpty()
C.resetNext()

C.addItem(1)
C.addItem(2)

loop while C.hasNext()
    output C.getNext()
end loop

output C.contains(2)
output C.contains(4)
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
false
1
2
4
true
1
2
true
false
"#,
    );
}

#[test]
fn queue() {
    let code = r#"
Q = new Queue()

output Q.isEmpty()
Q.enqueue(1)
Q.enqueue(2)
Q.enqueue(3)
Q.enqueue(4)
Q.enqueue(5)
Q.enqueue(6)
Q.enqueue(7)
output Q.isEmpty()

loop until Q.isEmpty()
    output Q.dequeue()
end loop

Q.enqueue(1)
Q.enqueue(2)
Q.enqueue(3)
Q.enqueue(4)
Q.enqueue(5)
Q.enqueue(6)
Q.enqueue(7)
output Q.isEmpty()

loop until Q.isEmpty()
    output Q.dequeue()
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
true
false
1
2
3
4
5
6
7
false
1
2
3
4
5
6
7
"#,
    );
}

#[test]
fn stack() {
    let code = r#"
S = new Stack()

output S.isEmpty()
S.push(1)
S.push(2)
S.push(3)
S.push(4)
S.push(5)
S.push(6)
S.push(7)
output S.isEmpty()

loop until S.isEmpty()
    output S.pop()
end loop

S.push(1)
S.push(2)
S.push(3)
S.push(4)
S.push(5)
S.push(6)
S.push(7)
output S.isEmpty()

loop until S.isEmpty()
    output S.pop()
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
true
false
7
6
5
4
3
2
1
false
7
6
5
4
3
2
1 
"#,
    );
}
