use std::any::TypeId;
use std::rc::Rc;

use buoy::prelude::*;
use buoy::primitives::*;
use buoy::render::color;

use buoy::util::arena::ABox;

#[derive(Copy, Clone)]
pub struct RedShift {
    pub state: State<bool>,
    pub target: Id,
}

impl Filter for RedShift {
    fn predicate(
        &self,
        id: Id,
        element: &dyn Element,
    ) -> PredicateResult {
        if id == self.target && element.type_id() == TypeId::of::<Fill>() {
            PredicateResult::RunFilter
        } else {
            PredicateResult::PassRecurse
        }
    }

    fn element<'ctx, 'frm>(
        &self,
        mut ctx: Context<'ctx, 'frm>,
        id: Id,
        element: ABox<'frm, dyn Element>
    ) -> LayoutNode<'frm> {
        let mut element = element.downcast::<Fill>().ok().unwrap();

        if ctx.read_state(self.state) {
            element.color = color::constants::RED;
        }

        // Run the element as usual
        let mut sub = ctx.open_sub(ctx.max_area(), id, element);
        sub.connect_all_sockets();
        sub.close()
    }
}

#[derive(Copy, Clone)]
pub struct BlueBox;
impl Element for BlueBox {
    fn run<'ctx, 'frm>(&self, mut ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm> {
        let max_area = ctx.max_area();
        let hover_state = ctx.new_state();
        let fill_id = id.append_str("fill");
        ctx.next_frame_filter(Rc::new(RedShift {
            target: fill_id,
            state: hover_state,
        }));

        let mut sub = Size::build(id.append_str("size"))
        .width(50_f32)
        .height(50_f32)
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
                        .color(color::constants::WHITE)
                        .begin(&mut sub);
                        sub.end(); // fill
                }
                sub.end(); // border

                // Create a hover state
                Hover::build(id.append_str("hover"), hover_state)
                .begin(&mut sub);
                sub.end(); // hover
            }
            sub.end(); // overlap
        }
        sub.close() // size
    }
}

#[derive(Copy, Clone)]
pub struct Repeating;
impl Element for Repeating {
    fn run<'ctx, 'frm>(&self, mut ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm> {
        let max_area = ctx.max_area();
        let mut sub = ctx.open_sub(max_area, id.append_str("list"), List::left_to_right());
        {
            for i in 0..10 {
                Border::build(id.append_str("padding").append_num(i))
                .right(30_f32)
                .begin(&mut sub);
                {
                    sub.begin(SocketName::default(), id.append_str("BlueBox").append_num(i), BlueBox)
                    .end();
                }
                sub.end(); // padding
            }
        }
        sub.close()
    }
}