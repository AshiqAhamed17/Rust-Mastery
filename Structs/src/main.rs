#[derive(Debug, PartialEq)]
enum Country {
    England,
    India,
    USA,
    Monaco,
    Italy,
    Switzerland
}

struct User {
    name: String,
    age: u32,
    salary: u32,
    country: Country
}

impl User {
    fn monthly_salary(&self) -> u32 {
        return self.salary / 12;
    }

    fn after_tax(&self) -> f32 {
        let salary = self.salary as f32;

        match self.country {

        Country::England => {
            if salary <= 50_000.0 {
                salary * 0.80
            } else if salary <= 100_000.0 {
                salary * 0.70
            } else {
                salary * 0.60
            }
        }
        Country::India => {
            if salary <= 50_000.0 {
                salary * 0.95
            } else if salary <= 100_000.0 {
                salary * 0.85
            } else {
                salary * 0.80
            }
        }
        Country::USA => {
            salary * 0.75
        }
        Country::Monaco => {
            salary
        }
        Country::Italy => {
            salary * 0.65
        }
        Country::Switzerland => {
            salary * 0.85
        }
    }

    
    }

    //static function
    fn info() {
        println!("This is a User struct which has name, age, salary along with some functions");
    }
}

fn main() {
   
    let user1 = User {
        name: String::from("ASH"),
        age: 20,
        salary: 10000000,
        country: Country::Monaco
    };

    println!("name: {}, age: {}", user1.name, user1.age);
    User::info();
    println!("Monthly salary : {}", user1.monthly_salary());
    println!("After tax : {}", user1.after_tax());
    println!("Country : {:?}", user1.country);

}
