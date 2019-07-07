use buoy::prelude::*;
use buoy::render::color;
use buoy::primitives::{
    fill::Fill,
    size::Size,
    border::Border,
    list::List,
};

#[derive(Clone)]
pub struct BlueBox;
impl Element for BlueBox {
    fn run(
        &self,
        mut ctx: Context,
    ) -> LayoutObj {
        let id = ctx.element_id();
        let max_area = ctx.max_area();
        let mut b = ctx.begin_element(max_area, id.append_str("border"), Border::uniform(10_f32).color(color::RGBA8(0x10_C0_C9_FF)));

        Fill::new(color::constants::WHITE)
        .begin(&mut b, SocketName::default(), id.append_str("fill"));

            Size::default().width(20_f32).height(10_f32)
            .begin(&mut b, SocketName::default(), id.append_str("inner")).end();

        b.end(); // fill

        b.finish()
    }
}

#[derive(Copy, Clone)]
pub struct Repeating;
impl Element for Repeating {
    fn run(
        &self,
        mut ctx: Context,
    ) -> LayoutObj {
        let max_area = ctx.max_area();
        let mut b = ctx.begin_element(max_area, Id::from("Repeating_stack"), List::left_to_right());

        for _ in 0..100 {
            Border::default().top(15_f32).bottom(15_f32).right(30_f32).begin(&mut b, SocketName::default(), Id::from("padding"));
                Size::default().height(100_f32).v_align(VAlign::Center).begin(&mut b, SocketName::default(), Id::from("size"));
                    BlueBox.begin(&mut b, SocketName::default(), Id::from("BlueBox")).end(); // BlueBox
                b.end(); // size
            b.end(); // padding
        }

        b.finish()
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

#[derive(Clone, Copy)]
pub struct Grower {
    pub target: Id,
    value: f32,
    max: f32,
    min: f32,
    delta: f32,
}

impl Grower {
    pub fn new(target: Id) -> Self {
        Grower {
            target,
            value: 100_f32,
            max: 200_f32,
            min: 20_f32,
            delta: 0.5_f32,
        }
    }

    pub fn grow(&self, bounds: &mut Size) {
        *bounds = bounds.width(self.value);
    }

    pub fn next(mut self) -> Self {
        self.value += self.delta;
        if self.value > self.max {
            self.value = self.max;
            self.delta = -self.delta;
        } else if self.value < self.min {
            self.value = self.min;
            self.delta = -self.delta;
        }

        self
    }
}

// impl Filter for Grower {
//     fn element<'a, C: TreeContext<'a>, E: Element, T: TreeProvider>(
//         &self,
//         ctx: C,
//         id: Id,
//         element: E,
//         children: T,
//         filters: &mut FilterStack,
//     ) {
//         // If this is the widget we're looking for
//         let element = if id == self.target {
//             let mut element = element.downcast::<Space>().ok().unwrap();

//             // Apply the effect
//             self.grow(&mut element);

//             // Create a new filter for the next frame
//             ctx.filter_next_frame(Rc::new(self.next()));

//             element.upcast()
//         } else {
//             // Recurse
//             filters.add_filter(Rc::new(*self));
//             element.upcast()
//         };

//         // Put it back into the context
//         ctx.element(id, element, children);
//     }
// }
