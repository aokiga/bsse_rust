fn find_max<I: Ord + Copy>(array: &[I; 10]) -> I {
    let ans = array.iter().reduce(|acc, x| acc.max(x)).unwrap();
    *ans
}

fn run_max_examples() {
    println!("Testing the implementation of calculating the maximum value in array.");

    fn test_max_example(arr: &[i32; 10], ans: i32) {
        let res = find_max(arr);
        assert_eq!(res, ans);
        println!("Max value of array [{}]: {}", arr.map(|x| x.to_string()).join(", "), ans)
    }

    let arr1 = [1, 2, 3, 4, 5, 10, 7, 6, 8, 9];
    let arr2 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let arr3 = [-1, 2, -3, 4, -5, -10, -7, 6, -8, -9];

    test_max_example(&arr1, 10);
    test_max_example(&arr2, 0);
    test_max_example(&arr3, 6);
}

fn is_prime(n: u32) -> bool {
    if n == 1 {
        return false;
    }
    let l = 2;
    let r = (n as f32).sqrt().floor() as u32 + 1;
    (l..r).fold(true, |acc, i| (acc && (n % i != 0)))
}

fn get_nth_prime(k: u32) -> u32 {
    let mut cnt = 0;
    (2..).find(|&x| {
        if is_prime(x) {
            cnt += 1;
        }
        cnt == k
    }).unwrap()
}

fn run_prime_examples() {
    println!("Testing the implementation of calculating the nth prime number.");

    fn test_prime_example(key: u32, ans: u32) {
        let res = get_nth_prime(key);
        assert_eq!(res, ans);
        println!("Prime №{}: {}", key, ans)
    }

    test_prime_example(1, 2);
    test_prime_example(2, 3);
    test_prime_example(3, 5);
    test_prime_example(4, 7);
    test_prime_example(5, 11);
    test_prime_example(7, 17);
    test_prime_example(10000, 104729);
}

fn bin_search_position<I: Ord>(a: &[I; 10], key: I) -> Option<usize> {
    let mut l = 0;
    let mut r = 10;
    while r - l > 1 {
        let m = (l + r) / 2;
        if a[m] <= key {
            l = m;
        } else {
            r = m;
        }
    }
    if a[l] == key {
        Some(l)
    } else {
        None
    }
}


fn run_bin_search_examples() {
    let arr = [1, 2, 3, 4, 5, 6, 8, 9, 10, 11];

    println!("Testing the implementation of binary search on array: [{}]", arr.map(|x| x.to_string()).join(", "));

    fn test_bin_search_example(arr: &[i32; 10], key: i32, ans: Option<usize>) {
        let res = bin_search_position(&arr, key);
        assert_eq!(res, ans);
        println!("Key {}: {}", key, match ans {
            Some(x) => ["Found on pos ".to_string(), x.to_string()].concat(),
            None => "Not found".to_string(),
        })
    }

    test_bin_search_example(&arr, 5, Some(4));
    test_bin_search_example(&arr, 7, None);
    test_bin_search_example(&arr, 9, Some(7));
    test_bin_search_example(&arr, 13, None);
    test_bin_search_example(&arr, 1, Some(0));
    test_bin_search_example(&arr, -2, None);
}

fn main() {
    run_max_examples();
    println!();
    run_prime_examples();
    println!();
    run_bin_search_examples();
}