//! This is a library which allows anyone to change various properties of their
//! strings with a heavy emphasis on performance. Allows programmers to
//! manipulate a single programatic name consistently in multiple contexts.
//!
//! # Example
//! ```rust
//! // Remember to import the `Inflect` trait!
//! use inflections::Inflect;
//!
//! assert_eq!("Hello World".to_camel_case(), "helloWorld".to_owned());
//! ```

pub mod case;

/// A trait which proxies the other methods from this crate in a method syntax
/// on `&str` and `String`. The only ways these methods change is the string
/// parameter becomes `&self`.
///
/// If your are trying to implement this trait for your own types, be warned
/// methods may be added without a breaking change in semantic versioning.
///
/// # Example
/// ```rust
/// // Remember to import the `Inflect` trait!
/// use inflections::Inflect;
///
/// assert_eq!("Hello World".to_camel_case(), "helloWorld".to_owned());
/// ```
pub trait Inflect {
  fn to_sentence_case(&self) -> String;
  fn is_sentence_case(&self) -> bool;
  fn to_title_case(&self) -> String;
  fn is_title_case(&self) -> bool;
  fn to_camel_case(&self) -> String;
  fn is_camel_case(&self) -> bool;
  fn to_pascal_case(&self) -> String;
  fn is_pascal_case(&self) -> bool;
  fn to_kebab_case(&self) -> String;
  fn is_kebab_case(&self) -> bool;
  fn to_train_case(&self) -> String;
  fn is_train_case(&self) -> bool;
  fn to_snake_case(&self) -> String;
  fn is_snake_case(&self) -> bool;
  fn to_constant_case(&self) -> String;
  fn is_constant_case(&self) -> bool;
}

impl<'a> Inflect for &'a str {
  #[inline] fn to_sentence_case(&self) -> String { case::to_sentence_case(self) }
  #[inline] fn is_sentence_case(&self) -> bool { case::is_sentence_case(self) }
  #[inline] fn to_title_case(&self) -> String { case::to_title_case(self) }
  #[inline] fn is_title_case(&self) -> bool { case::is_title_case(self) }
  #[inline] fn to_camel_case(&self) -> String { case::to_camel_case(self) }
  #[inline] fn is_camel_case(&self) -> bool { case::is_camel_case(self) }
  #[inline] fn to_pascal_case(&self) -> String { case::to_pascal_case(self) }
  #[inline] fn is_pascal_case(&self) -> bool { case::is_pascal_case(self) }
  #[inline] fn to_kebab_case(&self) -> String { case::to_kebab_case(self) }
  #[inline] fn is_kebab_case(&self) -> bool { case::is_kebab_case(self) }
  #[inline] fn to_train_case(&self) -> String { case::to_train_case(self) }
  #[inline] fn is_train_case(&self) -> bool { case::is_train_case(self) }
  #[inline] fn to_snake_case(&self) -> String { case::to_snake_case(self) }
  #[inline] fn is_snake_case(&self) -> bool { case::is_snake_case(self) }
  #[inline] fn to_constant_case(&self) -> String { case::to_constant_case(self) }
  #[inline] fn is_constant_case(&self) -> bool { case::is_constant_case(self) }
}

impl Inflect for String {
  #[inline] fn to_sentence_case(&self) -> String { case::to_sentence_case(&self) }
  #[inline] fn is_sentence_case(&self) -> bool { case::is_sentence_case(&self) }
  #[inline] fn to_title_case(&self) -> String { case::to_title_case(&self) }
  #[inline] fn is_title_case(&self) -> bool { case::is_title_case(&self) }
  #[inline] fn to_camel_case(&self) -> String { case::to_camel_case(&self) }
  #[inline] fn is_camel_case(&self) -> bool { case::is_camel_case(&self) }
  #[inline] fn to_pascal_case(&self) -> String { case::to_pascal_case(&self) }
  #[inline] fn is_pascal_case(&self) -> bool { case::is_pascal_case(&self) }
  #[inline] fn to_kebab_case(&self) -> String { case::to_kebab_case(&self) }
  #[inline] fn is_kebab_case(&self) -> bool { case::is_kebab_case(&self) }
  #[inline] fn to_train_case(&self) -> String { case::to_train_case(&self) }
  #[inline] fn is_train_case(&self) -> bool { case::is_train_case(&self) }
  #[inline] fn to_snake_case(&self) -> String { case::to_snake_case(&self) }
  #[inline] fn is_snake_case(&self) -> bool { case::is_snake_case(&self) }
  #[inline] fn to_constant_case(&self) -> String { case::to_constant_case(&self) }
  #[inline] fn is_constant_case(&self) -> bool { case::is_constant_case(&self) }
}
