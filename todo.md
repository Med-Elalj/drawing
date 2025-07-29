 ✅ TODO
🔧 Setup & Core

Create main.rs and copy the usage example

Create module file geometrical_shapes.rs

Add raster and rand to Cargo.toml

    Add .gitignore for ignoring image outputs

🧩 Traits

Drawable trait:

    fn draw(&self, image: &mut Image)

    fn color(&self) -> Color

    Displayable trait:

        fn display(&mut self, x: i32, y: i32, color: Color)

🔲 Structures & Implementations

Point::new(x: i32, y: i32) -> Self

Point::random(width: i32, height: i32) -> Self

Line::new(&Point, &Point) -> Self

Line::random(width: i32, height: i32) -> Self

Rectangle::new(&Point, &Point) -> Self

Triangle::new(&Point, &Point, &Point) -> Self

Circle::new(&Point, radius: i32) -> Self

    Circle::random(width: i32, height: i32) -> Self

🎨 Drawing Logic

Implement Drawable for each shape

Use Displayable to draw pixels onto the image

    Ensure shapes are properly drawn (Bresenham's algorithm for lines, Midpoint circle for circles, etc.)

🖼️ Output

Save image.png using raster::save

    Validate output matches the expected result

🔁 Bonus (Optional)

Pentagon

    Cube

🧠 Notions to Understand

    Traits and trait bounds in Rust

    Image pixel manipulation

    Random generation with rand

    Drawing algorithms (lines, circles, triangles)

