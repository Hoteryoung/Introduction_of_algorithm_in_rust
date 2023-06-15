// pub fn insertion_sort(vec: &mut Vec<i32>) {
//     let mut key: i32;
//     for i in 1..vec.len() {
//         key = vec[i];
//         let mut j = i - 1;
//         while j > 0 && vec[j] > key {
//             vec[j + 1] = vec[j];
//             j -= 1;
//         }
//         //usize can not be less than 0
//         if j == 0 && vec[j] > key {
//             vec[j + 1] = vec[j];
//             vec[j] = key;
//         }
//         else {
//             vec[j + 1] = key;
//         }
//     }
//     println!("sorted result: {:?}", vec);
// }

pub fn insertion_sort(vec: &mut Vec<i32>){
    for i in 1..vec.len() {
        let key = vec[i];
        let mut j = i;
        while j > 0 && vec[j-1] > key {
            vec[j] = vec[j - 1];
            j -= 1;
        }
        vec[j] = key;
    }
    println!("sorted result: {:?}", vec);
}

pub fn insertion_sort_demo() {
    let mut v = vec![3,4,2,6,8,1,5,7,2];
    insertion_sort(&mut v);
    assert_eq!(vec![1,2,2,3,4,5,6,7,8], v);
}

pub fn merge_two_n_bit_integer_2_1_4_demo() {
    let v1 = vec![0,2,5,1];
    let v2 = vec![3,0,8,6];
    let v3 = merge_two_n_bit_integer_2_1_4(v1, v2);
    assert_eq!(v3, [0, 3, 3, 3, 7]);
    let v4 = vec![8,2,5,1];
    let v5 = vec![3,0,8,6];
    let v6 = merge_two_n_bit_integer_2_1_4(v4, v5);
    assert_eq!(v6, [1, 1, 3, 3, 7]);
}

pub fn merge_two_n_bit_integer_2_1_4(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut v = Vec::<i32>::new();
    let mut sum = 0;
    let mut quotient = 0;
    for i in (0..v1.len()).rev() {
        sum = v1[i] + v2[i] + quotient;
        v.push(sum % 10);
        quotient = sum / 10;
    }
    v.push(quotient);
    v.reverse();
    println!("{:?}", v);
    v
}