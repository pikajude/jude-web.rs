///! The SessionWare middleware is used to store and retrieve user-specific encrypted data.
///!
///! ```
///! use jude_web::middleware::session;
///!
///! fn main() {
///!     let warePair = session::with_key_file("client-key.aes");
///!     let mut chain = Chain::new(warePair);
///!     Iron::new(chain).http("localhost:3000").unwrap();
///!     println!("On 3000!")
///! }
///! ```
pub mod session;
