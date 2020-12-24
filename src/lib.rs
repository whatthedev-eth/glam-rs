/*!
# glam

`glam` is a simple and fast linear algebra library for games and graphics.

## Features

`glam` is built with SIMD in mind. Currently only SSE2 on x86/x86_64 is
supported as this is what stable Rust supports.

* All types support for `f32` and `f64`
* Vector types supported for `i32` and `u32`
* SSE2 storage and optimization for many `f32` types, including `Mat2`, `Mat4`,
  `Quat`, `Vec3A` and `Vec4`
* Scalar fallback implementations exist when SSE2 is not available
* Most functionality includes unit tests and benchmarks

## Linear algebra conventions

`glam` interprets vectors as column matrices (also known as "column vectors")
meaning when transforming a vector with a matrix the matrix goes on the left.

```
use glam::{Mat3, Vec3};
let m = Mat3::identity();
let x = Vec3::unit_x();
let v = m * x;
assert_eq!(v, x);
```

Matrices are stored in memory in column-major order.

## Size and alignment of types

Some `glam` types use SIMD for storage meaning they are 16 byte aligned, these
types include `Mat2`, `Mat4`, `Quat`, `Vec3A` and `Vec4`.

When SSE2 is not available on the target architecture this type will still be 16
byte aligned so that object sizes and layouts will not change between
architectures.

SIMD support can be disabled entirely using the `scalar-math` feature. This
feature will also disable SIMD alignment meaning most types will use native
`f32` alignment of 4 bytes.

All the main `glam` types are `#[repr(C)]`, so they are possible to expose as
struct members to C interfaces if desired. Be mindful of Vec3A's extra padding
though.

## Vec3A

`Vec3A` is a SIMD optimized version of the `Vec3` type, which due to 16 byte
alignment results in `Vec3A` containing 4 bytes of padding making it 16 bytes
in size in total.

| Type  | `f32` bytes | Align bytes | Padding | Size bytes |
|:------|------------:|------------:|--------:|-----------:|
|`Vec3` |           12|            4|        0|          12|
|`Vec3A`|           12|           16|        4|          16|

Despite this wasted space the SIMD version tends to outperform the `f32`
implementation in [**mathbench**](https://github.com/bitshifter/mathbench-rs)
benchmarks.

`glam` treats `Vec3` as the default vector 3 type and `Vec3A` a special case for
optimization. When methods need to return a vector 3 type they will generally
return `Vec3`.

There are `From` trait implementations for converting from `Vec4` to a `Vec3A`
and between `Vec3` and `Vec3A` (and vice versa).

```
use glam::{Vec3, Vec3A, Vec4};

let v4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

// Convert from `Vec4` to `Vec3A`, this is a no-op if SIMD is supported.
let v3a = Vec3A::from(v4);
assert_eq!(Vec3A::new(1.0, 2.0, 3.0), v3a);

// Convert from `Vec3A` to `Vec3`.
let v3 = Vec3::from(v3a);
assert_eq!(Vec3::new(1.0, 2.0, 3.0), v3);

// Convert from `Vec3` to `Vec3A`.
let v3a = Vec3A::from(v3);
assert_eq!(Vec3A::new(1.0, 2.0, 3.0), v3a);
```

## Vector swizzles

`glam` vector types have functions allowing elements of vectors to be reordered,
this includes creating a vector of a different size from the vectors elements.

The swizzle functions are implemented using traits to add them to each vector
type. This is primarily because there are a lot of swizzle functions which can
obfuscate the other vector functions in documentation and so on. The traits are
`Vec2Swizzles`, `Vec3Swizzles`, `Vec3ASwizzles` and `Vec4Swizzles`.

Note that `Vec2Swizzles`, `Vec3Swizzles` and `Vec4Swizzles` will return a `Vec3`
type for 3 element swizzles. `Vec3ASwizzles` will return a `Vec3A` for 3 element
swizzles.

```
use glam::{swizzles::*, Vec2, Vec3, Vec3A, Vec4};

let v = Vec4::new(1.0, 2.0, 3.0, 4.0);

// Reverse elements of `v`, if SIMD is supported this will use a vector shuffle.
let wzyx = v.wzyx();
assert_eq!(Vec4::new(4.0, 3.0, 2.0, 1.0), wzyx);

// Swizzle the yzw elements of `v` into a `Vec3`
let yzw = v.yzw();
assert_eq!(Vec3::new(2.0, 3.0, 4.0), yzw);

// To swizzle a `Vec4` into a `Vec3A` swizzle the `Vec4` first then convert to
// `Vec3A`. If SIMD is supported this will use a vector shuffle. The last
// element of the shuffled `Vec4` is ignored by the `Vec3A`.
let yzw = Vec3A::from(v.yzwx());
assert_eq!(Vec3A::new(2.0, 3.0, 4.0), yzw);

// You can swizzle from a `Vec4` to a `Vec2`
let xy = v.xy();
assert_eq!(Vec2::new(1.0, 2.0), xy);

// And back again
let yyxx = xy.yyxx();
assert_eq!(Vec4::new(2.0, 2.0, 1.0, 1.0), yyxx);
```

## SIMD and scalar consistency

`glam` types implement `serde` `Serialize` and `Deserialize` traits to ensure
that they will serialize and deserialize exactly the same whether or not
SIMD support is being used.

The SIMD versions implement the `core::fmt::Debug` and `core::fmt::Display`
traits so they print the same as the scalar version.

```
use glam::Vec4;
let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
assert_eq!(format!("{}", a), "[1, 2, 3, 4]");
```

## Feature gates

All `glam` dependencies are optional, however some are required for tests
and benchmarks.

* `std` - the default feature, has no dependencies.
* `rand` - used to generate random values. Used in benchmarks.
* `serde` - used for serialization and deserialization of types.
* `mint` - used for interoperating with other linear algebra libraries.
* `scalar-math` - disables SIMD support and uses native alignment for all
  types.
* `debug-glam-assert` - adds assertions in debug builds which check the validity
  of parameters passed to `glam` to help catch runtime errors.
* `glam-assert` - adds assertions to all builds which check the validity of
  parameters passed to `glam` to help catch runtime errors.

### Minimum Supported Version or Rust (MSVR)

The minimum supported version of Rust for `glam` is `1.36.0`.

*/
#![doc(html_root_url = "https://docs.rs/glam/0.11.3")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(target_arch = "spirv", feature(register_attr, repr_simd))]

