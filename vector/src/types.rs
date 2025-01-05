pub enum Vector {
  Tuple2D(i32, i32),
  Tuple3D(i32, i32, i32),
  Struct2D { x: i32, y: i32 },
  Struct3D { x: i32, y: i32, z: i32 },
}