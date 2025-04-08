use serde::Deserialize;

use crate::{model::tag::TagModel, util::{AddToQuery, PostgresCmp, SQLQueryBuilder, ToPostgresCmp}};

use super::{QueryMethod, UpdateMethod};

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct CreateTagSchema {
    label: Option<String>,
    color: Option<String>,

    category: Option<String>,
}

impl<'a, 'b> AddToQuery<'a, 'b> for CreateTagSchema {
	fn add_to_query(&'a self, builder: &'b mut SQLQueryBuilder<'a>) {
		builder.set_table(TagModel::TABLE);

		if let Some(ref s) = self.label {
			builder.add_column(TagModel::LABEL, s);
		}
		if let Some(ref s) = self.color {
			builder.add_column(TagModel::COLOR, s);
		}

		if let Some(ref s) = self.category {
			builder.add_column(TagModel::CATEGORY, s);
		}
	}
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagSchema {
    label: Option<UpdateMethod<String>>,
    color: Option<UpdateMethod<String>>,

    category: Option<UpdateMethod<String>>,
}

impl<'a, 'b> AddToQuery<'a, 'b> for UpdateTagSchema {
	fn add_to_query(&'a self, builder: &'b mut SQLQueryBuilder<'a>) {
		builder.set_table(TagModel::TABLE);

		if let Some(ref u) = self.label {
			if matches!(u, UpdateMethod::Remove(true) | UpdateMethod::Change(..)) {
				builder.add_column(TagModel::LABEL, u);
			}
		}
		if let Some(ref u) = self.color {
			if matches!(u, UpdateMethod::Remove(true) | UpdateMethod::Change(..)) {
				builder.add_column(TagModel::COLOR, u);
			}
		}

		if let Some(ref u) = self.category {
			if matches!(u, UpdateMethod::Remove(true) | UpdateMethod::Change(..)) {
				builder.add_column(TagModel::CATEGORY, u);
			}
		}
	}
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct QueryTagSchema {
    label: Option<QueryMethod<String>>,

    category: Option<QueryMethod<String>>,
}

impl<'a, 'b> AddToQuery<'a, 'b> for QueryTagSchema {
	fn add_to_query(&'a self, builder: &'b mut SQLQueryBuilder<'a>) {
		builder.set_table(TagModel::TABLE);

		if let Some(ref q) = self.label {
			let cmp;
			match q {
				QueryMethod::NotNull(b) => {
					if *b {
						cmp = PostgresCmp::NotNull;
					} else {
						cmp = PostgresCmp::IsNull;
					}
				}
				QueryMethod::Match(_) => cmp = PostgresCmp::Like,
				QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
			}
			builder.add_condition(TagModel::LABEL, cmp, q);
		}

		if let Some(ref q) = self.category {
			let cmp;
			match q {
				QueryMethod::NotNull(b) => {
					if *b {
						cmp = PostgresCmp::NotNull;
					} else {
						cmp = PostgresCmp::IsNull;
					}
				}
				QueryMethod::Match(_) => cmp = PostgresCmp::Like,
				QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
			}
			builder.add_condition(TagModel::CATEGORY, cmp, q);
		}
	}
}

#[cfg(test)]
mod create_schema_test {
    use uuid::Uuid;

    use crate::{model::tag::TagModel, util::{AddToQuery, SQLQueryBuilder}};

    use super::CreateTagSchema;

    const ID: Uuid = Uuid::nil();

	#[test]
	fn empty() {
		let schema = CreateTagSchema::default();

		let mut builder = SQLQueryBuilder::new();
		builder.add_column(TagModel::USER_ID, &ID);
		schema.add_to_query(&mut builder);

		let (statement, params) = builder.build_insert();

		assert_eq!(statement.as_str(), "INSERT INTO data.tags (user_id) VALUES ($1)");
		assert_eq!(params.len(), 1);
	}

	#[test]
	fn full() {
		let mut schema = CreateTagSchema::default();
		schema.label = Some("Test Label".to_string());
		schema.color = Some("#2f78ed".to_string());
		schema.category = Some("Priority".to_string());

		let mut builder = SQLQueryBuilder::new();
		builder.add_column(TagModel::USER_ID, &ID);
		schema.add_to_query(&mut builder);

		let (statement, params) = builder.build_insert();

		assert_eq!(statement.as_str(), "INSERT INTO data.tags (user_id, tag_label, color, category) VALUES ($1, $2, $3, $4)");
		assert_eq!(params.len(), 4);
	}

