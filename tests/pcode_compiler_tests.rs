use crate::common::{compile_run_check_logs, compile_test, run_check_logs};

mod common;

#[test]
fn intro() {
    let code = r#"
output "Welcome"
loop COUNT from 1 to 5
  output COUNT
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Welcome
1
2
3
4
5
"#,
    );
}

#[test]
fn calculations() {
    let code = r#"
output "=== Simple Calculations ==="

output "Adding 1...10 = " , 1+2+3+4+5+6+7+8+9+10

output "10 Factorial = " , 1*2*3*4*5*6*7*8*9*10

output "Fractions = 1/2 + 1/4 + 1/5 = " , 1/2 + 1/4 + 1/5

output "Pythagoras = 3^2 + 4^2 = 5^2 = " , 3*3 + 4*4 , " and " , 5*5

output "Big Numbers = one trillion ^ 2 = " , 1000000000000 * 1000000000000

output "Easier big numbers = " , 2e12 * 3e12

output "10307 is not prime = " , 10307 / 11 , " * " , 11

output "15% of 12345 = " , 15/100*12345

output "Incorrect calculation = " , 1234567890 * 1234567890

output "Another error = " , 1/2 + 1/3 + 1/6

output "One more problem = " , 0.1+0.1+0.1+0.1+0.1+0.1+0.1+0.1

output "And another problem = " , 3.2 - 0.3
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
=== Simple Calculations ===
Adding 1...10 = 55
10 Factorial = 3628800
Fractions = 1/2 + 1/4 + 1/5 = 0.95
Pythagoras = 3^2 + 4^2 = 5^2 = 25 and 25
Big Numbers = one trillion ^ 2 = 1e24
Easier big numbers = 6e24
10307 is not prime = 937 * 11
15% of 12345 = 1851.75
Incorrect calculation = 1524157875019052000
Another error = 0.9999999999999999
One more problem = 0.7999999999999999
And another problem = 2.9000000000000004
"#,
    );
}

#[test]
fn solve_equations() {
    let code = r#"
X = 4
Y = X*X - 9*X + 14
output "x = " , X , " .... y = " , Y
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
x = 4 .... y = -6
"#,
    );
}

#[test]
fn solving2() {
    let code = r#"
A = 10
B = 100
output "Sum = " , A + B
output "Product = " , A * B
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Sum = 110
Product = 1000
"#,
    );
}

#[test]
fn ski_trip() {
    let code = r#"
 CARS = 8
 BUSSES = 8
 HOTEL = 10
 LODGE = 12
 SEATS = CARS*4 + BUSSES*20
 BEDS = HOTEL*4 + LODGE*12
 COST = CARS*250 + BUSSES*1000 + HOTEL*300 + LODGE*800

 output CARS , " cars and " , BUSSES , " busses = " , SEATS , " seats"
 output HOTEL , " rooms and " , LODGE , " lodges = " , BEDS , " beds"
 output "Total cost = " , COST
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
8 cars and 8 busses = 192 seats
10 rooms and 12 lodges = 184 beds
Total cost = 22600
"#,
    );
}

#[test]
fn if_then() {
    let code = r#"
  UNIT = input("Type a unit")

  if  UNIT = "km"  then
     output "1 km = 1000 m = 0.6 miles"
  end if

  if  UNIT = "mi"  then
     output "1 mi = 5280 ft = 1.6 km"
  end if

  if  UNIT = "ft"  then
     output "1 ft = 12 in = 30.5 cm"
  end if

  if  UNIT = "liter"  then
     output "1 liter = 1000 ml = 1/3 gallon"
     output "Don't forget that IMPERIAL GALLONS"
     output "are different than US GALLONS"
  end if
   "#;

    let ast = compile_test(code);

    run_check_logs(
        &ast,
        "km",
        r#"
1 km = 1000 m = 0.6 miles
"#,
    );
    run_check_logs(
        &ast,
        "mi",
        r#"
1 mi = 5280 ft = 1.6 km
"#,
    );
    run_check_logs(
        &ast,
        "ft",
        r#"
1 ft = 12 in = 30.5 cm
"#,
    );
    run_check_logs(
        &ast,
        "liter",
        r#"
1 liter = 1000 ml = 1/3 gallon
Don't forget that IMPERIAL GALLONS
are different than US GALLONS
"#,
    );
    run_check_logs(&ast, "WRONG", "");
}

