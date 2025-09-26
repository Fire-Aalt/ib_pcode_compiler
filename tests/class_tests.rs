use crate::common::compile_run_check_logs;

pub mod common;

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

