use dioxus::prelude::*;
use rand::seq::SliceRandom;
use std::cell::RefMut;

const N: usize = 4;
const SIZE: usize = N * N;
const DIR: [[i32; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let cell_style = r#"
        width: 100px;
        height: 100px;
        line-height: 100px;
        text-align: center;
        user-select: none;
        cursor: pointer;
        border-radius: 15px;
        background-color:  #a3ecea;
        box-shadow: -3px -3px 10px #81bcba,
            3px 3px 10px #affefc;
    "#;
    let blank_style = r#"
        background-color: #e0e0e0;
        box-shadow: none;
    "#;

    let random_array = random_array(SIZE);
    let arrays = use_ref(&cx, || random_array);

    let mut index = 0;
    let binding = arrays.read();

    let cells = binding.iter().map(|e: &i32| {
        let i = index / N;
        let j = index % N;
        let number = *e;
        index += 1;
        if number == -1 {
            rsx! {
                div {
                    style: "{cell_style} {blank_style}",
                    ""
                }
            }
        } else {
            rsx! {
                div {
                    onclick: move |_| {
                        let mut array: RefMut<'_, Vec<i32>> = arrays.write();
                        for k in 0..4 {
                            let new_x: i32 = i as i32 + DIR[k][0];
                            let new_y: i32 = j as i32 + DIR[k][1];
                            if in_area(new_x, new_y) {
                                let new_x: usize = new_x as usize;
                                let new_y: usize = new_y as usize;
                                let d_index: usize = new_x * N + new_y;
                                if array[d_index] == -1 {
                                    let index: usize = i * N + j;
                                    array.swap(index, d_index);
                                    if check_win(&array) {
                                        println!("success!!!");
                                    }
                                    break;
                                }
                            }
                        }
                    },
                    style: "{cell_style}",
                    "{number}"
                }
            }
        }
    });

    let width = N * 100 + 50;
    let height = N * 100 + 50;
    let container_style = format!(r#"
        width: {}px;
        height: {}px;
        margin: 50px auto;
        border-radius: 15px;
        display: flex;
        flex-wrap: wrap;
        align-content: space-around;
        justify-content: space-around;
        box-shadow: -20px 20px 60px #bebebe,
            20px -20px 60px #ffffff;
        "#, width, height
    );
    cx.render(rsx! {
        div {
            style: "{container_style}",
            cells
        }
    })
}

fn check_win(array: &RefMut<'_, Vec<i32>>) -> bool {
    let length = array.len() - 1;
    let mut flag = true;
    for i in 0..length {
        if array[i] != i as i32 {
            flag = false;
            break;
        }
    }
    flag && array[length] == -1
}

fn in_area(x: i32, y: i32) -> bool {
    let length = N as i32;
    (x >= 0 && x < length) && (y >= 0 && y < length)
}

fn random_array(n: usize) -> Vec<i32> {
    let length: usize = n - 1;
    let mut array: Vec<i32> = Vec::new();
    for i in 0..length {
        array.push(i as i32);
    }
    array.shuffle(&mut rand::thread_rng());
    array.push(-1);

    array
}

