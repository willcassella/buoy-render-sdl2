use buoy::render::color;
use buoy::core::*;
use element::Id;
use buoy::primitives::{
    fill::SolidFill,
    space::{Space, VAlign},
    border::BlockBorder,
    list::List,
    hover,
};
use buoy::builder::{Builder, BuilderContext};

#[derive(Clone)]
pub struct BlueBox;
impl Builder for BlueBox {
    fn run(self, ctx: &mut BuilderContext) {
        let id = ctx.element_id();

        // Create a state for the hover, and push a handler for it
        let hover_input = ctx.new_input();
        // let fill_id = id.append_str("fill");
        // ctx.filter_next_frame(Rc::new(HoverHandler{ target: fill_id, state: hover_input }));

        hover::Hover::new_no_action(hover_input)
        .begin(ctx, id.append_str("hover"));

            BlockBorder::uniform(10_f32)
            .color(color::RGBA8(0x10_C0_C9_FF))
            .begin(ctx, id.append_str("border"));

                SolidFill::new(color::constants::WHITE)
                .begin(ctx, id.append_str("fill"));

                    Space::default().width(20_f32).height(10_f32)
                    .begin(ctx, id.append_str("inner")).end();

                ctx.end(); // fill
            ctx.end(); // border
        ctx.end(); // hover
    }
}

#[derive(Clone, Copy)]
pub struct TestStub;
impl Builder for TestStub {
    fn run(self, ctx: &mut BuilderContext) {
        List::left_to_right().begin(ctx, Id::from("TestGenerator_stack"));

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).begin(ctx, Id::from("BlueBox_1_padding"));
                Space::default().height(100_f32).v_align(VAlign::Center).begin(ctx, Id::from("BlueBox_1_max"));
                    BlueBox.begin(ctx, Id::from("BlueBox_1")).end();
                ctx.end(); // BlueBox_1_max
            ctx.end(); // BlueBox_1_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).begin(ctx, Id::from("BlueBox_2_padding"));
                Space::default().height(200_f32).v_align(VAlign::Bottom).begin(ctx, Id::from("BlueBox_2_max"));
                    BlueBox.begin(ctx, Id::from("BlueBox_2")).end();
                ctx.end(); // BlueBox_2_max
            ctx.end(); // BlueBox_2_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).begin(ctx, Id::from("BlueBox_3_padding"));
                Space::default().height(300_f32).v_align(VAlign::Center).begin(ctx, Id::from("BlueBox_3_max"));
                    BlueBox.begin(ctx, Id::from("BlueBox_3")).end();
                ctx.end(); // BlueBox_3_max
            ctx.end(); // BlueBox_3_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).begin(ctx, Id::from("BlueBox_4_padding"));
                Space::default().height(400_f32).v_align(VAlign::Top).begin(ctx, Id::from("BlueBox_4_max"));
                    BlueBox.begin(ctx, Id::from("BlueBox_4")).end();
                ctx.end(); // BlueBox_4_max
            ctx.end(); // BlueBox_4_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).begin(ctx, Id::from("BlueBox_5_padding"));
                Space::default().height(500_f32).v_align(VAlign::Center).begin(ctx, Id::from("BlueBox_5_max"));
                    BlueBox.begin(ctx, Id::from("BlueBox_5")).end();
                ctx.end(); // BlueBox_5_max
            ctx.end(); // BlueBox_5_padding

        ctx.end(); // TestGenerator_stack
    }
}

// #[derive(Clone, Copy)]
// pub struct HoverHandler {
//     target: Id,
//     state: hover::HoverState,
// }

// impl UIFilterImpl for HoverHandler {
//     fn widget(
//         &self,
//         ctx: &mut Context,
//         mut widget: UIWidget,
//         filters: &mut FilterStack,
//     ) {
//         // If this is the element we're looking for
//         let widget = if widget.id == self.target {
//             // If the element was hovered
//             if ctx.read_input(self.state) {
//                 // Modify the color
//                 let mut widget = widget.downcast::<SolidFill>().ok().unwrap();
//                 widget.imp.color = color::constants::RED;
//                 widget.upcast()
//             } else {
//                 widget
//             }
//         } else {
//             // Recurse
//             filters.add_filter(Rc::new(*self));
//             widget
//         };

