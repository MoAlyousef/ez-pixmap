# ez-pixmap

A naive and easy inline pixmap image format crate. This is non-compliant with xpm image format, however it's close enough.
- It only supports rgba hex values, no monochrome nor symbolics.
- Only 1 character per pixel.

## Usage
```toml
[dependencies]
ez-pixmap = "0.1"
```

```rust
extern crate ez_pixmap;

const PXM: &[&str] = &[
    "50 34 4 1",
    "  c #000000ff",
    "o c #ff9900ff",
    "@ c #ffffffff",
    "# c #ffffff00", // transparent
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
    let my_image = ez_pixmap::RgbaImage::new(PXM)?;
    assert_eq!(my_image.width(), 50);
    assert_eq!(my_image.height(), 34);
    Ok(())
}
```

## Examples
Check the examples directory for an example usage with the image crate.
```
$ cargo run --example example1
```

![alt_test](examples/image.png)


