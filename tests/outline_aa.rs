#[test]
fn t20_outline_render() {
    use agg::{Pixfmt,Rgb8,Rgba8,SetColor};
    use agg::{RendererOutlineAA,RasterizerOutlineAA};
    use agg::PixelData;
    let pix = Pixfmt::<Rgb8>::new(100,100);
    let mut ren_base = agg::RenderingBase::new(pix);
    ren_base.clear( Rgba8::new(255, 255, 255, 255) );
    let mut ren = RendererOutlineAA::with_base(&mut ren_base);
    ren.color(agg::Rgba8::new(0,0,0,255));
    ren.profile.width(20.0);

    let mut path = agg::PathStorage::new();
    path.move_to(10.0, 10.0);
    path.line_to(50.0, 90.0);
    path.line_to(90.0, 10.0);

    let mut ras = RasterizerOutlineAA::with_renderer(&mut ren);
    ras.round_cap(true);
    ras.add_path(&path);
    agg::ppm::write_ppm(&ren_base.pixeldata(), 100,100, "outline_aa.ppm").unwrap();
    agg::ppm::compare_ppm("outline_aa.ppm", "tests/outline_aa.ppm");

}
