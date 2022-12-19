use std::ops::{ Sub, Index, IndexMut };

#[derive(Clone, Copy)]
pub enum Resource { Ore, Clay, Obsidian, Geode }

impl Resource
{
    pub fn iter() -> impl Iterator<Item = Resource>
    {
        [Resource::Ore,
         Resource::Clay,
         Resource::Obsidian,
         Resource::Geode].into_iter()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Inventory
{
    ore:      u16,
    clay:     u16,
    obsidian: u16,
    geode:    u16
}

impl Inventory
{
    pub const EMPTY : Inventory = Inventory { ore: 0, clay: 0, obsidian: 0, geode: 0 };

    pub fn checked_sub(&self, other : &Inventory) -> Option<Inventory>
    {
        Some(Inventory
        {
            ore:      self.ore.checked_sub(other.ore)?,
            clay:     self.clay.checked_sub(other.clay)?,
            obsidian: self.obsidian.checked_sub(other.obsidian)?,
            geode:    self.geode.checked_sub(other.geode)?
        })
    }
}

impl Sub for &Inventory
{
    type Output = Inventory;

    fn sub(self, other : &Inventory) -> Inventory
    {
        let mut inventory = self.clone();
        for resource in Resource::iter()
        {
            inventory[resource] -= other[resource]
        }
        inventory
    }
}

impl Index<Resource> for Inventory
{
    type Output = u16;

    fn index(&self, resource : Resource) -> &u16
    {
        match resource
        {
            Resource::Ore      => &self.ore,
            Resource::Clay     => &self.clay,
            Resource::Obsidian => &self.obsidian,
            Resource::Geode    => &self.geode
        }
    }
}

impl IndexMut<Resource> for Inventory
{
    fn index_mut(&mut self, resource : Resource) -> &mut u16
    {
        match resource
        {
            Resource::Ore      => &mut self.ore,
            Resource::Clay     => &mut self.clay,
            Resource::Obsidian => &mut self.obsidian,
            Resource::Geode    => &mut self.geode
        }
    }
}

pub struct Blueprint
{
    pub id:   u16,
    ore:      Inventory,
    clay:     Inventory,
    obsidian: Inventory,
    geode:    Inventory
}

impl Blueprint
{
    pub fn empty(id : u16) -> Blueprint
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

    pub fn parse(s : &str) -> Option<Blueprint>
    {
        let s       = s.strip_prefix("Blueprint ")?;
        let (id, s) = s.split_at(s.find(':')?);
        let s       = s.strip_prefix(':')?;

        let mut blueprint = Blueprint::empty(id.parse().ok()?);
        for (resource, mut words) in Resource::iter().zip(s.split('.').map(|s| s.split_whitespace()))
        {
            let inventory = &mut blueprint[resource];
            inventory[Resource::Ore] = words.nth(4)?.parse().ok()?;

            match resource
            {
                Resource::Obsidian => inventory[Resource::Clay]     = words.nth(2)?.parse().ok()?,
                Resource::Geode    => inventory[Resource::Obsidian] = words.nth(2)?.parse().ok()?,
                _                  => ()
            }
        }

        Some(blueprint)
    }
}

impl Index<Resource> for Blueprint
{
    type Output = Inventory;

    fn index(&self, resource : Resource) -> &Inventory
    {
        match resource
        {
            Resource::Ore      => &self.ore,
            Resource::Clay     => &self.clay,
            Resource::Obsidian => &self.obsidian,
            Resource::Geode    => &self.geode
        }
    }
}

impl IndexMut<Resource> for Blueprint
{
    fn index_mut(&mut self, resource : Resource) -> &mut Inventory
    {
        match resource
        {
            Resource::Ore      => &mut self.ore,
            Resource::Clay     => &mut self.clay,
            Resource::Obsidian => &mut self.obsidian,
            Resource::Geode    => &mut self.geode
        }
    }
}
