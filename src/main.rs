fn main() {
    println!("Hello, world!");
    const NUMBER: u32 = 20;

    if NUMBER > 10 {
        println!("NUMBER is greater than 10")
    } else if NUMBER == 10 {
        println!("NUMBER is equal to 10")
    } else {
        println!("NUMBER is less than 10")
    }

    let mut counter = 0;

    let result = loop {
        counter = counter + 1;

        if counter == 10 {
            break counter * 5;
        }
    };

    println!("The count is {result}");

    looping();
    while_loop();
    loop_an_array()
}

fn looping() {
    let mut first_loop_count = 20;

    'first_loop: loop {
        println!("count = {first_loop_count}");

        let mut second_loop_count = 1;

        loop {
            println!("inner loop remaining = {second_loop_count}");

            if second_loop_count == 2 {
                break;
            }

            if first_loop_count == 0 {
                break 'first_loop;
            }

            second_loop_count = second_loop_count + 1;
        }

        first_loop_count = first_loop_count - 1;
    }
}

fn while_loop() {
    let mut count = 5;

    while count >= 0 {
        println!("{count} !");

        count = count - 1;
    }

    println!("Power Rangers! Operation Overdrive!!!");
}

fn loop_an_array() {
    let array = [20, 30, 40, 50];
    let mut index = 0;

    while index < 4 {
        println!("The current value is {}", array[index]);

        index = index + 1;
    }
}
