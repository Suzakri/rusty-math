// Get Vector type
use super::types::Vector;

impl Vector {
    /// Scale the vector by a scalar value
    /// 
    /// # Parameters
    /// - `scalar`: Scalar value to multiply each element by
    /// 
    /// # Returns
    /// - New `Vector` of same type with each element scaled
    /// 
    /// # Example
    /// ```rust
    /// let v = Vector::Tuple2D(1, 2);
    /// let scaled = v.scale(2);
    /// 
    /// match scaled {
    ///   Vector::Tuple2D(x, y) => assert_eq!((x, y), (2, 4)),
    ///   _ => panic!("Unexpected vector type"),
    /// }
    /// ```
    pub fn scale(&self, scalar: i32) -> Self {

        match self {
            Vector::Tuple2D(x, y) => Vector::Tuple2D(x * scalar, y * scalar),
            Vector::Tuple3D(x, y, z) => Vector::Tuple3D(x * scalar, y * scalar, z * scalar),
            Vector::Struct2D { x, y } => Vector::Struct2D {
                x: x * scalar,
                y: y * scalar,
            },
            Vector::Struct3D { x, y, z } => Vector::Struct3D {
                x: x * scalar,
                y: y * scalar,
                z: z * scalar,
            },
        }
    }
}

#[cfg(test)]
mod test {
  use super::super::types::Vector;

  #[test]
  fn test_scale_tuple2d() {
    let v = Vector::Tuple2D(1, 2);
    let scaled = v.scale(2);
    
    match scaled {
      Vector::Tuple2D(x, y) => {
        assert_eq!(x, 2);
        assert_eq!(y, 4);
      }
      _ => panic!("Error scaling vector type")
    }
  }

  #[test]
  fn test_scale_tuple3d() {
    let v = Vector::Tuple3D(1, 2, 3);
    let scaled = v.scale(2);
    
    match scaled {
      Vector::Tuple3D(x, y, z) => {
        assert_eq!(x, 2);
        assert_eq!(y, 4);
        assert_eq!(z, 6);
      }
      _ => panic!("Error scaling vector type")
    }
  }

  #[test]
  fn test_scale_struct2d() {
    let v = Vector::Struct2D {x: 1, y: 2};
    let scaled = v.scale(2);
    
    match scaled {
      Vector::Struct2D {x, y} => {
        assert_eq!(x, 2);
        assert_eq!(y, 4);
      }
      _ => panic!("Error scaling vector type")
    }
  }

  #[test]
  fn test_scale_struct3d() {
    let v = Vector::Struct3D {x: 1, y: 2, z: 3};
    let scaled = v.scale(2);
    
    match scaled {
      Vector::Struct3D {x, y, z} => {
        assert_eq!(x, 2);
        assert_eq!(y, 4);
        assert_eq!(z, 6);
      }
      _ => panic!("Error scaling vector type")
    }
  }
}