pub enum Intent {
    // Query,
    Command,
}

pub struct Action {
    pub action: String,
    pub intent: Intent,
}

pub struct Schema {
    pub name: String,
    pub actions: Vec<Action>,
}

pub fn endpoint_urls(schema: Schema) -> Vec<String> {
    vec![format!("/{}/{}", schema.name, schema.actions[0].action)]
}