	#[test]
	fn return_some() {
		let mut schema = CreateTagSchema::default();
		schema.label = Some("Test Label".to_string());
		schema.color = Some("#2f78ed".to_string());
		schema.category = Some("Priority".to_string());

		let mut builder = SQLQueryBuilder::new();
		builder.add_column(TagModel::USER_ID, &ID);
		schema.add_to_query(&mut builder);
		builder.set_return(vec![TagModel::ID]);

		let (statement, params) = builder.build_insert();

		assert_eq!(statement.as_str(), "INSERT INTO data.tags (user_id, tag_label, color, category) VALUES ($1, $2, $3, $4) RETURNING tag_id");
		assert_eq!(params.len(), 4);
	}

	#[test]
	fn return_all() {
		let mut schema = CreateTagSchema::default();
		schema.label = Some("Test Label".to_string());
		schema.color = Some("#2f78ed".to_string());
		schema.category = Some("Priority".to_string());

		let mut builder = SQLQueryBuilder::new();
		builder.add_column(TagModel::USER_ID, &ID);
		schema.add_to_query(&mut builder);
		builder.set_return_all();

		let (statement, params) = builder.build_insert();

		assert_eq!(statement.as_str(), "INSERT INTO data.tags (user_id, tag_label, color, category) VALUES ($1, $2, $3, $4) RETURNING *");
		assert_eq!(params.len(), 4);
	}

    // TODO: make production examples
}

#[cfg(test)]
mod update_schema_test {
    use uuid::Uuid;

    use crate::{model::tag::TagModel, schema::UpdateMethod, util::{AddToQuery, PostgresCmp, SQLQueryBuilder}};

    use super::UpdateTagSchema;

    const ID: Uuid = Uuid::nil();

	#[test]
	fn full() {
		let mut schema = UpdateTagSchema::default();
		schema.label = Some(UpdateMethod::Change("Test Label".to_string()));
		schema.color = Some(UpdateMethod::Change("#2f78ed".to_string()));
		schema.category = Some(UpdateMethod::Change("Priority".to_string()));

		let mut builder = SQLQueryBuilder::new();
		schema.add_to_query(&mut builder);
		builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &ID);
		builder.add_condition(TagModel::ID, PostgresCmp::Equal, &ID);

		let (statement, params) = builder.build_update();

		assert_eq!(statement.as_str(), "UPDATE data.tags SET tag_label=$1, color=$2, category=$3 WHERE user_id = $4 AND tag_id = $5");
		assert_eq!(params.len(), 5);
	}

	#[test]
	fn return_some() {
		let mut schema = UpdateTagSchema::default();
		schema.label = Some(UpdateMethod::Change("Test Label".to_string()));
		schema.color = Some(UpdateMethod::Change("#2f78ed".to_string()));
		schema.category = Some(UpdateMethod::Change("Priority".to_string()));

		let mut builder = SQLQueryBuilder::new();
		schema.add_to_query(&mut builder);
		builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &ID);
		builder.add_condition(TagModel::ID, PostgresCmp::Equal, &ID);
		builder.set_return(vec![TagModel::ID]);

		let (statement, params) = builder.build_update();

		assert_eq!(statement.as_str(), "UPDATE data.tags SET tag_label=$1, color=$2, category=$3 WHERE user_id = $4 AND tag_id = $5 RETURNING tag_id");
		assert_eq!(params.len(), 5);
	}

	#[test]
	fn return_all() {
		let mut schema = UpdateTagSchema::default();
		schema.label = Some(UpdateMethod::Change("Test Label".to_string()));
		schema.color = Some(UpdateMethod::Change("#2f78ed".to_string()));
		schema.category = Some(UpdateMethod::Change("Priority".to_string()));

		let mut builder = SQLQueryBuilder::new();
		schema.add_to_query(&mut builder);
		builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &ID);
		builder.add_condition(TagModel::ID, PostgresCmp::Equal, &ID);
		builder.set_return_all();

		let (statement, params) = builder.build_update();

		assert_eq!(statement.as_str(), "UPDATE data.tags SET tag_label=$1, color=$2, category=$3 WHERE user_id = $4 AND tag_id = $5 RETURNING *");
		assert_eq!(params.len(), 5);
	}

    // TODO: make production examples
}

#[cfg(test)]
mod query_schema_test {
    use uuid::Uuid;

    use crate::{model::tag::TagModel, schema::QueryMethod, util::{AddToQuery, PostgresCmp, SQLQueryBuilder}};

