use crate::common::compile_run_check_logs;
mod common;

#[test]
fn averaging_array() {
    let code = r#"
STOCK = [1, 2, 3, 4, 5, 6, 7, 9, 10]

COUNT = 0
TOTAL = 0
loop N from 0 to STOCK.length - 1
if STOCK[N] > 0 then
COUNT = COUNT + 1
TOTAL = TOTAL + STOCK[N]
end if
end loop
if NOT COUNT = 0 then
AVERAGE = TOTAL / COUNT
output "Average = " , AVERAGE
else
output "There are no non-zero values"
end if
   "#;

    compile_run_check_logs(
        code,
        r#""#,
        r#"
Average = 5.222222222222222
"#,
    );
}

#[test]
fn copying_from_collection_to_array() {
    let code = r#"
NAMES = new Collection()
NAMES.addItem("Alex")
NAMES.addItem("Boris")
NAMES.addItem("Alex")
NAMES.addItem("Boris")
NAMES.addItem("Tyy")
NAMES.addItem("SASAS")
NAMES.addItem("ddd")
NAMES.addItem("ddd")
LIST = new Array()

COUNT = 0 // number of names currently in LIST
loop while NAMES.hasNext()
DATA = NAMES.getNext()
FOUND = false
loop POS from 0 to COUNT-1
if DATA = LIST[POS] then
FOUND = true
end if
end loop
if FOUND = false then
LIST[COUNT] = DATA
COUNT = COUNT + 1
end if
end loop

output LIST
   "#;

    compile_run_check_logs(
        code,
        r#""#,
        r#"
Alex,Boris,Tyy,SASAS,ddd
"#,
    );
}

#[test]
fn factors() {
    let code = r#"
// recall that
// 30 div 7 = 4
// 30 mod 7 = 2
NUM = 140 // code will print all factors of this number
F = 1
FACTORS = 0
loop until F*F > NUM //code will loop until F*F is greater than NUM
    if NUM mod F = 0 then
        D = NUM div F
        output NUM , " = " , F , "*" , D
        if F = 1 then
            FACTORS = FACTORS + 0
        else if F = D then
            FACTORS = FACTORS + 1
        else
            FACTORS = FACTORS + 2
        end if
    end if
    F = F + 1
end loop
output NUM , " has " , FACTORS , " factors "
   "#;

    compile_run_check_logs(
        code,
        r#""#,
        r#"
140 = 1 * 140
140 = 2 * 70
140 = 4 * 35
140 = 5 * 28
140 = 7 * 20
140 = 10 * 14
140 has 10 factors
"#,
    );
}

#[test]
fn copying_collection_reverse() {
    let code = r#"
SURVEY = new Collection()
SURVEY.addItem("Alex")
SURVEY.addItem("Moe")
SURVEY.addItem("Mud")
SURVEY.addItem("Clark")
SURVEY.addItem("Ope")

MYSTACK = new Stack()
MYARRAY = new Array()

COUNT = 0 // number of names
loop while SURVEY.hasNext()
MYSTACK.push( SURVEY.getNext() )
COUNT = COUNT + 1
end loop
// Fill the array, MYARRAY, with the names in the stack
loop POS from 0 to COUNT-1
MYARRAY[POS] = MYSTACK.pop()
end loop

output MYARRAY
   "#;

    compile_run_check_logs(
        code,
        r#""#,
        r#"
Ope,Clark,Mud,Moe,Alex
"#,
    );
}
