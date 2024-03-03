use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::model::world::mountain::MountainId;
use rpg_tools_core::model::world::river::RiverId;
use rpg_tools_core::model::world::town::terrain::Terrain;
use rpg_tools_core::model::world::town::tile::TownTile;
use rpg_tools_core::utils::map::tile::TileMap;
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::usecase::map::TileMapRenderer;

fn main() {
    println!("A town example!");

    let mut map = TileMap::simple(Size2d::new(2, 3), TownTile::new(Terrain::Plain));
    map.get_tile_mut(0).unwrap().terrain = Terrain::River {
        id: RiverId::default(),
    };
    map.get_tile_mut(5).unwrap().terrain = Terrain::Mountain {
        id: MountainId::default(),
    };

    let renderer = TileMapRenderer::new(100, 10, 1);

    let size = renderer.calculate_map_size(&map);
    let mut builder = SvgBuilder::new(size);

    renderer.render_color(&mut builder, &Point2d::default(), &map, TownTile::get_color);

    let svg = builder.finish();

    svg.save("town.svg").unwrap();
}
