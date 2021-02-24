use crate::colors;

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
    /// Generate RGBA data from a pixmap
    pub fn new(pixmap: &[&str]) -> Result<RgbaImage, EzPixmapError> {
        let x_color_map = colors::init_colors();
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
                col.c = chars[0];
                if chars[2] != 'c' {
                    return Err(EzPixmapError::Internal(EzPixmapErrorKind::InvalidFormat));
                }
                let color: String = chars[4..].iter().collect();
                if color.starts_with('#') {
                    // shouldn't fail
                    let color = color.strip_prefix("#").unwrap();
                    let r = u8::from_str_radix(&color[0..2], 16)?;
                    let g = u8::from_str_radix(&color[2..4], 16)?;
                    let b = u8::from_str_radix(&color[4..6], 16)?;
                    let a = 255;
                    col.col = (r, g, b, a);
                } else {
                    col.col = *x_color_map.get(color.as_str()).unwrap_or(&(255, 255, 255, 0));
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
