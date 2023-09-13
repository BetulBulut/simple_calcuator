use scanf::scanf;

fn main() {
    println!("Please enter the operation you want like this:");
    println!("Number operaiton Number: 4 + 4, 4 * 5, 4 / 6, 4 - 7");


    println!("enter any letter for quit");
    while(true){
        let mut i: String = String::new();
        let mut s: f64 = 0.0;
        let mut j: f64 = 0.0;
        
        if scanf!("{} {} {}",s,i,j).is_ok() {
           
           if i == String::from("+"){
            let op = Operation::Add(s, j);
            getResult(&op.calculate());
           }else if i == String::from("*") {
            let op = Operation::Multiply(s, j);
            getResult(&op.calculate());
           }else if i == String::from("-") {
            let op = Operation::Subtract(s, j);
            getResult(&op.calculate());
           }else if i == String::from("/") {
            let op = Operation::Divide(s, j);
            getResult(&op.calculate());
           }

            
           
        }else{
            break;
        }
    }
    

    
    //match ile ayrıştır operation oluştur
    //call calculator
}
pub fn getResult(res: &Result<f64,String>){

    match res{
    Ok(value) => println!("result is: {}",value),
    Err(error_message) => println!("Error: {}", error_message),
    }
}
enum Operation{
    Add(f64,f64),
    Subtract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64),
}

impl Operation {
    fn calculate(&self) -> Result<f64,String> {
        match self {
            Operation::Add(a,b) => Ok(a+b),
            Operation::Multiply(a,b) => Ok(a*b),
            Operation::Subtract(a,b) => Ok(a-b),
            Operation::Divide(a,b) => Ok(a/b),
        }
    }
}

