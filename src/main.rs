use rand::{Rng};
use std::{thread, time::Instant};
use sorts::insertion_sort;

fn main() {
    const SIZE: u32 = 10_000;

    //collections to sort.
    //println!("{:?}", random_vec);
    //Bubble sort
    //must explicity say is going to change when passing as arg O_o

    let handle_bubble = thread::spawn(|| {
        let mut random_vec: Vec<u32> = get_vec(SIZE, SIZE);//size, max value
        let now = Instant::now();
        bubble(&mut random_vec);
        println!("Bubble: {:.2?}", now.elapsed());
        //println!("{:?}", random_vec);
    });
    

    let handle_selection = thread::spawn(|| {
        let mut random_vec: Vec<u32> = get_vec(SIZE, SIZE);//size, max value
        //println!("{:?}", random_vec);
        let now = Instant::now();
        selection(&mut random_vec);
        println!("Selection: {:.2?}", now.elapsed());
        //println!("{:?}", random_vec);
    });

    let handle_insertion = thread::spawn(|| {
        let mut random_vec: Vec<u32> = get_vec(SIZE, SIZE);//size, max value
        //println!("{:?}", random_vec);
        let now = Instant::now();
        insertion(&mut random_vec);
        println!("Insertion: {:.2?}", now.elapsed());
        //println!("{:?}", random_vec);
    });

    /*
     * This is the sorts crate insertion. It leans on swap(), and performs a swap
     * inside the inner while loop. It was thought to be slower than below, 
     * and it is correct. 
     */
    let handle_insertion_import = thread::spawn(|| {
        let mut random_vec: Vec<u32> = get_vec(SIZE, SIZE);//size, max value
        //println!("{:?}", random_vec);
        let now = Instant::now();
        insertion_sort(&mut random_vec);
        println!("Insertion (imported): {:.2?}", now.elapsed());
        //println!("{:?}", random_vec);
    });

    //keep children running until main thread quits
    handle_bubble.join().unwrap();
    handle_selection.join().unwrap();
    handle_insertion.join().unwrap();
    handle_insertion_import.join().unwrap();

}

//receive a param called vec, that is a reference to a vector that is mutable
//also must say this will change when we pass to swap fn
//also, no return, gets access through the mutable ref.
fn bubble(vec: &mut Vec<u32>) {
    
    for _i in 0..vec.len() {//not inclusive range.
        for j in 1..(vec.len())  {
            if vec[j - 1] > vec[j] {
                swap(vec, j, j - 1);
            }
        }
    }

}

fn selection(vec: &mut Vec<u32>) {
    for i in 0..vec.len() {
        let mut min: usize = i;

        for j in (i + 1)..vec.len() {
            if vec[j] < vec[min] {
                min = j;
            }
        }

        swap(vec, i, min);
    }
}

//insertion sort. shifts things right then does the swap with curr.
//sorts::insertion_sort does a swap inside the loop. 
fn insertion(vec: &mut Vec<u32>) {
    for i in 1..vec.len() {
        let curr: u32 = vec[i];
        let mut j: usize = i; //usize because the vector uses usize for indexing

        while j != 0 && vec[j - 1] > curr {
            vec[j] = vec[j - 1]; //shift things right.
            
            j -= 1;//this is an about to be an underflow, but Rust wont even allow it to get to -1
        }

        vec[j] = curr;
    }
}

//give back a mutable vec.
//does this give ownership to main()?
fn get_vec(size: u32, max: u32) -> Vec<u32>{
    let mut rng = rand::thread_rng();
    let mut random_vec: Vec<u32> = Vec::new();

    for _i in 0..size {
        random_vec.push(rng.gen_range(0..=max));
    }

    return random_vec
}

//swapper
fn swap(vec: &mut Vec<u32>, l: usize, r: usize) {
    let temp = vec[l];
    vec[l] = vec[r];
    vec[r] = temp;
}

