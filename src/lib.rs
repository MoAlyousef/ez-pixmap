//! # ez-pixmap
//!
//! A naive and easy inline pixmap crate. This is non-compliant with xpm image format, however it's close enough.
//! - It only supports rgba hex values, no monochrome nor symbolics.
//! - Only 1 character per pixel.
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
//!     "  c #000000ff",
//!     "o c #ff9900ff",
//!     "@ c #ffffffff",
//!     "# c #ffffff00", // transparent
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
//!     Ok(())
//! }
//! ```


#![warn(missing_docs)]

/// EzPixmap Error types
#[derive(Debug)]
pub enum EzPixmapError {
    /// Parse error
    ParseError(std::num::ParseIntError),
    /// Internal error
    Internal(EzPixmapErrorKind),
}

/// EzPixmap error kinds
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EzPixmapErrorKind {
    /// Invalid EzPixmap format
    InvalidFormat,
    /// Xpm feature not implemented
    NotImplemented,
}

impl std::error::Error for EzPixmapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EzPixmapError::ParseError(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for EzPixmapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            EzPixmapError::ParseError(ref err) => err.fmt(f),
            EzPixmapError::Internal(ref err) => write!(f, "An internal error occured {:?}", err),
        }
    }
}

impl From<std::num::ParseIntError> for EzPixmapError {
    fn from(err: std::num::ParseIntError) -> EzPixmapError {
        EzPixmapError::ParseError(err)
    }
}

#[derive(Default, Clone)]
struct Header {
    w: u32,
    h: u32,
    num_colors: u32,
    ppc: u32,
}

#[derive(Default, Clone, Copy)]
struct ColorMap {
    c: char,
    col: (u8, u8, u8, u8),
}

/// Struct containing Rgba data
#[derive(Debug, Clone)]
pub struct RgbaImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl RgbaImage {
    /// Generate RGB data from a pixmap
    pub fn new(pixmap: &[&str]) -> Result<RgbaImage, EzPixmapError> {
        let mut header = Header::default();
        let mut data = vec![];
        let mut col_vec: Vec<ColorMap> = vec![];
        for i in 0..pixmap.len() {
            if i == 0 {
                let line = pixmap[0];
                let vals: Vec<&str> = line.split_ascii_whitespace().collect();
                header.w = vals[0].parse()?;
                header.h = vals[1].parse()?;
                header.num_colors = vals[2].parse()?;
                header.ppc = vals[3].parse()?;
                if header.ppc != 1 {
                    return Err(EzPixmapError::Internal(EzPixmapErrorKind::InvalidFormat));
                }
                continue;
            }
            if i <= header.num_colors as usize {
                let mut col = ColorMap::default();
                let line = pixmap[i];
                let chars: Vec<char> = line.chars().collect();
                let vals: Vec<&str> = line.split_ascii_whitespace().collect();
                col.c = chars[0];
                if chars[2] != 'c' {
                    return Err(EzPixmapError::Internal(EzPixmapErrorKind::InvalidFormat));
                }
                let last = vals.len() - 1;
                let color = vals[last];
                if color.starts_with('#') {
                    // shouldn't fail
                    let color = vals[last].strip_prefix("#").unwrap();
                    let r = u8::from_str_radix(&color[0..2], 16)?;
                    let g = u8::from_str_radix(&color[2..4], 16)?;
                    let b = u8::from_str_radix(&color[4..6], 16)?;
                    let a = u8::from_str_radix(&color[6..8], 16)?;
                    col.col = (r, g, b, a);
                } else {
                    return Err(EzPixmapError::Internal(EzPixmapErrorKind::NotImplemented));
                }
                col_vec.push(col);
                continue;
            }
            let line = pixmap[i];
            let chars: Vec<char> = line.chars().collect();
            for c in chars {
                for elem in &col_vec {
                    if c == elem.c {
                        data.push(elem.col.0);
                        data.push(elem.col.1);
                        data.push(elem.col.2);
                        data.push(elem.col.3);
                    }
                }
            }
        }
        if data.len() != (header.w * header.h * 4) as usize {
            return Err(EzPixmapError::Internal(EzPixmapErrorKind::InvalidFormat));
        }
        Ok(RgbaImage {
            data,
            width: header.w,
            height: header.h,
        })
    }

    /// Get the data of the image
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get the width of the RgbaImage
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the RgbaImage
    pub fn height(&self) -> u32 {
        self.height
    }
}
