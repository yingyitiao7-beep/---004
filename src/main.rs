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

/*use std::io;
use std::fmt;

#[derive(Debug)]
enum CalcError {
    InvalidFormat,
    ExpressionFormatError,
    MismatchedParenthese,
    NotNumber(String),//带数据的枚举变体
    DivisionByZero,
    InvalidPower,
    UnsupportedOperator(String),
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::InvalidFormat => {
                write!(f,"错误格式")
            }
            CalcError::ExpressionFormatError => {
                write!(f, "表达式格式错误")
            }
            CalcError::MismatchedParenthese => {
                write!(f, "括号不匹配")
            }
            CalcError::NotNumber(value) => {
                write!(f, "{}不是数字",value)
            }
            CalcError::DivisionByZero => {
                write!(f, "除数不为0")
            }
            CalcError::InvalidPower => {
                write!(f, "次方结果不是有效数字")
            }
            CalcError::UnsupportedOperator(op) => {
                write!(f, "不支持的运算符：{}",op)
            }
        }
    }
}

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

fn jisuan(input:&str) -> Result<f64 ,CalcError> {
    let mut ku_1:Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    while let Some(you_kuohao) = ku_1.iter().position(|x| x == ")") {
        let zuo_kuohao = match ku_1[..you_kuohao].iter().rposition(|x| x == "(") {
            Some(kuohao) => kuohao,
            None => {
                return Err(CalcError::MismatchedParenthese);
            }
        };

        let yangcong_1 = ku_1[zuo_kuohao + 1..you_kuohao].join(" ");

        let yangcong_result = jisuan(&yangcong_1)?;//开始递归调用自己

        ku_1.splice(zuo_kuohao..=you_kuohao, [yangcong_result.to_string()]);//把原来括号里面的字符串替换成算好的数字字符串
        println!("括号计算：{:?}",ku_1);
    }

    if ku_1.iter().any(|x| x == "(") {
        return Err(CalcError::MismatchedParenthese);
    }

    println!("最终 {:?}",ku_1);
    jisuan_basic(&ku_1)
}

fn jisuan_basic(ku_1:&[String]) -> Result<f64, CalcError> {
    if ku_1.len() < 3 {
        return Err(CalcError::InvalidFormat);
    }

    if ku_1.len() %2 == 0 {
        return Err(CalcError::ExpressionFormatError);
    }

    let mut numbers:Vec<f64> = Vec::new();
    let mut op:Vec<&str> = Vec::new();

    for(zhishu, ku_linshi) in ku_1.iter().enumerate() {
        if zhishu %2 == 0 {
            match ku_linshi.parse() {
                Ok(number) => numbers.push(number),

                Err(_) => {
                    return Err(CalcError::NotNumber(ku_linshi.clone()));
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
                return Err(CalcError::InvalidFormat);
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
                    return Err(CalcError::DivisionByZero);
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
                return Err(CalcError::UnsupportedOperator(op[zhishu].to_string()));
            }
        }
    }

    Ok(result)
}*///第六版 

use std::io;
use std::fmt;

#[derive(Debug)]
enum CalcError {
    InalidFormat,
    ExpressionFormatError,
    MismatchedParentheses,
    NotNumber(String),
    DivsionByZero,
    InvaildPower,
    UnsupporedOperator(String),
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::InalidFormat => {
                write!(f,"错误格式")
            }
            CalcError::ExpressionFormatError => {
                write!(f, "表达式格式错误")
            }
            CalcError::MismatchedParentheses => {
                write!(f, "括号不匹配")
            }
            CalcError::NotNumber(value) => {
                write!(f, "{}不是数字",value)
            }
            CalcError::DivsionByZero => {
                write!(f, "除数不为0")
            }
            CalcError::InvaildPower => {
                write!(f, "次方结果不是有效数字")
            }
            CalcError::UnsupporedOperator(op) => {
                write!(f, "不支持的运算符：{}",op)
            }
        }
    }
}

fn main() {
    println!("rust计算器");
    println!("输入q退出");

    loop {
        println!("请输入表达式：");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("读取失败");

        let input = input.trim();

        if input == "q" {
            println!("退出");
            break;
        }

        match calculate(input) {
            Ok(result) => {
                println!("结果: {}",result);
            }
            Err(e) => {
                println!("错误: {}",e);
            }
        }
    }
}

