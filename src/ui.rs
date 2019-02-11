use buoy::Context;
use buoy::element::{IntoUIElement, IntoObj, Stub, StubImpl, Id};
use buoy::elements::{fill::SolidFill, min_max::{MinMax, VAlign}, border::BlockBorder, list::List};
use buoy::render::color;

#[derive(Clone, Copy)]
pub struct BlueBox;
impl StubImpl for BlueBox {
    fn generate(self, ctx: &mut Context) {
        BlockBorder::uniform(10_f32)
        .color(color::constants::BLUE)
        .into_obj(Id::str("BlueBox_border"))
        .push(ctx);

            SolidFill::new(color::constants::WHITE)
            .into_obj(Id::str("BlueBox_fill"))
            .push(ctx);

                MinMax::default()
                .width(100_f32)
                .height(10_f32)
                .into_obj(Id::str("BlueBox_inner"))
                .push(ctx)
                .pop();

            ctx.pop(); // BlueBox_fill
        ctx.pop(); // BlueBox)border
    }
}

impl IntoUIElement for BlueBox {
    type Target = Stub<BlueBox>;
}

#[derive(Clone, Copy)]
pub struct TestStub;
impl StubImpl for TestStub {
    fn generate(self, ctx: &mut Context) {
        List::left_to_right().into_obj(Id::str("TestGenerator_stack")).push(ctx);

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).into_obj(Id::str("BlueBox_1_padding")).push(ctx);
                MinMax::default().height(100_f32).v_align(VAlign::Center).into_obj(Id::str("BlueBox_1_max")).push(ctx);
                    BlueBox.into_obj(Id::str("BlueBox_1")).push(ctx).pop();
                ctx.pop(); // BlueBox_1_max
            ctx.pop(); // BlueBox_1_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).into_obj(Id::str("BlueBox_2_padding")).push(ctx);
                MinMax::default().height(200_f32).v_align(VAlign::Center).into_obj(Id::str("BlueBox_2_max")).push(ctx);
                    BlueBox.into_obj(Id::str("BlueBox_1")).push(ctx).pop();
                ctx.pop(); // BlueBox_2_max
            ctx.pop(); // BlueBox_2_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).into_obj(Id::str("BlueBox_3_padding")).push(ctx);
                MinMax::default().height(300_f32).v_align(VAlign::Center).into_obj(Id::str("BlueBox_3_max")).push(ctx);
                    BlueBox.into_obj(Id::str("BlueBox_2")).push(ctx).pop();
                ctx.pop(); // BlueBox_3_max
            ctx.pop(); // BlueBox_3_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).into_obj(Id::str("BlueBox_4_padding")).push(ctx);
                MinMax::default().height(400_f32).v_align(VAlign::Center).into_obj(Id::str("BlueBox_4_max")).push(ctx);
                    BlueBox.into_obj(Id::str("BlueBox_3")).push(ctx).pop();
                ctx.pop(); // BlueBox_4_max
            ctx.pop(); // BlueBox_4_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).into_obj(Id::str("BlueBox_5_padding")).push(ctx);
                MinMax::default().height(500_f32).v_align(VAlign::Center).into_obj(Id::str("BlueBox_5_max")).push(ctx);
                    BlueBox.into_obj(Id::str("BlueBox_4")).push(ctx).pop();
                ctx.pop(); // BlueBox_5_max
            ctx.pop(); // BlueBox_5_padding

        ctx.pop(); // TestGenerator_stack
    }
}

impl IntoUIElement for TestStub {
    type Target = Stub<TestStub>;
}
