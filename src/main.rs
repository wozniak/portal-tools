#![windows_subsystem = "windows"]

mod ui;
use native_windows_gui as nwg;

use nwg::NativeUi;
use std::{fs, path};
use ui::PortalTools;
use image::Pixel;

#[derive(Default, Copy, Clone)]
// Simple color struct for referencing colors by their r/g/b channels
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Default, Copy, Clone)]
// Simple struct as a container for the portal and prop carry colors
struct Colors {
    blue: Color,
    orange: Color,
    carry: Color,
}

impl IntoIterator for Colors {
    type Item = Color;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.blue, self.orange, self.carry].into_iter()
    }
}

impl<T: PartialEq + Into<u8>> std::ops::Index<T> for Colors {
    type Output = Color;

    fn index(&self, index: T) -> &Self::Output {
        match index.into() {
            0 => &self.blue,
            1 => &self.orange,
            2 => &self.carry,
            _ => panic!("index out of bounds")
        }
    }
}

impl Colors {
    // create instance of `Colors`
    pub fn new(colors: &[Vec<u8>]) -> Self {
        let blue = Color {
            r: colors[0][0],
            g: colors[0][1],
            b: colors[0][2],
        };
        let carry = Color {
            r: colors[2][0],
            g: colors[2][1],
            b: colors[2][2],
        };
        let orange = Color {
            r: colors[1][0],
            g: colors[1][1],
            b: colors[1][2],
        };

        Self {
            blue,
            orange,
            carry,
        }
    }
}

fn grey_to_color(i: &image::DynamicImage, c: image::Rgba<u8>) -> image::DynamicImage {
    image::DynamicImage::ImageRgba8(imageproc::map::map_colors(
        &i.clone().into_luma_alpha(),
        |px| {
            let mut color = c.map_without_alpha(|c| (c as f32 * (px.0[0] as f32 / 255.)) as u8);
            color.channels_mut()[3] = px.0[1];

            color
        }
    ))
}

impl PortalTools {
    // main apply function
    pub fn apply(&self) {
        let do_portals = nwg::CheckBoxState::Checked == self.portals_check.check_state();
        let do_crosshair = nwg::CheckBoxState::Checked == self.crosshair_check.check_state();

        // if no boxes checked, do nothing
        if !(do_portals || do_crosshair) {
            return;
        }

        // check for client dll and gameinfo
        for file in &["portal/bin/client.dll", "portal/gameinfo.txt"] {
            if !std::path::Path::new(&self.game_box.text())
                .join(file)
                .is_file()
            {
                let _ = nwg::modal_error_message(
                    &self.window,
                    "Portal Tools",
                    "Invalid game dir (pick the folder with hl2.exe",
                );
                return;
            }
        }

        let colors = {
            let blue = if let Ok(v) = hex::decode(self.blue_box.text()) {
                v
            } else {
                self.invalid_colors();
                return;
            };
            let orange = if let Ok(v) = hex::decode(self.orange_box.text()) {
                v
            } else {
                self.invalid_colors();
                return;
            };
            let carry = if let Ok(v) = hex::decode(self.carry_box.text()) {
                v
            } else {
                self.invalid_colors();
                return;
            };

            Colors::new(&[blue, orange, carry])
        };

        let mut error = String::new();

        if do_portals {
            if let Err(e) = self.portals(colors) {
                error = e;
            }
        }
        if do_crosshair {
            if let Err(e) = self.crosshair(colors) {
                error = e;
            }
        }

        if error.len() == 0 {
            let _ = nwg::modal_info_message(&self.window, "Portal Tools", "Done!");
        } else {
            let _ = nwg::modal_error_message(
                &self.window,
                "Portal Tools",
                format!("Error: {}", error).as_str(),
            );
        }
    }

    // complain about invalid colors
    fn invalid_colors(&self) {
        let _ = nwg::modal_error_message(&self.window, "Portal Tools", "Invalid hex colors");
    }

    // open binary file based on unpack dir
    fn bytes(&self, p: &str) -> Vec<u8> {
        fs::read(std::path::Path::new(&self.game_box.text()).join(p)).unwrap()
    }

    // write bytes to file based on unpack dir
    fn write(&self, p: &str, vec: Vec<u8>) {
        let _ = fs::write(self.path(p), vec);
    }

    // helper func for getting path based on game dir
    fn path(&self, p: &str) -> path::PathBuf {
        std::path::Path::new(&self.game_box.text()).join(p)
    }

