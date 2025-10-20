export const samples = {
    welcome: `output "Welcome"
loop COUNT from 1 to 5
  output COUNT
end loop

// === IB Pcode Compiler =================================
//
// This tool is intended to help teachers prepare 
// Pseudocode that is consistent with the
// IB Computer Science standards for the 2014 syllabus.
// This tool is a comprehensive compiler that
// translates Pcode into an Abstract Syntax Tree (AST) in Rust.
// This is done with a Pest Parser, an AST Builder, an AST Validator,
// and an AST Evaluator. The tool is very sophisticated and it
// follows how real compilers work. Both compile and runtime errors
// are properly displayed with a line number, cause, and a note.
// For the IB Pseudocode documentation refer to the "Docs" tab
// This makes writing pseudocode not frustrating at all ^^
// Inspired by EZ Pseudocode (c) Dave Mulkey 2012
// ==============================================
`,
    calculations: `output "=== Simple Calculations ==="

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
`,
    solve_equations: `//=== Variables ===
//  We can use computers to solve equations, like :
//
//     x^2 - 8x + 12 = 0          (x^2 means x squared)
//
//  The simplest approach is "guess and check" -
//   the user guesses a value for the solution,
//   then the computer calculates the value of the formula, e.g.:
//      X = 4
//      Y = X*X - 8*X + 12  
//      output  Y     ===>   -4
//
//   Now the user needs to make a different guess, like:
//      X = 5
//      Y = X*X - 8*X + 12  
//      output  Y     ===>   -3
//
//  Finally :
//      X = 6
//      Y = X*X - 8*X + 1  ===>   0
//
//  So  X = 6  is the solution.
//
// By using VARIABLES for X and Y, it's easy to change the X-value
// and have it changed automatically in the formula.
//
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ 
//  Now try to solve this equation :
//      x^2 - 9x + 14 = 0
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~    

X = 4
Y = X*X - 9*X + 14
output "x = " , X , " .... y = " , Y
`,
    solving2: `// Here is another problem to solve by using VARIABLES and GUESS-CHECK
//
//   Find 2 numbers that add up to 65, and multiply to equal 1000.

A = 10
B = 100
output "Sum = " , A + B
output "Product = " , A * B
`,
    ski_trip: `// **** Ski Trip ************************************************
// Memorial High School is planning a ski trip for 188 students.
// They have some choices for transportation and lodging:
//
// - Transportation - They can use busses that each carry 20 students
//   Each bus costs 1000 Euros to drive. Each car carries 4 students
//   and costs 250 Euros to drive.
//
// - Lodging - A hotel room costs 300 Euros and accommodates 4 students.
//   A ski lodge room costs 800 Euros and accommodates 12 students.
//
// They can use a mixture of cars and busses, as well as a mixture
// of hotel rooms and youth hostel rooms.  The trip committee can change 
// the numbers of cars, busses, hotel rooms and lodges until they find
// a solution with enough seats and beds, but at minimum cost.
// **************************************************************/

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

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// By trying different values for CARS and BUSSES and HOTEL and LODGE,
// plan the cheapest possible trip for 188 students.
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
`,
    if_then: `//*** INPUT, IF..THEN.. , and STRINGS ********************
//
// Lucy has trouble remembering things like: 1 km = 0.6 miles
// She wants to make a program that pops-up conversion factors.
// She wants to be able to type in a unit, like "km",
// and see a list of conversions, e.g.: 1 km = 1000 m = 0.6 miles
// This requires the ability to have a pop-up box that inputs data.
//
//********************************************************

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

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Add a few more conversion facts -
// maybe some money values, like  1 BP = 1.3 EUROS
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  
//==========================================
// INPUT - the input command makes a box pop-up on the screen,
//         waits for the user to type an answer,
//         and then stores the answer in a variable (UNIT).
//
// IF..THEN.. - checks whether UNIT matches a specific value.
//         If so, the program executes the command(s) between then..end if
//         Notice that the user is going to type letters, not numbers,
//         so you need "quotation marks" around the matching STRING.
//
// STRING - a STRING is a value that contains letters and maybe
//         numbers and punctuation marks -
//         e.g. any characters on a keyboard.
//==========================================  
`,
    password_logic: `// --- LOGIC ---
// The computer can perform more complex LOGIC
// by using BOOLEAN OPERATORS like AND and OR.
// For example, consider the problem of inputting a user name and password.
// Both the account name and the password must be correct.
// So the NAME AND PASSWORD must be correct.
// This example program shows how to use AND / OR correctly.

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
`,
    discount_logic: `// === More Logic =================================
// Computers can do straightforward calculations, as well as complicated formulas.
// But that is not the most interesting thing.
//
// Computers can also perform LOGICAL operations, meaning that they
// can do COMPUTATIONS involving calculations PLUS making LOGICAL decisions.
//
// Logical decisions can be relatively simple, like deciding whether
// a number is larger or smaller than 50.  Other decisions are very complex,
// like deciding whether an IB student's test scores qualify
// for awarding an IB Diploma or not.  There are lots and lots of
// rules for an IB Diploma, so it requires lots and lots of if..then commands. 
//-----------------------------------------------
// Here is a common example - where the price of buying items changes
// if you buy a larger quantity - called a QUANTITY DISCOUNT.
//-----------------------------------------------

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

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// (1) READ the program above and PREDICT what it will print for each of these:
//       1 hamburger ,  6 hamburgers , 12 hamburgers
//
// (2) After making your predictions, run the program and check your answers.
//
// (3) Check what it costs for 9 hamburgers and 10 hamburgers.
//     Explain why this is a problem.  Suggest how this can be fixed.
//
// (4) On an IB exam, the following grade boundaries are used:
//       70 or above = grade 7
//       60 to 69  =   grade 6
//       50 to 59  =   grade 5
//       40 to 49  =   grade 4
//       30 to 39  =   grade 3
//       20 to 29  =   grade 2
//        0 to 19  =   grade 1
//     Write a program that inputs the raw score (e.g. 65)
//     and outputs the corresponding grade (e.g. 6)
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
`,
    mice_loops: `// === Mice ============
//  JavaScript has loops, including for loops
//  like Java and Basic and other languages.
//  This example uses loops for a poem.

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
`,
    money_decisions: `// === Money ====================================
//
// This program converts money from EUROS into
// several other currencies. Then it uses IF commands
// to make some decisions.
// ==============================================

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
`,
    common_factors: `//==== Common Factors ===============================
// This program prints all the common factors of A and B.
// This would be useful for reducing fractions.
// ==================================================

A = 28
B = 42

output "Common factors of " , A , " and " , B

loop C from 1 to B
    if (A mod C = 0) AND (B mod C = 0) then
       output C
    end if
end loop
`,
    math_values: `// === Table of Values for Math Function ===
// This program uses a simple loop to produce a table
// of x,y values for a math function.
// Notice that the LOOP FOR command can only count by 1,
// so the program divides by 2 to count by 0.5 each time.
// Dividing by 2 only works if you divide by 2.0,
// because C / 2 would do integer division, ignoring
// the fractional result.

output "X , Y"

loop C from 0 to 10
   X = C / 2.0
   Y = 3*X*X - 7*X + 2
   output X , " , " , Y
end loop
`,
    collatz_sequence: `//=== Collatz Sequence ==========================
//  Start with any integer.
//  If it's even, divide by 2.
//  If it's odd, multiply by 3 and add 1.
//  This produces a sequence of numbers that can continue
//  quite a long time.  If it ever hits a power of 2,
//  it falls back down to 1 and ends in a loop 4,2,1,4,2,1
//  It is believed that every number eventually returns to 1,
//  but this has not been proven.
// ==============================================

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
`,
    translate_strings: `// === Translate ================================
//
// This program knows a few German words.  
// If you choose an English word from the list, 
// it will tell you the matching German word.
//
// It knows the following words in German:
//  hello  goodbye  stop
// ==============================================

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
`,
    elapsed_minutes: `//=== Elapsed Minutes ===========================
// This program inputs a STARTING time and a FINISH time.
// Then it calculates the number of elapsed minutes
// between those two times.  For example:
//    START_HOURS = 8
//  START_MINUTES = 30
//     END_HOURS  = 10
//   END_MINUTES  = 10   
//  MINUTES = (10-8)*60 + (10-30) = 100
// ==============================================

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
`,
    date_validation: `//==== Date Validation ==========================
// This code inputs a date, using numbers for months,
// and decides whether the date is VALID or not.
// ==============================================

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
`,
    add_up_number: `//==== Adding Up Numbers =========================
// This program adds up a lot of numbers :
//  1 + 2 + 3 + ... + max
// ===============================================

MAX = 10

SUM = 0

loop COUNT from 0 to MAX
    output COUNT
    SUM = SUM + COUNT
end loop

output "Total = " , SUM
`,
    primes_array: `// ==== Arrays ==================================
//
// Arrays work just like Java and C++,
// except that they can contain any data type.
// This program uses the Sieve of Eratosthenes
// to produce a list of PRIMES below 100.
// ==============================================

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
`,
    binary_search: `//==== Binary Search ============================
// This program uses a Binary Search algorithm to search
// for an ID number, and then prints the corresponding name.
// Notice that div does not work properly in this tool,
// so Math.floor is used to truncate the average of LOW and HIGH.
// ==============================================

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
`,
    selection_sort: `//==== Selection Sort ===========================
// This program demonstrates a 
// standard Selection Sort algorithm.
// ==============================================

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
`,
    bubble_sort: `//==== Bubble Sort ==============================
// This demonstrates a simple, inefficient Bubble Sort.
// ==============================================

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
`,
    reverse_array: `//==== Reverse Array ============================
// This program is supposed to reverse the order
// of the names in the array.  Trace the program
// and predict what it will print after the
// loop executes.
// ==============================================

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
`,
    frequency_distribution: `//==== Frequency Distribution ====================
// This program examines the values stored in an array.
// It counts the values in each range : 0..9, 10..19, ...
// ===============================================

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
`,
    appointments_list: `//==== Appointment List =========================
// This program maintains a list of daily appointments in
// an array. The secretary types the name of a student and then the time,
// like this :  1215 for the time 12:15, or 850 for the time 8:50.
// After each entry, the program prints a list of all appointments.
// ==============================================

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
`,
    find_duplicates: `//==== Checking Lists for Duplicates ====================
// This program compares two lists, stored in Arrays,
// checking for duplicate names that appear in both lists.
// ====================================================== 

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
`,
    cities_array: `//==== Cities Array =============================
// This program contains an array with a list of city names.
// It counts and prints all the names that start with "D".
// ==============================================

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
`,
    names_collection: `//==== Cities Collection =========================
// This shows how to add names of students into 
// a Collection, and then search the collection and
// print all the student names that begin with "D".
// It includes a METHOD that returns 
// the first letter of a String.
// ===============================================

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
`,
    checkout_collection: `//==== Name List Collection =====================
// This program inputs NAMES of students who are leaving
// school early - for example to visit the doctor.
// The names are collected in a Collection list.  
// When a student returns, tying the same name again
// removes that name from the list.  At the end of the day,
// the secretary types "quit" to end the program and
// see a list of all students who left but did not return.
// ==============================================

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
`,
    stack_reverse_list: `//==== Stack Reverse List ============================
// This program uses a Stack to reverse the names
// that are stored in an array. This is admittedly
// inefficient, but does demonstrate how a stack
// can be used.
// ===================================================

NAMES = ["Alex","Bobby","Cho","Deke"]

STACK = new Stack()

loop COUNT from 0 to 3
   STACK.push(NAMES[COUNT])
end loop

loop while NOT(STACK.isEmpty())
   NAME = STACK.pop()
   output NAME
end loop
`,
    queues_merge: `//==== Queues Merge ============================
// This algorithm uses 2 queues to MERGE the contents
// of two lists - originally stored in arrays.
// This assigns dogs to people - to go for a walk.
// This is not the most efficient solution,
// but does demonstrate how to use Queues.
// =============================================

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
`,
    bank_classes: `//=================================================
// Bank accounts growing by
// (1) simple payments
// (2) percent interest
//
// This uses a CLASS to create OBJECTS
// to represent each of the two accounts.
//
// Try increasing the number of years in the loop
// to find out when the INTEREST account
// overtakes the PAYMENT account
//=================================================

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
`,
    magic_square: `A = [
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
`,
};