pub enum EntitySelector {
    NEAREST_PLAYER,
    RANDOM_PLAYER,
    ALL_PLAYERS,
    ALL_ENTITIES,
    SEL_ENTITY,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EntityAttributeValue<'a> {
    pub literal: &'a str,
    pub nested: Option<EntityAttributeValue<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EntityAttribute<'a> {
    pub key: &'a str,
    pub value: EntityAttributeValue<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EntityLit<'a> {
    pub selector: EntitySelector,
    pub attributes: Vec<EntityAttribute<'a>>,
}