#[test]
fn password_logic() {
    let code = r#"
 NAME = input("Type your user name")
 PASSWORD = input("Type your password")

 if  NAME = "bozo"  AND  PASSWORD = "clown"  then
    output "Correct!"
 end if

 if  NAME = "einstein"  AND  PASSWORD = "e=mc2"  then
    output "Correct!"
 end if

 if  NAME = "guest"  OR  NAME = "trial"  then
    output "You will be logged in as a GUEST"
 end if
   "#;

    let ast = compile_test(code);

    run_check_logs(
        &ast,
        r#"
bozo
clown
"#,
        r#"
Correct!
"#,
    );

    run_check_logs(
        &ast,
        r#"
einstein
e=mc2
"#,
        r#"
Correct!
"#,
    );

    run_check_logs(
        &ast,
        r#"
guest
WRONG
"#,
        r#"
You will be logged in as a GUEST
"#,
    );

    run_check_logs(
        &ast,
        r#"
trial
WRONG
"#,
        r#"
You will be logged in as a GUEST
"#,
    );

    run_check_logs(
        &ast,
        r#"
WRONG
WRONG
"#,
        r#"
"#,
    );
}

#[test]
fn discount_logic() {
    let code = r#"
QUANTITY = input("How many hamburgers do you want?")
PRICE = 0

if  QUANTITY >= 10  then
PRICE = 2.59
else if  QUANTITY <= 9  AND  QUANTITY >= 5  then
PRICE = 2.89
else if  QUANTITY < 5  then
PRICE = 3.25
end if

output "That costs " , PRICE , " per burger"
output "Total cost = " , PRICE * QUANTITY , " for " , QUANTITY , " burgers"
   "#;

    let ast = compile_test(code);

    run_check_logs(
        &ast,
        "12",
        r#"
That costs 2.59 per burger
Total cost = 31.08 for 12 burgers
"#,
    );
    run_check_logs(
        &ast,
        "7",
        r#"
That costs 2.89 per burger
Total cost = 20.23 for 7 burgers
"#,
    );
    run_check_logs(
        &ast,
        "3",
        r#"
That costs 3.25 per burger
Total cost = 9.75 for 3 burgers
"#,
    );
}

#[test]
fn mice_loops() {
    let code = r#"
loop A from 1 to 2
   output "Three blind mice"
end loop

loop B from 3 to 4
   output "See how they run"
end loop

output "They all ran up to the farmer's wife"
output "She cut off their tails with a carving knife"
output "Did you ever see such a sight in your life, as"

C = 5
loop while C < 20
   output "Three blind mice"
   C = C*2
end loop
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Three blind mice
Three blind mice
See how they run
See how they run
They all ran up to the farmer's wife
She cut off their tails with a carving knife
Did you ever see such a sight in your life, as
Three blind mice
Three blind mice
"#,
    );
}

#[test]
fn money_decisions() {
    let code = r#"
EUROS = 50.00

POUNDS = 0.8*EUROS

DOLLARS = EUROS / 0.75

YEN = EUROS * 90

output EUROS , " EUR"

output YEN , " Yen"

if YEN > 1000 then
output "That is a lot of Yen"
end if

output POUNDS , " BP"

if POUNDS < 100 then
output "That is a small number of Pounds"
end if

output "$" , DOLLARS

if DOLLARS = 100 then
output "BINGO"
end if
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
50 EUR
4500 Yen
That is a lot of Yen
40 BP
That is a small number of Pounds
$ 66.66666666666667
"#,
    );
}

#[test]
fn common_factors() {
    let code = r#"
A = 28
B = 42

output "Common factors of " , A , " and " , B

loop C from 1 to B
    if (A mod C = 0) AND (B mod C = 0) then
       output C
    end if
end loop
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Common factors of 28 and 42
1
2
7
14
"#,
    );
}

