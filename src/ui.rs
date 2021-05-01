use native_windows_derive as nwd;
use native_windows_gui as nwg;

use nwd::NwgUi;

#[derive(Default, NwgUi)]
pub struct PortalTools {
    #[nwg_control(size: (360, 150), position: (300, 300), title: "Portal Tools", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()] )]
    pub window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    pub grid: nwg::GridLayout,

    // text labels
    #[nwg_control(text: "Blue Portal")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    blue_text: nwg::Label,

    #[nwg_control(text: "Orange Portal")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0)]
    orange_text: nwg::Label,

    #[nwg_control(text: "Prop Carry")]
    #[nwg_layout_item(layout: grid, row: 2, col: 0)]
    carry_text: nwg::Label,

    // color text boxes
    #[nwg_control(text: "40A0FF")]
    #[nwg_layout_item(layout: grid, row: 0, col: 1, col_span: 2)]
    pub blue_box: nwg::TextInput,

    #[nwg_control(text: "ff9a00")]
    #[nwg_layout_item(layout: grid, row: 1, col: 1, col_span: 2)]
    pub orange_box: nwg::TextInput,

    #[nwg_control(text: "F2CAA7")]
    #[nwg_layout_item(layout: grid, row: 2, col: 1, col_span: 2)]
    pub carry_box: nwg::TextInput,

    // picker buttons
    #[nwg_layout_item(layout: grid, row: 0, col: 3)]
    #[nwg_control(text: "Pick")]
    #[nwg_events(OnButtonClick: [PortalTools::pick_blue])]
    blue_button: nwg::Button,

    #[nwg_layout_item(layout: grid, row: 1, col: 3)]
    #[nwg_control(text: "Pick")]
    #[nwg_events(OnButtonClick: [PortalTools::pick_orange])]
    orange_button: nwg::Button,

    #[nwg_layout_item(layout: grid, row: 2, col: 3)]
    #[nwg_control(text: "Pick")]
    #[nwg_events(OnButtonClick: [PortalTools::pick_carry])]
    carry_button: nwg::Button,

    // game folder stuff
    #[nwg_control(text: "Portal folder")]
    #[nwg_layout_item(layout: grid, col: 0, row: 3)]
    game_text: nwg::Label,

    #[nwg_layout_item(layout: grid, row: 3, col: 3)]
    #[nwg_control(text: "Browse")]
    #[nwg_events(OnButtonClick: [PortalTools::pick_game])]
    game_button: nwg::Button,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 3, col: 1, col_span: 2)]
    pub game_box: nwg::TextInput,

    // feature checkboxes and apply
    #[nwg_control(text: "Portals")]
    #[nwg_layout_item(layout: grid, row: 4, col: 1)]
    pub portals_check: nwg::CheckBox,

    #[nwg_control(text: "Crosshair")]
    #[nwg_layout_item(layout: grid, row: 4, col: 2)]
    pub crosshair_check: nwg::CheckBox,

    #[nwg_control(text: "Apply")]
    #[nwg_layout_item(layout: grid, col: 0, row: 4)]
    #[nwg_events(OnButtonClick: [PortalTools::apply])]
    pub hello_button: nwg::Button,

    // color pickers
    #[nwg_resource]
    blue_color: nwg::ColorDialog,

    #[nwg_resource]
    orange_color: nwg::ColorDialog,

    #[nwg_resource]
    carry_color: nwg::ColorDialog,

    // file dialog
    #[nwg_resource(title: "Portal Folder", multiselect: false, action: nwg::FileDialogAction::OpenDirectory)]
    game_dir: nwg::FileDialog,
}

impl PortalTools {
    pub fn pick_blue(&self) {
        self.blue_color.run(Some(&self.window));

        let c = self.blue_color.color();
        self.blue_box
            .set_text(format!("{:02X}{:02X}{:02X}", c[0], c[1], c[2]).as_str());
    }

    pub fn pick_orange(&self) {
        self.orange_color.run(Some(&self.window));

        let c = self.orange_color.color();
        self.orange_box
            .set_text(format!("{:02X}{:02X}{:02X}", c[0], c[1], c[2]).as_str());
    }

    pub fn pick_carry(&self) {
        self.carry_color.run(Some(&self.window));

        let c = self.carry_color.color();
        self.carry_box
            .set_text(format!("{:02X}{:02X}{:02X}", c[0], c[1], c[2]).as_str());
    }

    pub fn pick_game(&self) {
        if self.game_dir.run(Some(&self.window)) {
            self.game_box
                .set_text(self.game_dir.get_selected_item().unwrap().as_str());
        }
    }
}
