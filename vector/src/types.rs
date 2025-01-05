pub enum Vector {
  Tuple2D(i32, i32),
  Tuple3D(i32, i32, i32),
  Struct2D { x: i32, y: i32 },
  Struct3D { x: i32, y: i32, z: i32 },
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tuple2d_creation() {
    let v = Vector::Tuple2D(1, 2);
    if let Vector::Tuple2D(x, y) = v {
      assert_eq!(x, 1);
      assert_eq!(y, 2);
    } else {
      panic!("Expected Tuple2D");
    }
  }

  #[test]
  fn test_tuple3d_creation() {
      let v = Vector::Tuple3D(1, 2, 3);
      if let Vector::Tuple3D(x, y, z) = v {
          assert_eq!(x, 1);
          assert_eq!(y, 2);
          assert_eq!(z, 3);
      } else {
          panic!("Expected Tuple3D");
      }
  }

  #[test]
  fn test_struct2d_creation() {
    let v = Vector::Struct2D { x: 1, y: 2 };
    if let Vector::Struct2D {x, y} = v {
      assert_eq!(x, 1);
      assert_eq!(y, 2);
    } else {
      panic!("Expected Struct2D")
    }
  }

  #[test]
  fn test_struct3d_creation() {
    let v = Vector::Struct3D { x: 1, y: 2, z: 3 };
    if let Vector::Struct3D {x, y, z} = v {
      assert_eq!(x, 1);
      assert_eq!(y, 2);
      assert_eq!(z, 3);
    } else {
      panic!("Expected Struct3D")
    }
  }
}