#[test]
fn math_values() {
    let code = r#"
output "X , Y"

loop C from 0 to 10
   X = C / 2.0
   Y = 3*X*X - 7*X + 2
   output X , " , " , Y
end loop
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
X , Y
0 , 2
0.5 , -0.75
1 , -2
1.5 , -1.75
2 , 0
2.5 , 3.25
3 , 8
3.5 , 14.25
4 , 22
4.5 , 31.25
5 , 42
"#,
    );
}

#[test]
fn collatz_sequence() {
    let code = r#"
NUM = 29

loop until NUM = 1

output NUM

if NUM mod 2 = 0 then
NUM = NUM / 2
else
NUM = NUM * 3 + 1
end if

end loop

output NUM
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
29
88
44
22
11
34
17
52
26
13
40
20
10
5
16
8
4
2
1
"#,
    );
}

#[test]
fn translate_strings() {
    let code = r#"
input ENGLISH
GERMAN = ""

if ENGLISH = "hello" then
 GERMAN = "guten Tag"
else if ENGLISH = "goodbye" then
 GERMAN = "auf Wiedersehen"
else if ENGLISH = "stop" then
 GERMAN = "halt"
else
 GERMAN = "???"
end if

output "English = " , ENGLISH
output "German = " , GERMAN
   "#;

    let ast = compile_test(code);

    run_check_logs(
        &ast,
        "hello",
        r#"
English = hello
German = guten Tag
"#,
    );
    run_check_logs(
        &ast,
        "goodbye",
        r#"
English = goodbye
German = auf Wiedersehen
"#,
    );
    run_check_logs(
        &ast,
        "stop",
        r#"
English = stop
German = halt
"#,
    );
    run_check_logs(
        &ast,
        "WRONG",
        r#"
English = WRONG
German = ???
"#,
    );
}

#[test]
fn elapsed_minutes() {
    let code = r#"
input START_HOURS
input START_MINUTES

input END_HOURS
input END_MINUTES

if START_HOURS > 23 OR START_MINUTES > 59 then
 output "Start time is not valid"
else if END_HOURS > 23 OR END_MINUTES > 59 then
 output "Times are not valid"
else
 MINUTES = (END_HOURS - START_HOURS)*60 + (END_MINUTES-START_MINUTES)
 output "Elapsed time = " , MINUTES , " minutes"
end if
   "#;

    let ast = compile_test(code);

    run_check_logs(
        &ast,
        r#"
4
8
8
2"#,
        r#"
Elapsed time = 234 minutes
"#,
    );
    run_check_logs(
        &ast,
        r#"
8
8
28
8"#,
        r#"
Times are not valid
"#,
    );
}

#[test]
fn date_validation() {
    let code = r#"
input MONTH
input DAY
input YEAR

output MONTH , "/" , DAY , "/" , YEAR

FEBMAX = 0
if YEAR mod 4 = 0 then
   FEBMAX = 29
else
   FEBMAX = 28
end if

M = MONTH
D = DAY

if M < 1 OR M > 12 then
    output "Month is not valid"
else if D < 1 OR D > 31 then
    output "Day is not valid"
else if D = 31 AND (M = 4 OR M = 6 OR M = 9 OR M = 11) then
    output "That month does not have 31 days"
else if M = 2 AND D > FEBMAX then
    output "February only has " , FEBMAX , " days"
else
    output "Date is valid"
end if
   "#;

    let ast = compile_test(code);

    run_check_logs(
        &ast,
        r#"
8
84
2
"#,
        r#"
8 / 84 / 2
Day is not valid
"#,
    );
    run_check_logs(
        &ast,
        r#"
8
20
2004
"#,
        r#"
8 / 20 / 2004
Date is valid
"#,
    );
}

#[test]
fn add_up_number() {
    let code = r#"
MAX = 10

SUM = 0

loop COUNT from 0 to MAX
    output COUNT
    SUM = SUM + COUNT
end loop

output "Total = " , SUM
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
0
1
2
3
4
5
6
7
8
9
10
Total = 55
"#,
    );
}

