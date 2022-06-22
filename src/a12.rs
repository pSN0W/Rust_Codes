// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColor {
    Red,
    Blue,
    Green,
}

impl BoxColor {
    fn print_color(&self) {
        match self {
            BoxColor::Red => println!("Color of the box is Red"),
            BoxColor::Blue => println!("Color of the box is Blue"),
            BoxColor::Green => println!("Color of the box is Green"),
        };
    }
}

struct Dimensions {
    width : f32,
    height : f32,
    breadth : f32,
}


impl Dimensions {
    fn init (width:f32,height:f32,breadth:f32) -> Self {
        Self {
            width,
            height,
            breadth,
        }
    }
    fn print_dimensions (&self) {
        println!("Height of the box is {:?}",self.height);
        println!("Breadth of the box is {:?}",self.breadth);
        println!("Width of the box is {:?}",self.width);
    }
}

struct Box {
    weight : f32,
    color : BoxColor,
    dimensions : Dimensions,
}

impl Box {
    fn init(weight:f32,color:BoxColor,dimensions:Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn display_box (&self) {
        println!("Weight of the box is {:?}",self.weight);
        self.dimensions.print_dimensions();
        self.color.print_color();
    }
}

fn main() {
    let my_box = Box::init(25.0,BoxColor::Red,Dimensions::init(3.0,4.0,5.0));
    my_box.display_box();
}
