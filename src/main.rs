use rand::{Rng};


fn main() {
    //collections to sort.
    let mut random_vec: Vec<u32> = get_vec(1_000, 1_000);//size, max value

    println!("{:?}", random_vec);

    //Bubble sort
    //must explicity say is going to change when passing as arg O_o
    bubble(&mut random_vec);
    println!("{:?}", random_vec);

    let mut random_vec: Vec<u32> = get_vec(1_000, 1_000);//size, max value
    println!("{:?}", random_vec);
    selection(&mut random_vec);
    println!("{:?}", random_vec);
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

//TODO: insertion

//give back a mutable vec.
//does this give ownership to main()?
fn get_vec(size: usize, max: u32) -> Vec<u32>{
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

