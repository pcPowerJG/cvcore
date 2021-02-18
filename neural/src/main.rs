use neural::{
    Neuron,
    Layer,
    Sheme
};

//extern crate libc;
//use libc::size_t;
use std::ffi::CString;
use std::ptr;
use std::ffi::CStr;
use std::io;
use std::str::FromStr;

fn eval(str_: Vec<char>) -> f32 {
    let mut i: usize = 0;
    expr(str_, &mut i)
}

fn plus_one(u: &mut usize) {
    *u += 1;
}

fn number(ch_: Vec<char>, idx: &mut usize) -> f32 {
    let mut result: f32 = 0.0;
    //float result = 0.0;
    let mut div: f32 = 10.0;
    let mut sign: f32 = 1.0;
    if ch_[*idx] == '-'{
        sign = -1.0;
        *idx += 1;
    }
    
    while *idx < ch_.len() &&
        match ch_[*idx] {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => { true },
            _ => { false }
        }
    {
        result = result * 10.0 + (f32::from_str(&ch_[*idx].to_string()).expect("не удалось форматировать строку"));
        
        *idx += 1;
    }
    
    if *idx < ch_.len() && (ch_[*idx] == '.'){
        *idx += 1;        
        while *idx < ch_.len() &&
            match ch_[*idx] {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => { true },
                _ => { false }
            } 
        {
            result = result + (f32::from_str(&ch_[*idx].to_string()).expect("не удалось форматировать строку")) / div;
            div *= 10.0;
            *idx += 1;
        }
    }
    sign * result
}

fn expr(ch_: Vec<char>, idx: &mut usize) -> f32 {
    let mut result: f32 = term(ch_.clone(), idx);    
    while *idx < ch_.len() && (ch_[*idx] == '+' || ch_[*idx] == '-') {
        match ch_[*idx] {
            '+' => {
                *idx += 1;
                result += term(ch_.clone(), idx);
            },
            '-' => {
                *idx += 1;    
                result -= term(ch_.clone(), idx);
            },
            _ => {},
        } 
    } result
}

fn term(ch_: Vec<char>, idx: &mut usize) -> f32 {
    let mut result: f32 = factor(ch_.clone(), idx);
    let mut div: f32 = 0.0;

    while *idx < ch_.len() && (ch_[*idx] == '*' || ch_[*idx] == '/') {
        match ch_[*idx] {
            '*' => {
                *idx += 1;
                result *= factor(ch_.clone(), idx);
            },
            '/' => {
                *idx += 1;    
                div = factor(ch_.clone(), idx);    
                if (div != 0.0) {
                    result /= div;
                } else {
                    panic!("Division by zero!\n");                    
                }
            },
            _ => {},
        }
    } result
}

fn factor(ch_: Vec<char>, idx: &mut usize) -> f32 {
    let mut result: f32 = 0.0;
    let mut sign: f32 = 1.0;

    if (ch_[*idx] == '-') {
        sign = -1.0;
        *idx += 1;
    }

    if (ch_[*idx] == '(') {
        *idx += 1;
        result = expr(ch_.clone(), idx);

        if (ch_[*idx] != ')') {
            panic!("Brackets unbalanced!\n");
        }
        *idx += 1;
    } else { result = number(ch_, idx); }
    /*if (ch_[*idx] == '^')
    {
        *idx += 1;

        result = pow(result, factor(ch_, idx));
    }*/
    sign * result
}


use std::net::TcpListener;
use std::io::{Read};
use std::net::TcpStream;

fn action(commands: String) -> String {
    /* "включи" | "запусти" | "посмотри" | 
                            "обнаружить" | "слежение" | "как" | "покажи" | "ответ" */
    let cmd: Vec<&str> = vec![
        "включи","запусти","посмотри","обнаружить","слежение","как","покажи","ответ"
    ];
    let mut i: usize = 0;
    let text: Vec<&str> = commands.split(' ').collect::<Vec<&str>>();
    let mut c: usize = 0;
    let mut find: bool = false;
    let mut result_: String = String::new();
    while i < text.len().clone() {
        if !find {
            let mut k: usize = 0;
            for key_word in text.clone() {
                if key_word == text[i].clone() {
                    find = true; c = k.clone();
                    i = 0; continue;
                }
                k += 1;
            }
        } else {
            match c {
                0 | 1 => {
                    match text[i] {
                        "браузер" => result_ += "browser_run ",
                        "музыку" => result_ += "music ",
                        _ => { /* --- */ },
                    }
                },
                7 => {  
                    match text[i] {
                        "ноль" => result_ += "0",
                        "один" => result_ += "1",
                        "два" => result_ += "2",
                        "три" => result_ += "3",
                        "четыре" => result_ += "4",
                        "пять" => result_ += "5",
                        "шесть" => result_ += "6",
                        "семь" => result_ += "7",
                        "восемь" => result_ += "8",
                        "девять" => result_ += "9",
                        "умножить" => result_ += "*",
                        "умножение" => result_ += "*",
                        "плюс" => result_ += "+",
                        "минус" => result_ += "-",
                        "делить" => result_ += "/",
                        "открыть" => result_ += "(",
                        "закрыть" => result_ += ")",
                        "конец" | "выход" => {
							let mut _temp_var: Vec<char> = Vec::new();
							for ch in result_.chars() {
								_temp_var.push(ch.clone());
							}
                            let mut result: f32 = eval(_temp_var);
                            break;
                        },
                        _ => { /* --- */ },
                    }
                },
                _ => { /* --- */ },
            }
        }
        i += 1;
    }
    result_
}
fn main() {
    //Neuron::new(5).clone().print();
    // 127.0.0.1: 7097 действие
    let listener = TcpListener::bind("127.0.0.1:8010").unwrap();
    for mut stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("new client!");
                stream.set_nonblocking(true).expect("set_nonblocking call failed");

                let mut buf: Vec<u8> = vec![];
                match stream.read_to_end(&mut buf) {
                    Ok(_) => break,
                    Err(e) => panic!("encountered IO error: {}", e),
                };
                let mut text: String = String::from_utf8(buf)
                    .expect("fuck of comand from fucking alice you right?");
                println!("Принято: {}", text.clone());
                let mut commands: Vec<&str> = text.clone().split(' ').collect::<Vec<&str>>();
                for word in commands.clone() {
                    // search key word to command
                    match word {
                        "включи" | "запусти" | "посмотри" | 
                            "обнаружить" | "слежение" | "как" | "покажи" | "ответ" => {
                                action(text.clone());
                        },
                        _ => { /* none */ },
                    }
                }
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
