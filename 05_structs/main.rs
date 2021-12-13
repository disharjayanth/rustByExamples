#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String ,
    age: u8 ,
}

// A unit struct
#[allow(dead_code)]
struct Unit;

// A tuple struct
struct Pair (i32, f32);

// A struct with two field
#[derive(Debug)]
struct  Point {
    x: f32 ,
    y: f32 ,
}

// struct used as field in another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point ,
    botton_right: Point ,
}

fn main() {
    let name = String::from("Peter");
    let age = 28;
    let peter = Person{ name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instanitate a point
    let point: Point = Point{ x : 10.3, y : 0.4};
    println!("{:?}", point);

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    let botton_right = Point{ x : 5.2, ..point};

    // botton_right.y will be same as point.y because that field
    // is used from point
    println!("botton_right x and y points: {} , {}.", botton_right.x, botton_right.y);

    // Destructure the point using `let` binding
    let Point { x : left_edge, y : top_edge } = point;
    println!("point destructed into left_edge and top_edge: {}, {} ", left_edge, top_edge);

    let rectangle : Rectangle =  Rectangle{
        top_left: Point{
            x: left_edge ,
            y: top_edge ,
        } ,
        botton_right: botton_right ,
    };

    println!("{:?}", rectangle);

    // Instantiate a unit struct
    // let unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("Pair contains {} and {}", pair.0, pair.1);

    // Destructre a tuple struct
    let Pair(integer, decimal) = pair;

    println!("Pair contains {:?} and {:?} after destructing.", integer, decimal);
}