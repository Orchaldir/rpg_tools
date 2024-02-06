use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::model::world::mountain::MountainId;
use rpg_tools_core::model::world::river::RiverId;
use rpg_tools_core::model::world::town::terrain::Terrain;
use rpg_tools_core::model::world::town::tile::TownTile;
use rpg_tools_core::utils::map::border::BorderMap;
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::usecase::map::BorderMapRenderer;

fn main() {
    println!("A town example!");

    let mut map = BorderMap::simple(Size2d::new(2, 3), TownTile::new(Terrain::Plain), true);
    map.get_tile_mut(0).unwrap().terrain = Terrain::River {
        id: RiverId::default(),
    };
    map.get_tile_mut(5).unwrap().terrain = Terrain::Mountain {
        id: MountainId::default(),
    };

    let renderer = BorderMapRenderer::new(100, 1);

    let size = renderer.calculate_size(&map);
    let mut builder = SvgBuilder::new(size);

    renderer.render_tiles(&mut builder, &Point2d::default(), &map, |tile| {
        match tile.terrain {
            Terrain::Hill { .. } => Color::SaddleBrown,
            Terrain::Mountain { .. } => Color::Gray,
            Terrain::Plain => Color::Green,
            Terrain::River { .. } => Color::Blue,
        }
    });

    let svg = builder.finish();

    svg.save("town.svg").unwrap();
}
