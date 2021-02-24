//! # ez-pixmap
//!
//! A naive and easy inline pixmap (xpm-like) image crate. This is non-compliant with xpm image format, however it's close enough.
//! - Doesn't support monochrome nor symbolics.
//! - Supports nly 1 character per pixel.
//!
//! ## Usage
//! ```ignored
//! [dependencies]
//! ez-pixmap = "0.1"
//! ```
//!
//! ```no_run
//! extern crate ez_pixmap;
//!
//! const PXM: &[&str] = &[
//!     "50 34 4 1",
//!     "  c black",
//!     "o c #ff9900",
//!     "@ c white",
//!     "# c None",
//!     "##################################################",
//!     "###      ##############################       ####",
//!     "### ooooo  ###########################  ooooo ####",
//!     "### oo  oo  #########################  oo  oo ####",
//!     "### oo   oo  #######################  oo   oo ####",
//!     "### oo    oo  #####################  oo    oo ####",
//!     "### oo     oo  ###################  oo     oo ####",
//!     "### oo      oo                     oo      oo ####",
//!     "### oo       oo  ooooooooooooooo  oo       oo ####",
//!     "### oo        ooooooooooooooooooooo        oo ####",
//!     "### oo     ooooooooooooooooooooooooooo    ooo ####",
//!     "#### oo   ooooooo ooooooooooooo ooooooo   oo #####",
//!     "####  oo oooooooo ooooooooooooo oooooooo oo  #####",
//!     "##### oo oooooooo ooooooooooooo oooooooo oo ######",
//!     "#####  o ooooooooooooooooooooooooooooooo o  ######",
//!     "###### ooooooooooooooooooooooooooooooooooo #######",
//!     "##### ooooooooo     ooooooooo     ooooooooo ######",
//!     "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
//!     "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
//!     "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
//!     "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
//!     "##### ooooooooo     ooooooooo     ooooooooo ######",
//!     "###### oooooooooooooo       oooooooooooooo #######",
//!     "###### oooooooo@@@@@@@     @@@@@@@oooooooo #######",
//!     "###### ooooooo@@@@@@@@@   @@@@@@@@@ooooooo #######",
//!     "####### ooooo@@@@@@@@@@@ @@@@@@@@@@@ooooo ########",
//!     "######### oo@@@@@@@@@@@@ @@@@@@@@@@@@oo ##########",
//!     "########## o@@@@@@ @@@@@ @@@@@ @@@@@@o ###########",
//!     "########### @@@@@@@     @     @@@@@@@ ############",
//!     "############  @@@@@@@@@@@@@@@@@@@@@  #############",
//!     "##############  @@@@@@@@@@@@@@@@@  ###############",
//!     "################    @@@@@@@@@    #################",
//!     "####################         #####################",
//!     "##################################################",
//! ];
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let my_image = ez_pixmap::RgbaImage::new(PXM)?;
//!     assert_eq!(my_image.width(), 50);
//!     assert_eq!(my_image.height(), 34);
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

mod colors;
mod image;
pub use crate::image::*;

#[macro_use]
extern crate lazy_static;
