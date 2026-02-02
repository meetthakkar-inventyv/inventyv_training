pub fn run() {
    
    //print even numbers from 1 to 10(exclusive) but don't include 8 so print 2,4,6 but from 1 to 10
    
    let n = 10;

    println!("Using loop:");
    let mut i = 0;
    loop {
        i += 1;

        if i >= n {
            break;
        }

        if i % 2 != 0 {
            continue;
        }

        if i == 8 {
            break;
        }

        println!("{i}");
    }

    println!("\nUsing while:");
    let mut j = 0;
    while j < n {
        j += 1;

        if j % 2 != 0 {
            continue;
        }

        if j == 8 {
            break;
        }

        println!("{j}");
    }

    println!("\nUsing for:");
    for k in 1..n {
        if k % 2 != 0 {
            continue;
        }

        if k == 8 {
            break;
        }

        println!("{k}");
    }
}


