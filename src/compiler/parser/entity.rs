pub enum EntitySelector {
    NearestPlayer,
    RandomPlayer,
    AllPlayers,
    AllEntities,
    SelfEntity,
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