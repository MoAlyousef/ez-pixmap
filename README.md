# ez-pixmap

A naive and easy inline pixmap (xpm-like) image decoder. 
This is non-compliant with the xpm image format, however it's close enough.
- Doesn't support monochrome nor symbolics.
- Supports only 1 character per pixel.

Main use case: Simple icon art.

## Usage
```toml
[dependencies]
ez-pixmap = "0.2"
```

```rust
extern crate ez_pixmap;

const PXM: &[&str] = &[
    "50 34 4 1", // <width> <height> <num of colors> <chars/pixels>
    "  c black", // <char> c <color>
    "o c #ff9900",
    "@ c white",
    "# c None", 
    // pixels
    "##################################################",
    "###      ##############################       ####",
    "### ooooo  ###########################  ooooo ####",
    "### oo  oo  #########################  oo  oo ####",
    "### oo   oo  #######################  oo   oo ####",
    "### oo    oo  #####################  oo    oo ####",
    "### oo     oo  ###################  oo     oo ####",
    "### oo      oo                     oo      oo ####",
    "### oo       oo  ooooooooooooooo  oo       oo ####",
    "### oo        ooooooooooooooooooooo        oo ####",
    "### oo     ooooooooooooooooooooooooooo    ooo ####",
    "#### oo   ooooooo ooooooooooooo ooooooo   oo #####",
    "####  oo oooooooo ooooooooooooo oooooooo oo  #####",
    "##### oo oooooooo ooooooooooooo oooooooo oo ######",
    "#####  o ooooooooooooooooooooooooooooooo o  ######",
    "###### ooooooooooooooooooooooooooooooooooo #######",
    "##### ooooooooo     ooooooooo     ooooooooo ######",
    "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
    "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
    "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
    "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
    "##### ooooooooo     ooooooooo     ooooooooo ######",
    "###### oooooooooooooo       oooooooooooooo #######",
    "###### oooooooo@@@@@@@     @@@@@@@oooooooo #######",
    "###### ooooooo@@@@@@@@@   @@@@@@@@@ooooooo #######",
    "####### ooooo@@@@@@@@@@@ @@@@@@@@@@@ooooo ########",
    "######### oo@@@@@@@@@@@@ @@@@@@@@@@@@oo ##########",
    "########## o@@@@@@ @@@@@ @@@@@ @@@@@@o ###########",
    "########### @@@@@@@     @     @@@@@@@ ############",
    "############  @@@@@@@@@@@@@@@@@@@@@  #############",
    "##############  @@@@@@@@@@@@@@@@@  ###############",
    "################    @@@@@@@@@    #################",
    "####################         #####################",
    "##################################################",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_image = ez_pixmap::RgbaImage::from(PXM)?;
    assert_eq!(my_image.width(), 50);
    assert_eq!(my_image.height(), 34);
    assert_eq!(my_image.data().len(), 50 * 34 * 4); // since it's rgba
    Ok(())
}
```

The list of supported color names can be found [here](https://github.com/MoAlyousef/ez-pixmap/blob/main/src/colors.rs).

## Examples
Check the examples directory for an example usage with the minifb and image crates.
```
$ cargo run --example image
$ cargo run --example minifb
```

![alt_test](screenshots/image.png)

![alt_test](screenshots/minifb.png)