#[cfg(all(target_arch = "spirv", feature = "std"))]
compile_error!("`std` feature is not supported when building for SPIRV");

#[cfg(all(target_arch = "spirv", feature = "glam-assert"))]
compile_error!("`glam-assert` feature is not supported when building for SPIRV");

#[cfg(all(target_arch = "spirv", feature = "debug-glam-assert"))]
compile_error!("`debug-glam-assert` feature is not supported when building for SPIRV");

#[cfg(all(target_arch = "spirv", feature = "serde"))]
compile_error!("`serde` feature is not supported when building for SPIRV");

#[cfg(all(target_arch = "spirv", feature = "rand"))]
compile_error!("`rand` feature is not supported when building for SPIRV");

#[cfg(all(target_arch = "spirv", feature = "bytemuck"))]
compile_error!("`bytemuck` feature is not supported when building for SPIRV");

#[macro_use]
mod macros;
#[macro_use]
mod vec;

mod core;
mod mat2;
mod mat3;
mod mat4;
mod quat;
pub mod swizzles;
mod vec2;
mod vec3;
mod vec4;
mod vec_mask;

#[doc(hidden)]
pub mod f32;

#[doc(hidden)]
pub use self::core::storage::{XY, XYZ, XYZW};

pub use self::mat2::{dmat2, DMat2};
pub use self::mat3::{dmat3, DMat3};
pub use self::mat4::{dmat4, DMat4};
pub use self::quat::{dquat, DQuat};
pub use self::vec2::{dvec2, DVec2};
pub use self::vec3::{dvec3, DVec3};
pub use self::vec4::{dvec4, DVec4};

pub use self::vec2::{ivec2, IVec2};
pub use self::vec3::{ivec3, IVec3};
pub use self::vec4::{ivec4, IVec4};

pub use self::vec2::{uvec2, UVec2};
pub use self::vec3::{uvec3, UVec3};
pub use self::vec4::{uvec4, UVec4};
pub use self::vec_mask::{UVec2Mask, UVec3Mask, UVec4Mask};

pub use self::mat2::{mat2, Mat2};
pub use self::mat3::{mat3, Mat3};
pub use self::mat4::{mat4, Mat4};
pub use self::quat::{quat, Quat};
pub use self::vec2::{vec2, Vec2};
pub use self::vec3::{vec3, vec3a, Vec3, Vec3A};
pub use self::vec4::{vec4, Vec4};
pub use self::vec_mask::{Vec2Mask, Vec3AMask, Vec3Mask, Vec4Mask};

pub use self::swizzles::{Vec2Swizzles, Vec3ASwizzles, Vec3Swizzles, Vec4Swizzles};

#[cfg(feature = "transform-types")]
pub use self::f32::{TransformRT, TransformSRT};
