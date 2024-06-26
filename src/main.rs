use colored::*;
use num_traits::ToPrimitive;
use console::Term;
// unintuitively, Y is horizontal
const SCREEN_Y_SIZE: usize = 50;
// and X is vertical
const SCREEN_X_SIZE: usize = 30;

const WORLD_SIZE: usize = 10;

const SIZE_MULTIPLIER: f32 = 10.0;

const PI: f32 = 3.14159265358979323846264338327950288419716939937510582;

const CAMERA_DISTANCE: f32 = 5.0; // this is kind of like FOV but in theory (UNTESTED) higher value = lower FOV.
const SUN_DIRECTION: Vector3 = Vector3{x: 0.0, y: 0.0, z: -1.0};

const GRASS_COLOR: CustomColor = CustomColor { r: 0, g: 255, b: 0 };
const STONE_COLOR: CustomColor = CustomColor { r: 128, g: 128, b: 128 };
const WOOD_COLOR: CustomColor = CustomColor { r: 128, g: 128, b: 0 };

fn main() {
  //        y
  //        |
  //    z   |
  //     `. |
  //       `+------ x
  //        E
  let mut camera_position: Vector3 = Vector3 { x: 0.0, y: 11.0, z: 0.0 };
  // achtung! CAMERA ROTATION IS IN RADIANS     convert to radians
  let mut camera_rotation_vertical: f32 = 0.0 * (PI/180.0);
  let mut camera_rotation_horizontal: f32 = 0.0 * (PI/180.0);

  // initialise the screen
  let mut main_screen: Screen = Screen { pixels: Vec::new(), size_x: SCREEN_X_SIZE, size_y: SCREEN_Y_SIZE };

  // initialise the world
  // put the whole world in memory because reading from disk is slow
  let mut world_data: Vec<CubeType> = load_world("world.rmc");

  // GAME LOOP
  loop {

    let stdout = Term::buffered_stdout();

    let char = stdout.read_char().expect("ass");

    if char == 'w' {
      camera_position.z += 1.0;
    }
    if char == 'a' {
      camera_position.x += -1.0;
    }
    if char == 's' {
      camera_position.z += -1.0;
    }
    if char == 'd' {
      camera_position.x += 1.0;
    }
    if char == 'q' {
      camera_position.y += -1.0;
    }
    if char == 'e' {
      camera_position.y += 1.0;
    }
    if char == 'y' {
      camera_rotation_horizontal += -5.0;
    }
    if char == 'x' {
      camera_rotation_horizontal += 5.0;
    }

    //println!("char: {:?}", char);

    
    // update main screen
    let frame_start_time = std::time::Instant::now();
    main_screen = draw_world(world_data.clone(), camera_position, camera_rotation_vertical, camera_rotation_horizontal);
    let frame_duration = frame_start_time.elapsed();
    // update main screen
    
    // Draw the screen and sleep for a few milliseconds (to let the screen render)
    clearscreen::clear().expect("failed to clear screen");
    main_screen.draw();
    let idle_interval = std::time::Duration::from_millis(10);
    //std::thread::sleep(idle_interval);
    println!("RENDER : {:?}", frame_duration);
    println!("POS    : {:?}", camera_position);
    println!("CAM_ROT: {:?}", camera_rotation_horizontal);
    
  }
}