#[test]
fn primes_array() {
    let code = r#"
NUMS = new Array()

loop N from 1 to 100
   NUMS[N] = 0
end loop

loop P from 2 to 50
   N = P * 2
   loop while N <= 100
      NUMS[N] = 1
      N = N + P
   end loop
end loop

output "These are the PRIME numbers under 100"

loop N from 2 to 100
   if NUMS[N] = 0 then
      output N
   end if
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
These are the PRIME numbers under 100
2
3
5
7
11
13
17
19
23
29
31
37
41
43
47
53
59
61
67
71
73
79
83
89
97
"#,
    );
}

#[test]
fn binary_search() {
    let code = r#"
ID = [1001,1002,1050,1100,1120,1180,1200,1400]
NAME = ["Apple","Cherry","Peach","Banana","Fig","Grape","Olive","Mango"]

output "Type the ID number that you wish to find"
input TARGET

LOW = 0
HIGH = 7
FOUND = -1

loop while FOUND = -1 AND LOW <= HIGH
MID = div( LOW + HIGH , 2 )   // should be (LOW + HIGH) div 2
// but (A div B) doesn't work correctly
// so there is a special method below
if ID[MID] = TARGET then
FOUND = MID
else if TARGET < ID[MID] then
HIGH = MID - 1
else
LOW = MID + 1
end if
end while

if FOUND >= 0 then
output TARGET , ":" , NAME[FOUND]
else
output TARGET , " was not found"
end if
    "#;

    compile_run_check_logs(
        code,
        "1050",
        r#"
Type the ID number that you wish to find
1050 : Peach
"#,
    );
}

#[test]
fn selection_sort() {
    let code = r#"
NUMS = [15,30,85,25,40,90,50,65,20,60]

output "Before sorting"
printNums()

loop FIRST from 0 to 9
    LEAST = FIRST
    loop CURRENT from FIRST+1 to 9
        if NUMS[CURRENT] > NUMS[LEAST] then
           LEAST = CURRENT
        end if
    end loop
    TEMP = NUMS[LEAST]
    NUMS[LEAST] = NUMS[FIRST]
    NUMS[FIRST] = TEMP
end loop

output "After sorting"
printNums()

method printNums()
   loop C from 0 to 9
      output NUMS[C]
   end loop
   output "========"
end method
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Before sorting
15
30
85
25
40
90
50
65
20
60
========
After sorting
90
85
65
60
50
40
30
25
20
15
========
"#,
    );
}

#[test]
fn bubble_sort() {
    let code = r#"
NUMS = [15,30,85,25,40,90,50,65,20,60]

output "Before sorting"
loop C from 0 to 9
   output NUMS[C]
end loop

output "========"

loop PASS from 0 to 8
    loop CURRENT from 0 to 8
        if NUMS[CURRENT] < NUMS[CURRENT + 1] then
          TEMP = NUMS[CURRENT]
          NUMS[CURRENT] = NUMS[CURRENT+1]
          NUMS[CURRENT+1] = TEMP
        end if
    end loop
end loop

output "After sorting"

loop C from 0 to 9
   output NUMS[C]
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Before sorting
15
30
85
25
40
90
50
65
20
60
========
After sorting
90
85
65
60
50
40
30
25
20
15
"#,
    );
}

#[test]
fn reverse_array() {
    let code = r#"
NAMES = ["Robert","Boris","Brad","George","David"]

N = 5     // the number of elements in the array
K = 0     // this is the first index in the array

loop while K < N - 1
TEMP = NAMES[K]
NAMES[K] = NAMES[N - K - 1]
NAMES[N - K - 1] = TEMP
K = K + 1
end loop

loop C from 0 to N-1
output NAMES[C]
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
David
Boris
Brad
George
Robert
"#,
    );
}

#[test]
fn frequency_distribution() {
    let code = r#"
DATA = [17,20,23,29,33,42,60,61,75,75,90,99]
FREQS = [0,0,0,0,0,0,0,0,0,0]

loop C from 0 to 11
VALUE = DATA[C]
loop F from 0 to 9
if VALUE >= 10*F AND VALUE < 10*(F+1) then
FREQS[F] = FREQS[F] + 1
end if
end loop
end loop

output "Data"

loop D from 0 to 11
output DATA[D]
end loop

output "Range : Frequency"

loop F from 0 to 9
output F*10 , " - " , (F+1)*10 , " : " , FREQS[F]
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Data
17
20
23
29
33
42
60
61
75
75
90
99
Range : Frequency
0 - 10 : 0
10 - 20 : 1
20 - 30 : 3
30 - 40 : 1
40 - 50 : 1
50 - 60 : 0
60 - 70 : 2
70 - 80 : 2
80 - 90 : 0
90 - 100 : 2
"#,
    );
}

