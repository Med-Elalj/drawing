use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image) {}
    fn color(&self) {}
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
    pub fn new(x: i32, y: i32) -> Self {
        return Point { x: x, y: y };
    }
    pub fn random(width: i32, height: i32) -> Point {
        let mut rng = rand::rng();

        let x: i32 = rng.random_range(0..width);
        let y: i32 = rng.random_range(0..height);
        return Point::new(x, y);
    }

    pub fn draw(self, image: &mut Image) {
        let red = Color::rgba(255, 0, 0, 255); // solid red
        image.set_pixel(self.x, self.y, red).unwrap();
    }
}
// Line

pub struct Line {
    first: Point,
    last: Point,
}

impl Line {
    pub fn new(x: Point, y: Point) -> Self {
        return Line { first: x, last: y };
    }

    pub fn random(width: i32, height: i32) -> Line {
        let first: Point = Point::random(width, height);
        let last: Point = Point::random(width, height);
        println!(
            "width {} -- height {} -- first.x {} -- first.y {} -- last.x {} -- last.y {} -- ",
            width, height, first.x, first.y, last.x, last.y
        );
        return Line::new(first, last);
    }

    pub fn draw(&self, image: &mut Image) {
        let red = Color::rgba(255, 0, 0, 255);

        if self.last.x == self.first.x {
            for y in self.first.y..=self.last.y {
                image.set_pixel(self.first.x, y, red.clone())
            .unwrap();
            }
            return;
        }

        let a = (self.last.y - self.first.y) as f64 / (self.last.x - self.first.x) as f64;
        let b = self.first.y as f64- a * self.first.x as f64;

        // println!("{:?}",ratio);
        for x in self.first.x..=self.last.x {
            // let j: f64 = i as f64;
            image.set_pixel(x,(x as f64 * a + b) as i32, red.clone())
            .unwrap();
        }
    }

    pub fn ratio(&self,size : i32) -> (f64,f64) {
        (((self.last.x-self.first.x) as f64) / size as f64,
        ((self.last.y-self.first.y) as f64) / size as f64)
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
