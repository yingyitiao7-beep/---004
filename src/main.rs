/*use std::io;

fn main() {
    println!("==rust计算器==");
    println!("支持加减乘除");
    println!("按q退出");

    loop {
        println!("\n输入表达式：");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("读取失败");

        let input = input.trim();

        if input == "q" {
            println!("退出");
            break;
        }

    let ku_01:Vec<&str> = input.split_whitespace().collect();//根据空格分割字符串，然后在打包收集

    if ku_01.len() != 3{
        println!("输入格式错误");
        continue;
    }

    let shuzi_1:f64 = match ku_01[0].parse() {
        Ok(number) => number,
        Err(_) => {
            println!("第一个字错误");
            continue;
        }
    };

    let op = ku_01[1];

    let shuzi_2:f64 = match ku_01[2].parse() {
        Ok(number) => number,
        Err(_) => {
            println!("第二个字错误");
            continue;
        }
    };

    let faze = match op {
        "+" => shuzi_1 + shuzi_2,
        "-" => shuzi_1 - shuzi_2,
        "*" => shuzi_1 * shuzi_2,
        "/" => {
            if shuzi_2 == 0.0 {
                println!("除数不能为零");
                continue;
            }
            shuzi_1 / shuzi_2
        }
        _ => {
            println!("不支持的运算符");
            continue;
        }
    };
    println!("结果: {}", faze);

    }
}*///第三版

/*use std::io;

fn main() {
    println!("==rust计算器==");
    println!("按q退出");

    loop {
        println!("输入表达式:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取失败");

        let input = input.trim();

        if input == "q" {
            println!("退出");
            break;
        }

        let ku_01: Vec<&str> = input.split_whitespace().collect();

        if ku_01.len() != 3 {
            println!("输入格式错误");
            continue;
        }

        let mut result = match ku_01[0].parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("第一个数字错误");
                continue;
            }
        };

        let mut weizhi = 1;

        while weizhi < ku_01.len() {
            let op = ku_01[weizhi];

            let shuzi:f64 = match ku_01[weizhi + 1].parse() {
                Ok(number) => number,

                Err(_) => {
                    println!("数字错误");
                    break;
                }
            };

            match op {
                "+" => {
                    result += shuzi;
                }
                "-" => {
                    result -= shuzi;
                }
                "*" => {
                    result *= shuzi;
                }
                "/" => {
                    if shuzi == 0.0 {
                        println!("除数不为0");
                        break;
                    }

                    result /= shuzi;
                }
                _ => {
                    print!("现不支持");
                    break;
                }
            }

            weizhi += 2;
        }

        print!("结果：{result}")
    }
}*///第四步

use core::num;
use std::io;

fn main() {
    println!("==rust计算器==");
    println!("按q退出");

    loop {
        println!("请输入");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("读取失败");

        let input = input.trim();

        if input == "q" {
            println!("退出");
            break;
        }

        match jisuanqi(input) {
            Ok(result) => {
                println!("结果：{result}")
            }
            Err(e) => {
                print!("错误:{e}")
            }
        }
    }
}

fn jisuanqi(input:&str) -> Result<f64, String> {
    let ku_01:Vec<&str> = input.split_whitespace().collect();

    if ku_01.len() < 3{
        return Err("错误".to_string());
    }

    if ku_01.len() %2 == 0{
        return Err("表达式错误".to_string());
    }

    let mut shuzi:Vec<f64> = Vec::new();
    let mut op:Vec<&str> = Vec::new();

    for (zhishu,ku_02) in ku_01.iter().enumerate() {
        if zhishu %2 == 0 {
            match ku_02.parse() {
                Ok(number) => shuzi.push(number),
                Err(_) => {
                    return Err(format!("{}不是数字",ku_02));
                }
            }
        } else {
            op.push(ku_02);
        }
    }

    let mut zhishu = 0;

    while zhishu < op.len() {
        if op[zhishu] == "*" || op[zhishu] == "/"{
            let result;

            if op[zhishu] == "*"{
                result = shuzi[zhishu] * shuzi[zhishu + 1];
    } else {
        if shuzi[zhishu + 1] == 0.0{
            return Err("除数不为0".to_string());
        }



        result = shuzi[zhishu] / shuzi[zhishu + 1];
    }

    shuzi[zhishu] =  result;
    shuzi.remove(zhishu + 1);
    op.remove(zhishu);
    } else {
        zhishu += 1;
    }
}

    let mut result = shuzi[0];

    for zhishu in 0..op.len() {
        match op[zhishu] {
            "+" => {
                result += shuzi[zhishu + 1];
            }
            "-" => {
                result -= shuzi[zhishu + 1]
            }
            _ => {
                return Err("不支持运算符".to_string());
            }
        }
    }

    Ok(result)
}