#[test]
fn appointments_list() {
    let code = r#"
APPS = new Array()
NAME = ""

loop T from 0 to 2400
APPS[T] = ""
end loop

loop while NAME <> "quit"
input NAME
input TIME
if TIME >= 0 AND TIME <= 2359 then
APPS[TIME] = NAME
end if

loop T from 0 to 2400
if APPS[T] <> "" then
output T , " : " , APPS[T]
end if
end loop
output "=================="
end loop
    "#;

    compile_run_check_logs(
        code,
        r#"
Me
1230
Brown
2350
quit
0
        "#,
        r#"
1230 : Me
==================
1230 : Me
2350 : Brown
==================
0 : quit
1230 : Me
2350 : Brown
==================
"#,
    );
}

#[test]
fn find_duplicates() {
    let code = r#"
TENNIS = ["Al","Bobby","Carla","Dave","Ellen"]
BBALL = ["Lou","Dave","Hellen","Alan","Al"]

output "The following appear in both lists"

loop T from 0 to 4
loop B from 0 to 4
if TENNIS[T] = BBALL[B] then
output TENNIS[T]
end if
end loop
end loop
    "#;

    compile_run_check_logs(
        code,
        r#""#,
        r#"
The following appear in both lists
Al
Dave
"#,
    );
}

#[test]
fn cities_array() {
    let code = r#"
CITIES = ["Athens","Berlin","Dallas","Denver","London","New York","Rome"]

COUNT = 0

loop C from 0 to 6
if firstLetter( CITIES[C] ) = "D" then
COUNT = COUNT + 1
output CITIES[C]
end if
end loop

output "That was " , COUNT , " D-cities"

method firstLetter(s)
return s.substring(0,1)
end method
    "#;

    compile_run_check_logs(
        code,
        r#""#,
        r#"
Dallas
Denver
That was 2 D-cities
"#,
    );
}

#[test]
fn magic_square() {
    let code = r#"
    A = [
      [8,1,6] ,
      [3,5,7] ,
      [4,9,2]
    ]

OK = "correct"

loop R from 0 to 2
   output A[R][0] , A[R][1] , A[R][2]
end loop

loop R from 0 to 2
   SUM = 0
   loop C from 0 to 2
      SUM = SUM + A[R][C]
   end loop

   if SUM != 15 then
      output "Row " , R , " is wrong"
      OK = "wrong"
   end if
end loop

loop C from 0 to 2
   SUM = 0
   loop R from 0 to 2
      SUM = SUM + A[R][C]
   end loop

   if SUM != 15 then
      output "Column " , C , " is wrong"
      OK = "wrong"
   end if
end loop

SUM = 0
loop X from 0 to 2
   R = X
   C = X
   SUM = SUM + A[R][C]
end loop

if SUM != 15 then
   output "Main diag is wrong"
   OK = "wrong"
end if

SUM = 0
loop X from 0 to 2
   R = X
   C = 2-X
   SUM = SUM + A[R][C]
end loop

if SUM != 15 then
   output "Other diag is wrong"
   OK = "wrong"
end if

output "Entire square is " , OK
"#;

    compile_run_check_logs(
        code,
        r#""#,
        r#"
8 1 6
3 5 7
4 9 2
Entire square is correct
"#,
    );
}

#[test]
fn names_collection() {
    let code = r#"
NAMES = new Collection()

NAMES.addItem("Bob")
NAMES.addItem("Dave")
NAMES.addItem("Betty")
NAMES.addItem("Kim")
NAMES.addItem("Debbie")
NAMES.addItem("Lucy")

NAMES.resetNext()

output "These names start with D"

loop while NAMES.hasNext()
    NAME = NAMES.getNext()
    if firstLetter(NAME) = "D" then
      output NAME
    end if
end loop

method firstLetter(s)
   return s.substring(0,1)
end method
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
These names start with D
Dave
Debbie
"#,
    );
}

