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

        let x = rng.random::<i32>();
        let y = rng.random::<i32>();
        return Point::new(x, y);
    }

    pub fn draw(self, image :&mut Image){

    }
}
