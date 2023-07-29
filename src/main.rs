fn main() {
    let dat = vec![
        vec!["Den","11","10","12"],
        vec!["Kate","8","4","9"],
        vec!["John","6","5","2"],
    ];

    for mut student in dat {
        print!("For {} :",student[0]);
        while let Some(value)=student.pop() {
            if let Ok(result)=value.parse::<i32>(){
                print!(" {} ",result);
            }else {
                println!(" ");
            }
        }
    }
}

fn counter_func(){
    let mut counter = Some(0);

    while let Some(value) = counter {
        if value == 31 {
            println!("End!");
            counter = None;
        } else {
            println!("{}", value);
            counter = Some(value + 1);
        }
    }

    /*loop {
        match counter {
            None => break,
            Some(value) => {
                if value==31 {
                    println!("End!");
                    counter=None;
                }else {
                    println!("{}",value);
                    counter = Some(value+1);
                }
            }
        }
    }*/
}