use buoy::prelude::*;
use buoy::primitives::*;
use buoy::render::color;

#[derive(Clone)]
pub struct BlueBox;
impl Element for BlueBox {
    fn run(&self, mut ctx: Context, id: Id) -> LayoutObj {
        let max_area = ctx.max_area();
        let mut sub = ctx.open_element(
            max_area,
            id.append_str("overlap"),
            Overlap,
        );

        Border::build(id.append_str("border"))
        .uniform(10_f32)
        .color(color::RGBA8(0x10_C0_C9_FF))
        .begin(&mut sub);

            Size::build(id.append_str("size"))
            .min_width(20_f32)
            .begin(&mut sub);

                Fill::build(id.append_str("fill"))
                .color(color::constants::WHITE)
                .begin(&mut sub);
                sub.end(); // fill

            sub.end(); // size
        sub.end(); // border

        // Create a hover state
        let hover_state = sub.new_state();
        Hover::build(id.append_str("hover"), hover_state)
        .begin(&mut sub);
        sub.end(); // hover

        sub.close() // overlap
    }
}

#[derive(Copy, Clone)]
pub struct Repeating;
impl Element for Repeating {
    fn run(&self, mut ctx: Context, id: Id) -> LayoutObj {
        let max_area = ctx.max_area();
        let mut sub = ctx.open_element(max_area, id.append_str("list"), List::left_to_right());

        for i in 0..100 {
            Border::build(id.append_str("padding").append_num(i))
            .right(30_f32)
            .begin(&mut sub);

                Size::build(id.append_str("size").append_num(i))
                .height(100_f32)
                .v_align(VAlign::Center)
                .begin(&mut sub);

                    sub.begin(SocketName::default(), id.append_str("BlueBox").append_num(i), BlueBox)
                    .end();

                sub.end(); // size
            sub.end(); // padding
        }

        sub.close()
    }
}
