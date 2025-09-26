use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"
Class Account(name,amount)
    this.id = name + amount
    this.balance = amount

    this.addInterest = function(percent)
    {
       this.balance = this.balance + this.balance*percent/100
    }

    this.addMoney = function(money)
    {
       this.balance = this.balance + money
       return this.balance*percent/100
    }

    this.show = function()
    {
       output this.id + "  " + this.balance
    }
end Class

PAYMENTS = new Account("Abbey",100.0)
output PAYMENTS
    "#;

    compile_and_run(code);
}