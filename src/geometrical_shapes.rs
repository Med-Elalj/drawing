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
