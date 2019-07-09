use buoy::prelude::*;
use buoy::primitives::{border::Border, fill::Fill, list::List, size::Size};
use buoy::render::color;

#[derive(Clone)]
pub struct BlueBox;
impl Element for BlueBox {
    fn run(&self, mut ctx: Context, id: Id) -> LayoutObj {
        let max_area = ctx.max_area();
        let mut sub = ctx.open_element(
            max_area,
            id.str("border"),
            Border::uniform(10_f32).color(color::RGBA8(0x10_C0_C9_FF)),
        );

        Size::default()
        .min_width(20_f32)
        .begin(&mut sub, SocketName::default(), id.str("size"));

            Fill::new(color::constants::WHITE)
            .begin(&mut sub, SocketName::default(), id.str("fill"))
            .end();

        sub.end(); // size

        sub.close()
    }
}

#[derive(Copy, Clone)]
pub struct Repeating;
impl Element for Repeating {
    fn run(&self, mut ctx: Context, id: Id) -> LayoutObj {
        let max_area = ctx.max_area();
        let mut sub = ctx.open_element(max_area, id.str("list"), List::left_to_right());

        for i in 0..100 {
            Border::default()
            .right(30_f32)
            .begin(&mut sub, SocketName::default(), id.str("padding").num(i));

                Size::default()
                .height(100_f32)
                .v_align(VAlign::Center)
                .begin(&mut sub, SocketName::default(), id.str("size").num(i));

                    BlueBox
                    .begin(&mut sub, SocketName::default(), id.str("BlueBox").num(i))
                    .end();

                sub.end(); // size
            sub.end(); // padding
        }

        sub.close()
    }
}