fn calculate(input: &str) -> Result<f64, CalcError> {

    let mut expression = input.trim().to_string();

    while let Some(left_index) = expression.rfind('(') {

        let after_left = expression.split_at(left_index).1;

        let after_left = after_left.split_at(1).1;

        let (inside, right_part) = match after_left.split_once(')') {
            Some(value) => value,

            None => {
                return Err(CalcError::MismatchedParentheses);
            }
        };

        let inside_result = calculate(inside)?;

        let left_part = expression.split_at(left_index).0;

        expression = format!("{}{}{}",left_part,inside_result,right_part);
    }

    if expression.contains(')') {
        return Err(CalcError::MismatchedParentheses);
    }

    calculate_basic(&expression)

}

fn calculate_basic(input: &str) -> Result<f64, CalcError> {

    let mut tokens = input.split_whitespace();

    let first = match tokens.next() {
        Some(value) => value,

        None => {
            return Err(CalcError::InalidFormat);
        }
    };

    let mut current_number = match first.parse::<f64>() {
        Ok(number) => number,

        Err(_) => {
            return Err(CalcError::NotNumber(first.to_string()));
        }
    };

    let mut expressions: Vec<(String, f64)> = Vec::new();

    loop {
        let op = match tokens.next() {
            Some(value) => value,

            None => {
                break;
            }
        };

        let number_text = match tokens.next() {
            Some(value) => value,

            None => {
                return Err(CalcError::ExpressionFormatError);
            }
        };

        let number = match number_text.parse::<f64>() {
            Ok(value) => value,

            Err(_) => {
                return Err(CalcError::NotNumber(number_text.to_string()));
            }
        };

        expressions.push((op.to_string(),number));
    }

    if expressions.is_empty() {
        return Ok(current_number);
    }

    let mut new_expressions: Vec<(String, f64)> = Vec::new();

    let mut expression_iter = expressions.into_iter();

    while let Some((op, number)) = expression_iter.next() {
        if op == "^" {
            current_number = current_number.powf(number);

            if current_number.is_nan() {
                return Err(CalcError::InvaildPower);
            }
        } else {
            new_expressions.push((op, number));
        }
    }

    let mut final_expressions: Vec<(String, f64)> = Vec::new();

    let mut expression_iter = new_expressions.into_iter();

    while let Some((op, number)) = expression_iter.next() {
        if op == "*" {
            current_number *= number;
        } else if op == "/" {
            if number == 0.0 {
                return Err(CalcError::DivsionByZero);
            }

            current_number /= number;
        } else {
            final_expressions.push((op, number));
        }
    }

    for (op, number) in final_expressions {
        match op.as_str() {
            "+" => {
                current_number += number;
            }

            "-" => {
                current_number -= number;
            }

            _ => {
                return Err(CalcError::UnsupporedOperator(op));
            }
        }
    }

    Ok(current_number)
}//第六版02

