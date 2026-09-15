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

/*use core::num;
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

    if ku_01.len() %2 == 0{//这是检查代码的完整性的，在算式中现在只有奇数是成立的
        return Err("表达式错误".to_string());
    }

    let mut shuzi:Vec<f64> = Vec::new();
    let mut op:Vec<&str> = Vec::new();

    for (zhishu,ku_02) in ku_01.iter().enumerate() {//开始遍历，iter会遍历每一个元素，enumerate会同时给我们 位置+数据
        if zhishu %2 == 0 {//这是判断数字位的，偶数都是数字位
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
    }//判断优先级

    shuzi[zhishu] =  result;
    shuzi.remove(zhishu + 1);
    op.remove(zhishu);//开始修改整合
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
}*///第五步

use std::io;

fn main() {
    println!("rust计算器");
    println!("q退出");

    loop {
        println!("输入表达式:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("失败");

        let input =  input.trim();

        if input == "q" {
            println!("计算器退出");
            break;
        }

        match jisuan(input) {
            Ok(result) => {
                println!("结果: {result}");
            }

            Err(e) => {
                println!("错误: {e}");
            }
        }
    }
}

fn jisuan(input:&str) -> Result<f64 ,String> {
    let mut ku_1:Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    while let Some(you_kuohao) = ku_1.iter().position(|x| x == ")") {
        let zuo_kuohao = match ku_1[..you_kuohao].iter().rposition(|x| x == "(") {
            Some(kuohao) => kuohao,
            None => {
                return Err("括号不匹配".to_string());
            }
        };

        let yangcong_1 = ku_1[zuo_kuohao + 1..you_kuohao].join(" ");

        let yangcong_result = jisuan(&yangcong_1)?;//开始递归调用自己

        ku_1.splice(zuo_kuohao..=you_kuohao, [yangcong_result.to_string()]);//把原来括号里面的字符串替换成算好的数字字符串
        println!("括号计算：{:?}",ku_1);
    }

    if ku_1.iter().any(|x| x == "(") {
        return Err("括号不匹配".to_string());
    }

    println!("最终 {:?}",ku_1);
    jisuan_basic(&ku_1)
}

fn jisuan_basic(ku_1:&[String]) -> Result<f64, String> {
    if ku_1.len() < 3 {
        return Err("错误的格式".to_string());
    }

    if ku_1.len() %2 == 0 {
        return Err("表达式错误".to_string());
    }

    let mut numbers:Vec<f64> = Vec::new();
    let mut op:Vec<&str> = Vec::new();

    for(zhishu, ku_linshi) in ku_1.iter().enumerate() {
        if zhishu %2 == 0 {
            match ku_linshi.parse() {
                Ok(number) => numbers.push(number),

                Err(_) => {
                    return Err(format!("{} 不是数字",ku_linshi));
                }
            }
        }else {
            op.push(ku_linshi);
        }
    }

    let mut zhishu = 0;

    while zhishu < op.len() {
        if op[zhishu] == "^" {
            let result = numbers[zhishu].powf(numbers[zhishu + 1]);

            if result.is_nan() {//nan = is nat a number!
                return Err("结果不是有效数字".to_string());
            }

            numbers[zhishu] = result;

            numbers.remove(zhishu + 1);

            op.remove(zhishu);
        }else {
            zhishu += 1;
        }
    }

    zhishu =  0;

    while zhishu < op.len() {
        if op[zhishu] == "*" || op[zhishu] == "/" {
            let result;

            if op[zhishu] == "*" {
                result = numbers[zhishu] * numbers[zhishu + 1];
            }else {
                if numbers[zhishu + 1] == 0.0 {
                    return Err("除数不为0".to_string());
                }

                result = numbers[zhishu] / numbers[zhishu + 1];
            }

            numbers[zhishu] = result;

            numbers.remove(zhishu + 1);

            op.remove(zhishu);
        }else {
            zhishu += 1;
        }
    }

    let mut result = numbers[0];

    for zhishu in 0..op.len() {
        match op[zhishu] {
            "+" => {
                result += numbers[zhishu + 1];
            }
            "-" => {
                result -= numbers[zhishu + 1];
            }

            _ => {
                return Err(format!("不支持的运算符{}",op[zhishu]));
            }
        }
    }

    Ok(result)
}