#[test]
fn checkout_collection() {
    let code = r#"
NAMES = new Collection()
NAME = ""

loop while NAME <> "quit"
   input NAME
   if NAME <> "quit" then
       if NAMES.contains(NAME) then
           output NAME , " returned"
           NAMES.remove(NAME)
       else
           output NAME , " is leaving"
           NAMES.addItem(NAME)
       end if
   end if
end loop

output "The following students left and did not return"

NAMES.resetNext()

loop while NAMES.hasNext()
   output NAMES.getNext()
end loop
    "#;

    compile_run_check_logs(
        code,
        r#"
Numy
RRR
Tom
Tom
Wert
Numy
Cl
quit
        "#,
        r#"
Numy is leaving
RRR is leaving
Tom is leaving
Tom returned
Wert is leaving
Numy returned
Cl is leaving
The following students left and did not return
RRR
Wert
Cl
"#,
    );
}

#[test]
fn stack_reverse_list() {
    let code = r#"
NAMES = ["Alex","Bobby","Cho","Deke"]

STACK = new Stack()

loop COUNT from 0 to 3
   STACK.push(NAMES[COUNT])
end loop

loop while NOT(STACK.isEmpty())
   NAME = STACK.pop()
   output NAME
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Deke
Cho
Bobby
Alex
"#,
    );
}

#[test]
fn queues_merge() {
    let code = r#"
PEOPLE = ["Alex","Bobby","Cho","Deke","Ellen"]
DOGS = ["spot","woofie","bruiser"]

PQ = new Queue()          // Queue for People names
DQ = new Queue()          // Queue for dog names

/////////////////////////// copy people names
loop P from 0 to 4
   PQ.enqueue(PEOPLE[P])
end loop

//////////////////////////// copy dog names
loop D from 0 to 2
   DQ.enqueue(DOGS[D])
end loop

loop while NOT(PQ.isEmpty()) OR NOT(DQ.isEmpty())
   if NOT(PQ.isEmpty()) then
      output "Person = " , PQ.dequeue()
   else
      output "People list is empty"
   end if

   if NOT(DQ.isEmpty()) then
      output "Dog = " , DQ.dequeue()
   else
      output "Dog list is empty"
   end if
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Person = Alex
Dog = spot
Person = Bobby
Dog = woofie
Person = Cho
Dog = bruiser
Person = Deke
Dog list is empty
Person = Ellen
Dog list is empty
"#,
    );
}

#[test]
fn bank_classes() {
    let code = r#"
Class Account(name,amount)
    this.id = name
    this.balance = amount

    this.addInterest = function(percent)
    {
       this.balance = this.balance + this.balance*percent/100
    }

    this.addMoney = function(money)
    {
       this.balance = this.balance + money
    }

    this.show = function()
    {
       output this.id + " " + this.balance
    }
end Class

PAYMENTS = new Account("Abbey",100.0)

INTEREST = new Account("Pat",100.0)

loop YEARS from 0 to 10
    output "== Year : " + YEARS + " =="
    PAYMENTS.show()
    INTEREST.show()

    PAYMENTS.addMoney(100)
    INTEREST.addInterest(10)
end loop

    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
== Year : 0 ==
Abbey 100
Pat 100
== Year : 1 ==
Abbey 200
Pat 110
== Year : 2 ==
Abbey 300
Pat 121
== Year : 3 ==
Abbey 400
Pat 133.1
== Year : 4 ==
Abbey 500
Pat 146.41
== Year : 5 ==
Abbey 600
Pat 161.051
== Year : 6 ==
Abbey 700
Pat 177.15609999999998
== Year : 7 ==
Abbey 800
Pat 194.87170999999998
== Year : 8 ==
Abbey 900
Pat 214.35888099999997
== Year : 9 ==
Abbey 1000
Pat 235.79476909999997
== Year : 10 ==
Abbey 1100
Pat 259.37424601
"#,
    );
}
