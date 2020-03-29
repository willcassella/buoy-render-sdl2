use buoy::prelude::*;
use buoy::primitives::*;
use buoy::render::color;

#[derive(Copy, Clone)]
pub struct RedShift {
    pub phase: f32,
    pub target: Id,
}

// impl Filter for RedShift {
//     fn predicate(
//         &self,
//         id: Id,
//         element: &dyn Element,
//     ) -> PredicateResult {
//         if id == self.target && element.type_id() == TypeId::of::<Fill>() {
//             PredicateResult::RunFilter
//         } else {
//             PredicateResult::PassRecurse
//         }
//     }

//     fn element<'ctx, 'frm>(
//         &self,
//         mut ctx: Context<'ctx, 'frm>,
//         id: Id,
//         element: ABox<'frm, dyn Element>
//     ) -> LayoutNode<'frm> {
//         let mut element = element.downcast::<Fill>().ok().unwrap();

//         if ctx.read_state(self.state) {
//             element.color = color::constants::RED;
//         }

//         // Run the element as usual
//         let mut sub = ctx.open_sub(ctx.max_area(), id, element);
//         sub.connect_all_sockets();
//         sub.close()
//     }
// }

pub struct BlueBox;
impl Element for BlueBox {
    fn run<'ctx, 'frm>(self, mut ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm> {
        let max_area = ctx.max_area();
        let (hover_in, hover_out) = ctx.message(id.append_str("hovered"));
        let (hover_time_in, hover_time_out) = ctx.message(id.append_str("hover_time"));
        let fill_id = id.append_str("fill");

        let hover_time = ctx.read_message(hover_time_in).unwrap_or(0_f32);

        let inner_color = match ctx.read_message(hover_in) {
            Some(_) => {
                ctx.write_message(hover_time_out, hover_time + 0.5_f32);
                color::constants::RED
            }
            None => {
                ctx.write_message(hover_time_out, (hover_time - 0.5_f32).max(0_f32));
                color::constants::WHITE
            }
        };

        let mut sub = Size::build(id.append_str("size"))
        .width(50_f32 + hover_time)
        .height(50_f32 + hover_time)
        .open(&mut ctx, max_area);
        {
            Overlap::build(id.append_str("overlap"))
            .begin(&mut sub);
            {
                Border::build(id.append_str("border"))
                .uniform(5_f32)
                .color(color::RGBA8(0x10_C0_C9_FF))
                .begin(&mut sub);
                {
                        Fill::build(fill_id)
                        .color(inner_color)
                        .begin(&mut sub);
                        sub.end(); // fill
                }
                sub.end(); // border

                // Create a hover state
                Hover::build(id.append_str("hover"), hover_out)
                .begin(&mut sub);
                sub.end(); // hover
            }
            sub.end(); // overlap
        }
        sub.close() // size
    }
}

pub struct Repeating;
impl Element for Repeating {
    fn run<'ctx, 'frm>(self, mut ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm> {
        let mut sub = ctx.open_sub(ctx.max_area(), (id.append_str("list"), List::left_to_right()));
        {
            for i in 0..10 {
                Border::build(id.append_str("padding").append_num(i))
                .right(30_f32)
                .begin(&mut sub);
                {
                    sub.begin(SocketName::default(), (id.append_str("BlueBox").append_num(i), BlueBox))
                    .end();
                }
                sub.end(); // padding
            }
        }
        sub.close()
    }
}

pub struct GridRepeating;
impl Element for GridRepeating {
    fn run<'ctx, 'frm>(self, mut ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm> {
        let grid = Grid {
            rows: vec![GridTrack::Fr(2), GridTrack::Fr(1), GridTrack::Auto, GridTrack::Fr(1), GridTrack::Fr(2)],
            cols: vec![GridTrack::Fr(1), GridTrack::Auto, GridTrack::Fr(1), GridTrack::Auto, GridTrack::Fr(1), GridTrack::Auto, GridTrack::Fr(1)],
            regions: vec![
                GridRegion::col_span(SocketName::from("top"), ColIndex::from_left(0), ColIndex::from_right(0), RowIndex::from_top(0)),
                GridRegion::cell(SocketName::from("first"), ColIndex::from_left(1), RowIndex::from_top(2)),
                GridRegion::cell(SocketName::from("second"), ColIndex::from_left(3), RowIndex::from_top(2)),
                GridRegion::cell(SocketName::from("third"), ColIndex::from_left(5), RowIndex::from_top(2)),
                GridRegion::col_span(SocketName::from("bot"), ColIndex::from_left(0), ColIndex::from_right(0), RowIndex::from_bot(0)),
            ],
        };

        let mut sub = ctx.open_sub(ctx.max_area(), (id.append_str("grid"), grid));
        {
            sub.begin(SocketName::from("top"), (id.append_str("top"), Fill::new(color::constants::RED))).end();
            sub.begin(SocketName::from("first"), (id.append_str("first"), BlueBox)).end();
            sub.begin(SocketName::from("second"), (id.append_str("second"), BlueBox)).end();
            sub.begin(SocketName::from("third"), (id.append_str("third"), BlueBox)).end();
            sub.begin(SocketName::from("bot"), (id.append_str("bot"), Fill::new(color::constants::BLUE))).end();
        }
        sub.close()
    }
}
