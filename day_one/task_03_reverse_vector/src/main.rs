fn reverse_vector<T> (vec: &mut Vec<T>){
    let mut left: usize = 0;
    let mut right = vec.len().checked_sub(1).unwrap_or(0);

    while left < right{
        vec.swap(left, right);
        left = left.checked_add(1).unwrap_or(0);
        right = right.checked_sub(1).unwrap_or(0);
    }
}

fn main() {
    println!("Hello, world!");
    let mut my_vector = vec![2,4,6,8,10,12,14,16];
        println!("{:?}", &my_vector);
        reverse_vector(&mut my_vector);
        println!("{:?}", &my_vector);
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn odd_length_vector(){
        let mut my_vector = vec![1,3,5,7,9];
        reverse_vector(&mut my_vector);
        assert_eq!(my_vector, vec![9,7,5,3,1]);
    }

    #[test]
    fn even_length_vector(){
        let mut my_vector = vec![2,4,6,8,10];
        reverse_vector(&mut my_vector);
        assert_eq!(my_vector, vec![10,8,6,4,2])
    }

    #[test]
    fn empty_vector(){
        let mut my_vector = Vec::<i32>::new();
        reverse_vector(&mut my_vector);
        assert_eq!(my_vector, Vec::<i32>::new());
    }
}