#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EntitySelector {
    NearestPlayer,
    RandomPlayer,
    AllPlayers,
    AllEntities,
    SelfEntity,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EntityAttribute<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EntityLit<'a> {
    pub selector: EntitySelector,
    pub attributes: Vec<EntityAttribute<'a>>,
}