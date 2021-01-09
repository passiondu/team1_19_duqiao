#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
    // -------------------------------------
    // 矩形：width-> 宽, height-> 高
    // -------------------------------------
}
impl Rectangle {
    fn new(wh: f32, ht: f32) -> Rectangle {
        Rectangle {
            width: wh,
            height: ht,
        }
    }
}
#[derive(Debug)]
struct Triangle {
    base: f32,
    height: f32,    
    // -------------------------------------
    // 三角形：base-> 底, height-> 高
    // -------------------------------------
}
impl Triangle {
    fn new(be: f32, ht: f32) -> Triangle {
        Triangle {
            base: be,
            height: ht,
        }
    }
}
#[derive(Debug)]
struct Circle {
    radius: f32,
    // -------------------------------------
    // 圆形：radius-> 半径
    // -------------------------------------
}
impl Circle {
    fn new(rs: f32) -> Circle {
        Circle {
            radius: rs
        }
    } 
}

trait ComputeArea {
    #![allow(non_snake_case)]
    fn getArea(&self) -> f32;
}

impl ComputeArea for Rectangle {
    fn getArea(&self) -> f32 {
        self.width * self.height  // 矩形面積：(底 x 高)
    
    }
}

impl ComputeArea for Triangle {
    fn getArea(&self) -> f32 {
        (self.base * self.height) / 2.0  // 三角形面積：(底 x 高) / 2
    }
}

impl ComputeArea for Circle {
    fn getArea(&self) -> f32 {
        3.14 * self.radius.powf(2.0)  // 圆形面积：π x 半径平方
    }
}

fn main() {
    #![allow(non_snake_case)]
    let rectangleObj = Rectangle::new(12.0, 10.0);
    let triangleObj = Triangle::new(8.0, 6.0);
    let circleObj = Circle::new(12.0);
    //矩形面积
    println!("矩形：{:?}, 面积 => {}", rectangleObj, rectangleObj.getArea());
    //三角形面积
    println!("三角形：{:?}, 面积 => {}", triangleObj, triangleObj.getArea());
    //圆形面积
    println!("圆形：{:?}, 面积 => {}", circleObj, circleObj.getArea());
}
