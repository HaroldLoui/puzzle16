use rand::seq::SliceRandom;
use std::fmt::Debug;
use std::marker::Copy;
use std::io::stdin;
use std::process::exit;

const N: usize = 4;
const SIZE: usize = N * N;
const DIR: [[i32; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

fn main() {
    let array = random_array(SIZE);

    let mut board: [[i32; N]; N] = [[0; N]; N];
    for i in 0..N {
        for j in 0..N {
            let index = i * N + j;
            board[i][j] = if index == SIZE - 1 { -1 } else { array[index] as i32 };
        }
    }

    start(&mut board);
}

fn start<T, const N1: usize, const M1: usize>(array: &mut [[T; N1]; M1]) 
where T: Copy + Debug + PartialEq<i32>
{
    let mut is_win = false;
    while !is_win {
        print_array(array);
        println!("请输入坐标(x, y)：");
        let mut read_line = String::new();
        let _b = stdin().read_line(&mut read_line).unwrap();
        if "quit".to_string().eq(read_line.trim()) {
            println!("程序退出");
            exit(0);
        }
        let a: Vec<&str> = read_line.split(",").collect();
        if a.len() != 2 {
            println!("输入格式有误");
            continue;
        }
        let x: i32 = match a[0].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("坐标只能为数字");
                continue;
            }
        };
        let y: i32 = match a[1].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("坐标只能为数字");
                continue;
            }
        };
        if in_area(x, y) {
            for i in 0..4 {
                let new_x = x + DIR[i][0];
                let new_y = y + DIR[i][1];
                if in_area(new_x, new_y) {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    if array[new_x][new_y] == -1 {
                        swap_elements(array, (x as usize, y as usize), (new_x, new_y));
                        is_win = check_win(array);
                    }
                }
            }
        }
    }
    print_array(array);
    println!("游戏胜利！");
}

fn check_win<T, const N1: usize, const M1: usize>(array: &mut [[T; N1]; M1]) -> bool
where T: Copy + Debug + PartialEq<i32>
{
    let length = SIZE - 1;
    let mut flag = true;
    for i in 0..length {
        let x = i / N;
        let y = i % N;
        if array[x][y] != i as i32 {
            flag = false;
            break;
        }
    }
    flag && array[N - 1][N - 1] == -1
}

fn in_area(x: i32, y: i32) -> bool {
    let length = N as i32;
    (x >= 0 && x < length) && (y >= 0 && y < length)
}

fn random_array(n: usize) -> Vec<usize> {
    let length: usize = n - 1;
    let mut array: Vec<usize> = Vec::new();
    for i in 0..length {
        array.push(i);
    }
    array.shuffle(&mut rand::thread_rng());

    array
}

fn swap_elements<T, const N1: usize, const M1: usize>(
    array: &mut [[T; N1]; M1],
    index1: (usize, usize),
    index2: (usize, usize),
) where
    T: Copy,
{
    let (x1, y1) = index1;
    let (x2, y2) = index2;

    let temp = array[x1][y1];
    array[x1][y1] = array[x2][y2];
    array[x2][y2] = temp;
}

fn print_array<T, const N1: usize, const M1: usize>(array: &mut [[T; N1]; M1])
where
    T: Debug + PartialEq<i32>
{
    println!();
    for row in array {
        for t in row {
            if *t == -1 {
                print!(" \t");
            } else {
                print!("{:?}\t", t);
            }
        }
        println!();
    }
}