    use super::QueryTagSchema;

	const ID: Uuid = Uuid::nil();

	#[test]
	fn empty() {
		let schema = QueryTagSchema::default();

		let mut builder = SQLQueryBuilder::new();
		schema.add_to_query(&mut builder);
		builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &ID);

		let (statement, params) = builder.build_select();

		assert_eq!(statement.as_str(), "SELECT * FROM data.tags WHERE user_id = $1");
		assert_eq!(params.len(), 1);
	}

	#[test]
	fn full() {
		let mut schema = QueryTagSchema::default();
		schema.label = Some(QueryMethod::Match("Test Label".to_string()));
		schema.category = Some(QueryMethod::Match("Priority".to_string()));

		let mut builder = SQLQueryBuilder::new();
		schema.add_to_query(&mut builder);
		builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &ID);

		let (statement, params) = builder.build_select();

		assert_eq!(statement.as_str(), "SELECT * FROM data.tags WHERE tag_label LIKE %$1% AND category LIKE %$2% AND user_id = $3");
		assert_eq!(params.len(), 3);
	}

	#[test]
	fn limit() {
		let mut schema = QueryTagSchema::default();
		schema.label = Some(QueryMethod::Match("Test Label".to_string()));
		schema.category = Some(QueryMethod::Match("Priority".to_string()));

		let mut builder = SQLQueryBuilder::new();
		schema.add_to_query(&mut builder);
		builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &ID);
		builder.set_limit(25);

		let (statement, params) = builder.build_select();

		assert_eq!(statement.as_str(), "SELECT * FROM data.tags WHERE tag_label LIKE %$1% AND category LIKE %$2% AND user_id = $3 LIMIT 25");
		assert_eq!(params.len(), 3);
	}

	#[test]
	fn offset() {
		let mut schema = QueryTagSchema::default();
		schema.label = Some(QueryMethod::Match("Test Label".to_string()));
		schema.category = Some(QueryMethod::Match("Priority".to_string()));

		let mut builder = SQLQueryBuilder::new();
		schema.add_to_query(&mut builder);
		builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &ID);
		builder.set_offset(50);

		let (statement, params) = builder.build_select();

		assert_eq!(statement.as_str(), "SELECT * FROM data.tags WHERE tag_label LIKE %$1% AND category LIKE %$2% AND user_id = $3 OFFSET 50");
		assert_eq!(params.len(), 3);
	}

	#[test]
	fn limit_offset() {
		let mut schema = QueryTagSchema::default();
		schema.label = Some(QueryMethod::Match("Test Label".to_string()));
		schema.category = Some(QueryMethod::Match("Priority".to_string()));

		let mut builder = SQLQueryBuilder::new();
		schema.add_to_query(&mut builder);
		builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &ID);
		builder.set_limit(25);
		builder.set_offset(50);

		let (statement, params) = builder.build_select();

		assert_eq!(statement.as_str(), "SELECT * FROM data.tags WHERE tag_label LIKE %$1% AND category LIKE %$2% AND user_id = $3 LIMIT 25 OFFSET 50");
		assert_eq!(params.len(), 3);
	}

	#[test]
	fn return_some() {
		let mut schema = QueryTagSchema::default();
		schema.label = Some(QueryMethod::Match("Test Label".to_string()));
		schema.category = Some(QueryMethod::Match("Priority".to_string()));

		let mut builder = SQLQueryBuilder::new();
		schema.add_to_query(&mut builder);
		builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &ID);
		builder.set_return(vec![TagModel::ID]);

		let (statement, params) = builder.build_select();

		assert_eq!(statement.as_str(), "SELECT tag_id FROM data.tags WHERE tag_label LIKE %$1% AND category LIKE %$2% AND user_id = $3");
		assert_eq!(params.len(), 3);
	}

	#[test]
	fn return_all() {
		let mut schema = QueryTagSchema::default();
		schema.label = Some(QueryMethod::Match("Test Label".to_string()));
		schema.category = Some(QueryMethod::Match("Priority".to_string()));

		let mut builder = SQLQueryBuilder::new();
		schema.add_to_query(&mut builder);
		builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &ID);
		builder.set_return_all();

		let (statement, params) = builder.build_select();

		assert_eq!(statement.as_str(), "SELECT * FROM data.tags WHERE tag_label LIKE %$1% AND category LIKE %$2% AND user_id = $3");
		assert_eq!(params.len(), 3);
	}

    // TODO: make production examples
}
