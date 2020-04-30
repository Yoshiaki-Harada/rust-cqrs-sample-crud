use crate::domain::todos::resolver::TodoResolver;

pub struct Resolver {
    todo_resolver: TodoResolver,
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver{
            todo_resolver: TodoResolver::default()
        }
    }
}

impl Resolver {
    pub(in crate::domain) fn todos(&self) -> &TodoResolver {
        &self.todo_resolver
    }
}