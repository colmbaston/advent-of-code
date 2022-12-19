use std::ops::{ Index, IndexMut };

fn main()
{
    let blueprints = include_str!("../input.txt").lines().filter_map(Blueprint::parse).collect::<Vec<Blueprint>>();

    for blueprint in blueprints
    {
        println!("{blueprint:?}");
    }
}

#[derive(Debug, Clone, Copy)]
enum Material { Ore, Clay, Obsidian, Geode }

impl Material
{
    fn iter() -> impl Iterator<Item = Material>
    {
        [Material::Ore,
         Material::Clay,
         Material::Obsidian,
         Material::Geode].into_iter()
    }
}

#[derive(Debug)]
struct Inventory
{
    ore:      u32,
    clay:     u32,
    obsidian: u32,
    geode:    u32
}

impl Inventory
{
    const EMPTY : Inventory = Inventory { ore: 0, clay: 0, obsidian: 0, geode: 0 };

    fn subset(&self, other : &Inventory) -> bool
    {
        Material::iter().all(|m| self[m] <= other[m])
    }
}

impl Index<Material> for Inventory
{
    type Output = u32;

    fn index(&self, material : Material) -> &u32
    {
        match material
        {
            Material::Ore      => &self.ore,
            Material::Clay     => &self.clay,
            Material::Obsidian => &self.obsidian,
            Material::Geode    => &self.geode
        }
    }
}

impl IndexMut<Material> for Inventory
{
    fn index_mut(&mut self, material : Material) -> &mut u32
    {
        match material
        {
            Material::Ore      => &mut self.ore,
            Material::Clay     => &mut self.clay,
            Material::Obsidian => &mut self.obsidian,
            Material::Geode    => &mut self.geode
        }
    }
}

#[derive(Debug)]
struct Blueprint
{
    id:       u32,
    ore:      Inventory,
    clay:     Inventory,
    obsidian: Inventory,
    geode:    Inventory
}

impl Blueprint
{
    fn empty(id : u32) -> Blueprint
    {
        Blueprint
        {
            id,
            ore:      Inventory::EMPTY,
            clay:     Inventory::EMPTY,
            obsidian: Inventory::EMPTY,
            geode:    Inventory::EMPTY
        }
    }

    fn parse(s : &str) -> Option<Blueprint>
    {
        let s       = s.strip_prefix("Blueprint ")?;
        let (id, s) = s.split_at(s.find(':')?);
        let s       = s.strip_prefix(':')?;

        let mut blueprint = Blueprint::empty(id.parse().ok()?);
        for (material, mut words) in Material::iter().zip(s.split('.').map(|s| s.split_whitespace()))
        {
            let inventory = &mut blueprint[material];
            inventory[Material::Ore] = words.nth(4)?.parse().ok()?;

            match material
            {
                Material::Obsidian => inventory[Material::Clay]     = words.nth(2)?.parse().ok()?,
                Material::Geode    => inventory[Material::Obsidian] = words.nth(2)?.parse().ok()?,
                _                  => ()
            }
        }

        Some(blueprint)
    }
}

impl Index<Material> for Blueprint
{
    type Output = Inventory;

    fn index(&self, material : Material) -> &Inventory
    {
        match material
        {
            Material::Ore      => &self.ore,
            Material::Clay     => &self.clay,
            Material::Obsidian => &self.obsidian,
            Material::Geode    => &self.geode
        }
    }
}

impl IndexMut<Material> for Blueprint
{
    fn index_mut(&mut self, material : Material) -> &mut Inventory
    {
        match material
        {
            Material::Ore      => &mut self.ore,
            Material::Clay     => &mut self.clay,
            Material::Obsidian => &mut self.obsidian,
            Material::Geode    => &mut self.geode
        }
    }
}
