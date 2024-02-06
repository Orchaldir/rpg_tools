use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::model::world::town::cell::TownCell;
use rpg_tools_core::model::world::town::terrain::Terrain;
use rpg_tools_core::utils::map::border::BorderMap;
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::usecase::map::BorderMapRenderer;

fn main() {
    println!("A town example!");

    let map = BorderMap::simple(Size2d::new(2, 3), TownCell::new(Terrain::Plain), true);

    let renderer = BorderMapRenderer::new(100, 5);

    let size = renderer.calculate_size(&map);
    let mut builder = SvgBuilder::new(size);

    renderer.render_tiles(&mut builder, &Point2d::default(), &map);

    let svg = builder.finish();

    svg.save("town.svg").unwrap();
}
