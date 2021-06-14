use mongodb::{
    bson::Document,
    options::{FindOneOptions, FindOptions},
    Collection,
};

struct QueryBuilder {
    pub limit: Option<i64>,
    pub skip: Option<u64>,
    pub sort: Option<Document>,
    pub projection: Option<Document>,
    pub filter: Option<Document>,
}

impl QueryBuilder {
    // Doesn't work because it doesn't take options anymore
    pub async fn execute_opt_builder(self, collection: Collection) {
        let options = FindOptions::builder()
            .projection(self.projection)
            .limit(self.limit)
            .skip(self.skip)
            .sort(self.sort)
            .build();

        let cursor = collection.find(self.filter, options).await.unwrap();
        // ...
    }

    // Doesn't work because of the builder typings on each method call.
    pub async fn execute_if_let(self, collection: Collection) {
        let options = FindOptions::builder();

        let options = if let Some(projection) = self.projection {
            options.projection(projection)
        } else {
            // The typing changes on `.projection()` prevents this pattern
            options
        };

        // ... Repeat if let for all options and `.build()` ...

        let cursor = collection.find(self.filter, Some(options)).await.unwrap();
        // ...
    }

    // Okay maybe no builder then, just build the options struct directly.
    // Doesn't work because of non-exhaustive annotation.
    pub async fn execute_struct(self, collection: Collection) {
        let options = FindOneOptions {
            projection: self.projection,
            skip: self.skip,
            sort: self.sort,
            ..Default::default()
        };

        let cursor = collection.find(self.filter, Some(options)).await.unwrap();
        // ...
    }

    // The only option that would work for me (apart from maybe complicated options chaining or filling the fields with default empty documents / values).
    pub async fn execute_manual_match(self, collection: Collection) {
        let options = match (self.limit, self.projection, self.skip, self.sort) {
            (None, None, None, None) => FindOptions::builder().build(),
            (Some(limit), None, None, None) => FindOptions::builder().limit(limit).build(),
            (Some(limit), Some(projection), None, None) => FindOptions::builder()
                .limit(limit)
                .projection(projection)
                .build(),
            // And many more cases ...
            _ => panic!("You get the idea."),
        };

        let cursor = collection.find(self.filter, options).await.unwrap();
        // ...
    }
}

fn main() {}