/*use std::fmt;
use std::io;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum CalcError {
    InvalidFormat,
    ExpressionFormatError,
    MismatchedParentheses,
    NotNumber(String),
    DivisionByZero,
    InvalidPower,
    UnsupportedOperator(String),
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::InvalidFormat => {
                write!(f, "错误格式")
            }

            CalcError::ExpressionFormatError => {
                write!(f, "表达式格式错误")
            }

            CalcError::MismatchedParentheses => {
                write!(f, "括号不匹配")
            }

            CalcError::NotNumber(value) => {
                write!(f, "{}不是数字", value)
            }

            CalcError::DivisionByZero => {
                write!(f, "除数不能为0")
            }

            CalcError::InvalidPower => {
                write!(f, "次方结果不是有效数字")
            }

            CalcError::UnsupportedOperator(op) => {
                write!(f, "不支持的运算符：{}", op)
            }
        }
    }
}

fn main() {
    println!("========== Rust 计算器 ==========");
    println!("支持：+ - * / ^");
    println!("支持：括号 ()");
    println!("支持：负数");
    println!("不需要空格，例如：2+3*4");
    println!("输入 q 退出");

    loop {
        println!("\n请输入计算表达式：");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("读取输入失败");

        let input = input.trim();

        if input == "q" {
            println!("计算器已退出！");
            break;
        }

        match calculate(input) {
            Ok(result) => {
                println!("结果：{}", result);
            }

            Err(error) => {
                println!("错误：{}", error);
            }
        }
    }
}


// ========================================
// 计算入口
// ========================================

fn calculate(input: &str) -> Result<f64, CalcError> {
    if input.trim().is_empty() {
        return Err(CalcError::InvalidFormat);
    }

    let mut parser = Parser::new(input);

    let result = parser.parse_expression()?;

    parser.skip_spaces();

    // 如果计算结束以后还有东西
    // 说明表达式格式有问题
    if let Some(ch) = parser.chars.peek().copied() {
        if ch == ')' {
            return Err(CalcError::MismatchedParentheses);
        }

        if "+-* /^".contains(ch) {
            return Err(CalcError::ExpressionFormatError);
        }

        return Err(CalcError::UnsupportedOperator(
            ch.to_string()
        ));
    }

    Ok(result)
}


// ========================================
// Parser：表达式解析器
// ========================================

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {

    // 创建 Parser
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }


    // ========================================
    // 跳过空格
    // ========================================

    fn skip_spaces(&mut self) {
        while let Some(ch) = self.chars.peek() {
            if ch.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }


    // ========================================
    // 处理 + -
    // ========================================

    fn parse_expression(&mut self) -> Result<f64, CalcError> {

        let mut result = self.parse_term()?;

        loop {
            self.skip_spaces();

            match self.chars.peek().copied() {

                Some('+') => {
                    self.chars.next();

                    let right = self.parse_term()?;

                    result += right;
                }

                Some('-') => {
                    self.chars.next();

                    let right = self.parse_term()?;

                    result -= right;
                }

                _ => {
                    break;
                }
            }
        }

        Ok(result)
    }


    // ========================================
    // 处理 * /
    // ========================================

    fn parse_term(&mut self) -> Result<f64, CalcError> {

        let mut result = self.parse_unary()?;

        loop {
            self.skip_spaces();

            match self.chars.peek().copied() {

                Some('*') => {
                    self.chars.next();

                    let right = self.parse_unary()?;

                    result *= right;
                }

                Some('/') => {
                    self.chars.next();

                    let right = self.parse_unary()?;

                    if right == 0.0 {
                        return Err(CalcError::DivisionByZero);
                    }

                    result /= right;
                }

                _ => {
                    break;
                }
            }
        }

        Ok(result)
    }

    fn parse_unary(&mut self) -> Result<f64, CalcError> {

        self.skip_spaces();

        match self.chars.peek().copied() {
            Some('-') => {
                self.chars.next();

                let value = self.parse_power()?;

                Ok(-value)
            }

            Some('+') => {
                Err(CalcError::ExpressionFormatError)
            }

            _ => {
                self.parse_power()
            }
        }
    }


    // ========================================
    // 处理 ^ 次方
    // ========================================

    fn parse_power(&mut self) -> Result<f64, CalcError> {

        let left = self.parse_primary()?;

        self.skip_spaces();

        if self.chars.peek() == Some(&'^') {

            self.chars.next();

            let right = self.parse_unary()?;

            let result = left.powf(right);

            if result.is_nan() {
                return Err(CalcError::InvalidPower);
            }

            return Ok(result);
        }

        Ok(left)
    }


    // ========================================
    // 处理数字和括号
    // ========================================

    fn parse_primary(&mut self) -> Result<f64, CalcError> {

        self.skip_spaces();

        match self.chars.peek().copied() {

            // ----------------------------
            // 左括号
            // ----------------------------

            Some('(') => {

                self.chars.next();

                let result = self.parse_expression()?;

                self.skip_spaces();

                match self.chars.next() {

                    Some(')') => {
                        Ok(result)
                    }

                    _ => {
                        Err(CalcError::MismatchedParentheses)
                    }
                }
            }


            // ----------------------------
            // 数字
            // ----------------------------

            Some(ch) if ch.is_ascii_digit() || ch == '.' => {
                self.parse_number()
            }


            // ----------------------------
            // 右括号
            // ----------------------------

            Some(')') => {
                Err(CalcError::MismatchedParentheses)
            }


            // ----------------------------
            // 运算符位置错误
            // ----------------------------

            Some(ch) if "+-* /^".contains(ch) => {
                Err(CalcError::ExpressionFormatError)
            }


            // ----------------------------
            // 其他字符
            // ----------------------------

            Some(ch) => {

                let mut value = String::new();

                value.push(ch);

                self.chars.next();

                Err(CalcError::NotNumber(value))
            }


            // ----------------------------
            // 什么都没有
            // ----------------------------

            None => {
                Err(CalcError::InvalidFormat)
            }
        }
    }

     fn parse_number(&mut self) -> Result<f64, CalcError> {

        let mut number = String::new();

        let mut has_dot = false;

        while let Some(ch) = self.chars.peek().copied() {

            if ch.is_ascii_digit() {

                number.push(ch);

                self.chars.next();

            } else if ch == '.' && !has_dot {

                has_dot = true;

                number.push(ch);

                self.chars.next();

            } else {

                break;
            }
        }

        if number.is_empty() || number == "." {

            return Err(
                CalcError::NotNumber(number)
            );
        }

        match number.parse::<f64>() {

            Ok(value) => Ok(value),

            Err(_) => {
                Err(CalcError::NotNumber(number))
            }
        }
    }
}*///第六版03