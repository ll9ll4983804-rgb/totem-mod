use mod_api::{ClientHost, Corner, Hooks, RenderGuiPostEvent};

const ID: &str = "totem_helper";
const BACKGROUND: u32 = 0x7800_0000;
const INK: u32 = 0xFFFF_FFFF;
const LINE: f32 = 10.0;
const PADDING: f32 = 2.0;
const CORNER: Corner = Corner::TopLeft;

#[derive(Default)]
struct TotemHelper;

#[mod_api::horizon_mod]
impl Hooks for TotemHelper {
    fn render_gui_post(&self, host: &dyn ClientHost, event: &mut RenderGuiPostEvent<'_>) {
        if !host.on(ID) || !host.native_corner_shown(CORNER) {
            return;
        }
        let text = "Totem Helper: test build";
        let draw = &mut *event.draw;
        let width = draw.text_width(text) + 2.0 * PADDING;
        let height = LINE + PADDING;
        let (x, y) = event.corners.place(CORNER, width, height);
        draw.fill([x, y, width, height], BACKGROUND);
        draw.text(text, [x + PADDING, y + PADDING], INK, true);
    }
}
