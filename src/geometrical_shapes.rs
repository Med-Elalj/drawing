use raster::{Color, Image};
use rand::Rng;  

pub trait Drawable {
    fn draw(&self,  image:&mut Image) {}
    fn color(&self){}
}


pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color) {}
}


/*__________________________shapes_______________________*/
// Poit
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x:i32, y:i32)-> Self{
        return Point{x: x, y:y};
    }
    pub fn random(width: i32, height: i32)-> Point{
        let mut rng = rand::rng();

        let x : i32 = rng.random_range(0..width);
        let y : i32 = rng.random_range(0..height);
        return Point::new(x, y);
    }

    pub fn draw(self, image :&mut Image){
        let red = Color::rgba(255, 0, 0, 255); // solid red
        image.set_pixel(self.x,self.y, red).unwrap();
    }
}
// Line

pub struct Line{
    first: Point,
    last: Point,
}

impl Line{
    pub fn new(x: Point, y: Point)-> Self{
        return Line{first: x, last: y};
    }

    pub fn random(width: i32, height: i32)-> Line {

        let first : Point = Point::random(width, height); 
        let last : Point = Point::random(width, height); 

        return Line::new(first, last);
    }

    pub fn draw(&self, image :&mut Image){
        let red = Color::rgba(255, 0, 0, 255); // solid red
        // fine the line equation 
        // find a
        let dx = self.first.x - self.last.x ;
        let dy = self.first.y - self.last.y ;
        let a = dx/dy;
        // find b
        let b = self.first.y + self.last.y; 

        for i in 0..image.width {
            for j in 0..image.height {
                if j == a*i + b /* && self.range(i , j)*/{
                    image.set_pixel(i , j, red.clone()).unwrap();
                }
            }    
        }
    }
    pub fn range(&self, i : i32, j : i32 ) -> bool{
        i > i32::min(self.first.x , self.last.x) && i < i32::max(self.first.x , self.last.x)
        && j > i32::min(self.first.y , self.last.y) && i < i32::max(self.first.y , self.last.y)
    }
}



// // Circle
// pub struct Circle{
//     center : Point,
//     raduis : u32,
// }

// impl Circle{
//     pub fn new(center:Point, raduis: u32)-> Self{
//         return Circle{center: center, raduis:raduis};
//     }

//     pub fn random(width: i32, height: i32)-> Circle{
//         let c  : Point = Point::random(width, height); 
//         let mut rng = rand::rng();

//         let r = rng.random_range(0..height) as u32;

//         return Circle::new(c, r);
//     }

//     pub fn draw(self, image :&mut Image){
//         let red = Color::rgba(255, 0, 0, 255); // solid red
//         image.set_pixel(self.x,self.y, red).unwrap();
//     }
// }