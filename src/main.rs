#![windows_subsystem = "windows"]

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
    let is_win = use_state(&cx, || false);

    let arrays = use_ref(&cx, || random_array(SIZE));

    let mut index = 0;
    let binding = arrays.read();

    let cells = binding.iter().map(|e: &i32| {
        let i = index / N;
        let j = index % N;
        let number = *e;
        index += 1;
        if number == -1 {
            rsx! { div { class: "cell blank", "" } }
        } else {
            rsx! {
                div {
                    onclick: move |_| {
                        if *is_win.get() {
                            return;
                        }
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
                                        is_win.set(true);
                                    }
                                    break;
                                }
                            }
                        }
                    },
                    class: "cell",
                    "{number}"
                }
            }
        }
    });

    let width = N * 100 + 50;
    let height = N * 100 + 50;
    let container_style = format!("width: {}px; height: {}px;", width, height);
    cx.render(rsx! {
        // link {
        //     rel: "stylesheet",
        //     href: "/src/assets/app.css"
        // }
        style { include_str!("./assets/app.css") }
        div {
            class: "btn",
            onclick: move |_| {
                let mut array: RefMut<'_, Vec<i32>> = arrays.write();
                let new_array = random_array(SIZE);
                array.clone_from(&new_array);

                is_win.set(false);
            },
            "reset"
        }
        div {
            class: "container",
            style: "{container_style}",
            cells
        }
        p {
            style: "text-align: center;",
            if *is_win.get() { "success!!!" } else { "" }
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
