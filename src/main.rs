fn main() {
    let vec_a:Vec<i32> = vec![1,2,2,2,3];
    let vec_b:Vec<i32> = vec![2]; 
    let mut final_vec: Vec<> = vec![];

    for element in vec_a{
        if vec_b.contains(&element){
            continue;
        } 
        else{
            final_vec.push(element);
        }
    }
    for el in final_vec{
        dbg![el];
    }
}