fn draw_world(world_data: Vec<CubeType>, camera_position: Vector3, camera_rotation_vertical: f32, camera_rotation_horizontal: f32) -> Screen {

  let mut triangles_to_draw: Vec<Triangle3D> = Vec::new();

  for (linear_index, cube_type) in world_data.iter().enumerate() {
    // now we must, for each index:
    // DONE - if air, ignore
    // DONE - calculate position from index
    // DONE - find all cube edge vertices
    // DONE UNTESTED - construct all 12 triangles into Triangle3Ds and give them preassigned normals
    // DONE UNTESTED - if (dot product is negative), ignore the fucker
    // DONE - render whatever remains into list of Triangle2D with depth attached
    //   DONE - if depth is negative dont render
    // - draw to screen in order (since deepest get drawn first, less depp ones will get drawn on top of deep ones :thumbsup:)

    if cube_type == &CubeType::Air {
      continue; // ignore air blocks
    }

    let cube_color: CustomColor = match cube_type {
      CubeType::Grass => GRASS_COLOR,
      CubeType::Stone => STONE_COLOR,
      CubeType::Wood  => WOOD_COLOR,
      _ => panic!()
    };

    // Let's call our cube ABCDEFGH
    //    A      B
    //    +------+.    
    //    |`. D  | `.  
    //    |  `+--+---+ C
    //    |   |  |   | 
    //    +---+--+.  | 
    //    H`. |  G `.| 
    //       `+------+ 
    //       E       F
    // Coordinates for reference
    //        y
    //        |
    //    z   |
    //     `. |
    //       `+------ x
    //        E
    //
    // normals
    // +-----------+
    // |\          |\
    // | \     T   | \
    // |  \   Ba   |  \
    // |   +-----------+
    // | L |       | R |
    // +---|-------+   |
    //  \  |     F  \  |
    //   \ |   B     \ |
    //    \|          \|
    //     +-----------+
    //
    //    face       xyz
    // B  (bottom) = ( -1  0  0 )
    // Ba (back)   = (  0  1  0 )
    // L  (left)   = (  0  0 -1 )
    // R  (right)  = (  0  0  1 )
    // F  (front)  = (  0 -1  0 )
    // T  (top)    = (  1  0  0 )

    // construct all vertices of a cube.
    let vertex_e: Vector3 = linear_index_to_vector3(linear_index);
    let vertex_f: Vector3 = Vector3{ z: vertex_e.x      , y: vertex_e.y      , x: vertex_e.z + 1.0};
    let vertex_h: Vector3 = Vector3{ z: vertex_e.x      , y: vertex_e.y + 1.0, x: vertex_e.z      };
    let vertex_g: Vector3 = Vector3{ z: vertex_e.x      , y: vertex_e.y + 1.0, x: vertex_e.z + 1.0};
    let vertex_d: Vector3 = Vector3{ z: vertex_e.x + 1.0, y: vertex_e.y      , x: vertex_e.z      };
    let vertex_c: Vector3 = Vector3{ z: vertex_e.x + 1.0, y: vertex_e.y      , x: vertex_e.z + 1.0};
    let vertex_b: Vector3 = Vector3{ z: vertex_e.x + 1.0, y: vertex_e.y + 1.0, x: vertex_e.z + 1.0};
    let vertex_a: Vector3 = Vector3{ z: vertex_e.x + 1.0, y: vertex_e.y + 1.0, x: vertex_e.z      };


    let mut triangles: Vec<Triangle3D> = Vec::new();

    // FRONT
    // Construct ECF
    triangles.push( Triangle3D {
      a: vertex_e,
      b: vertex_c,
      c: vertex_f,
      color: cube_color, 
      normal: Vector3 { y: 0.0, z: -1.0, x: 0.0 },
    });
    // Construct ECD
    triangles.push( Triangle3D {
      a: vertex_e,
      b: vertex_c,
      c: vertex_d,
      color: cube_color, 
      normal: Vector3 { y: 0.0, z: -1.0, x: 0.0 },
    });
    // BACK
    // construct ABG
    triangles.push( Triangle3D {
      a: vertex_a,
      b: vertex_b,
      c: vertex_g,
      color: cube_color, 
      normal: Vector3 { y: 0.0, z: 1.0, x: 0.0 },
    });
    // construct AHG
    triangles.push( Triangle3D {
      a: vertex_a,
      b: vertex_h,
      c: vertex_g,
      color: cube_color, 
      normal: Vector3 { y: 0.0, z: 1.0, x: 0.0 },
    });
    // TOP
    // construct ADC
    triangles.push( Triangle3D {
      a: vertex_a,
      b: vertex_d,
      c: vertex_c,
      color: cube_color, 
      normal: Vector3 { y: 1.0, z: 0.0, x: 0.0 },
    });
    // construct BAC
    triangles.push( Triangle3D {
      a: vertex_b,
      b: vertex_a,
      c: vertex_c,
      color: cube_color, 
      normal: Vector3 { y: 1.0, z: 0.0, x: 0.0 },
    });
    // BOTTOM
    // construct HEG
    triangles.push( Triangle3D {
      a: vertex_h,
      b: vertex_e,
      c: vertex_g,
      color: cube_color, 
      normal: Vector3 { y: -1.0, z: 0.0, x: 0.0 },
    });
    // construct GFE
    triangles.push( Triangle3D {
      a: vertex_g,
      b: vertex_f,
      c: vertex_e,
      color: cube_color, 
      normal: Vector3 { y: -1.0, z: 0.0, x: 0.0 },
    });
    // LEFT
    // construct ADH
    triangles.push( Triangle3D {
      a: vertex_a,
      b: vertex_d,
      c: vertex_h,
      color: cube_color, 
      normal: Vector3 { y: 0.0, z: 0.0, x: -1.0 },
    });
    // construct DHE
    triangles.push( Triangle3D {
      a: vertex_d,
      b: vertex_h,
      c: vertex_e,
      color: cube_color, 
      normal: Vector3 { y: 0.0, z: 0.0, x: -1.0 },
    });
    // RIGHT
    // construct FGB
    triangles.push( Triangle3D {
      a: vertex_f,
      b: vertex_g,
      c: vertex_b,
      color: cube_color, 
      normal: Vector3 { y: 0.0, z: 0.0, x: 1.0 },
    });
    // construct BFC
    triangles.push( Triangle3D {
      a: vertex_b,
      b: vertex_f,
      c: vertex_c,
      color: cube_color, 
      normal: Vector3 { y: 0.0, z: 0.0, x: 1.0 },
    });

    let camera_rotation_vectorial: Vector3 = angle_couple_to_vector3(camera_rotation_horizontal, camera_rotation_vertical);

    for triangle in triangles {
      // if the normal is not negative, add to triangles to draw list
      if vector3_dot(camera_rotation_vectorial, triangle.normal) < 0.0 {
        triangles_to_draw.push(triangle);
      }
    }
  }

  // NOTE: surprisingly this shitty sorting algorithm is only 10-20ms slower than unsorted.
  // where are the 380ms frame times coming from????????????

  // now: render triangle and add it to a sorted list, going from deep to not deep
  // (the depth of a triangle is how far it is from the camera on the camera's depth axis)
  // (doing things this way may be problematic in a normal renderer, but in a cube came we're chilling)

  // a loop where:
  // - render a triangle, get its depth as well (this one will later be referred to as "current")
  // - put it in a struct that contains it and its depth
  // - create bool already_added = false;
  // - iterate through already rendered triangles
  //   - if depth of current is higher than the triangle already rendered and already_added is false
  //     - add current to new vector
  //     - set already_added to true
  //   - else
  //      - add already rendered to new vector
  // - set already rendered to new vector
  // - repeat for the next triangle
  let mut rendered_triangles: Vec<RenderedTriangle2D> = Vec::new();
  for triangle in triangles_to_draw {
      let (rendered_triangle, depth): (Triangle2D, f32) = render_triangle(triangle, camera_position, camera_rotation_vertical, camera_rotation_horizontal);
      let rendered_triangle_to_append = RenderedTriangle2D {
          triangle: rendered_triangle,
          depth,
      };
  
      let mut inserted = false;
      let mut new_rendered_triangles: Vec<RenderedTriangle2D> = Vec::new();
      for already_rendered_triangle in rendered_triangles.iter() {
          if !inserted && already_rendered_triangle.depth > depth {
              new_rendered_triangles.push(rendered_triangle_to_append.clone());
              inserted = true;
          }
          new_rendered_triangles.push(already_rendered_triangle.clone());
      }
      if !inserted {
          new_rendered_triangles.push(rendered_triangle_to_append);
      }
      rendered_triangles = new_rendered_triangles;
  }

  //println!("{:?}", rendered_triangles.clone());
  
  let mut screen: Screen = Screen { pixels: Vec::new(), size_x: SCREEN_X_SIZE, size_y: SCREEN_Y_SIZE };
  screen.init();

  let mut new_pixels: Vec<Vec<CustomColor>> = Vec::new();

  for (x, x_row) in screen.pixels.iter().enumerate().clone() {
    let mut new_pixels_x: Vec<CustomColor> = Vec::new();
    for (y, _) in x_row.iter().enumerate() {
      let mut new_pixel: CustomColor = CustomColor { r: 0, g: 0, b: 0 };
      for triangle in rendered_triangles.clone() {
        let x_centered = (x as i32).wrapping_sub(SCREEN_X_SIZE as i32 / 1) as f32;
        let y_centered = (y as i32).wrapping_sub(SCREEN_Y_SIZE as i32 / 1) as f32;
        if triangle.triangle.contains(Vector2 { x: x_centered, y: y_centered }) {
          new_pixel = triangle.triangle.color;
        }
      }
      new_pixels_x.push(new_pixel);
    }
    new_pixels.push(new_pixels_x);
  }
  screen.pixels = new_pixels;
    

  // so the linter shuts up
  return screen;
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct RenderedTriangle2D {
  triangle: Triangle2D,
  depth: f32,
}

fn angle_couple_to_vector3(horizontal: f32, vertical: f32) -> Vector3 {
  let vector: Vector3 = Vector3{
    z: round_hundredth(f32::cos(horizontal)),
    y: round_hundredth(f32::sin(vertical)),
    x: round_hundredth(f32::sin(horizontal)),
  };
  return vector;
}
/// round a number to the hundredth <br>
/// 0.015205001 => 0.01
fn round_hundredth(num: f32) -> f32 {
  return (num * 100.0).round() / 100.0;
}

/// get a set of coordinates with an index
fn linear_index_to_vector3(linear_index: usize) -> Vector3 {
  let z = linear_index % WORLD_SIZE;
  let y = (linear_index / WORLD_SIZE) % WORLD_SIZE;
  let x = linear_index / (WORLD_SIZE * WORLD_SIZE);
  return Vector3 { x: x as f32, y: y as f32, z: z as f32};
}
/// get an index from a set of coordinates
fn vector3_to_linear_index(position: Vector3) -> usize {
  let z = position.x.to_usize().expect("idc");
  let y = position.y.to_usize().expect("idc");
  let x = position.z.to_usize().expect("idc");
  let linear_index: usize = x + WORLD_SIZE * (y * WORLD_SIZE + z);
  return linear_index;
}

fn load_world(path: &str) -> Vec<CubeType> {
  let mut cubes: Vec<CubeType> = Vec::new();

  let contents: String = std::fs::read_to_string(path)
    .expect("Should have been able to read the file, or not idk");

  for char in contents.as_bytes() {
    match char {

      b'0' => { cubes.push(CubeType::Air)   }
      b'1' => { cubes.push(CubeType::Grass) }
      b'2' => { cubes.push(CubeType::Stone) }
      b'3' => { cubes.push(CubeType::Wood)  }

      _ => {
        panic!("invalid element in save file.");
      }
    }

  }

  return cubes
}

fn render_triangle(triangle: Triangle3D, camera_position: Vector3, camera_rotation_vertical: f32, camera_rotation_horizontal: f32) -> (Triangle2D, f32) {
  let (vertex_a, depth_a) = render_vertex(triangle.a, camera_position, camera_rotation_vertical, camera_rotation_horizontal);
  let (vertex_b, _) = render_vertex(triangle.b, camera_position, camera_rotation_vertical, camera_rotation_horizontal);
  let (vertex_c, _) = render_vertex(triangle.c, camera_position, camera_rotation_vertical, camera_rotation_horizontal);

  let test_colour: CustomColor = triangle.color;

  // again, imprecise depth but whatever, good enough
  return (Triangle2D { a: vertex_a, b: vertex_b, c: vertex_c, color: test_colour }, depth_a)
}

/// 3D point -> 2D point (to be put on screen)
fn render_vertex(vertex: Vector3, camera_position: Vector3, camera_rotation_vertical: f32, camera_rotation_horizontal: f32) -> (Vector2, f32) {

  let position_on_vertical_plane: Vector2 = Vector2   { x: vertex.y, y: vertex.z };
  let position_on_horizontal_plane: Vector2 = Vector2 { x: vertex.x, y: vertex.z };  

  let position_camera_vertical_plane: Vector2 =  Vector2   { x: camera_position.y, y: camera_position.z };
  let position_camera_horizontal_plane: Vector2 =  Vector2 { x: camera_position.x, y: camera_position.z };

  let (screen_x, depth_x) = project_2d_to_1d(position_on_vertical_plane, position_camera_vertical_plane, camera_rotation_vertical);
  let (screen_y, depth_y) = project_2d_to_1d(position_on_horizontal_plane, position_camera_horizontal_plane, camera_rotation_horizontal);
  // imprecise depth calculation, but probs decent enough for a block game
  return (Vector2 { x: screen_x, y: screen_y }, depth_x+depth_y);
}

/// point on 2D plane -> point on 1D axis
fn project_2d_to_1d(vertex: Vector2, camera_position: Vector2, camera_rotation: f32) -> (f32, f32){
  //      this line represents the screen we project on
  //                          |
  //            ----\         |
  //            |    ----\   \'/    hypotenuse = camera_vertex_distance
  //            |         ----\
  // relative_y |             |----\
  //            |    screen_y |    ----\
  //            |             |          ----\
  //            -----------------------------------X  camera_position
  //                           <---CAMERA_DISTANCE--->
  //            <---------------depth---------------->

  // distance between camera and the vertex
  let camera_vertex_distance: f32 = vector2_distance(camera_position.clone(), vertex.clone());
  // find the vector that connects camera and vertex
  let camera_vertex_distance_vector: Vector2 = Vector2 { x: vertex.x - camera_position.x, y: vertex.y - camera_position.y };
  // world angle of vertex relative to camera
  let vertex_relative_angle: f32 = f32::atan(camera_vertex_distance_vector.x/camera_vertex_distance_vector.y);
  // relative to camera angle of vertex relative to camera
  let vertex_camera_angle: f32 = camera_rotation - vertex_relative_angle;
  
  // find depth
  let depth: f32 = f32::cos(vertex_camera_angle) * camera_vertex_distance;
  // find relative_y
  let relative_y: f32 = f32::sin(vertex_camera_angle) * camera_vertex_distance;
  // Now we apply Thales' Theorem to find screen_y
  let screen_y: f32 = (CAMERA_DISTANCE * relative_y) / (depth - CAMERA_DISTANCE);
  return (screen_y * SIZE_MULTIPLIER, depth);
}


/// Calculate the distance between two 3D points.
/// 
/// returns `sqrt((vec2.x - vec1.x)² + (vec2.y - vec1.y)² + (vec2.z - vec1.z)²)`
fn vector3_distance(vec1: Vector3, vec2: Vector3) -> f32 {

  let x_dist_sq: f32 = f32::powf(vec2.x - vec1.x, 2.0);
  let y_dist_sq: f32 = f32::powf(vec2.y - vec1.y, 2.0);
  let z_dist_sq: f32 = f32::powf(vec2.z - vec1.z, 2.0);

  let distance = f32::sqrt(x_dist_sq + y_dist_sq + z_dist_sq);
  return distance;
}

/// Calculate the distance between two 2D points.
/// 
/// returns `sqrt((vec2.x - vec1.x)² + (vec2.y - vec1.y)²)`
fn vector2_distance(vec1: Vector2, vec2: Vector2) -> f32 {

  let x_dist_sq: f32 = f32::powf(vec2.x - vec1.x, 2.0);
  let y_dist_sq: f32 = f32::powf(vec2.y - vec1.y, 2.0);

  let distance = f32::sqrt(x_dist_sq + y_dist_sq);
  return distance;
}

/// Calculate the dot product between two vectors
fn vector3_dot(vec1: Vector3, vec2: Vector3) -> f32 {
  let dot_product = vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z;
  return dot_product
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector3 {
  x : f32,
  y : f32,
  z : f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2 {
  x : f32,
  y : f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Triangle3D {
  a:      Vector3,
  b:      Vector3,
  c:      Vector3,
  color:  CustomColor,
  normal: Vector3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Triangle2D {
  a:     Vector2,
  b:     Vector2,
  c:     Vector2,
  color: CustomColor,
  //parent_cube_idex: usize,
}
impl Triangle2D {
  /// Check if a triangle contains a certain point
  fn contains(&self, point: Vector2) -> bool {
    // say ABC = triangle and P = point:
    // test if area ABC = area ABP + area ACP + area BCP
    let real_triangle_area = triangle_area(self.clone());

    let mut area_sum: f32 = 0.0;
    area_sum += triangle_area(Triangle2D { a: self.a, b: self.b, c: point, color: CustomColor {r: 0, g: 0, b: 0} });
    area_sum += triangle_area(Triangle2D { a: self.a, b: self.c, c: point, color: CustomColor {r: 0, g: 0, b: 0} });
    area_sum += triangle_area(Triangle2D { a: self.b, b: self.c, c: point, color: CustomColor {r: 0, g: 0, b: 0} });

    return real_triangle_area == area_sum;
  }
}
/// calculate the area of a 2D triangle
fn triangle_area(triangle: Triangle2D) -> f32 {
  let ab: Vector2 = Vector2 { x: triangle.b.x - triangle.a.x, y: triangle.b.y - triangle.a.y };
  let ac: Vector2 = Vector2 { x: triangle.c.x - triangle.a.x, y: triangle.c.y - triangle.a.y };
  let cross_product: f32 = ab.x * ac.y - ab.y * ac.x;
  let area = f32::abs(cross_product);
  return area/2.0;
}

struct Screen {
  pixels: Vec<Vec<CustomColor>>,
  size_x: usize,
  size_y: usize,
}
impl Screen {
  fn init(&mut self) {
    let mut screen: Vec<Vec<CustomColor>> = Vec::new();
    for _ in 0..self.size_x {
      let mut x_row: Vec<CustomColor> = Vec::new();
      for _ in 0..self.size_y {
        x_row.push(CustomColor{r: 0, g: 0, b: 0});
      }
      screen.push(x_row);
    }
    self.pixels = screen;
  }
  fn draw(&self) {
    let pixel_char: ColoredString = ColoredString::from("██");
    for column in 0..self.size_x {
      for pixel in 0..self.size_y {
        let color: CustomColor = self.pixels[column][pixel];
        print!("{}", pixel_char.clone().custom_color(color));
      }
      println!();
    }
  }
}

/// enumarates all possible cubes
#[derive(Debug, Clone, PartialEq)]
enum CubeType {
  Air,
  Grass,
  Stone,
  Wood,
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn dot_product() {
      let vec1: Vector3 = Vector3 { x: 0.0, y: 4.0, z: -4.0 };
      let vec2: Vector3 = Vector3 { x: 4.0, y: -4.0, z: 4.0 };
      let actual: f32 = vector3_dot(vec1, vec2);
      let expected: f32 = -32.0;
      assert_eq!(actual, expected);
    }

    /// Test the distance function
    #[test]
    fn distance() {
      let vec1: Vector3 = Vector3 { x: 0.0, y: 4.0, z: -4.0 };
      let vec2: Vector3 = Vector3 { x: 4.0, y: -4.0, z: 4.0 };
      // should be 12
      let actual: f32 = vector3_distance(vec1, vec2);
      let expected: f32 = 12.0;
      assert_eq!(actual, expected);
    }

    /// Test the 2D -> 1D projection function
    #[test]
    fn projection_2d_to_1d() {
      // test data
      let camera_position: Vector2 = Vector2 { x: 1.0, y: 1.0 };
      let camera_rot: f32 = 45.0 * (PI/180.0);
      let vertex: Vector2 = Vector2 { x: 12.0, y: 12.0 };

      let (actual, _ )= project_2d_to_1d(vertex, camera_position, camera_rot);
      let expected: f32 = 0.0; // in theory camera is looking directly at point

      assert_eq!(actual, expected);
    }

    #[test]
    fn vertex_render() {
      // the camera should be pointing directly at the vertex
      let vertex: Vector3 = Vector3 { x: 10.0, y: 10.0, z: 1.0 };
      let camera_position: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
      let camera_rotation_horizontal: f32 = 45.0 * (PI/180.0);
      let camera_rotation_vertical: f32 = 0.0 * (PI/180.0);

      // so expected result is (0, 0)
      let (actual, _) = render_vertex(vertex, camera_position, camera_rotation_vertical, camera_rotation_horizontal);
      let expected: Vector2 = Vector2 { x: 0.0, y: 0.0 };

      assert_eq!(actual, expected);
    }

    #[test]
    fn triangle_contains() {
      let a: Vector2 = Vector2 { x: 0.0, y: 0.0 };
      let c: Vector2 = Vector2 { x: 0.0, y: 12.0 };
      let b: Vector2 = Vector2 { x: 12.0, y: 0.0 };
      let triangle: Triangle2D = Triangle2D { a, b, c, color: CustomColor { r: 0, g: 0, b: 0 } };

      let expected_1: bool = true;
      let actual_1: bool = triangle.contains(Vector2 { x: 2.0, y: 2.0 });
      assert_eq!(actual_1, expected_1);

      let expected_2: bool = false;
      let actual_2: bool = triangle.contains(Vector2 { x: 12.0, y: 12.0 });
      assert_eq!(actual_2, expected_2);
    }
    #[test]
    fn indexing() {
      let position: Vector3 = Vector3 { z: 12.0, x: 12.0, y: 12.0 };
      let index: usize = vector3_to_linear_index(position);
      let calculated_position = linear_index_to_vector3(index);

      assert_eq!(position, calculated_position);
    }

    #[test]
    fn triangle_depth() {
      let camera_position: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
      let camera_rotation_horizontal: f32 = 0.0 * (PI/180.0);
      let camera_rotation_vertical: f32 = 0.0 * (PI/180.0);
      let c: Vector3 = Vector3 { x: 0.0, y: 12.0, z: 0.0 };
      let a: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
      let b: Vector3 = Vector3 { x: 12.0, y: 0.0, z: 0.0 };
      let n: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
      let triangle1: Triangle3D = Triangle3D { a, b, c, color: CustomColor { r: 0, g: 0, b: 0 }, normal: n };

      let a2: Vector3 = Vector3 { x: 10.0, y: 10.0, z: 10.0 };
      let c2: Vector3 = Vector3 { x: 10.0, y: 12.0, z: 10.0 };
      let b2: Vector3 = Vector3 { x: 12.0, y: 10.0, z: 10.0 };
      let n2: Vector3 = Vector3 { x: 10.0, y: 10.0, z: 10.0 };
      let triangle2: Triangle3D = Triangle3D { a: a2, b: b2, c: c2, color: CustomColor { r: 0, g: 0, b: 0 }, normal: n2 };

      let (_, depth_1) = render_triangle(triangle1, camera_position, camera_rotation_vertical, camera_rotation_horizontal);
      let (_, depth_2) = render_triangle(triangle2, camera_position, camera_rotation_vertical, camera_rotation_horizontal);

      assert!(depth_1 < depth_2);
    }
    #[test]
    fn angle_to_vector3() {
      let horizontal: f32 = 45.0 * (PI/180.0);
      let vertical: f32 = 45.0 * (PI/180.0);
      let actual_vector: Vector3 = angle_couple_to_vector3(horizontal, vertical);
      let expected_vector = Vector3{x: 0.71, y:0.71, z:0.71};
      assert_eq!(actual_vector, expected_vector);
    }
    /// test if the math to check if the camera is pointing at triangles is correct
    #[test]
    fn triangle_normal_and_camera_dot() {
      // actual coordinates dont matter
      let a: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
      let b: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
      let c: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
      let normal: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
      let tris: Triangle3D = Triangle3D { a, b, c, color: CustomColor { r: 0, g: 0, b: 0 }, normal };

      let camera_rotation_horizontal: f32 = -180.0 * (PI/180.0);
      let camera_rotation_vertical: f32 = -0.0 * (PI/180.0);

      let camera_direction_vector: Vector3 = angle_couple_to_vector3(camera_rotation_horizontal, camera_rotation_vertical);

      println!("tris normal: {:?}\ncamera direction: {:?}", tris.normal, camera_direction_vector);

      let dot_product: f32 = vector3_dot(camera_direction_vector, tris.normal);
      // the camera should be able to see the triangle (although at an angle), so the dot product should be megative
      assert!(dot_product < 0.0);
    }
}