//! # 简单区块链演示程序
//! 
//! 展示区块链的基本功能

fn main() {
    println!("🚀 简单区块链演示程序");
    println!("🚀 Simple Blockchain Demo Program");
    println!("{}", "=".repeat(60));
    println!();

    // 演示 Rust 特性
    println!("1️⃣ Rust 语言特性演示");
    
    // 模式匹配
    let number = 42;
    match number {
        0..=10 => println!("   📊 数字 {} 在 0-10 范围内", number),
        11..=50 => println!("   📊 数字 {} 在 11-50 范围内", number),
        _ => println!("   📊 数字 {} 大于 50", number),
    }
    
    // 所有权和借用
    let vec = vec![1, 2, 3, 4, 5];
    let sum: i32 = vec.iter().sum();
    println!("   🔢 向量 {:?} 的和是 {}", vec, sum);
    
    // 闭包
    let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    println!("   🔄 向量 {:?} 翻倍后是 {:?}", vec, doubled);
    println!();

    // 演示错误处理
    println!("2️⃣ 错误处理演示");
    
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(value) => println!("   ✅ 操作成功，值为 {}", value),
        Err(error) => println!("   ❌ 操作失败: {}", error),
    }
    
    let error_result: Result<i32, &str> = Err("Something went wrong");
    match error_result {
        Ok(value) => println!("   ✅ 操作成功，值为 {}", value),
        Err(error) => println!("   ❌ 预期的错误: {}", error),
    }
    println!();

    // 演示并发编程
    println!("3️⃣ 并发编程演示");
    
    use std::thread;
    use std::sync::{Arc, Mutex};
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("   🔄 多线程计数结果: {}", *counter.lock().unwrap());
    println!();

    // 演示函数式编程
    println!("4️⃣ 函数式编程演示");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 过滤偶数
    let even_numbers: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .copied()
        .collect();
    println!("   🔍 偶数: {:?}", even_numbers);
    
    // 映射和求和
    let squares_sum: i32 = numbers.iter()
        .map(|x| x * x)
        .sum();
    println!("   📐 平方和: {}", squares_sum);
    
    // 折叠操作
    let factorial = numbers.iter()
        .fold(1, |acc, x| acc * x);
    println!("   🧮 阶乘: {}", factorial);
    println!();

    // 演示泛型编程
    println!("5️⃣ 泛型编程演示");
    
    fn print_type<T: std::fmt::Debug>(value: T) {
        println!("   🔤 类型: {:?}", value);
    }
    
    print_type(42);
    print_type("Hello, Rust!");
    print_type(vec![1, 2, 3]);
    println!();

    // 演示生命周期
    println!("6️⃣ 生命周期演示");
    
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = "short";
    let string2 = "much longer string";
    let result = longest(string1, string2);
    println!("   📏 较长的字符串: {}", result);
    println!();

    // 演示 trait 系统
    println!("7️⃣ Trait 系统演示");
    
    trait Drawable {
        fn draw(&self);
    }
    
    struct Circle {
        radius: f64,
    }
    
    struct Rectangle {
        width: f64,
        height: f64,
    }
    
    impl Drawable for Circle {
        fn draw(&self) {
            println!("   🔴 绘制圆形，半径: {}", self.radius);
        }
    }
    
    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("   🔲 绘制矩形，尺寸: {}x{}", self.width, self.height);
        }
    }
    
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 8.0 }),
    ];
    
    for shape in shapes {
        shape.draw();
    }
    println!();

    // 演示宏系统
    println!("8️⃣ 宏系统演示");
    
    macro_rules! greet {
        ($name:expr) => {
            println!("   👋 你好, {}!", $name);
        };
        ($name:expr, $greeting:expr) => {
            println!("   👋 {}, {}!", $greeting, $name);
        };
    }
    
    greet!("Rust");
    greet!("开发者", "欢迎");
    println!();

    // 演示智能指针
    println!("9️⃣ 智能指针演示");
    
    use std::rc::Rc;
    
    let data = Rc::new(42);
    let data_clone = Rc::clone(&data);
    
    println!("   📊 引用计数: {}", Rc::strong_count(&data));
    println!("   📊 数据值: {}", data);
    println!("   📊 克隆值: {}", data_clone);
    println!();

    // 总结
    println!("🎯 总结");
    println!("   ✅ 展示了 Rust 的核心特性:");
    println!("      🔸 所有权和借用系统");
    println!("      🔸 模式匹配");
    println!("      🔸 错误处理");
    println!("      🔸 并发编程");
    println!("      🔸 函数式编程");
    println!("      🔸 泛型编程");
    println!("      🔸 生命周期管理");
    println!("      🔸 Trait 系统");
    println!("      🔸 宏系统");
    println!("      🔸 智能指针");
    println!();

    println!("🎉 演示完成！");
    println!("💡 这些特性使 Rust 成为构建安全、高性能区块链系统的理想选择。");
}