//         // Put it back into the context
//         ctx.widget_begin(widget);
//             ctx.children_all();
//         ctx.end();
//     }
// }

// #[derive(Clone, Copy)]
// pub struct Fader {
//     target: Id,
//     value: f32,
//     delta: f32,
// }

// impl Fader {
//     pub fn new(target: Id) -> Self {
//         Fader {
//             target,
//             value: 1_f32,
//             delta: -0.01_f32,
//         }
//     }

//     fn fade_color(&self, col: color::RGBA8) -> color::RGBA8 {
//         let red = (f32::from(col.red()) * self.value) as u8;
//         let green = (f32::from(col.green()) * self.value) as u8;
//         let blue = (f32::from(col.blue()) * self.value) as u8;
//         color::RGBA8::new(red, green, blue, 0xFF)
//     }

//     fn next(mut self) -> Self {
//         self.value += self.delta;
//         if self.value > 1_f32 {
//             self.value = 1_f32;
//             self.delta = -0.01_f32;
//         } else if self.value < 0_f32 {
//             self.value = 0_f32;
//             self.delta = 0.01_f32;
//         }

//         self
//     }
// }

// impl UIFilterImpl for Fader {
//     fn widget(
//         &self,
//         ctx: &mut Context,
//         mut widget: UIWidget,
//         filters: &mut FilterStack,
//     ) {
//         // If this is the widget we're looking for
//         let widget = if widget.id == self.target {
//             // Modify the color
//             let mut widget = widget.downcast::<BlockBorder>().ok().unwrap();
//             widget.imp.color = self.fade_color(widget.imp.color);

//             // Create a new filter, with a different value
//             ctx.filter_next_frame(Rc::new(self.next()));

//             widget.upcast()
//         } else {
//             // Recurse
//             filters.add_filter(Rc::new(*self));
//             widget
//         };

//         // Put the widget back into the context
//         ctx.widget_begin(widget);
//             ctx.children_all();
//         ctx.end();
//     }
// }

// #[derive(Clone, Copy)]
// pub struct Grower {
//     pub target: Id,
//     value: f32,
//     max: f32,
//     min: f32,
//     delta: f32,
// }

// impl Grower {
//     pub fn new(target: Id) -> Self {
//         Grower {
//             target,
//             value: 100_f32,
//             max: 200_f32,
//             min: 20_f32,
//             delta: 0.5_f32,
//         }
//     }

//     pub fn grow(&self, bounds: &mut Space) {
//         *bounds = bounds.width(self.value);
//     }

//     pub fn next(mut self) -> Self {
//         self.value += self.delta;
//         if self.value > self.max {
//             self.value = self.max;
//             self.delta = -self.delta;
//         } else if self.value < self.min {
//             self.value = self.min;
//             self.delta = -self.delta;
//         }

//         self
//     }
// }

// impl UIFilterImpl for Grower {
//     fn widget(
//         &self,
//         ctx: &mut Context,
//         widget: UIWidget,
//         filters: &mut FilterStack,
//     ) {
//         // If this is the widget we're looking for
//         let widget = if widget.id == self.target {
//             let mut widget = widget.downcast::<Space>().ok().unwrap();

//             // Apply the effect
//             self.grow(&mut widget.imp);

//             // Create a new filter for the next frame
//             ctx.filter_next_frame(Rc::new(self.next()));

//             widget.upcast()
//         } else {
//             // Recurse
//             filters.add_filter(Rc::new(*self));
//             widget
//         };

//         // Put it back into the context
//         ctx.widget_begin(widget);
//             ctx.children_all();
//         ctx.end();
//     }
// }