    fn steampipe(&self) -> bool {
        self.path("portal/portal_pak_dir.vpk").is_file()
    }

    fn prefix(&self) -> String {
        if self.steampipe() {
            fs::create_dir_all(
                self.path("portal/custom/portaltools/materials/models/portals/"),
            )
            .expect("your hard drive is dying");
            "portal/custom/portaltools/".to_string()
        } else {
            "portal/".to_string()
        }
    }

    fn portals(&self, colors: Colors) -> Result<(), String> {
        let grey_dx9 = image::load_from_memory(include_bytes!("dx9.png")).unwrap();
        let grey_dx8 = image::load_from_memory(include_bytes!("dx8.png")).unwrap();

        for i in 0..2u8 {
            let color = colors[i];

            let color_name = match i {
                0 => "blue",
                1 => "orange",
                _ => unreachable!(),
            };

            let color_dx9 = grey_to_color(&grey_dx9, image::Rgba([color.r, color.g, color.b, 255]));
            let color_dx8 = grey_to_color(&grey_dx8, image::Rgba([color.r, color.g, color.b, 255]));

            let vtf_bytes_dx9 = vtf::create(color_dx9, vtf::ImageFormat::Rgba8888).unwrap();
            let vtf_bytes_dx8 = vtf::create(color_dx8, vtf::ImageFormat::Rgba8888).unwrap();

            let mut prefix = "portal";

            self.write(
                &format!(
                    "{}/materials/models/portals/portal-{}-color.vtf",
                    self.prefix(), color_name
                ),
                vtf_bytes_dx9,
            );
            self.write(
                &format!(
                    "{}/materials/models/portals/portal-{}-color-dx8.vtf",
                    self.prefix(), color_name
                ),
                vtf_bytes_dx8,
            );
        }

        Ok(())
    }

    // apply crosshair changes
    fn crosshair(&self, colors: Colors) -> Result<(), String> {
        let mut cdll = self.bytes("portal/bin/client.dll");

        let bl = colors.blue;
        let or = colors.orange;
        let ca = colors.carry;

        if !self.steampipe() {
            let patch = [
                0x8B, 0x44, 0x24, 0x08, 0x83, 0xE8, 0x00, 0x74, 0x37, 0x83, 0xE8, 0x01, 0xB1, 0xFF,
                0x74, 0x20, 0x83, 0xE8, 0x01, 0x8B, 0x44, 0x24, 0x04, 0xC6, 0x00, or.r, 0xC6, 0x40,
                0x03, 0xFF, 0x74, 0x07, 0x88, 0x48, 0x01, 0x88, 0x48, 0x02, 0xC3, 0xC6, 0x40, 0x01,
                or.g, 0xC6, 0x40, 0x02, or.b, 0xC3, 0x8B, 0x44, 0x24, 0x04, 0xC6, 0x00, bl.r, 0xC6,
                0x40, 0x01, bl.g, 0xC6, 0x40, 0x02, bl.b, 0xC3, 0x8B, 0x44, 0x24, 0x04, 0xC6, 0x00,
                ca.r, 0xC6, 0x40, 0x01, ca.g, 0xC6, 0x40, 0x02, ca.b, 0xC6,
            ];

            let pos = if let Some(n) = cdll
                .windows(8)
                .position(|s| s == [0x40, 0x03, 0xFF, 0xC3, 0xCC, 0xCC, 0xCC, 0xCC])
            {
                n - patch.len()
            } else {
                return Err("invalid client.dll".to_string());
            };

            for i in 0..patch.len() {
                cdll[i + pos] = patch[i];
            }
        } else {
            cdll[0x001c7a49] = colors.blue.r;
            cdll[0x001c7a49 + 1] = colors.blue.g;
            cdll[0x001c7a49 + 2] = colors.blue.b;

            cdll[0x001c7a3e] = colors.orange.r;
            cdll[0x001c7a3e + 1] = colors.orange.g;
            cdll[0x001c7a3e + 2] = colors.orange.b;

            cdll[0x001c7a54] = colors.carry.r;
            cdll[0x001c7a54 + 1] = colors.carry.g;
            cdll[0x001c7a54 + 2] = colors.carry.b;
        }

        if let Err(e) = fs::write(self.path("portal/bin/client.dll"), cdll) {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init NWG");

    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .family("Segoe UI")
        .size(18)
        .build(&mut font)
        .expect("what the fuck!?");

    nwg::Font::set_global_default(Some(font));

    let _app = PortalTools::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
