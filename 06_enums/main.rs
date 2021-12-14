enum WebEvent {
    PageLoad ,
    PageUnload ,
    KeyPress(char) ,
    Paste(String) ,
    Click { x : i64 , y : i64 } ,
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded") ,
        WebEvent::PageUnload => println!("Page unloaded") ,
        WebEvent::KeyPress(c) => println!("pressed `{}`", c) ,
        WebEvent::Paste(s) => println!("pasted \"{}\".", s) ,
        WebEvent::Click{ x, y } => {
            println!("Clicked at x = {}, y = {}.", x, y) ;
        } ,
    }
}

enum Operations {
    Add(i64, i64) ,
    Subtract(i64, i64) ,
    Multiply(i64, i64) ,
    Divide(i64, i64) ,
    Modulus(i64, i64) ,
}

fn ArithmaticOperation(op: Operations) {
    match op {
        Operations::Add(num1, num2) => println!("{}", num1 + num2) ,
        Operations::Subtract(num1, num2) => println!("{}", num1 - num2) ,
        Operations::Multiply(num1, num2) => println!("{}", num1 * num2) ,
        Operations::Divide(num1, num2) => println!("{}", num2 / num1)  ,
        Operations::Modulus(num1, num2) => println!("{}", num1 % num2) ,
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('A');
    let pasted = WebEvent::Paste("some random text".to_string());
    let click = WebEvent::Click{ x : 20, y : 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let add = Operations::Add(2, 2);
    let sub = Operations::Subtract(4, 2);
    let mul = Operations::Multiply(2, 2);
    let div = Operations::Divide(4, 2);
    let modu = Operations::Modulus(4, 2);

    ArithmaticOperation(add);
    ArithmaticOperation(sub);
    ArithmaticOperation(mul);
    ArithmaticOperation(div);
    ArithmaticOperation(modu);
}