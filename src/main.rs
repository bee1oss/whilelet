fn main() {
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
